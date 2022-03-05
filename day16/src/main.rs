use std::io;
use std::io::prelude::*;
use std::io::Error;
use std::io::ErrorKind;

use bitvec::prelude::*;

type Num = u32;
type Endian = Msb0;
type BitNum = u64;
type Bv = BitVec<BitNum, Endian>;
type Bs = BitSlice<BitNum, Endian>;

#[derive(Debug)]
struct Bits {
    bv: Bv,
    i: usize,
}

impl Bits {
    fn new(buf: &str) -> Self {
        let num = BitNum::from_str_radix(buf, 16).expect("Input not valid hex!");
        Bits {
            bv: num.view_bits().to_bitvec(),
            i: 64 - (buf.len()*4)
        }
    }
    fn raw(&mut self, n: usize) -> &Bs {
        let ret = &self.bv[self.i..self.i+n];
        println!("RAW: {}", ret);
        self.i += n;
        ret
    }
    fn num(&mut self, n: usize) -> Num {
        self.raw(n).load()
    }
    fn bit(&mut self) -> bool {
        let c = self.i;
        self.i += 1;
        self.bv[c]
    }
    fn consumed(&self) -> usize {
        self.i
    }
    fn back(&mut self, n: usize) {
        self.i -= n;
    }
    fn forward(&mut self, n: usize) {
        self.i += n;
    }
}

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
            println!("RAW BITS: {:?}", bits);
            return Ok(Self::new(&mut bits));
        }
        Err(Error::new(ErrorKind::Other, "No Valid Packets!"))
    }
    fn new(bits: &mut Bits) -> Self {
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
        let mut bv: Bv = BitVec::new();
        loop {
            // If the first bit is 0, then we're done
            let cont = bits.bit();
            bv.extend_from_bitslice(&bits.raw(4));
            if !cont {
                break;
            }
        }
        PacketData::Literal(bv.load())
    }
    fn operator(size: OperatorLength, bits: &mut Bits) -> PacketData {
        let mut pd = Vec::new();
        match size {
            OperatorLength::TotalBits(tb) => {
                let start = bits.consumed();
                while  (bits.consumed() - start) < tb {
                    pd.push(Packet::new(bits));
                    println!("Bits Consumed: {}", bits.consumed() - start);
                }
            }
            OperatorLength::SubPackets(sp) => {
                for _ in 0..sp {
                    pd.push(Packet::new(bits));
                }
            },
        }
        PacketData::Operator(pd)
    }
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
    fn test_version_sum() {
        assert_eq!(Packet::from_bufread(&b"8A004A801A8002F478"[..]).unwrap().version_sum(), 16);
        assert_eq!(Packet::from_bufread(&b"620080001611562C8802118E34"[..]).unwrap().version_sum(), 12);
        assert_eq!(Packet::from_bufread(&b"C0015000016115A2E0802F182340"[..]).unwrap().version_sum(), 23);
        assert_eq!(Packet::from_bufread(&b"A0016C880162017C3686B18A3D4780"[..]).unwrap().version_sum(), 31);
    }
}

fn main() -> io::Result<()>{
    //let mut stdin = Vec::new();
    //io::stdin().read_to_end(&mut stdin)?;
    println!("BITS: {:?}", Packet::from_bufread(io::stdin().lock())?);
    Ok(())
}
