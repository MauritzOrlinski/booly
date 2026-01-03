pub(crate) mod dlcs;
pub(crate) mod dlis;
pub(crate) mod from_shortest_clause;
pub(crate) mod mom;
pub(crate) mod trivial;
use crate::cnf::cnf_formula::CnfFormula;
use crate::dpll::assignment::Assignment;

pub trait Heuristic {
    fn chose_next_assignment(&mut self, cnf_formula: &CnfFormula) -> Assignment;
}
