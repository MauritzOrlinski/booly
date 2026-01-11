pub mod dlcs;
pub mod dlis;
pub mod from_shortest_clause;
pub mod jeroslaw_wang;
pub mod mom;
pub mod trivial;
use crate::cnf::cnf_formula::CnfFormula;
use crate::dpll::assignment::Assignment;

pub trait Heuristic {
    fn chose_next_assignment(&mut self, cnf_formula: &CnfFormula) -> Assignment;
}
