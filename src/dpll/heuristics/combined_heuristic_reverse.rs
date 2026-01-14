use crate::cnf::cnf_formula::CnfFormula;
use crate::dpll::assignment::Assignment;
use crate::dpll::heuristics::Heuristic;

#[derive(Debug)]
pub struct CombinedHeuristicReverse {
    expensive: Box<dyn Heuristic>,
    cheap: Box<dyn Heuristic>,
    threshold: i32,
}

impl Heuristic for CombinedHeuristicReverse {
    fn chose_next_assignment(
        &mut self,
        cnf_formula: &CnfFormula,
        decision_level: i32,
    ) -> Assignment {
        if decision_level < cnf_formula.variables.len() as i32 - self.threshold {
            self.expensive
                .chose_next_assignment(cnf_formula, decision_level)
        } else {
            self.cheap
                .chose_next_assignment(cnf_formula, decision_level)
        }
    }
}

impl CombinedHeuristicReverse {
    pub fn new(expensive: Box<dyn Heuristic>, cheap: Box<dyn Heuristic>, threshold: i32) -> Self {
        Self {
            expensive,
            cheap,
            threshold,
        }
    }
}
