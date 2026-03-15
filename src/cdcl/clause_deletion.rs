use crate::cdcl::cdcl::Cdcl;
use crate::cdcl::implication_graph::ImplicationGraph;
use crate::cnf::clause::{Clause, ClauseID};
use crate::cnf::cnf_formula::CnfFormula;
use crate::cnf::literals::Literals;
use itertools::Itertools;

impl Cdcl {

    pub fn determine_clauses_for_deletion(&self) -> Vec<&ClauseID> {
        let learned_clauses = self.cnf_formula.get_learned_clauses();

        if self.clauses_limit >= learned_clauses.len() {
            return vec![];
        }

        let locked_clauses = self.implication_graph.locked_clauses(&self.cnf_formula);

        let learned_clauses_filtered: Vec<_> = learned_clauses
            .iter()
            .filter(|(learned_clause_id, _)| !locked_clauses.contains(learned_clause_id))
            .collect();

        if learned_clauses_filtered.len() <= self.clauses_limit  {
            return vec![];
        }

        let learned_clauses_sorted: Vec<_> = learned_clauses_filtered.iter()
            .sorted_by_key(|(_, clause)| clause.literal_block_distance)
            .map(|(id, _)| *id)
            .collect();

        learned_clauses_sorted.get(self.clauses_limit..).unwrap().to_vec()
    }
}

impl ImplicationGraph {
    pub fn calculate_lbd(&self, literals: &Literals) -> usize {
        let mut decision_levels = literals
            .iter()
            .map(|(v, _)| self.get_decision_level(&v).unwrap())
            .collect::<Vec<_>>();
        decision_levels.sort();
        decision_levels.dedup();
        decision_levels.len()
    }

    // Clauses that must not be deleted because they force an assignment
    pub fn locked_clauses(&self, cnf: &CnfFormula) -> Vec<ClauseID> {
        self.trail
            .iter()
            .filter_map(|assignment| assignment.reason)
            .collect()
    }
}
