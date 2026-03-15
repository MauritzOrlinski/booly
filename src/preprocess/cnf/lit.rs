use crate::preprocess::cnf::var::VarId;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Lit(i32);

impl Lit {
    pub fn new(lit: i32) -> Lit {
        Lit(lit)
    }

    pub fn not(self) -> Lit {
        Lit::new(-self.0)
    }

    pub fn pos(self) -> bool {
        match self.0.signum() {
            1 => true,
            -1 => false,
            _ => unreachable!("variable with id = 0 found!"),
        }
    }

    pub fn hash(self) -> u8 {
        self.0 as u8 & 0b111111
    }

    pub fn var_id(self) -> VarId {
        self.0.unsigned_abs()
    }
}
