use crate::cnf::cnf_formula::CnfFormula;
use crate::dpll::assignment::Assignment;
use crate::dpll::heuristics::Heuristic;

#[derive(Clone, Debug)]
pub struct DLCS1;

impl Heuristic for DLCS1 {
    fn chose_next_assignment(&mut self, cnf_formula: &CnfFormula) -> Assignment {
        let (variable_id, variable) = cnf_formula
            .variables
            .iter()
            .enumerate()
            .filter(|(_, v)| v.value.is_none())
            .max_by_key(|(_, v)| v.positive_occurrences.len() + v.negative_occurrences.len())
            .unwrap();

        let assignment_value =
            variable.positive_occurrences.len() >= variable.negative_occurrences.len();

        Assignment::new(variable_id as u32 + 1, assignment_value)
    }
}
