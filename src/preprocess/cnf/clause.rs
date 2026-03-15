use crate::preprocess::cnf::lit::Lit;

pub type ClauseID = u32;

#[derive(Debug, PartialEq, Clone)]
pub struct Clause {
    pub lits: Vec<Lit>,
    pub sig: u128,
    pub active: bool,
}

impl Clause {
    pub fn new(lits: Vec<Lit>) -> Self {
        Clause {
            sig: lits.iter().fold(0u128, |acc, &lit| {
                let h1 = lit.hash1() as u32;
                acc | (1u128 << h1)
            }),
            lits,
            active: true,
        }
    }
}
