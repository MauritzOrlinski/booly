use crate::dpll::assignment::{Assignment, AssignmentResult};
use crate::dpll::assignment::AssignmentReason::Forced;
use crate::dpll::dpll::Dpll;

impl Dpll {
    pub(crate) fn propagate_unit_clauses(&mut self) -> Option<crate::dpll::dpll::Result> {
        while let Some(unit_clause_id) = self.unit_queue.pop_front() {
            let unit_clause = self.cnf_formula.clauses.get(unit_clause_id).unwrap();
            if matches!(unit_clause.satisfied_by, Some(_)) {
                continue;
            }
            let satisfying_assignment = unit_clause
                .literals
                .iter()
                .find_map(|(variable_id, polarity)| {
                    let variable = self.cnf_formula.variables.get(*variable_id);
                    match variable.value {
                        None => Some(Assignment::new(
                            *variable_id,
                            polarity.get_satisfying_assignment(),
                            Forced,
                        )),
                        Some(_) => None,
                    }
                })
                .unwrap();

            let assignment_result = self
                .cnf_formula
                .apply_assignment(&satisfying_assignment, &mut self.unit_queue);
            self.assignment_stack.push((self.current_search_depth, satisfying_assignment));

            if matches!(assignment_result, AssignmentResult::Conflict) {
                return Some(crate::dpll::dpll::Result::Conflict);
            }
            if self.cnf_formula.is_satisfied() {
                return Some(crate::dpll::dpll::Result::Satisfied);
            }
        }
        None
    }
}