use crate::cnf::cnf_formula::CnfFormula;
use crate::dpll::assignment::Assignment;
use crate::dpll::heuristics::{Heuristic, Stats};

#[derive(Debug)]
pub struct CombinedHeuristic {
    expensive: Box<dyn Heuristic>,
    cheap: Box<dyn Heuristic>,
    threshold: i32,
}

impl Heuristic for CombinedHeuristic {
    fn chose_next_assignment(
        &mut self,
        cnf_formula: &CnfFormula,
        decision_level: i32,
    ) -> Assignment {
        if decision_level < self.threshold {
            self.expensive
                .chose_next_assignment(cnf_formula, decision_level)
        } else {
            self.cheap
                .chose_next_assignment(cnf_formula, decision_level)
        }
    }

    #[allow(unused_variables)]
    fn feedback(&mut self, feedback: Stats) {}

    fn is_learning(&self) -> bool {
        false
    }
    #[allow(unused_variables)]
    fn save(&self, path: &str) -> std::io::Result<()> {
        Result::Ok(())
    }
}

impl CombinedHeuristic {
    pub fn new(expensive: Box<dyn Heuristic>, cheap: Box<dyn Heuristic>, threshold: i32) -> Self {
        Self {
            expensive,
            cheap,
            threshold,
        }
    }
}
