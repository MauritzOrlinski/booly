use crate::cnf::cnf_formula::CnfFormula;
use crate::dpll::assignment::{Assignment, AssignmentResult};
use crate::dpll::heuristics::from_shortest_clause::FromShortestClause;
use crate::dpll::heuristics::heuristic::Heuristic;
use std::collections::VecDeque;
use crate::dpll::assignment_stack::AssignmentStack;

#[derive(Debug, PartialEq)]
pub enum DpllResult {
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

    pub fn solve(&mut self) -> DpllResult {
        self.assignment_stack.start_decision_level();

        if let Some(result) = self.propagate_unit_clauses() {
            if result == DpllResult::Conflict {
                self.assignment_stack.revert_assignment_current_decision_level(&mut self.cnf_formula);
            }
            return result;
        }

        let branch_a = FromShortestClause::chose_next_assignment(&self.cnf_formula);
        let branch_b = branch_a.inverse();


        for branch in [branch_a, branch_b] {

            let assignment_result = self
                .cnf_formula
                .apply_assignment(&branch, &mut self.unit_queue);

            if assignment_result == AssignmentResult::Conflict {
                self.cnf_formula.reverse_assignment(&branch);
                self.unit_queue.clear();
                continue;
            }

            self.assignment_stack.push_assignment(branch);

            if self.cnf_formula.is_satisfied() {
                return DpllResult::Satisfied;
            }

            let branch_result = self.solve();

            match branch_result {
                DpllResult::Conflict => {
                    self.assignment_stack.revert_last_assignment(&mut self.cnf_formula);
                    self.unit_queue.clear();
                }
                DpllResult::Satisfied => return DpllResult::Satisfied
            }
        }

        self.assignment_stack.revert_assignment_current_decision_level(&mut self.cnf_formula);
        DpllResult::Conflict
    }
}
