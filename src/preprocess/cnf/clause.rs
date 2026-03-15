use crate::preprocess::cnf::lit::Lit;

pub type ClauseID = u32;

#[derive(Debug, PartialEq, Clone)]
pub struct Clause {
    pub lits: Vec<Lit>,
    pub sig: u64,
    pub active: bool,
}

impl Clause {
    pub fn new(lits: Vec<Lit>) -> Self {
        Clause {
            sig: lits.iter().fold(0, |acc, &lit| acc | (1 << lit.hash())),
            lits,
            active: true,
        }
    }
}
