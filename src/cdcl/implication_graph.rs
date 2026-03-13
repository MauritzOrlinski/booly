use crate::cdcl::assignment::Assignment;
use crate::cnf::literals::Literals;
use crate::cnf::variable::{VariableId, Variables};

/// The assignment stack
///
/// # Fields
/// * `trail` - A trail of past assignments
/// * `decision_level_start` - A list of indices in the trail at which new decision level start
#[derive(Debug, Clone)]
pub struct ImplicationGraph {
    trail: Vec<Assignment>,
    decision_level_start: Vec<usize>,
}

impl ImplicationGraph {
    pub fn new() -> Self {
        Self {
            trail: Vec::new(),
            decision_level_start: vec![],
        }
    }

    pub fn get_assignments_of_current_decision_level(&self) -> Vec<Assignment> {
        let current_decision_level_start = match self.decision_level_start.last() {
            Some(decision_level_start) => decision_level_start.clone(),
            None => return vec![],
        };

        self.trail[current_decision_level_start..].to_vec()
    }

    pub fn push_decision(&mut self, assignment: Assignment) {
        self.decision_level_start.push(self.trail.len());
        self.trail.push(assignment);
    }

    pub fn push_forced(&mut self, assignment: Assignment) {
        self.trail.push(assignment);
    }

    pub fn get_latest_assignment(&self, clause: &Literals) -> Option<&Assignment> {

        self.trail.iter().rfind(|assignment| {
            clause.iter()
                .any(|(variable_id, _)| variable_id == assignment.variable_id)
        })
    }

    pub fn get_decision_level(&self, variable: &VariableId) -> Option<usize> {
        let trail_index = self.trail.iter().position(|a| a.variable_id == *variable)?;
        let level = self.decision_level_start.partition_point(|&start_index| start_index <= trail_index);
        Some(level)
    }
}
