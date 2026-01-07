use crate::cnf::clause::Clause;
use crate::dpll::assignment::Assignment;
use crate::dpll::dpll::Dpll;
use crate::dpll::dpll::DpllStatus::Conflict;
use crate::dpll::heuristics::Heuristic;

impl<T: Heuristic> Dpll<T> {
    /// Execute unit propagation. As long as there are unit clauses present in the unit queue,
    /// satisfy them. If this creates any new unit clauses, add them to the queue.
    ///
    /// # Returns
    /// An optional DPLL result. `Satisfied`, if the cnf has been satisfied during unit propagation.
    /// `Conflict`, if there has been a conflict during unit propagation. `None` otherwise.
    pub(crate) fn propagate_unit_clauses(&mut self) {
        while let Some(unit_clause_id) = self.unit_queue.pop_front()
            && self.status != Conflict
        // TODO: Does this save perfomance?
        // && self.status != DpllStatus::Sat
        {
            let unit_clause = self.cnf_formula.clauses.get(unit_clause_id).unwrap();
            if unit_clause.satisfied_by.is_some() {
                continue;
            }

            self.assign(self.find_satisfying_assignment_for_unit_clause(unit_clause));
        }
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
                match self.cnf_formula.variables.get(variable_id).value {
                    None => Some(Assignment::new(
                        variable_id,
                        polarity.get_satisfying_assignment(),
                    )),
                    Some(_) => None,
                }
            })
            .unwrap()
    }
}
