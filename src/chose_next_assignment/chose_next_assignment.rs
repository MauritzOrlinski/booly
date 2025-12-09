use crate::assignment::assignment::Assignment;
use crate::cnf::cnf_formula::CnfFormula;

pub trait ChooseNextAssignment {
    fn choose_next_assignment(cnf_formula: CnfFormula) -> Vec<Assignment>;
}