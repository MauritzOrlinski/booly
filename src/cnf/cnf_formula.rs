use crate::cdcl::assignment::AssignmentResult::{Conflict, Success};
use crate::cdcl::assignment::{Assignment, AssignmentResult};
use crate::cnf::clause::{Clause, ClauseID};
use crate::cnf::literals::{Polarity, to_lit};
use crate::cnf::variable::Variables;
use std::collections::VecDeque;
use std::fmt;
use std::fmt::Formatter;
use std::mem::swap;

#[derive(Debug, PartialEq, Clone)]
pub struct CnfFormula {
    pub(crate) clauses: Vec<Clause>,
    pub variables: Variables,
    unset_vars: usize,
    pub(crate) variable_count: usize,
    /// we store the variable assignments now in this assignments vector, as it makes the values
    /// lay closer to each other
    pub(crate) assignments: Vec<Option<bool>>,
}

impl CnfFormula {
    pub fn new(clauses: Vec<Clause>, variables: Variables) -> Self {
        CnfFormula {
            clauses: clauses,
            unset_vars: variables.len(),
            assignments: vec![None; variables.len()],
            variable_count: variables.len(),
            variables: variables,
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
        let var_index = var_id as usize - 1;

        match self.assignments[var_index] {
            Some(v) if v == assignment.value => return Success,
            Some(_) => unreachable!(), // we assume that we do not assign twice
            None => {
                self.unset_vars -= 1;
            }
        }
        let assignee = self.variables.get_mut(assignment.variable_id);
        self.assignments[var_index] = Some(assignment.value);

        let (_, unsatisfied_clause_ids) = assignee.associated_clauses(assignment.value);

        let mut is_conflict_id = None;
        let mut newly_watched = Vec::with_capacity(unsatisfied_clause_ids.len());

        for &clause_id in unsatisfied_clause_ids {
            let falsified_lit = if assignment.value {
                -(var_id as i32)
            } else {
                var_id as i32
            };

            let clause = self.clauses.get_mut(clause_id).unwrap();

            if clause.watched1 == falsified_lit {
                swap(&mut clause.watched1, &mut clause.watched2);
            }
            assert!(clause.watched2 == falsified_lit);
            // Invariant: watched2 is our assignee that we want to switch out

            if self.assignments[clause.watched1.unsigned_abs() as usize - 1]
                == Some(clause.watched1.is_positive())
            {
                continue;
            }
            let other = clause.watched1;

            let new_watched = clause.literals.iter().find(|lit| {
                let (id, pol) = lit;
                let lit = to_lit(lit);
                if lit == other {
                    return false;
                }

                let satisfying_assignment = Some(pol.is_positive());

                let current_index = *id as usize - 1;

                self.assignments[current_index].is_none()
                    || self.assignments[current_index] == satisfying_assignment
            });

            if let Some(lit) = new_watched {
                let lit = to_lit(&lit);
                newly_watched.push((lit, clause_id, clause.watched2));
                clause.watched2 = lit;
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
                is_conflict_id = Some(clause_id);
                break;
            }
        }

        for (lit, clause_id, old_lit) in newly_watched {
            self.update_watchlists(lit, clause_id, old_lit);
        }

        if is_conflict_id.is_some() {
            Conflict(is_conflict_id.unwrap())
        } else {
            Success
        }
    }

    fn update_watchlists(&mut self, lit: i32, clause_id: usize, old_lit: i32) {
        let id = lit.unsigned_abs();
        if lit.is_positive() {
            self.variables
                .get_mut(id)
                .positive_watched_occurrences
                .push(clause_id);
        } else {
            self.variables
                .get_mut(id)
                .negative_watched_occurrences
                .push(clause_id);
        }

        let assignee = self.variables.get_mut(old_lit.unsigned_abs());
        if old_lit.is_positive() {
            let occurrences = &mut assignee.positive_watched_occurrences;
            let pos = occurrences.iter().position(|&x| x == clause_id).unwrap();
            occurrences.swap_remove(pos);
        } else {
            let occurrences = &mut assignee.negative_watched_occurrences;
            let pos = occurrences.iter().position(|&x| x == clause_id).unwrap();
            occurrences.swap_remove(pos);
        }
    }

    /// Undos an assignment.
    ///
    /// # Arguments
    /// * `assignment` - The assignment
    pub fn undo_assignment(&mut self, assignment: &Assignment) {
        if self.assignments[assignment.variable_id as usize - 1].is_some() {
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

    pub fn get_assignment_view(&self) -> AssignedVarsView<'_> {
        AssignedVarsView(&self.variables, &self.assignments)
    }

    pub fn get_next_clause_id(&self) -> ClauseID {
        let id = self.clauses.len() as ClauseID;
        // assert!(!self.clauses.contains_key(&id));
        id
    }
}
pub struct AssignedVarsView<'a>(pub &'a Variables, pub &'a [Option<bool>]);

impl<'a> fmt::Display for AssignedVarsView<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .0
                .iter()
                .enumerate()
                .filter_map(|(i, _)| self.1[i].map(|value| (i, value)))
                .map(|(var_id, value)| format!(
                    "{}{}",
                    match value {
                        false => "-",
                        _ => "",
                    },
                    var_id + 1
                ))
                .collect::<Vec<String>>()
                .join(" ")
        )
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_cnf;

    #[test]
    fn test_formula_assign_is_reversible() {
        let mut cnf = parse_cnf(
            "\
p cnf 6 2
1 2 3 0
4 5 6 0",
        )
        .unwrap();

        let snapshot = cnf.clone();

        let mut assignment = Assignment::new(1, true, None);
        let _ = cnf.apply_assignment(&mut assignment, &mut VecDeque::new());
        let _ = cnf.undo_assignment(&mut assignment);

        assert_eq!(snapshot, cnf);
    }

    #[test]
    fn test_unit_queue() {
        let mut cnf = parse_cnf(
            "\
p cnf 5 2
1 2 0
3 4 5 0",
        )
        .unwrap();

        let mut assignment = Assignment::new(1, false, None);
        let mut queue: VecDeque<ClauseID> = VecDeque::new();

        let _ = cnf.apply_assignment(&mut assignment, &mut queue);
        assert!(!queue.is_empty());
    }

    #[test]
    fn test_satisfy_occurance_in_single_clause() {
        let mut cnf = parse_cnf(
            "\
p cnf 4 2
1 2 0
3 4 0",
        )
        .unwrap();

        assert!(!cnf.all_assigned());

        let unit_queue = &mut VecDeque::new();

        let first_assignment = &Assignment::new(1, true, None);
        let second_assignment = &Assignment::new(3, true, None);

        let _ = cnf.apply_assignment(first_assignment, unit_queue);

        assert!(!cnf.all_assigned());

        let _ = cnf.apply_assignment(second_assignment, unit_queue);

        assert!(!cnf.all_assigned());

        let _ = cnf.undo_assignment(second_assignment);

        assert!(!cnf.all_assigned());
    }
}
