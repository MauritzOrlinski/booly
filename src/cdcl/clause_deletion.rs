use crate::cdcl::cdcl::Cdcl;
use crate::cdcl::implication_graph::ImplicationGraph;
use crate::cnf::clause::ClauseID;
use crate::cnf::cnf_formula::CnfFormula;
use crate::cnf::literals::Literals;
use itertools::Itertools;

impl Cdcl {
    pub fn delete_clauses(&mut self) {
        let learned_clauses = self.cnf_formula.get_learned_clauses();

        if self.clauses_limit >= learned_clauses.len() {
            return;
        }

        let locked_clauses = self.implication_graph.locked_clauses(&self.cnf_formula);

        let learned_clauses_filtered: Vec<_> = learned_clauses
            .iter()
            .filter(|(learned_clause_id, clause)| {
                !locked_clauses.contains(learned_clause_id)
                    && !self.unit_queue.contains(learned_clause_id)
                    && clause.literal_block_distance > 2
            })
            .collect();

        if learned_clauses_filtered.len() <= self.clauses_limit {
            return;
        }

        let learned_clauses_sorted: Vec<_> = learned_clauses_filtered
            .iter()
            .sorted_unstable_by_key(|(_, clause)| clause.literal_block_distance)
            .map(|(id, _)| *id)
            .collect();

        let delete_candidates: Vec<_> = learned_clauses_sorted
            .get(self.clauses_limit..)
            .unwrap()
            .iter()
            .map(|id| *id)
            .collect();

        for clause_id in &delete_candidates {
            if let Some(clause) = self.cnf_formula.clauses.get_mut(*clause_id) {
                if let Some(proof_logger) = &mut self.proof_logger {
                    proof_logger.log_delete(clause.into()).unwrap();
                }
                for &lit in &clause.literals.0 {
                    let variable = self.cnf_formula.variables.get_mut(lit.unsigned_abs());

                    if lit.is_positive() {
                        variable
                            .positive_watched_occurrences
                            .retain(|&w_id| w_id != *clause_id);
                    } else {
                        variable
                            .negative_watched_occurrences
                            .retain(|&w_id| w_id != *clause_id);
                    }
                }
                self.cnf_formula.delete_clause(*clause_id);
            }
        }
        self.cnf_formula
            .free_indices
            .sort_unstable_by_key(|&x| -(x as i32));
        self.unit_queue.clear();
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
    pub fn locked_clauses(&self, _: &CnfFormula) -> Vec<ClauseID> {
        self.trail
            .iter()
            .filter_map(|assignment| assignment.reason)
            .collect()
    }
}
