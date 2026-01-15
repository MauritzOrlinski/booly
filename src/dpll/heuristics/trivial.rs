use crate::cnf::cnf_formula::CnfFormula;
use crate::dpll::assignment::Assignment;
use crate::dpll::heuristics::Heuristic;

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
}
