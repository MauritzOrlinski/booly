use crate::preprocess::cnf::var::VarId;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Lit(i32);

#[inline]
fn mix32(mut x: u32) -> u32 {
    // MurmurHash3 fmix32
    x ^= x >> 16;
    x = x.wrapping_mul(0x85eb_ca6b);
    x ^= x >> 13;
    x = x.wrapping_mul(0xc2b2_ae35);
    x ^= x >> 16;
    x
}

impl Lit {
    pub fn new(lit: i32) -> Lit {
        Lit(lit)
    }

    pub fn neg(self) -> Lit {
        Lit::new(-self.0)
    }

    pub fn pos(self) -> bool {
        match self.0.signum() {
            1 => true,
            -1 => false,
            _ => unreachable!("variable with id = 0 found!"),
        }
    }

    #[inline]
    pub fn hash1(self) -> u8 {
        (mix32(self.0 as u32) & 0x7f) as u8
    }

    pub fn var_id(self) -> VarId {
        self.0.unsigned_abs()
    }
}
