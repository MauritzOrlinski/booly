use crate::cnf::assignment::Assignment;
use crate::cnf::assignment::AssignmentReason::Branching;
use crate::cnf::assignment::AssignmentValue::True;
use crate::cnf::cnf_formula::CnfFormula;
use crate::dpll::heuristics::heuristic::Heuristic;

pub struct Trivial;

impl Heuristic for Trivial {
    fn chose_next_assignment(cnf_formula: &CnfFormula) -> Assignment {
        Assignment::new(cnf_formula.variables.find_unassigned(), True, Branching)
    }
}