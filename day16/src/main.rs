use std::io;
use std::io::prelude::*;
use std::io::Error;
use std::io::ErrorKind;

use bitvec::prelude::*;

type Num = u32;
type Endian = Msb0;
type Bv = BitVec<u32, Endian>;
type Bs = BitSlice<u32, Endian>;

#[derive(Debug)]
struct Bits {
    bv: Bv,
    i: usize,
}

impl Bits {
    fn new(n: Num) -> Self {
        let mut ret = Bits {
            bv: n.view_bits().to_bitvec(),
            i: 0
        };
        // Find first set bit in num
        for x in &ret.bv {
            println!("{}", x);
            if *x {
                break;
            }
            else {
                ret.i += 1;
            }
        }
        ret
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
    fn back(&mut self, n: usize) {
        self.i -= n;
    }
    fn forward(&mut self, n: usize) {
        self.i += n;
    }
}

#[derive(Debug)]
enum OperatorLength {
    TotalBits(Num),
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
            let num = Num::from_str_radix(&line?, 16).expect("Input not valid hex!");
            let mut bits = Bits::new(num);
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
                        false => OperatorLength::TotalBits(bits.num(15)),
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
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use crate::Packet;
    use crate::PacketData;

    const EXAMPLE: &[u8] = b"D2FE28";
    const OPERATOR: &[u8] = b"EE00D40C823060";

    #[test]
    fn test_example() {
        assert_eq!(Packet::from_bufread(EXAMPLE).unwrap(), Packet {
            version: 6,
            data: PacketData::Literal(2021),
        })
    }
}

fn main() -> io::Result<()>{
    //let mut stdin = Vec::new();
    //io::stdin().read_to_end(&mut stdin)?;
    println!("BITS: {:?}", Packet::from_bufread(io::stdin().lock())?);
    Ok(())
}
