use crate::cnf::clause::Clause;
use crate::dpll::assignment::{Assignment, AssignmentResult};
use crate::dpll::dpll::Dpll;
use crate::dpll::dpll::DpllResult;

impl Dpll {

    /// Execute unit propagation. As long as there are unit clauses present in the unit queue,
    /// satisfy them. If this creates any new unit clauses, add them to the queue.
    ///
    /// # Returns
    /// An optional DPLL result. `Satisfied`, if the cnf has been satisfied during unit propagation.
    /// `Conflict`, if there has been a conflict during unit propagation. `None` otherwise.
    pub(crate) fn propagate_unit_clauses(&mut self) -> Option<DpllResult> {
        while let Some(unit_clause_id) = self.unit_queue.pop_front() {
            let unit_clause = self.cnf_formula.clauses.get(unit_clause_id).unwrap();
            if matches!(unit_clause.satisfied_by, Some(_)) {
                continue;
            }
            let satisfying_assignment = self.find_satisfying_assignment_for_unit_clause(unit_clause);

            let assignment_result = self
                .cnf_formula
                .apply_assignment(&satisfying_assignment, &mut self.unit_queue);
            self.assignment_stack.push_assignment(satisfying_assignment);

            if matches!(assignment_result, AssignmentResult::Conflict) {
                return Some(DpllResult::Conflict);
            }
            if self.cnf_formula.is_satisfied() {
                return Some(DpllResult::Satisfied);
            }
        }
        None
    }

    /// Finds the satisfying assignment for a unit clause.
    ///
    /// # Arguments
    /// * `unit_clause` - The unit clause. Expected to have only one unassigned literal left.
    ///
    /// # Returns
    /// The satisfying assignment.
    fn find_satisfying_assignment_for_unit_clause(&self, unit_clause: &Clause) -> Assignment {
        unit_clause
            .literals
            .iter()
            .find_map(|(variable_id, polarity)| {
                let variable = self.cnf_formula.variables.get(*variable_id);
                match variable.value {
                    None => Some(Assignment::new(
                        *variable_id,
                        polarity.get_satisfying_assignment(),
                    )),
                    Some(_) => None,
                }
            })
            .unwrap()
    }
}
