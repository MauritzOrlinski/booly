use crate::dpll::assignment::Assignment;
use crate::dpll::assignment::AssignmentReason::Branching;
use crate::dpll::assignment::AssignmentValue::True;
use crate::cnf::cnf_formula::CnfFormula;
use crate::dpll::heuristics::heuristic::Heuristic;

pub struct Trivial;

impl Heuristic for Trivial {
    fn chose_next_assignment(cnf_formula: &CnfFormula) -> Assignment {
        Assignment::new(cnf_formula.variables.find_unassigned(), True, Branching)
    }
}