use crate::cdcl::assignment::Assignment;
use crate::cnf::cnf_formula::CnfFormula;
use crate::cdcl::implication_graph::ImplicationGraph;

pub struct Cdcl {
    pub cnf_formula: CnfFormula,
    pub(crate) implication_graph: ImplicationGraph,
}

impl Cdcl {
    pub fn new(
        cnf_formula: CnfFormula
    ) -> Cdcl {
        Cdcl {
            cnf_formula,
            implication_graph: ImplicationGraph::new(),
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