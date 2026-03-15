use crate::cdcl::implication_graph::ImplicationGraph;
use crate::cnf::clause::{Clause, ClauseID};
use crate::cnf::literals::Literals;
use crate::preprocess::cnf::cnf::CNF;

impl ImplicationGraph {
    pub fn calculate_lbd(&self, literals: &Literals) -> usize {
        let mut decision_levels = literals.iter()
            .map(|(v, _)| self.get_decision_level(&v).unwrap())
            .collect::<Vec<_>>();
        decision_levels.sort();
        decision_levels.dedup();
        decision_levels.len()
    }
    
    // Clauses that must not be deleted because they force an assignment 
    pub fn locked_clauses(&self, cnf: CNF) -> Vec<ClauseID> {
        self.trail.iter()
            .filter_map(|assignment| {assignment.reason})
            .collect()
    }
}