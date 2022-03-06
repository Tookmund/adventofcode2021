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
    Operator(Vec<Packet>),
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
                _ => {
                    let size = match bits.bit() {
                        false => OperatorLength::TotalBits(bits.num(15)
                                                           .try_into()
                                                           .expect("Can't fit 15 bits into a usize???")),
                        true => OperatorLength::SubPackets(bits.num(11))
                    };
                    Self::operator(size, bits)
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
    fn operator(size: OperatorLength, bits: &mut Bits) -> PacketData {
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
        PacketData::Operator(pd)
    }
    #[allow(dead_code)]
    fn total_packets(&self) -> Num {
        match &self.data {
            PacketData::Literal(_) => 1,
            PacketData::Operator(pd) => 1 + pd.iter().map(|p| p.total_packets()).sum::<Num>()
        }
    }
    #[allow(dead_code)]
    fn version_sum(&self) -> Num {
        let mut vs = self.version;
        match &self.data {
            PacketData::Operator(pd) => {
                for p in pd {
                    vs += p.version_sum();
                }
            },
            PacketData::Literal(_) => (),
        }
        vs
    }
}

#[cfg(test)]
mod test {
    use crate::Packet;
    use crate::PacketData;

    #[test]
    fn test_literal() {
        assert_eq!(Packet::from_bufread(&b"D2FE28"[..]).unwrap(),
        Packet {
            version: 6,
            data: PacketData::Literal(2021),
        })
    }

    #[test]
    fn test_totalbits_operator() {
        assert_eq!(Packet::from_bufread(&b"38006F45291200"[..]).unwrap(),
        Packet {
            version: 1,
            data: PacketData::Operator(
                vec![
                    Packet {
                        version: 6,
                        data: PacketData::Literal(10)
                    },
                    Packet {
                        version: 2,
                        data: PacketData::Literal(20)
                    }
                ]
            )
        })
    }

    #[test]
    fn test_subpacket_operator() {
        assert_eq!(Packet::from_bufread(&b"EE00D40C823060"[..]).unwrap(),
        Packet {
            version: 7,
            data: PacketData::Operator(
                vec![
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
            )
        })
    }

    #[test]
    fn test_version_sum_packet_packet_packet() {
        assert_eq!(Packet::from_bufread(&b"8A004A801A8002F478"[..]).unwrap().version_sum(), 16);
    }

    #[test]
    fn test_version_sum_2_subpackets() {
        assert_eq!(Packet::from_bufread(&b"620080001611562C8802118E34"[..]).unwrap().version_sum(), 12);
    }

    #[test]
    fn test_version_sum_2_subpackets_totalbits() {
        assert_eq!(Packet::from_bufread(&b"C0015000016115A2E0802F182340"[..]).unwrap().version_sum(), 23);
    }

    #[test]
    fn test_version_sum_5_subpackets_countsubpackets() {
        assert_eq!(Packet::from_bufread(&b"A0016C880162017C3686B18A3D4780"[..]).unwrap().version_sum(), 31);
    }
}

fn main() -> io::Result<()>{
    env_logger::init();
    //let mut stdin = Vec::new();
    //io::stdin().read_to_end(&mut stdin)?;
    let packet = Packet::from_bufread(io::stdin().lock())?;
    log::debug!("BITS: {:?}", packet);
    println!("Packet Version Sum: {}", packet.version_sum());
    Ok(())
}
