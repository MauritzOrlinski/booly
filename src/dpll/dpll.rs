use crate::cnf::assignment::Assignment;
use crate::cnf::assignment::AssignmentReason::Forced;
use crate::cnf::cnf_formula::CnfFormula;
use crate::dpll::heuristics::heuristic::{Heuristic};
use crate::dpll::dpll::DpllResult::{Satisfied, Unknown, Unsatisfiable};
use std::collections::VecDeque;
use crate::dpll::heuristics::from_shortest_clause::FromShortestClause;

#[derive(Debug, PartialEq)]
pub enum DpllResult {
    Satisfied,
    Unknown,
    Unsatisfiable,
}

impl DpllResult {
    pub fn as_str(&self) -> &str {
        match self {
            Satisfied => "SATISFIABLE",
            Unknown => "UNKNOWN",
            Unsatisfiable => "UNSATISFIABLE",
        }
    }
}

#[derive(Debug)]
pub struct Dpll {
    unit_queue: VecDeque<usize>,
    pub cnf_formula: CnfFormula,
    assignment_stack: Vec<(u32, Assignment)>,
}

impl Dpll {
    pub fn new(cnf_formula: CnfFormula) -> Dpll {
        Dpll {
            cnf_formula,
            unit_queue: VecDeque::new(),
            assignment_stack: Vec::new(),
        }
    }

    pub fn dpll(&mut self, depth: u32) -> DpllResult {
        if self.cnf_formula.is_satisfied() {
            return Satisfied;
        }

        let unit_propagation_result = self.propagate_unit_clauses(depth);
        match unit_propagation_result {
            Satisfied => return Satisfied,
            Unsatisfiable => return Unsatisfiable,
            _ => (),
        }

        let branch_a = FromShortestClause::chose_next_assignment(&self.cnf_formula);
        let branch_b = branch_a.inverse();

        match self.handle_assign_single_branch(branch_a, depth) {
            Satisfied => return Satisfied,
            Unknown => panic!("This should not happen."),
            Unsatisfiable => (),
        }

        match self.handle_assign_single_branch(branch_b, depth) {
            Satisfied => Satisfied,
            Unknown => panic!("This should not happen."),
            Unsatisfiable => Unsatisfiable,
        }
    }

    fn handle_assign_single_branch(&mut self, assignment: Assignment, depth: u32) -> DpllResult {
        let assignment_result = self
            .cnf_formula
            .apply_assignment(&assignment, &mut self.unit_queue);

        match assignment_result {
            Ok(_) => {
                self.assignment_stack.push((depth, assignment));
                let branch_result = self.dpll(depth + 1);
                match branch_result {
                    Satisfied => Satisfied,
                    Unknown => panic!("This should not happen."),
                    Unsatisfiable => {
                        self.undo_assignment_stack(depth + 1);
                        let (_, assignment) = self.assignment_stack.pop().unwrap();
                        self.cnf_formula.reverse_assignment(&assignment);
                        self.unit_queue.clear();
                        Unsatisfiable
                    }
                }
            }
            Err(_) => {
                self.cnf_formula.reverse_assignment(&assignment);
                self.unit_queue.clear();
                Unsatisfiable
            }
        }
    }

    fn propagate_unit_clauses(&mut self, depth: u32) -> DpllResult {
        while let Some(unit_clause_id) = self.unit_queue.pop_front() {
            let unit_clause = self.cnf_formula.clauses.get(unit_clause_id).unwrap();
            if matches!(unit_clause.satisfied_by, Some(_)) {
                continue;
            }
            let satisfying_assignment = unit_clause
                .literals
                .iter()
                .find_map(|(variable_id, polarity)| {
                    let variable = self.cnf_formula.variables.get(*variable_id);
                    match variable.value {
                        None => Some(Assignment::new(
                            *variable_id,
                            polarity.get_satisfying_assignment(),
                            Forced,
                        )),
                        Some(_) => None,
                    }
                })
                .unwrap();

            let assignment_result = self
                .cnf_formula
                .apply_assignment(&satisfying_assignment, &mut self.unit_queue);
            self.assignment_stack.push((depth, satisfying_assignment));

            if matches!(assignment_result, Err(_)) {
                self.undo_assignment_stack(depth);
                return Unsatisfiable;
            }
            if self.cnf_formula.is_satisfied() {
                return Satisfied;
            }
        }
        Unknown
    }

    pub fn undo_assignment_stack(&mut self, depth: u32) {
        while let Some((stack_depth, assignment)) = self.assignment_stack.pop() {
            if stack_depth >= depth {
                self.cnf_formula.reverse_assignment(&assignment);
            } else {
                self.assignment_stack.push((stack_depth, assignment));
                break;
            }
        }
    }
}
