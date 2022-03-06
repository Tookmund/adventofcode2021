use std::io;
//use std::io::prelude::*;
use std::io::Error;
use std::io::ErrorKind;

use log;

type Num = u64;

mod bits;
use bits::*;

#[derive(Debug)]
enum OperatorLength {
    TotalBits(usize),
    SubPackets(Num)
}

#[derive(Debug,PartialEq)]
enum PacketData {
    Literal(Num),
    Operator {
        id: Num,
        pd: Vec<Packet>,
    }
}

#[derive(Debug,PartialEq)]
struct Packet {
    version: Num,
    data: PacketData
}

impl Packet {
    fn from_bufread<B: io::BufRead>(bufread: B) -> io::Result<Self> {
        for line in bufread.lines() {
            let mut bits = Bits::new(&line?);
            log::debug!("RAW BITS: {:?}", bits);
            return Ok(Self::new(&mut bits));
        }
        Err(Error::new(ErrorKind::Other, "No Valid Packets!"))
    }
    fn new(bits: &mut Bits) -> Self {
        log::debug!("NEW PACKET!");
        Packet {
            version: bits.num(3),
            data:  match bits.num(3) {
                4 => Self::literal(bits),
                i => {
                    let size = match bits.bit() {
                        false => OperatorLength::TotalBits(bits.num(15)
                                                           .try_into()
                                                           .expect("Can't fit 15 bits into a usize???")),
                        true => OperatorLength::SubPackets(bits.num(11))
                    };
                    Self::operator(i, size, bits)
                }
            }
        }
    }
    fn literal(bits: &mut Bits) -> PacketData {
        let mut bv = Bv::new();
        loop {
            // If the first bit is 0, then we're done
            let cont = bits.bit();
            bv.extend_from_bitslice(bits.raw(4));
            if !cont {
                break;
            }
        }
        PacketData::Literal(bv.load_be())
    }
    fn operator(id: Num, size: OperatorLength, bits: &mut Bits) -> PacketData {
        let mut pd = Vec::new();
        match size {
            OperatorLength::TotalBits(tb) => {
                log::debug!("Operator: Total Bits: {}", tb);
                let start = bits.consumed();
                while (bits.consumed() - start) < tb {
                    pd.push(Packet::new(bits));
                    log::debug!("Bits Consumed: {}", bits.consumed() - start);
                }
            },
            OperatorLength::SubPackets(sp) => {
                log::debug!("Making {} subpackets", sp);
                for _ in 0..sp {
                    pd.push(Packet::new(bits));
                }
            }
        }
        PacketData::Operator { id, pd }
    }
    #[allow(dead_code)]
    fn total_packets(&self) -> Num {
        match &self.data {
            PacketData::Literal(_) => 1,
            PacketData::Operator { pd, .. } => 1 + pd.iter().map(|p| p.total_packets()).sum::<Num>()
        }
    }
    #[allow(dead_code)]
    fn version_sum(&self) -> Num {
        let mut vs = self.version;
        match &self.data {
            PacketData::Operator { pd, .. } => {
                for p in pd {
                    vs += p.version_sum();
                }
            },
            PacketData::Literal(_) => (),
        }
        vs
    }
    fn to_literal(&self) -> Num {
        match &self.data {
            PacketData::Literal(lpd) => *lpd,
            PacketData::Operator { id, pd } => {
                let mut m = pd.iter().map(|p| p.to_literal());
                match id {
                    0 => m.sum(),
                    1 => m.product(),
                    2 => m.min().expect("No Values In Operator for Min?"),
                    3 => m.max().expect("No Values In Operator for Max?"),
                    4 => panic!("Invalid Operator Packet Type, Should be Literal!"),
                    5 => if m.next() > m.next() { 1 } else { 0 },
                    6 => if m.next() < m.next() { 1 } else { 0 },
                    7 => if m.next() == m.next() { 1 } else { 0 },
                    i => panic!("Invalid Packet Type {}!", i)
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Packet;
    use crate::PacketData;

    macro_rules! make_tests {
        ( $func:expr, $( $name:ident, $input:expr, $output:expr ), +) => {
            $(
                #[test]
                fn $name () {
                    assert_eq!($func($input.as_bytes()).unwrap(), $output);
                }
             )*
        }
    }

    macro_rules! make_tests_method {
        ( $func:expr, $( $name:ident, $input:expr, $method:ident, $output:expr ), +) => {
            $(
                #[test]
                fn $name () {
                    assert_eq!($func($input.as_bytes()).unwrap().$method(), $output);
                }
             )*
        }
    }

    make_tests! {
        Packet::from_bufread,
            literal,
                "D2FE28",
                Packet {
                    version: 6,
                    data: PacketData::Literal(2021)
                },
            totalbits_operator,
                "38006F45291200",
                Packet {
                    version: 1,
                    data: PacketData::Operator {
                        id: 6,
                        pd: vec![
                            Packet {
                                version: 6,
                                data: PacketData::Literal(10)
                            },
                            Packet {
                                version: 2,
                                data: PacketData::Literal(20)
                            }
                        ]
                    }
                },
            subpacket_operator,
                "EE00D40C823060",
                Packet {
                    version: 7,
                    data: PacketData::Operator {
                        id: 3,
                        pd: vec![
                            Packet {
                                version: 2,
                                data: PacketData::Literal(1),
                            },
                            Packet {
                                version: 4,
                                data: PacketData::Literal(2),
                            },
                            Packet {
                                version: 1,
                                data: PacketData::Literal(3)
                            }
                        ]
                    }
                }
    }

    make_tests_method! {
        Packet::from_bufread,
            version_sum_packet_packet_packet,
                "8A004A801A8002F478", version_sum, 16,
            version_sum_2_subpackets,
                "620080001611562C8802118E34", version_sum, 12,
            version_sum_2_subpackets_totalbits,
                "C0015000016115A2E0802F182340", version_sum, 23,
            version_sum_5_subpackets_countsubpackets,
                "A0016C880162017C3686B18A3D4780", version_sum, 31,
            literal_sum,
                "C200B40A82", to_literal, 3,
            literal_product,
                "04005AC33890", to_literal, 54,
            literal_min,
                "880086C3E88112", to_literal, 7,
            literal_max,
                "CE00C43D881120", to_literal, 9,
            literal_less,
                "D8005AC2A8F0", to_literal, 1,
            literal_greater,
                "F600BC2D8F", to_literal, 0,
            literal_equal,
                "9C005AC2F8F0", to_literal, 0,
            literal_sum_product_equal,
                "9C0141080250320F1802104A08", to_literal, 1
    }
}

fn main() -> io::Result<()>{
    env_logger::init();
    //let mut stdin = Vec::new();
    //io::stdin().read_to_end(&mut stdin)?;
    let packet = Packet::from_bufread(io::stdin().lock())?;
    log::debug!("BITS: {:?}", packet);
    println!("Packet Version Sum: {}", packet.version_sum());
    println!("Packet Literal: {}", packet.to_literal());
    Ok(())
}
