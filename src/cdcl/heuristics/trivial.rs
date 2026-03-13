use crate::cnf::cnf_formula::CnfFormula;
use crate::cdcl::assignment::Assignment;
use crate::cdcl::heuristics::{Heuristic, Stats};

#[derive(Debug)]
pub struct Trivial;

impl Heuristic for Trivial {
    fn chose_next_assignment(&mut self, cnf_formula: &CnfFormula, _: i32) -> Assignment {
        let unassigned_clause = cnf_formula
            .clauses
            .iter()
            .find(|clause| !clause.is_satisfied_by_watched(&cnf_formula.assignments));
        match unassigned_clause {
            Some(unassigned_clause) => {
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
