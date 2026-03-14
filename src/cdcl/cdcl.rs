use priority_queue::PriorityQueue;

use crate::cdcl::assignment::{Assignment, AssignmentResult};
use crate::cdcl::cdcl::CdclStatus::{Conflict, Incomplete, Sat, Unsat};
use crate::cdcl::implication_graph::{DecisionLevel, ImplicationGraph};
use crate::cnf::clause::{Clause, ClauseID};
use crate::cnf::cnf_formula::CnfFormula;
use crate::cnf::literals::Literal;
use std::collections::{BTreeMap, VecDeque};

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
    pub(crate) lit_prio: PriorityQueue<Literal, usize>,
    pub(crate) lit_counter: BTreeMap<Literal, usize>,
}

impl Cdcl {
    pub fn new(cnf_formula: CnfFormula) -> Cdcl {
        Cdcl {
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
            unit_queue: VecDeque::new(),
        }
    }

    pub fn solve(&mut self) -> CdclStatus {
        let mut count: u32 = 0;
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
                Sat | Unsat => return self.status,
                Conflict => {
                    if self.implication_graph.get_current_decision_level() == 0 {
                        return Unsat;
                    }
                    let latest_assignment = self
                        .implication_graph
                        .get_latest_assignment()
                        .expect("If there is a conflict, there must at least be one assignment.");
                    let conflict_clause_id: ClauseID = latest_assignment.reason
                        .expect("If there is a conflict, the latest assignment must not have been made as a decision.");
                    let conflict_clause: &Clause =
                        &self.cnf_formula.clauses.get(&conflict_clause_id).unwrap();
                    let learned_clause = self.generate_learned_clause(&conflict_clause);
                    for &literal in learned_clause.0.iter() {
                        self.lit_counter
                            .entry(literal)
                            .and_modify(|counter| *counter += 1);
                    }
                    let backjump_decision_level =
                        self.get_backjump_level_for_learned_clause(&learned_clause);
                    self.backjump(backjump_decision_level);
                    let learned_clause_id = self.cnf_formula.add_clause(learned_clause);
                    self.unit_queue.push_back(learned_clause_id);
                }
                Incomplete => {
                    if self.cnf_formula.all_assigned() {
                        return Sat;
                    }
                    let next_assignment: Assignment;
                    loop {
                        let (lit, _) = self.lit_prio.pop().unwrap();
                        if self
                            .cnf_formula
                            .assignments
                            .get(lit.unsigned_abs() as usize)
                            .is_none()
                        {
                            next_assignment =
                                Assignment::new(lit.unsigned_abs(), lit.signum() == 1, None);
                            break;
                        }
                    }
                    self.decide(next_assignment);
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
            AssignmentResult::Conflict => {
                panic!(
                    "There should not be a conflict in a decision. There must be something wrong with the heuristic."
                )
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
}
