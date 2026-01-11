pub mod dlcs;
pub mod dlcs1;
pub mod dlis;
pub mod dlis1;
pub mod from_shortest_clause;
pub mod mom;
pub mod trivial;

use std::fmt::Debug;
use crate::cnf::cnf_formula::CnfFormula;
use crate::dpll::assignment::Assignment;

pub trait Heuristic: Debug {
    fn chose_next_assignment(&mut self, cnf_formula: &CnfFormula) -> Assignment;
}
