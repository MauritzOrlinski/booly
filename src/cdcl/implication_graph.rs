use std::fmt;
use std::fmt::Formatter;
use crate::cdcl::assignment::Assignment;
use crate::cnf::clause::ClauseID;
use crate::cnf::cnf_formula::CnfFormula;
use crate::cnf::literals::Literals;
use crate::cnf::variable::{VariableId};

pub type DecisionLevel = usize;

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
    pub fn get_latest_assignment_for_given_literals(&self, clause: &Literals) -> Option<&Assignment> {
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

    pub fn get_current_decision_level(&self) -> usize {
        self.decision_level_start.len()
    }

    pub fn get_latest_assignment(&self) -> Option<&Assignment> {
        self.trail.last()
    }

    /// Reverts the solver to a previous decision level, undoing all assignments made after that point.
    ///
    /// # Arguments
    /// * `cnf_formula` - The formula for this backjump
    /// * `desired_decision_level` - The target level to return to. All assignments above this decision level will be removed.
    ///
    /// # Panics
    /// Panics if `desired_decision_level` is higher than current decision level or negative.
    pub fn backjump(&mut self, cnf_formula: &mut CnfFormula, desired_decision_level: DecisionLevel) {
        let split_point = self.decision_level_start[desired_decision_level];
        for assignment in &self.trail[split_point..]{
            cnf_formula.undo_assignment(assignment);
        }
        self.trail.truncate(split_point);
        self.decision_level_start.truncate(desired_decision_level);

    }
}

impl fmt::Display for ImplicationGraph {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.trail.iter()
                .map(|assignment| {
                    match assignment.reason {
                        None => format!(" D:{}", assignment),
                        Some(_) => format!("→{}", assignment),
                    }

                })
                .collect::<Vec<String>>().join("")
        )
    }
}
