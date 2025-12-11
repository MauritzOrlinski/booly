use crate::assignment::assignments::Assignments;
use crate::assignment::single_assignment::SingleAssignment;
use crate::cnf::cnf_formula::CnfFormula;

pub type MultiAssignment = Vec<SingleAssignment>;

pub trait Branching {
    fn chose_branches(cnf_formula: &CnfFormula) -> Vec<Assignments>;
}