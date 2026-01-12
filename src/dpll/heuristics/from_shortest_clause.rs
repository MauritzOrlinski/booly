use crate::cnf::cnf_formula::CnfFormula;
use crate::dpll::assignment::Assignment;
use crate::dpll::heuristics::Heuristic;

#[derive(Debug)]
pub struct FromShortestClause;

impl Heuristic for FromShortestClause {
    fn chose_next_assignment(&mut self, cnf_formula: &CnfFormula, _ : i32) -> Assignment {
        let shortest_clause = cnf_formula
            .clauses
            .iter()
            .filter(|clause| clause.satisfied_by.is_none())
            .min_by_key(|clause| clause.unassigned_variables)
            .unwrap();

        let (variable_id, assignment_value) = shortest_clause
            .literals
            .iter()
            .find_map(|(variable_id, literal_polarity)| {
                let variable = cnf_formula.variables.get(variable_id);
                if variable.value.is_none() {
                    return Some((variable_id, literal_polarity.get_satisfying_assignment()));
                }
                None
            })
            .unwrap();

        Assignment::new(variable_id, assignment_value)
    }
}
