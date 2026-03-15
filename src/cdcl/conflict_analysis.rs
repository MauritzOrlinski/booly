use crate::cdcl::cdcl::Cdcl;
use crate::cdcl::implication_graph::DecisionLevel;
use crate::cnf::clause::{Clause, ClauseID};
use crate::cnf::literals::{Literal, Literals, to_lit};
use crate::cnf::variable::{VariableId, Variables};
use itertools::Itertools;

impl Cdcl {
    pub(crate) fn generate_learned_clause(
        &mut self,
        conflict_clause: &Clause,
        clause_id: usize,
    ) -> Clause {
        let mut learned_clause_literals: Literals = conflict_clause.literals.clone();
        let variables_from_current_decision_level = self
            .implication_graph
            .get_assignments_of_current_decision_level()
            .iter()
            .map(|assignment| assignment.variable_id)
            .collect::<Vec<_>>();

        let first_uip: Literal;

        loop {
            let variables_from_current_decision_level_in_learned_clause: Vec<Literal> =
                learned_clause_literals
                    .iter()
                    .filter_map(|(learned_variable, polarity)| {
                        if variables_from_current_decision_level.contains(&learned_variable) {
                            Some(to_lit(&(learned_variable, polarity)))
                        } else {
                            None
                        }
                    })
                    .collect();

            match variables_from_current_decision_level_in_learned_clause.len() {
                n if n == 1 => {
                    first_uip = variables_from_current_decision_level_in_learned_clause[0];
                    break;
                }
                n if n > 1 => (),
                _ => panic!(
                    "At least one variable in learned clause should have been assigned in current decision level."
                ),
            }

            let most_recent_assignment = self
                .implication_graph
                .get_latest_assignment_for_given_literals(&learned_clause_literals)
                .expect(
                    // Must be present, because otherwise there would be no conflict
                    &format!(
                        "Could not find latest assignment for conflict clause: {}",
                        conflict_clause
                    ),
                );
            let antecedent_clause_id = most_recent_assignment.reason.expect(
                // Must be present, because otherwise there would be no conflict
                &format!(
                    "Could not find antecedent clause for assignment: {}",
                    most_recent_assignment
                ),
            );
            let antecedent_clause = &self.cnf_formula.clauses.get(&antecedent_clause_id).unwrap();

            let resolution = Self::resolve(
                &learned_clause_literals,
                &antecedent_clause.literals,
                most_recent_assignment.variable_id,
            );

            learned_clause_literals = resolution;
        }

        let second_latest = self
            .implication_graph
            .get_2nd_latest_assignment_for_given_literals(&learned_clause_literals);

        if let Some(second_latest) = second_latest {
            let watched2 =
                second_latest.variable_id as i32 * if second_latest.value { 1 } else { -1 };

            // TODO: Update watchlists
            Clause::new_with_watched(
                learned_clause_literals,
                first_uip,
                watched2,
                &mut self.cnf_formula.variables,
                clause_id,
            )
        } else {
            Clause::new_with_watched(
                learned_clause_literals,
                first_uip,
                first_uip,
                &mut self.cnf_formula.variables,
                clause_id,
            )
        }
    }

    pub(crate) fn get_backjump_level_for_learned_clause(&self, clause: &Clause) -> DecisionLevel {
        if clause.literals.len() < 2 {
            return 0;
        }

        let decision_levels = clause
            .literals
            .iter()
            .map(|(variable, _)| {
                self.implication_graph
                    .get_decision_level(&variable)
                    .expect("Variables in learned clause must have a decision level.")
            })
            .sorted()
            .collect::<Vec<_>>();

        decision_levels[decision_levels.len() - 2]
    }

    fn resolve(clause_a: &Literals, clause_b: &Literals, pivot: VariableId) -> (Literals) {
        let mut literals_a = clause_a.clone();
        literals_a.remove(pivot);
        let mut literals_b = clause_b.clone();
        literals_b.remove(pivot);
        Literals::combine(&literals_a, &literals_b)
    }
}

#[cfg(test)]
mod tests {
    use crate::cdcl::assignment::Assignment;
    use crate::cdcl::cdcl::{Cdcl, CdclStatus};
    use crate::cdcl::heuristics::trivial::Trivial;
    use crate::cdcl::implication_graph::ImplicationGraph;
    use crate::cnf::clause::{Clause, ClauseID};
    use crate::cnf::cnf_formula::CnfFormula;
    use crate::cnf::literals::Literals;
    use crate::cnf::variable::VariableId;
    use crate::parser::parse_cnf;
    use smallvec::smallvec;
    use std::collections::VecDeque;

    struct ConflictAnalysisTest {
        cnf: CnfFormula,
        implication_graph: ImplicationGraph,
    }

    impl ConflictAnalysisTest {
        fn new(cnf_string: &str) -> Self {
            let cnf = parse_cnf(cnf_string).unwrap();
            let implication_graph = ImplicationGraph::new();
            ConflictAnalysisTest {
                cnf,
                implication_graph,
            }
        }

        fn decide(&mut self, variable_id: VariableId, value: bool) -> &mut Self {
            let assignment = Assignment::new(variable_id, value, None);
            self.cnf.apply_assignment(&assignment, &mut VecDeque::new());
            self.implication_graph.push_decision(assignment);
            self
        }

        fn propagate(
            &mut self,
            variable_id: VariableId,
            value: bool,
            reason: ClauseID,
        ) -> &mut Self {
            let assignment = Assignment::new(variable_id, value, Some(reason));
            self.cnf.apply_assignment(&assignment, &mut VecDeque::new());
            self.implication_graph.push_forced(assignment);
            self
        }

        fn execute(&self, conflict_clause_id: ClauseID) -> Clause {
            let conflict_clause = &self.cnf.clauses.get(&conflict_clause_id).unwrap();

            let mut cdcl = Cdcl {
                cnf_formula: self.cnf.clone(),
                implication_graph: self.implication_graph.clone(),
                status: CdclStatus::Incomplete,
                unit_queue: Default::default(),
                heuristic: Box::new(Trivial),
            };

            let learned = cdcl
                .generate_learned_clause(conflict_clause, cdcl.cnf_formula.get_next_clause_id());
            learned
        }
    }

    #[test]
    fn test_single_resolution() {
        let cnf = "\
p cnf 3 2
1 2 3 0
1 2 -3 0";

        let mut test = ConflictAnalysisTest::new(cnf);

        test.decide(1, false).decide(2, false).propagate(3, true, 0);

        let result = test.execute(1);
        assert_eq!(result.literals, Literals(smallvec![1, 2]));
    }

    #[test]
    fn test_deep_multiple_resolutions() {
        let cnf = "\
p cnf 5 4
1 2 3 0
-3 4 0
-3 5 0
-4 -5 0";

        let mut test = ConflictAnalysisTest::new(cnf);

        test.decide(1, false)
            .decide(2, false)
            .propagate(3, true, 0)
            .propagate(4, true, 1)
            .propagate(5, true, 2);

        let result = test.execute(3);
        println!("{:#?}", result);
    }
}
