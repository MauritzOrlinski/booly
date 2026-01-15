use crate::cnf::cnf_formula::CnfFormula;
use crate::dpll::assignment::Assignment;
use crate::dpll::heuristics::{Heuristic, Stats};

#[derive(Debug)]
pub struct Trivial;

impl Heuristic for Trivial {
    fn chose_next_assignment(&mut self, cnf_formula: &CnfFormula, _: i32) -> Assignment {
        let unassigned_clause = cnf_formula
            .clauses
            .iter()
            .find_map(|clause| {
                if clause.satisfied_by.is_none() {
                    Some(clause)
                } else {
                    None
                }
            })
            .unwrap();
        let (variable_id, assignment_value) = unassigned_clause
            .literals
            .iter()
            .find_map(|(variable_id, polarity)| {
                if cnf_formula.variables.get(variable_id).value.is_none() {
                    Some((variable_id, polarity.get_satisfying_assignment()))
                } else {
                    None
                }
            })
            .unwrap();

        Assignment::new(variable_id, assignment_value)
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
