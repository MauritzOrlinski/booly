use std::collections::VecDeque;
use crate::cdcl::assignment::{Assignment, AssignmentResult};
use crate::cdcl::cdcl::CdclStatus::{Incomplete, Sat, Unsat, Conflict};
use crate::cdcl::heuristics::Heuristic;
use crate::cdcl::heuristics::trivial::Trivial;
use crate::cnf::cnf_formula::CnfFormula;
use crate::cdcl::implication_graph::{DecisionLevel, ImplicationGraph};
use crate::cnf::clause::{Clause, ClauseID};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CdclStatus {
    Sat,
    Unsat,
    Incomplete,
    Conflict,
}

pub struct Cdcl {
    pub cnf_formula: CnfFormula,
    pub(crate) implication_graph: ImplicationGraph,
    pub(crate) status: CdclStatus,
    pub(crate) unit_queue: VecDeque<ClauseID>,
    pub(crate) heuristic: Box<dyn Heuristic>,
}

impl Cdcl {
    pub fn new(
        cnf_formula: CnfFormula,
        heuristic: Box<dyn Heuristic>,
    ) -> Cdcl {
        Cdcl {
            cnf_formula,
            implication_graph: ImplicationGraph::new(),
            status: Incomplete,
            unit_queue: VecDeque::new(),
            heuristic,
        }
    }

    pub fn solve(&mut self) -> CdclStatus  {
        loop {
            self.propagate_unit_clauses();
            match self.status {
                Sat | Unsat => return self.status,
                Conflict => {
                    if self.implication_graph.get_current_decision_level() == 0 {
                        return Unsat;
                    }
                    let latest_assignment = self.implication_graph
                        .get_latest_assignment()
                        .expect("If there is a conflict, there must at least be one assignment.");
                    let conflict_clause_id: ClauseID = latest_assignment.reason
                        .expect("If there is a conflict, the latest assignment must not have been made as a decision.");
                    let conflict_clause: &Clause = &self.cnf_formula.clauses.get(&conflict_clause_id).unwrap();
                    let learned_clause = self.generate_learned_clause(&conflict_clause);
                    let backjump_decision_level = self.get_backjump_level_for_learned_clause(&learned_clause);
                    self.backjump(backjump_decision_level);
                    let clause_id = self.cnf_formula.get_next_clause_id();
                    self.cnf_formula.clauses.insert(clause_id, learned_clause);
                    self.unit_queue.push_back(clause_id);
                },
                Incomplete => {
                    if self.cnf_formula.all_assigned() {
                        return Sat;
                    }
                    let next_assignment= self.heuristic.chose_next_assignment(&self.cnf_formula);
                    self.decide(next_assignment);
                },
            }
        }
    }

    pub fn decide(&mut self, assignment: Assignment) {
        let assignment_result = self
            .cnf_formula
            .apply_assignment(&assignment, &mut self.unit_queue);
        self.implication_graph.push_decision(assignment);

        match assignment_result {
            AssignmentResult::Conflict => {
                panic!("There should not be a conflict in a decision. There must be something wrong with the heuristic.")
            },
            AssignmentResult::Success => {
                if self.cnf_formula.all_assigned() {
                    self.status = Sat;
                }
            },
        }
    }

    pub fn backjump(&mut self, decision_level: DecisionLevel) {
        self.status = Incomplete;
        self.implication_graph.backjump(&mut self.cnf_formula, decision_level);
        self.unit_queue.clear();
    }
}