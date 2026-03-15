use crate::cdcl::assignment::{Assignment, AssignmentResult};
use crate::cdcl::cdcl::Cdcl;
use crate::cdcl::cdcl::CdclStatus::{Conflict, Sat};
use crate::cnf::clause::ClauseID;

impl Cdcl {
    /// Execute unit propagation. As long as there are unit clauses present in the unit queue,
    /// satisfy them. If this creates any new unit clauses, add them to the queue.
    ///
    /// # Returns
    /// An optional DPLL result. `Satisfied`, if the cnf has been satisfied during unit propagation.
    /// `Conflict`, if there has been a conflict during unit propagation. `None` otherwise.
    pub(crate) fn propagate_unit_clauses(&mut self) {
        while let Some(unit_clause_id) = self.unit_queue.pop_front()
            && !self.status.is_conflict()
        {
            if !self.cnf_formula.clauses.contains_key(&unit_clause_id) {
                continue;
            }

            let new_assignment = self.find_satisfying_assignment_for_unit_clause(unit_clause_id);
            let old_assignment =
                self.cnf_formula.assignments[new_assignment.variable_id as usize - 1];

            match old_assignment {
                Some(value) if value != new_assignment.value => {
                    self.status = Conflict(unit_clause_id);
                    break;
                }
                Some(_) => {
                    continue;
                }
                None => {
                    self.assign_propagation(new_assignment);
                }
            }
        }
    }

    fn assign_propagation(&mut self, assignment: Assignment) {
        let assignment_result = self
            .cnf_formula
            .apply_assignment(&assignment, &mut self.unit_queue);
        self.implication_graph.push_forced(assignment);
        match assignment_result {
            AssignmentResult::Conflict(clause_id) => {
                self.status = {
                    self.unit_queue.clear();
                    Conflict(clause_id)
                }
            }
            AssignmentResult::Success if self.cnf_formula.all_assigned() => self.status = Sat,
            AssignmentResult::Success => (),
        }
    }

    /// Finds the satisfying assignment for a unit clause.
    ///
    /// # Arguments
    /// * `unit_clause` - The unit clause. Expected to have only one unassigned literal left.
    ///
    /// # Returns
    /// The satisfying assignment.
    fn find_satisfying_assignment_for_unit_clause(&self, unit_clause_id: ClauseID) -> Assignment {
        let unit_clause = &self.cnf_formula.clauses.get(&unit_clause_id).unwrap();
        Assignment::new(
            unit_clause.watched1.unsigned_abs(),
            unit_clause.watched1.is_positive(),
            Some(unit_clause_id),
        )
    }
}
