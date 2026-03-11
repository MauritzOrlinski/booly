use crate::cnf::clause::{Clause, ClauseID, to_lit};
use crate::cnf::variable::Variables;
use crate::dpll::assignment::AssignmentResult::{Conflict, Success};
use crate::dpll::assignment::{Assignment, AssignmentResult};
use std::collections::VecDeque;
use std::fmt;
use std::fmt::Formatter;
use std::mem::swap;

#[derive(Debug, PartialEq, Clone)]
pub struct CnfFormula {
    pub(crate) clauses: Vec<Clause>,
    pub variables: Variables,
    pub(crate) unsat_clauses: usize,
    unset_vars: usize,
    pub(crate) assignments: Vec<Option<bool>>,
}

impl CnfFormula {
    pub fn new(clauses: Vec<Clause>, variables: Variables) -> Self {
        CnfFormula {
            unsat_clauses: clauses.len(),
            clauses,
            unset_vars: variables.len(),
            assignments: vec![None; variables.len()],
            variables,
        }
    }

    /// Applies an assignment to this formula.
    ///
    /// # Arguments
    /// * `assignment` - The assignment to apply
    /// * `unit_queue` - If the assignment results in any unit clauses, add them to this queue
    ///
    /// # Returns
    /// `AssignmentResult::Success`, if the assignment was successful
    /// `AssignmentResult::Conflict`, if the assignment resulted in an unsatisfiable clause
    pub fn apply_assignment(
        &mut self,
        assignment: &Assignment,
        unit_queue: &mut VecDeque<ClauseID>,
    ) -> AssignmentResult {
        let var_id = assignment.variable_id;

        match self.assignments[var_id as usize - 1] {
            Some(v) if v == assignment.value => return Success,
            Some(_) => return Conflict,
            None => {
                self.unset_vars -= 1;
            }
        }
        let assignee = self.variables.get_mut(assignment.variable_id);
        assignee.value = Some(assignment.value);
        self.assignments[var_id as usize - 1] = Some(assignment.value);

        // TODO: associated_clauses should only be clauses where the var is watched
        let (_, unsatisfied_clause_ids) = assignee.associated_clauses(assignment.value);

        for &clause_id in unsatisfied_clause_ids {
            let falsified_lit = if assignment.value {
                -(var_id as i32)
            } else {
                var_id as i32
            };
            let clause = self.clauses.get_mut(clause_id).unwrap();

            if !clause.is_watched(&assignment.variable_id)
                || clause.is_satisfied_by_watched(&self.assignments)
            {
                continue;
            }

            if clause.watched1 == falsified_lit {
                swap(&mut clause.watched1, &mut clause.watched2);
            }
            assert!(clause.watched2 == falsified_lit);

            let other = clause.watched1;

            let new_watched = clause.literals.iter().find(|(id, pol)| {
                let lit = match pol {
                    super::literals::Polarity::Positive => *id as i32,
                    super::literals::Polarity::Negative => -(*id as i32),
                };

                if lit == other {
                    return false;
                }

                let satisfying_assignment = Some(match pol {
                    super::literals::Polarity::Positive => true,
                    super::literals::Polarity::Negative => false,
                });

                self.assignments[*id as usize - 1].is_none()
                    || self.assignments[*id as usize - 1] == satisfying_assignment
            });

            if let Some((id, pol)) = new_watched {
                clause.watched2 = match pol {
                    super::literals::Polarity::Positive => id as i32,
                    super::literals::Polarity::Negative => -(id as i32),
                };
                continue;
            }
            let other_id = other.unsigned_abs() as usize - 1;
            let other_satisfying_assignment = Some(other.is_positive());

            if self.assignments[other_id].is_none() {
                assert!(clause.is_unit(&self.assignments));
                unit_queue.push_back(clause_id);
            } else if self.assignments[other_id] == other_satisfying_assignment {
                continue;
            } else {
                return Conflict;
            }
        }

        Success
    }

    /// Undos an assignment.
    ///
    /// # Arguments
    /// * `assignment` - The assignment
    pub fn undo_assignment(&mut self, assignment: &Assignment) {
        let assignee = self.variables.get_mut(assignment.variable_id);
        if assignee.value.is_some() {
            assignee.value = None;
            self.assignments[assignment.variable_id as usize - 1] = None;
            self.unset_vars += 1;
        }
    }

    /// Checks if the formula is satisfied
    pub fn all_assigned(&self) -> bool {
        self.unset_vars == 0
    }

    /// Generates a unit queue for this formula. Since this call is expensive, it must only be executed once in the beginning. Inside the search tree
    /// rely on the automatic expansion of the unit queue through assignments instead.
    pub fn generate_unit_queue(&self) -> VecDeque<ClauseID> {
        self.clauses
            .iter()
            .enumerate()
            .filter_map(|(clause_id, clause)| {
                if clause.is_unit(&self.assignments) {
                    Some(clause_id)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn pure_literals(&self) -> Vec<Assignment> {
        self.variables
            .iter()
            .enumerate()
            .filter(|(_, v)| v.value.is_none())
            .filter_map(|(i, v)| {
                if !v.positive_occurrences.is_empty() && v.positive_occurrences.is_empty() {
                    Some(Assignment {
                        variable_id: i as u32 + 1,
                        value: true,
                    })
                } else if v.positive_occurrences.is_empty() && !v.negative_occurrences.is_empty() {
                    Some(Assignment {
                        variable_id: i as u32 + 1,
                        value: false,
                    })
                } else {
                    None
                }
            })
            .collect()
    }
}

impl fmt::Display for CnfFormula {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "p cnf {} {}\n{}",
            self.variables.len(),
            self.clauses.len(),
            self.clauses
                .iter()
                .map(|clause| clause.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

// TODO: Fix test with twl
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::parser::parse_cnf;
//
//     #[test]
//     fn test_formula_assign_is_reversible() {
//         let mut cnf = parse_cnf(
//             "\
// p cnf 6 2
// 1 2 3 0
// 4 5 6 0",
//         )
//         .unwrap();
//
//         let snapshot = cnf.clone();
//
//         let mut assignment = Assignment::new(1, true);
//         let _ = cnf.apply_assignment(&mut assignment, &mut VecDeque::new());
//         let _ = cnf.undo_assignment(&mut assignment);
//
//         assert_eq!(snapshot, cnf);
//     }
//
//     #[test]
//     fn test_unit_queue() {
//         let mut cnf = parse_cnf(
//             "\
// p cnf 5 2
// 1 2 0
// 3 4 5 0",
//         )
//         .unwrap();
//
//         let mut assignment = Assignment::new(1, false);
//         let mut queue: VecDeque<ClauseID> = VecDeque::new();
//
//         let _ = cnf.apply_assignment(&mut assignment, &mut queue);
//         assert!(!queue.is_empty());
//     }
//
//     #[test]
//     fn test_satisfy_occurance_in_single_clause() {
//         let mut cnf = parse_cnf(
//             "\
// p cnf 4 2
// 1 2 0
// 3 4 0",
//         )
//         .unwrap();
//
//         assert!(!cnf.all_assigned());
//
//         let unit_queue = &mut VecDeque::new();
//
//         let first_assignment = &Assignment::new(1, true);
//         let second_assignment = &Assignment::new(3, true);
//
//         let _ = cnf.apply_assignment(first_assignment, unit_queue);
//
//         assert!(!cnf.all_assigned());
//
//         let _ = cnf.apply_assignment(second_assignment, unit_queue);
//
//         assert!(!cnf.all_assigned());
//
//         let _ = cnf.undo_assignment(second_assignment);
//
//         assert!(!cnf.all_assigned());
//     }
//
//     //     #[test]
//     //     fn test_satisfy_occurance_in_multiple_clauses() {
//     //         let mut cnf = parse_cnf(
//     //             "\
//     // p cnf 3 2
//     // 1 2 0
//     // 1 3 0",
//     //         )
//     //         .unwrap();
//     //
//     //         assert!(!cnf.is_satisfied());
//     //
//     //         let unit_queue = &mut VecDeque::new();
//     //         let assignment = &Assignment::new(1, true);
//     //
//     //         let _ = cnf.apply_assignment(assignment, unit_queue);
//     //
//     //         assert!(cnf.is_satisfied());
//     //
//     //         let _ = cnf.undo_assignment(assignment);
//     //
//     //         assert!(!cnf.is_satisfied());
//     //     }
// }
