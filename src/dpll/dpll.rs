use crate::dpll::assignment::{Assignment, AssignmentResult::{Success, Conflict}};
use crate::cnf::cnf_formula::CnfFormula;
use crate::dpll::heuristics::heuristic::{Heuristic};
use std::collections::VecDeque;
use crate::dpll::heuristics::from_shortest_clause::FromShortestClause;

#[derive(Debug, PartialEq)]
pub enum Result {
    Satisfied,
    Conflict,
}

#[derive(Debug)]
pub struct Dpll {
    pub(crate) unit_queue: VecDeque<usize>,
    pub cnf_formula: CnfFormula,
    pub(crate) assignment_stack: Vec<(u32, Assignment)>,
    pub(crate) current_search_depth: u32
}

impl Dpll {
    pub fn new(cnf_formula: CnfFormula) -> Dpll {
        Dpll {
            cnf_formula,
            unit_queue: VecDeque::new(),
            assignment_stack: Vec::new(),
            current_search_depth: 1
        }
    }

    pub fn dpll(&mut self) -> Result {
        if self.cnf_formula.is_satisfied() {
            return Result::Satisfied;
        }

        if let Some(result) = self.propagate_unit_clauses() {
            return match result {
                Result::Satisfied => Result::Satisfied,
                Result::Conflict => {
                    self.undo_assignment_stack();
                    Result::Conflict
                },
            }
        }

        let branch_a = FromShortestClause::chose_next_assignment(&self.cnf_formula);
        let branch_b = branch_a.inverse();

        match self.handle_assign_single_branch(branch_a) {
            Result::Satisfied => return Result::Satisfied,
            Result::Conflict => (),
        }

        match self.handle_assign_single_branch(branch_b) {
            Result::Satisfied => return Result::Satisfied,
            Result::Conflict => (),
        }

        self.undo_assignment_stack();
        Result::Conflict

    }

    fn handle_assign_single_branch(&mut self, assignment: Assignment) -> Result {
        let assignment_result = self
            .cnf_formula
            .apply_assignment(&assignment, &mut self.unit_queue);

        match assignment_result {
            Success => {
                self.assignment_stack.push((self.current_search_depth, assignment));
                self.current_search_depth += 1;
                let branch_result = self.dpll();
                match branch_result {
                    Result::Satisfied => Result::Satisfied,
                    Result::Conflict => {
                        let (_, assignment) = self.assignment_stack.pop().unwrap();
                        self.cnf_formula.reverse_assignment(&assignment);
                        self.unit_queue.clear();
                        Result::Conflict
                    }
                }
            }
            Conflict => {
                self.cnf_formula.reverse_assignment(&assignment);
                self.unit_queue.clear();
                Result::Conflict
            }
        }
    }

    pub fn undo_assignment_stack(&mut self) {
        while let Some((stack_depth, assignment)) = self.assignment_stack.pop() {
            if stack_depth >= self.current_search_depth {
                self.cnf_formula.reverse_assignment(&assignment);
            } else {
                self.assignment_stack.push((stack_depth, assignment));
                break;
            }
        }
        self.current_search_depth -= 1;
    }
}
