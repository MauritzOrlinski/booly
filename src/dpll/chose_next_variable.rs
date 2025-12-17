use crate::cnf::assignment::Assignment;
use crate::cnf::assignment::AssignmentReason::Branching;
use crate::cnf::assignment::AssignmentValue::True;
use crate::cnf::cnf_formula::CnfFormula;

pub trait ChooseNextVariable {
    fn chose(cnf_formula: &CnfFormula) -> Assignment;
}

pub struct ChooseNextVariableFromShortestClause;

impl ChooseNextVariable for ChooseNextVariableFromShortestClause {
    fn chose(cnf_formula: &CnfFormula) -> Assignment {
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

pub struct TrivialChooseNextVariable;

impl ChooseNextVariable for TrivialChooseNextVariable {
    fn chose(cnf_formula: &CnfFormula) -> Assignment {
        Assignment::new(cnf_formula.variables.find_unassigned(), True, Branching)
    }
}
