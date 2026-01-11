use std::cmp::max;

use crate::cnf::cnf_formula::CnfFormula;
use crate::dpll::assignment::Assignment;
use crate::dpll::heuristics::Heuristic;

#[derive(Debug)]
pub struct DLIS;

impl Heuristic for DLIS {
    fn chose_next_assignment(&mut self, cnf_formula: &CnfFormula) -> Assignment {
        let count_unsat = |occ: &Vec<usize>| {
            occ.iter()
                .filter(|&&clause_id| {
                    cnf_formula
                        .clauses
                        .get(clause_id)
                        .unwrap()
                        .satisfied_by
                        .is_none()
                })
                .count()
        };

        let (variable_id, variable) = cnf_formula
            .variables
            .iter()
            .enumerate()
            .filter(|(_, v)| v.value.is_none())
            .max_by_key(|(_, v)| {
                max(
                    count_unsat(&v.positive_occurrences),
                    count_unsat(&v.negative_occurrences),
                )
            })
            .unwrap();

        let assignment_value = count_unsat(&variable.positive_occurrences)
            >= count_unsat(&variable.negative_occurrences);

        Assignment::new(variable_id as u32 + 1, assignment_value)
    }
}
