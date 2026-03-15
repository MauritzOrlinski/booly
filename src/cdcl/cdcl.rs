use crate::cdcl::assignment::{Assignment, AssignmentResult};
use crate::cdcl::cdcl::CdclStatus::{Conflict, Incomplete, Sat, Unsat};
use crate::cdcl::heuristics::restart::GeometricHeuristic;
use crate::cdcl::heuristics::{RestartHeuristic, SolverStats};
use crate::cdcl::implication_graph::{DecisionLevel, ImplicationGraph};
use crate::cnf::clause::{Clause, ClauseID};
use crate::cnf::cnf_formula::CnfFormula;
use crate::cnf::literals::Literal;
use priority_queue::PriorityQueue;
use rustc_hash::FxHashMap;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CdclStatus {
    Sat,
    Unsat,
    Incomplete,
    Conflict(usize),
}

impl CdclStatus {
    pub fn is_conflict(&self) -> bool {
        match self {
            Conflict(_) => true,
            _ => false,
        }
    }
}
#[derive(Debug)]
pub struct Cdcl {
    pub cnf_formula: CnfFormula,
    pub(crate) implication_graph: ImplicationGraph,
    pub(crate) status: CdclStatus,
    pub(crate) unit_queue: VecDeque<ClauseID>,
    pub(crate) restart_heuristic: Box<dyn RestartHeuristic>,
    pub(crate) stats: SolverStats,
    pub(crate) lit_prio: PriorityQueue<Literal, usize>,
    pub(crate) lit_counter: FxHashMap<Literal, usize>,
}

impl Cdcl {
    pub fn new(cnf_formula: CnfFormula) -> Cdcl {
        Cdcl {
            unit_queue: cnf_formula.generate_unit_queue(),
            lit_counter: {
                (-(cnf_formula.variable_count as i32)..0)
                    .chain(1..=cnf_formula.variable_count as i32)
                    .map(|lit| (lit, 0))
                    .collect()
            },
            lit_prio: {
                (-(cnf_formula.variable_count as i32)..0)
                    .chain(1..=cnf_formula.variable_count as i32)
                    .map(|lit| {
                        (lit, {
                            let var = cnf_formula.variables.get(lit.unsigned_abs());
                            if lit.signum() == 1 {
                                var.positive_occurrences_count
                            } else {
                                var.negative_occurrences_count
                            }
                        })
                    })
                    .collect()
            },
            cnf_formula: cnf_formula,
            implication_graph: ImplicationGraph::new(),
            status: Incomplete,
            stats: SolverStats::new(),
            restart_heuristic: Box::new(GeometricHeuristic {
                threshold: 400,
                max_restarts: 0,
                factor: 1.5,
            }),
        }
    }

    pub fn solve(&mut self) -> CdclStatus {
        let mut count: u32 = 0;
        if self.cnf_formula.clauses.is_empty() {
            return CdclStatus::Sat;
        }
        loop {
            count += 1;
            if count == 255 {
                count = 0;
                for lit in (-(self.cnf_formula.variable_count as i32)..0)
                    .chain(1..=self.cnf_formula.variable_count as i32)
                {
                    self.lit_prio.change_priority_by(&lit, |p| {
                        *p = *p / 2 + self.lit_counter.get(&lit).unwrap()
                    });
                    self.lit_counter.insert(lit, 0);
                }
            }
            self.propagate_unit_clauses();
            match self.status {
                Sat | Unsat => {
                    return self.status;
                }
                Conflict(conflict_clause_id) => {
                    if self.implication_graph.get_current_decision_level() == 0 {
                        return Unsat;
                    }
                    let conflict_clause = self
                        .cnf_formula
                        .clauses
                        .get(conflict_clause_id)
                        .unwrap()
                        .clone();
                    let clause_id = self.cnf_formula.get_next_clause_id();
                    let learned_clause = self.generate_learned_clause(&conflict_clause, clause_id);
                    for &literal in learned_clause.literals.0.iter() {
                        self.lit_counter
                            .entry(literal)
                            .and_modify(|counter| *counter += 1);
                    }
                    let backjump_decision_level =
                        self.get_backjump_level_for_learned_clause(&learned_clause);

                    self.backjump(backjump_decision_level);
                    self.cnf_formula.clauses.insert(clause_id, learned_clause);
                    self.unit_queue.push_back(clause_id);

                    // Check if restart is needed
                    self.stats.conflict_count += 1;
                    if self.restart_heuristic.should_restart(&self.stats) {
                        self.restart();
                    }
                }
                Incomplete => {
                    if self.cnf_formula.all_assigned() {
                        return Sat;
                    }
                    let (lit, _) = self
                        .lit_prio
                        .clone()
                        .into_sorted_iter()
                        .find(|(lit, _)| {
                            self.cnf_formula
                                .assignments
                                .get(lit.unsigned_abs() as usize - 1)
                                .unwrap()
                                .is_none()
                        })
                        .unwrap();
                    self.decide(Assignment::new(lit.unsigned_abs(), lit.signum() == 1, None));
                }
            }
        }
    }

    pub fn decide(&mut self, assignment: Assignment) {
        let assignment_result = self
            .cnf_formula
            .apply_assignment(&assignment, &mut self.unit_queue);
        self.implication_graph.push_decision(assignment);

        match assignment_result {
            AssignmentResult::Conflict(conflict_clause_id) => {
                self.status = Conflict(conflict_clause_id);
            }
            AssignmentResult::Success => {
                if self.cnf_formula.all_assigned() {
                    self.status = Sat;
                }
            }
        }
    }

    pub fn backjump(&mut self, decision_level: DecisionLevel) {
        self.status = Incomplete;
        self.implication_graph
            .backjump(&mut self.cnf_formula, decision_level);
        self.unit_queue.clear();
    }

    pub fn restart(&mut self) {
        self.backjump(0);
        self.stats.conflict_count = 0;
        self.stats.number_of_restarts += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::cdcl::cdcl::Cdcl;
    use crate::parser::parse_cnf;

    #[test]
    fn test_cdcl() {
        let raw_cnf = "\
p cnf 50 300
25 27 32 0
25 -27 32 0
21 -32 42 0
-21 25 -32 0
3 37 -42 0
1 11 37 0
25 37 45 0
25 37 -45 0
-1 29 -42 0
2 11 -29 0
2 22 -42 0
2 -11 -22 0
-2 -3 -29 0
-3 -11 46 0
-3 -29 -46 0
-11 -29 -36 0
6 34 42 0
20 34 -42 0
6 -20 26 0
6 11 -20 0
6 -8 -20 0
6 -20 34 0
27 44 50 0
27 35 44 0
-27 40 44 0
2 35 44 0
-2 36 44 0
21 -34 36 0
-21 27 -44 0
-21 36 -44 0
-27 -34 35 0
-34 -35 45 0
6 -35 36 0
-15 -35 -45 0
-12 32 43 0
-12 32 -43 0
6 -12 -32 0
-25 -36 50 0
6 -25 -50 0
-25 -44 -50 0
-15 -25 -44 0
-25 -48 -50 0
-25 37 43 0
-6 37 -43 0
3 9 50 0
3 9 39 0
3 9 -50 0
3 11 -37 0
-9 -11 49 0
3 8 42 0
8 -42 -49 0
3 -8 -9 0
3 -8 -50 0
-3 5 -12 0
-3 7 -12 0
-12 13 -32 0
-7 -12 13 0
-3 -12 -13 0
-31 -37 38 0
12 -31 -37 0
8 12 -38 0
-23 -38 49 0
12 -31 -49 0
-2 -23 -37 0
-10 12 -38 0
-5 -31 -38 0
22 24 42 0
22 -24 46 0
34 -37 -46 0
2 31 34 0
-2 22 31 0
8 26 45 0
26 -42 -45 0
8 -29 -42 0
22 -29 -42 0
11 26 31 0
-22 26 43 0
31 34 43 0
21 31 34 0
-21 31 -43 0
14 23 40 0
-22 35 40 0
-22 23 40 0
-14 23 -34 0
33 -34 40 0
-23 -33 40 0
35 41 -46 0
-12 41 -46 0
27 -41 -46 0
-27 32 -46 0
-32 -40 -41 0
-3 -35 -46 0
21 24 46 0
21 -24 46 0
21 -37 46 0
13 -34 46 0
4 -21 -24 0
-4 18 -24 0
-13 -24 42 0
-13 -21 -24 0
-13 -24 -28 0
16 30 -36 0
11 17 -30 0
-17 24 -36 0
-10 16 24 0
-11 12 16 0
-16 34 -36 0
-16 24 -36 0
12 46 50 0
33 -34 50 0
13 29 50 0
-13 -33 50 0
-29 -46 50 0
-13 -40 -43 0
-9 39 -50 0
-9 -40 43 0
-9 16 39 0
-7 -9 16 0
-9 -16 -40 0
-9 -39 -40 0
11 -17 24 0
-11 -17 -40 0
17 32 48 0
7 16 -48 0
17 32 -48 0
-16 17 32 0
2 38 43 0
2 -32 38 0
-2 38 45 0
-32 38 -45 0
5 -7 24 0
1 -7 43 0
-5 13 24 0
3 -5 -13 0
-3 -7 33 0
-3 -7 43 0
7 47 48 0
7 47 -48 0
9 24 47 0
-24 -43 47 0
-14 18 47 0
7 29 47 0
7 17 26 0
22 -26 40 0
1 36 45 0
1 37 -45 0
1 -17 36 0
-1 12 36 0
-1 -17 36 0
22 -26 -50 0
15 -26 -36 0
15 41 -48 0
14 15 -41 0
-10 27 -48 0
-10 40 -48 0
-10 -36 37 0
-10 -37 -48 0
4 15 22 0
-4 -14 15 0
-15 17 26 0
-15 -26 -36 0
-15 -17 -50 0
9 25 -38 0
9 -11 -38 0
9 -26 33 0
-26 33 -50 0
9 33 -40 0
-14 -32 -38 0
29 -33 -41 0
14 -33 -41 0
-14 -33 -41 0
-22 28 -38 0
-27 -28 49 0
-28 41 -49 0
-27 -28 -41 0
4 27 38 0
4 25 -38 0
22 25 27 0
-4 11 25 0
8 14 -23 0
1 -8 -23 0
-1 -23 26 0
14 -23 -26 0
14 -39 41 0
14 23 -41 0
-14 20 -22 0
-1 20 -22 0
11 -20 -22 0
-11 -14 -20 0
14 -22 45 0
5 23 46 0
2 5 14 0
5 23 34 0
-2 5 -34 0
6 9 31 0
21 -25 -33 0
-21 -31 -33 0
-9 -25 -33 0
-25 -33 -39 0
-10 44 -45 0
-10 35 -44 0
-35 -44 -45 0
-10 -19 -45 0
10 -19 39 0
-4 44 49 0
-44 47 49 0
-4 -47 49 0
-4 10 -49 0
-5 18 23 0
-1 -6 19 0
13 -15 19 0
-15 19 39 0
-19 31 39 0
-19 -31 39 0
-5 -6 -19 0
17 20 35 0
-18 -20 35 0
2 -18 45 0
-6 -17 -18 0
-17 -18 35 0
-1 -18 40 0
-17 -18 -40 0
10 13 39 0
10 -13 -20 0
-6 36 -39 0
-6 -20 26 0
5 -7 -39 0
5 -6 -26 0
-5 30 -39 0
-30 -39 41 0
10 -30 41 0
-5 32 -39 0
-6 -32 -39 0
7 8 12 0
8 12 -18 0
1 8 -18 0
-15 19 41 0
-15 19 -41 0
15 -16 19 0
10 20 -43 0
10 -26 28 0
-8 20 28 0
-8 28 -40 0
-11 23 28 0
4 28 -35 0
4 28 -43 0
-4 28 -42 0
20 21 28 0
-21 -23 -47 0
20 -28 -29 0
20 -29 -47 0
1 29 -44 0
-35 -48 49 0
16 30 -49 0
16 -30 -35 0
-30 46 47 0
-30 44 -46 0
17 -30 -47 0
7 -30 -47 0
7 -14 -47 0
-7 15 -47 0
5 15 -47 0
15 -30 -47 0
-28 48 -49 0
37 39 48 0
43 48 -49 0
-43 48 -49 0
-37 48 50 0
-37 48 -49 0
-5 -31 48 0
27 42 44 0
29 42 -44 0
-8 29 42 0
1 42 -45 0
-8 -27 29 0
-1 -27 29 0
-1 -27 -35 0
23 30 45 0
19 -23 31 0
19 30 -31 0
19 30 -31 0
4 -19 45 0
-4 -28 30 0
-2 -28 30 0
13 18 30 0
17 41 47 0
-18 -19 0
-8 18 -24 0
10 -16 -34 0
13 -14 16 0
4 38 0
-2 10 -28 0
-5 18 38 0
33 -43 0
4 -7 18 0
-6 -16 49 0
-4 -13 -21 0
-16 -19 49 0
-2 18 33 0
-16 18 21 0";

        let cnf = parse_cnf(raw_cnf).unwrap();
        let mut cdcl = Cdcl::new(cnf);
        cdcl.solve();

        println!("\n\n\n\n\n\n{:#?}", cdcl.status);
    }
}
