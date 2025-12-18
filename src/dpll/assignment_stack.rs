use crate::cnf::cnf_formula::CnfFormula;
use crate::dpll::assignment::Assignment;

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

    pub fn start_decision_level(&mut self) {
        self.decision_level_start.push(self.trail.len());
    }

    pub fn push_assignment(&mut self, assignment: Assignment) {
        self.trail.push(assignment);
    }

    pub fn revert_assignment_current_decision_level(&mut self, cnf_formula: &mut CnfFormula) {
        let current_decision_level_start = self.decision_level_start.pop().unwrap();
        while current_decision_level_start < self.trail.len() {
            cnf_formula.reverse_assignment(&self.trail.pop().unwrap());
        }
    }
    
    pub fn revert_last_assignment(&mut self, cnf_formula: &mut CnfFormula) {
        cnf_formula.reverse_assignment(&self.trail.pop().unwrap());
    }
}