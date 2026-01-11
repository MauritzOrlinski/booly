use std::cmp::max_by;

use crate::cnf::cnf_formula::CnfFormula;
use crate::dpll::assignment::Assignment;
use crate::dpll::heuristics::Heuristic;

pub struct JeroslawWang;

impl Heuristic for JeroslawWang {
    fn chose_next_assignment(&mut self, cnf_formula: &CnfFormula) -> Assignment {
        let j = |occ: &Vec<usize>| {
            occ.iter()
                .filter_map(|&clause_id| {
                    let clause = cnf_formula.clauses.get(clause_id).unwrap();
                    match clause.satisfied_by {
                        None => None,
                        _ => Some(1.0 / (1u64 << clause.unassigned_variables) as f64),
                    }
                })
                .sum::<f64>()
        };

        let (variable_id, variable, _) = cnf_formula
            .variables
            .iter()
            .enumerate()
            .filter(|(_, v)| v.value.is_none())
            .map(|(vid, v)| {
                (
                    vid,
                    v,
                    max_by(
                        j(&v.positive_occurrences),
                        j(&v.negative_occurrences),
                        |a, b| a.partial_cmp(b).unwrap(),
                    ),
                )
            })
            .max_by(|(_, _, a), (_, _, b)| a.partial_cmp(b).unwrap())
            .unwrap();

        Assignment::new(
            variable_id as u32 + 1,
            j(&variable.positive_occurrences) >= j(&variable.negative_occurrences),
        )
    }
}
