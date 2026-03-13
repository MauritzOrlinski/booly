use crate::cdcl::assignment::Assignment;
use crate::cnf::literals::Literals;
use crate::cnf::variable::{VariableId, Variables};

pub type DecisionLevel = usize;

/// The assignment stack
///
/// # Fields
/// * `trail` - A trail of past assignments
/// * `decision_level_start` - A list of indices in the trail at which new decision level start
#[derive(Debug, Clone)]
pub struct ImplicationGraph {
    trail: Vec<Assignment>,
    decision_level_start: Vec<DecisionLevel>,
}

impl ImplicationGraph {
    pub fn new() -> Self {
        Self {
            trail: Vec::new(),
            decision_level_start: vec![],
        }
    }

    /// Returns all assignments made at the most recent decision level.
    /// If no decisions have been made yet (level 0), this returns an empty vector.
    pub fn get_assignments_of_current_decision_level(&self) -> Vec<Assignment> {
        let current_decision_level_start = match self.decision_level_start.last() {
            Some(decision_level_start) => decision_level_start.clone(),
            None => return vec![],
        };

        self.trail[current_decision_level_start..].to_vec()
    }

    /// Records a new decision assignment and increments the decision level.
    /// This should be called when the solver picks a literal to satisfy that isn't currently forced by unit propagation.
    pub fn push_decision(&mut self, assignment: Assignment) {
        self.decision_level_start.push(self.trail.len());
        self.trail.push(assignment);
    }

    /// Records an assignment forced by unit propagation.
    /// Forced assignments stay within the current decision level and do not create a new entry in `decision_level_start`.
    pub fn push_forced(&mut self, assignment: Assignment) {
        self.trail.push(assignment);
    }

    /// Finds the assignment within a clause that occurred most recently on the trail.
    /// This is used during conflict analysis to identify the most recent contributor to a conflict.
    pub fn get_latest_assignment(&self, clause: &Literals) -> Option<&Assignment> {
        self.trail.iter().rfind(|assignment| {
            clause.iter()
                .any(|(variable_id, _)| variable_id == assignment.variable_id)
        })
    }

    /// Calculates the decision level of a specific variable.
    ///
    /// # Returns
    /// * `Some(usize)` - The level (1-indexed based on the partition).
    /// * `None` - If the variable has not been assigned.
    pub fn get_decision_level(&self, variable: &VariableId) -> Option<usize> {
        let trail_index = self.trail.iter().position(|a| a.variable_id == *variable)?;
        let level = self.decision_level_start.partition_point(|&start_index| start_index <= trail_index);
        Some(level)
    }
}
