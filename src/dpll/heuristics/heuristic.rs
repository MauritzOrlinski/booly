use crate::dpll::assignment::Assignment;
use crate::cnf::cnf_formula::CnfFormula;

pub trait Heuristic {
    fn chose_next_assignment(cnf_formula: &CnfFormula) -> Assignment;
}
