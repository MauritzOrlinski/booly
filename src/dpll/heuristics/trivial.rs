use crate::cnf::cnf_formula::CnfFormula;
use crate::dpll::assignment::Assignment;
use crate::dpll::heuristics::{Heuristic, Stats};

#[derive(Debug)]
pub struct Trivial;

impl Heuristic for Trivial {
    fn chose_next_assignment(&mut self, cnf_formula: &CnfFormula, _: i32) -> Assignment {
        // let unassigned_clause = cnf_formula
        //     .clauses
        //     .iter()
        //     .find_map(|clause| {
        //         if !clause.is_satisfied_by_watched(&cnf_formula.assignments) {
        //             Some(clause)
        //         } else {
        //             None
        //         }
        //     })
        //     .unwrap();
        // let (variable_id, assignment_value) = (
        //     unassigned_clause.watched1.unsigned_abs(),
        //     unassigned_clause.watched1.is_positive(),
        // );
        // .literals
        // .iter()
        // .find_map(|(variable_id, polarity)| {
        //     if cnf_formula.variables.get(variable_id).value.is_none() {
        //         Some((variable_id, polarity.get_satisfying_assignment()))
        //     } else {
        //         None
        //     }
        // })
        // .unwrap();

        Assignment::new(
            cnf_formula
                .assignments
                .iter()
                .position(|x| x.is_none())
                .unwrap() as u32
                + 1,
            true,
        )
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
