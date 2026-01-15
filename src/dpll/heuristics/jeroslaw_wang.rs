use std::cmp::max_by;

use crate::cnf::cnf_formula::CnfFormula;
use crate::dpll::assignment::Assignment;
use crate::dpll::heuristics::{Heuristic, Stats};

#[derive(Debug)]
pub struct JeroslawWang;

impl Heuristic for JeroslawWang {
    fn chose_next_assignment(&mut self, cnf_formula: &CnfFormula, _: i32) -> Assignment {
        let j = |occ: &Vec<usize>| {
            occ.iter()
                .filter_map(|&clause_id| {
                    let clause = cnf_formula.clauses.get(clause_id).unwrap();
                    match clause.satisfied_by {
                        None => None,
                        _ => Some(2f64.powf(-(clause.unassigned_variables as f64))),
                    }
                })
                .sum::<f64>()
        };

        let (variable_id, _, (_, assignment_value)) = cnf_formula
            .variables
            .iter()
            .enumerate()
            .filter(|(_, v)| v.value.is_none())
            .map(|(vid, v)| {
                (
                    vid,
                    v,
                    max_by(
                        (j(&v.positive_occurrences), true),
                        (j(&v.negative_occurrences), false),
                        |a, b| a.partial_cmp(b).unwrap(),
                    ),
                )
            })
            .max_by(|(_, _, a), (_, _, b)| a.partial_cmp(b).unwrap())
            .unwrap();

        Assignment::new(variable_id as u32 + 1, assignment_value)
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
