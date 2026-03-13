use std::collections::VecDeque;
use crate::cdcl::assignment::{Assignment, AssignmentResult};
use crate::cdcl::cdcl::CdclStatus::{Incomplete, Sat};
use crate::cnf::cnf_formula::CnfFormula;
use crate::cdcl::implication_graph::{DecisionLevel, ImplicationGraph};
use crate::cnf::clause::ClauseID;

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
}

impl Cdcl {
    pub fn new(
        cnf_formula: CnfFormula
    ) -> Cdcl {
        Cdcl {
            cnf_formula,
            implication_graph: ImplicationGraph::new(),
            status: Incomplete,
            unit_queue: VecDeque::new(),
        }
    }

    pub fn solve(&self)  {
        todo!()
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


    }
}