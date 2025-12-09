use crate::assignment::assignment::Assignment;
use crate::cnf::cnf_formula::CnfFormula;

pub type MultiAssignment = Vec<Assignment>;

pub trait Branching {
    fn chose_branches(cnf_formula: &CnfFormula) -> Vec<MultiAssignment>;
}