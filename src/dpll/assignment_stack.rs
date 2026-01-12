use crate::cnf::cnf_formula::CnfFormula;
use crate::dpll::assignment::Assignment;

/// The assignment stack
///
/// # Fields
/// * `trail` - A trail of past assignments
/// * `decision_level_start` - A list of indices in the trail at which new decision level start
#[derive(Debug)]
pub struct AssignmentStack {
    trail: Vec<Assignment>,
    decision_level_start: Vec<i32>,
}

impl AssignmentStack {
    pub fn new() -> Self {
        Self {
            trail: Vec::new(),
            decision_level_start: vec![-1, 0],
        }
    }

    /// Starts a new decision level
    pub fn start_decision_level(&mut self) {
        self.decision_level_start.push(self.trail.len() as i32);
    }

    /// Pushes a new assignment onto the stack
    pub fn push_assignment(&mut self, assignment: Assignment) {
        self.trail.push(assignment);
    }

    /// Undos all assignments at the current decision level excluding the last one
    pub fn undo_assignment_current_decision_level(&mut self, cnf_formula: &mut CnfFormula) {
        let current_decision_level_start = self.decision_level_start.pop().unwrap();
        while current_decision_level_start + 1 < self.trail.len() as i32 {
            cnf_formula.undo_assignment(&self.trail.pop().unwrap());
        }
    }

    /// Undos only the last assignment
    pub fn undo_last_assignment(&mut self, cnf_formula: &mut CnfFormula) -> Assignment {
        let assignment = self.trail.pop().unwrap();
        cnf_formula.undo_assignment(&assignment);
        assignment
    }

    pub fn is_empty(&self) -> bool {
        self.trail.is_empty()
    }

    pub fn get_decision_level(&self) -> i32 {
        self.trail.len() as i32
    }
}
