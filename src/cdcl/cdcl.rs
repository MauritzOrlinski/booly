use std::collections::VecDeque;
use crate::cdcl::assignment::Assignment;
use crate::cdcl::cdcl::CdclStatus::Incomplete;
use crate::cnf::cnf_formula::CnfFormula;
use crate::cdcl::implication_graph::ImplicationGraph;
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

    pub fn unit_propagation() {
        todo!();
    }
    
    pub fn assign(&mut self, assignment: Assignment) {
        todo!();
    }
}