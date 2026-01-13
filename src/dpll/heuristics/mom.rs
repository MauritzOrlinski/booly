use crate::cnf::cnf_formula::CnfFormula;
use crate::cnf::literals::Polarity;
use crate::dpll::assignment::Assignment;
use crate::dpll::heuristics::Heuristic;

#[derive(Debug)]
pub struct MOM;

impl Heuristic for MOM {
    fn chose_next_assignment(&mut self, cnf_formula: &CnfFormula, _: i32) -> Assignment {
        let alpha: u32 = 10;

        let min_width = cnf_formula
            .clauses
            .iter()
            .filter(|c| c.satisfied_by.is_none())
            .min_by_key(|c| c.unassigned_variables)
            .unwrap()
            .unassigned_variables;

        let clauses = cnf_formula.clauses.iter().filter_map(|c| {
            if c.satisfied_by.is_none() && c.unassigned_variables == min_width {
                Some(&c.literals)
            } else {
                None
            }
        });

        let mut vars: Vec<(usize, usize)> = vec![(0, 0); cnf_formula.variables.len()];
        let unassigned_variables = cnf_formula.variables.find_all_unassigned();
        for l in clauses {
            l.iter().for_each(|(variable_id, polarity)| {
                if unassigned_variables.contains(&variable_id) {
                    if polarity == Polarity::Positive {
                        vars[variable_id as usize - 1].0 += 1;
                    } else {
                        vars[variable_id as usize - 1].1 += 1;
                    }
                }
            });
        }

        let (variable_id, (h_pos, h_neg)) = vars
            .iter()
            .enumerate()
            .max_by_key(|(_, (h_pos, h_neg))| (h_pos + h_neg) * (2 << alpha) + h_pos * h_neg)
            .unwrap();

        Assignment::new(variable_id as u32 + 1, *h_pos >= *h_neg)
    }
}
