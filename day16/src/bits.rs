pub use bitvec::prelude::*;
use log;

type Endian = Msb0;
type BitNum = u32;
pub type Bv = BitVec<BitNum, Endian>;
pub type Bs = BitSlice<BitNum, Endian>;


use crate::Num;

#[derive(Debug)]
pub struct Bits {
    bv: Bv,
    i: usize,
}

impl Bits {
    pub fn new(buf: &str) -> Self {
        let mut b = Bits {
            bv: Bv::new(),
            i: 0
        };
        for c in buf.as_bytes().chunks(2) {
            let hex = std::str::from_utf8(c).expect("Invalid UTF-8?");
            let num = u8::from_str_radix(hex, 16)
                .expect("Input not valid hex!");
            let nbs = num.view_bits::<Endian>();
            log::debug!("Hex: {} Num: {} Bits: {}", hex, num, nbs);
            b.bv.extend_from_bitslice(nbs);
        }
        b
    }
    pub fn raw(&mut self, n: usize) -> &Bs {
        let ret = &self.bv[self.i..self.i+n];
        log::debug!("RAW: {}", ret);
        self.i += n;
        ret
    }
    pub fn num(&mut self, n: usize) -> Num {
        self.raw(n).load_be()
    }
    pub fn bit(&mut self) -> bool {
        let c = self.i;
        self.i += 1;
        log::debug!("BIT: {}", self.bv[c]);
        self.bv[c]
    }
    pub fn consumed(&self) -> usize {
        self.i
    }
    pub fn back(&mut self, n: usize) {
        self.i -= n;
    }
    pub fn forward(&mut self, n: usize) {
        self.i += n;
    }
}
