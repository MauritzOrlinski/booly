use crate::cnf::cnf_formula::CnfFormula;
use crate::dpll::assignment::{Assignment, AssignmentResult};
use crate::dpll::heuristics::from_shortest_clause::FromShortestClause;
use crate::dpll::heuristics::heuristic::Heuristic;
use std::collections::VecDeque;
use crate::dpll::assignment_stack::AssignmentStack;

#[derive(Debug, PartialEq)]
pub enum Result {
    Satisfied,
    Conflict,
}

#[derive(Debug)]
pub struct Dpll {
    pub(crate) unit_queue: VecDeque<usize>,
    pub cnf_formula: CnfFormula,
    pub(crate) assignment_stack: AssignmentStack,
}

impl Dpll {
    pub fn new(cnf_formula: CnfFormula) -> Dpll {
        Dpll {
            cnf_formula,
            unit_queue: VecDeque::new(),
            assignment_stack: AssignmentStack::new(),
        }
    }

    pub fn dpll(&mut self) -> Result {
        if self.cnf_formula.is_satisfied() {
            return Result::Satisfied;
        }

        self.assignment_stack.start_decision_level();

        if let Some(result) = self.propagate_unit_clauses() {
            if let Result::Conflict = result {
                self.assignment_stack.revert_assignment_current_decision_level(&mut self.cnf_formula);
            }
            return result;
        }

        let branch_a = FromShortestClause::chose_next_assignment(&self.cnf_formula);
        let branch_b = branch_a.inverse();

        if let Result::Satisfied = self.handle_assign_single_branch(branch_a) {
            return Result::Satisfied;
        }

        if let Result::Satisfied = self.handle_assign_single_branch(branch_b) {
            return Result::Satisfied;
        }

        self.assignment_stack.revert_assignment_current_decision_level(&mut self.cnf_formula);
        Result::Conflict
    }

    fn handle_assign_single_branch(&mut self, assignment: Assignment) -> Result {
        let assignment_result = self
            .cnf_formula
            .apply_assignment(&assignment, &mut self.unit_queue);

        if let AssignmentResult::Conflict = assignment_result {
            self.cnf_formula.reverse_assignment(&assignment);
            self.unit_queue.clear();
            return Result::Conflict
        }

        self.assignment_stack.push_assignment(assignment);
        let branch_result = self.dpll();

        if let Result::Conflict = branch_result {
            self.assignment_stack.revert_last_assignment(&mut self.cnf_formula);
            self.unit_queue.clear();
        }

        branch_result
    }
}
