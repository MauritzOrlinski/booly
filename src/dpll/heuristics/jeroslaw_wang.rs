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

        let (variable_id_pos, value_pos) = cnf_formula
            .variables
            .iter()
            .enumerate()
            .filter(|(_, v)| v.value.is_none())
            .map(|(vid, v)| (vid, j(&v.positive_occurrences)))
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();

        let (variable_id_neg, value_neg) = cnf_formula
            .variables
            .iter()
            .enumerate()
            .filter(|(_, v)| v.value.is_none())
            .map(|(vid, v)| (vid, j(&v.negative_occurrences)))
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();

        let assignment_value = value_pos >= value_neg;
        let variable_id = if assignment_value {
            variable_id_pos
        } else {
            variable_id_neg
        };

        Assignment::new(variable_id as u32 + 1, assignment_value)
    }
}
