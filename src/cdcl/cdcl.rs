use crate::cdcl::assignment::{Assignment, AssignmentResult};
use crate::cdcl::cdcl::CdclStatus::{Conflict, Incomplete, Sat, Unsat};
use crate::cdcl::heuristics::restart::LubyHeuristic;
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
    pub(crate) enable_phase_saving: bool,
    pub(crate) phase: Vec<Option<bool>>,
}

impl Cdcl {
    pub fn new(
        cnf_formula: CnfFormula,
        enable_phase_saving: bool,
        restart_heuristic: Box<dyn RestartHeuristic>,
    ) -> Cdcl {
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
            phase: cnf_formula.assignments.clone(),
            cnf_formula: cnf_formula,
            implication_graph: ImplicationGraph::new(),
            status: Incomplete,
            stats: SolverStats::new(),
            restart_heuristic: restart_heuristic,
            enable_phase_saving: enable_phase_saving,
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
                    let conflict_clause: &Clause = &self
                        .cnf_formula
                        .clauses
                        .get(conflict_clause_id)
                        .unwrap()
                        .clone();
                    let clause_id = self.cnf_formula.get_next_clause_id();
                    let learned_clause = self.generate_learned_clause(conflict_clause, clause_id);
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
                    // choose next assignment (VSIDS)
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
                    if self.enable_phase_saving
                        && self
                            .phase
                            .get(lit.unsigned_abs() as usize - 1)
                            .unwrap()
                            .is_some()
                    {
                        self.decide(Assignment::new(
                            lit.unsigned_abs(),
                            self.phase
                                .get(lit.unsigned_abs() as usize - 1)
                                .unwrap()
                                .unwrap(),
                            None,
                        ));
                    } else {
                        self.decide(Assignment::new(lit.unsigned_abs(), lit.signum() == 1, None));
                    }
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
        if self.enable_phase_saving {
            self.phase = self.cnf_formula.assignments.clone();
        }
        self.backjump(0);
        self.stats.conflict_count = 0;
        self.stats.number_of_restarts += 1;
    }
}
