use crate::cnf::cnf_formula::CnfFormula;
use crate::cdcl::assignment::Assignment;
use crate::cdcl::heuristics::{Heuristic};

#[derive(Debug)]
pub struct Trivial;

impl Heuristic for Trivial {
    fn chose_next_assignment(&mut self, cnf_formula: &CnfFormula, _: i32) -> Assignment {
        let unassigned_clause = cnf_formula
            .clauses
            .iter()
            .find(|(_, clause)| !clause.is_satisfied_by_watched(&cnf_formula.assignments));
        match unassigned_clause {
            Some((_, unassigned_clause)) => {
                let (variable_id, assignment_value) = (
                    unassigned_clause.watched1.unsigned_abs(),
                    unassigned_clause.watched1.is_positive(),
                );

                Assignment::new(variable_id, assignment_value, None)
            }
            None => Assignment::new(
                cnf_formula
                    .assignments
                    .iter()
                    .position(|x| x.is_none())
                    .unwrap() as u32
                    + 1,
                true,
                None
            ),
        }
    }
}
