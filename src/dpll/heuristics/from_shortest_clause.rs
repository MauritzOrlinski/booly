use crate::dpll::assignment::Assignment;
use crate::dpll::assignment::AssignmentReason::Branching;
use crate::cnf::cnf_formula::CnfFormula;
use crate::dpll::heuristics::heuristic::Heuristic;

pub struct FromShortestClause;

impl Heuristic for FromShortestClause {
    fn chose_next_assignment(cnf_formula: &CnfFormula) -> Assignment {
        let shortest_clause = cnf_formula
            .clauses
            .iter()
            .filter(|clause| matches!(clause.satisfied_by, None))
            .min_by(|a, b| (&a.unassigned_variables).cmp(&b.unassigned_variables))
            .unwrap();

        let (variable_id, assignment_value) = shortest_clause
            .literals
            .iter()
            .find_map(|(variable_id, literal_polarity)| {
                let variable = cnf_formula.variables.get(*variable_id);
                if matches!(variable.value, None) {
                    return Some((*variable_id, literal_polarity.get_satisfying_assignment()));
                }
                None
            })
            .unwrap();

        Assignment::new(variable_id, assignment_value, Branching)
    }
}