use crate::cnf::clause::ClauseID;
use crate::cnf::cnf_formula::CnfFormula;
use crate::dpll::assignment::{Assignment, AssignmentResult};
use crate::dpll::assignment_stack::AssignmentStack;
use crate::dpll::dpll::DpllStatus::{Conflict, Incomplete, Sat, Unsat};
use crate::dpll::heuristics::{Heuristic, Stats};
use std::collections::VecDeque;
use std::sync::atomic::AtomicBool;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DpllStatus {
    Sat,
    Unsat,
    Incomplete,
    Conflict,
}

#[derive(Debug)]
pub struct Dpll {
    pub(crate) heuristic: Box<dyn Heuristic>,
    pub(crate) unit_queue: VecDeque<ClauseID>,
    pub cnf_formula: CnfFormula,
    pub(crate) assignment_stack: AssignmentStack,
    pub(crate) status: DpllStatus,
}

impl Dpll {
    pub fn new(cnf_formula: CnfFormula, heuristic: Box<dyn Heuristic>) -> Dpll {
        Dpll {
            heuristic,
            cnf_formula,
            unit_queue: VecDeque::new(),
            assignment_stack: AssignmentStack::new(),
            status: Incomplete,
        }
    }

    pub fn dpll(&mut self, cancel_flag: &AtomicBool) {
        while self.status == Incomplete && !cancel_flag.load(std::sync::atomic::Ordering::Relaxed) {
            let assigment = self.heuristic.chose_next_assignment(
                &self.cnf_formula,
                self.assignment_stack.get_decision_level(),
            );

            self.assignment_stack.start_decision_level();
            self.assign(assigment);

            self.propagate_unit_clauses();

            while self.status == Conflict {
                self.backtrack();
            }
        }
    }

    pub fn learning_dpll(&mut self, cancel_flag: &AtomicBool) {
        while self.status == Incomplete && !cancel_flag.load(std::sync::atomic::Ordering::Relaxed) {
            let assigment = self.heuristic.chose_next_assignment(
                &self.cnf_formula,
                self.assignment_stack.get_decision_level(),
            );

            self.assignment_stack.start_decision_level();
            self.assign(assigment);
            let units = self.propagate_unit_clauses();
            self.heuristic.feedback(Stats {
                unit_clauses: units,
            });

            while self.status == Conflict {
                self.backtrack();
            }
        }
    }

    pub fn solve_interruptable(&mut self, cancel_flag: &AtomicBool) -> DpllStatus {
        self.preprocess();
        if self.heuristic.is_learning() {
            self.learning_dpll(cancel_flag);
        } else {
            self.dpll(cancel_flag);
        }

        self.postprocess();

        self.status
    }

    pub fn solve(&mut self) -> DpllStatus {
        let cancel_flag = AtomicBool::new(false);

        self.solve_interruptable(&cancel_flag)
    }

    fn backtrack(&mut self) {
        self.status = Incomplete;
        self.assignment_stack
            .undo_assignment_current_decision_level(&mut self.cnf_formula);

        if self.assignment_stack.is_empty() {
            self.status = Unsat;
        } else {
            let branching = self
                .assignment_stack
                .undo_last_assignment(&mut self.cnf_formula);
            self.unit_queue.clear();
            self.assign(branching.inverse());
            self.propagate_unit_clauses();
        }
    }

    pub(crate) fn assign(&mut self, assignment: Assignment) {
        let assignment_result = self
            .cnf_formula
            .apply_assignment(&assignment, &mut self.unit_queue);
        self.assignment_stack.push_assignment(assignment);

        if assignment_result == AssignmentResult::Conflict {
            self.status = Conflict;
        } else if self.cnf_formula.all_assigned() {
            // if all are assigned and no conflict has arisen => satisfied
            self.status = Sat;
        }
    }

    fn preprocess(&mut self) {
        self.pure_literals();

        self.unit_queue = self.cnf_formula.generate_unit_queue();
        self.propagate_unit_clauses();

        if self.status == Conflict {
            self.status = Unsat;
        }
    }

    fn pure_literals(&mut self) {
        self.cnf_formula
            .pure_literals()
            .iter()
            .for_each(|assignment| {
                self.cnf_formula
                    .apply_assignment(assignment, &mut self.unit_queue);
            });
    }

    /// Assigns values to all unset variables if formula is satisfiable
    // TODO: With 2wl there won't be any unset varibales
    fn postprocess(&mut self) {
        if self.status == DpllStatus::Sat {
            self.cnf_formula
                .variables
                .find_all_unassigned(&self.cnf_formula.assignments)
                .iter()
                .for_each(|&variable_id| {
                    self.cnf_formula.assignments[variable_id as usize - 1] = Some(false);
                });
        }
    }

    pub fn save_heuristic(&self, path: &str) {
        match self.heuristic.save(path) {
            Ok(_) => (),
            Err(_) => panic!("Failed to save heuristic state"),
        }
    }
}
