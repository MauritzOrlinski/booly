use crate::cnf::cnf_formula::CnfFormula;
use crate::dpll::assignment::Assignment;
use crate::dpll::heuristics::{Heuristic, Stats};

#[derive(Debug)]
pub struct Trivial;

impl Heuristic for Trivial {
    fn chose_next_assignment(&mut self, cnf_formula: &CnfFormula, _: i32) -> Assignment {
        Assignment::new(cnf_formula.variables.find_unassigned(), true)
    }

    fn is_learning(&self) -> bool {
        false
    }
    #[allow(unused_variables)]
    fn feedback(&mut self, feedback: Stats) {}

    #[allow(unused_variables)]
    fn save(&self, path: &str) -> std::io::Result<()> {
        Result::Ok(())
    }
}
