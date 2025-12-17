use crate::cnf::cnf_formula::CnfFormula;
use crate::dpll::assignment::Assignment;

pub trait Heuristic {
    fn chose_next_assignment(cnf_formula: &CnfFormula) -> Assignment;
}
