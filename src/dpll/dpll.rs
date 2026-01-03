use crate::cnf::clause::ClauseID;
use crate::cnf::cnf_formula::CnfFormula;
use crate::dpll::assignment::{Assignment, AssignmentResult};
use crate::dpll::assignment_stack::AssignmentStack;
use crate::dpll::heuristics::dlcs::DLCS;
use crate::dpll::heuristics::dlis::DLIS;
use crate::dpll::heuristics::from_shortest_clause::FromShortestClause;
use crate::dpll::heuristics::heuristic::Heuristic;
use crate::dpll::heuristics::mom::MOM;
use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub enum DpllResult {
    Satisfied,
    Conflict,
}

#[derive(Debug, PartialEq)]
pub enum DpllStatus {
    Sat,
    Unsat,
    Incomplete,
}

#[derive(Debug)]
pub struct Dpll {
    pub(crate) unit_queue: VecDeque<ClauseID>,
    pub cnf_formula: CnfFormula,
    pub(crate) assignment_stack: AssignmentStack,
    pub(crate) conflict: bool,
    pub(crate) status: DpllStatus,
}

impl Dpll {
    pub fn new(cnf_formula: CnfFormula) -> Dpll {
        Dpll {
            cnf_formula,
            unit_queue: VecDeque::new(),
            assignment_stack: AssignmentStack::new(),
            conflict: false,
            status: DpllStatus::Incomplete,
        }
    }

    pub fn solve(&mut self) -> DpllResult {
        self.preprocess();

        while self.status == DpllStatus::Incomplete {
            let assigment = FromShortestClause::chose_next_assignment(&self.cnf_formula);

            self.assignment_stack.start_decision_level();
            self.assign(assigment);

            self.propagate_unit_clauses();

            while self.conflict {
                self.backtrack();
            }
        }

        match self.status {
            DpllStatus::Sat => DpllResult::Satisfied,
            DpllStatus::Unsat => DpllResult::Conflict,
            DpllStatus::Incomplete => unreachable!(),
        }
    }

    fn backtrack(&mut self) {
        self.conflict = false;
        self.assignment_stack
            .undo_assignment_current_decision_level(&mut self.cnf_formula);

        if self.assignment_stack.is_empty() {
            self.status = DpllStatus::Unsat;
        } else {
            let branching = self
                .assignment_stack
                .undo_last_assignment(&mut self.cnf_formula)
                .inverse();
            self.unit_queue.clear();
            self.assign(branching);
            self.propagate_unit_clauses();
        }
    }

    fn assign(&mut self, assignment: Assignment) {
        let assignment_result = self
            .cnf_formula
            .apply_assignment(&assignment, &mut self.unit_queue);
        self.assignment_stack.push_assignment(assignment);

        if assignment_result == AssignmentResult::Conflict {
            self.conflict = true;
        } else if self.cnf_formula.is_satisfied() {
            self.status = DpllStatus::Sat;
        }
    }

    fn preprocess(&mut self) {
        self.pure_literals();

        self.unit_queue = self.cnf_formula.generate_unit_queue();
        self.propagate_unit_clauses();

        if self.conflict {
            self.status = DpllStatus::Unsat;
        }
    }

    fn pure_literals(&mut self) {
        self.cnf_formula
            .pure_literals()
            .iter()
            .for_each(|assignment| {
                self.cnf_formula
                    .apply_assignment(&assignment, &mut self.unit_queue);
            });
    }
}
