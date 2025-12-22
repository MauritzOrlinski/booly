use std::cmp::max;

use crate::cnf::cnf_formula::CnfFormula;
use crate::dpll::assignment::{Assignment, AssignmentValue};
use crate::dpll::heuristics::heuristic::Heuristic;

pub struct DLIS;

impl Heuristic for DLIS {
    fn chose_next_assignment(cnf_formula: &CnfFormula) -> Assignment {
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

        let assignment_value =
            if variable.positive_occurrences.len() >= variable.negative_occurrences.len() {
                AssignmentValue::True
            } else {
                AssignmentValue::False
            };

        Assignment::new(variable_id + 1, assignment_value)
    }
}
