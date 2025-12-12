use crate::assignment::single_assignment::{SingleAssignment};
use crate::assignment::single_assignment::AssignmentReason::Branching;
use crate::cnf::cnf_formula::CnfFormula;

pub trait ChooseNextVariable {
    fn chose(cnf_formula: &CnfFormula) -> SingleAssignment;
}

pub struct TrivialChooseNextVariable;

impl ChooseNextVariable for TrivialChooseNextVariable {
    fn chose(cnf_formula: &CnfFormula) -> SingleAssignment {
        let shortest_clause = cnf_formula
            .clauses
            .iter()
            .map(|(_, clause)| clause)
            .filter(|clause| matches!(clause.satisfied_by, None))
            .min_by(|a, b| (&a.unassigned_variables).cmp(&b.unassigned_variables))
            .unwrap();

        let (variable_id, assignment_value) = shortest_clause.literals.iter()
            .find_map(|(variable_id, literal_polarity)| {
                let variable = cnf_formula.variables.get(variable_id).unwrap();
                if matches!(variable.value, None) {
                    return Some((*variable_id, literal_polarity.get_satisfying_assignment()))
                }
                None
            }).unwrap();

        SingleAssignment::new(variable_id, assignment_value, Branching)
    }
}