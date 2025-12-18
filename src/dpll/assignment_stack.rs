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
    decision_level_start: Vec<usize>,
}

impl AssignmentStack {
    pub fn new() -> Self {
        Self {
            trail: Vec::new(),
            decision_level_start: vec![0],
        }
    }
    
    /// Starts a new decision level
    pub fn start_decision_level(&mut self) {
        self.decision_level_start.push(self.trail.len());
    }

    /// Pushes a new assignment onto the stack
    pub fn push_assignment(&mut self, assignment: Assignment) {
        self.trail.push(assignment);
    }

    /// Reverts all assignments at the current decision level
    pub fn revert_assignment_current_decision_level(&mut self, cnf_formula: &mut CnfFormula) {
        let current_decision_level_start = self.decision_level_start.pop().unwrap();
        while current_decision_level_start < self.trail.len() {
            cnf_formula.reverse_assignment(&self.trail.pop().unwrap());
        }
    }
    
    /// Reverts only the last assignment
    pub fn revert_last_assignment(&mut self, cnf_formula: &mut CnfFormula) {
        cnf_formula.reverse_assignment(&self.trail.pop().unwrap());
    }
}