use crate::cnf::clause::{Clause, ClauseID};
use crate::cnf::literals::Polarity;
use crate::cnf::variable::Variables;
use crate::dpll::assignment::AssignmentResult::{Conflict, Success};
use crate::dpll::assignment::{Assignment, AssignmentResult};
use std::collections::VecDeque;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, PartialEq, Clone)]
pub struct CnfFormula {
    pub(crate) clauses: Vec<Clause>,
    pub variables: Variables,
    pub(crate) unsat_clauses: usize,
}

impl CnfFormula {
    pub fn new(clauses: Vec<Clause>, variables: Variables) -> Self {
        CnfFormula {
            unsat_clauses: clauses.len(),
            clauses,
            variables,
        }
    }

    pub fn add_clause(&mut self, clause: Clause) {
        let clause_id = self.clauses.len();
        for (variable_id, polarity) in clause.literals.iter() {
            let variable = self.variables.get_mut(variable_id);
            match polarity {
                Polarity::Positive => variable.positive_occurrences.push(clause_id),
                Polarity::Negative => variable.negative_occurrences.push(clause_id),
            }
        }
        self.clauses.push(clause);
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
        let assignee = self.variables.get_mut(assignment.variable_id);

        assignee.value = Some(assignment.value);

        let (satisfied_clause_ids, unsatisfied_clause_ids) =
            assignee.associated_clauses(assignment.value);

        satisfied_clause_ids.iter().for_each(|&clause_id| {
            let clause = self.clauses.get_mut(clause_id).unwrap();
            if clause.satisfied_by.is_none() {
                self.unsat_clauses -= 1;
                clause.satisfied_by = Some(assignment.variable_id);
            }
        });

        let mut assignment_conflict = false;

        unsatisfied_clause_ids.iter().for_each(|&clause_id| {
            let clause = self.clauses.get_mut(clause_id).unwrap();
            if clause.satisfied_by.is_none() {
                clause.unassigned_variables -= 1;
                if clause.unassigned_variables == 1 {
                    unit_queue.push_back(clause_id);
                } else if clause.unassigned_variables == 0 {
                    assignment_conflict = true;
                }
            }
        });

        if assignment_conflict {
            Conflict
        } else {
            Success
        }
    }

    /// Undos an assignment.
    ///
    /// # Arguments
    /// * `assignment` - The assignment
    pub fn undo_assignment(&mut self, assignment: &Assignment) {
        let assignee = self.variables.get_mut(assignment.variable_id);

        assignee.value = None;

        let (satisfied_clause_ids, unsatisfied_clause_ids) =
            assignee.associated_clauses(assignment.value);

        for clause_id in satisfied_clause_ids {
            let clause = self.clauses.get_mut(*clause_id).unwrap();
            if clause.satisfied_by == Some(assignment.variable_id) {
                self.unsat_clauses += 1;
                clause.satisfied_by = None;
            }
        }

        unsatisfied_clause_ids.iter().for_each(|&clause_id| {
            let clause = self.clauses.get_mut(clause_id).unwrap();
            if clause.satisfied_by.is_none() {
                clause.unassigned_variables += 1;
            }
        })
    }

    /// Checks if the formula is satisfied
    pub fn is_satisfied(&self) -> bool {
        self.unsat_clauses == 0
    }

    /// Generates a unit queue for this formula. Since this call is expensive, it must only be executed once in the beginning. Inside the search tree
    /// rely on the automatic expansion of the unit queue through assignments instead.
    pub fn generate_unit_queue(&self) -> VecDeque<ClauseID> {
        self.clauses
            .iter()
            .enumerate()
            .filter_map(|(clause_id, clause)| {
                if clause.unassigned_variables == 1 {
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

        let mut assignment = Assignment::new(1, true);
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

        let mut assignment = Assignment::new(1, false);
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

        assert!(!cnf.is_satisfied());

        let unit_queue = &mut VecDeque::new();

        let first_assignment = &Assignment::new(1, true);
        let second_assignment = &Assignment::new(3, true);

        let _ = cnf.apply_assignment(first_assignment, unit_queue);

        assert!(!cnf.is_satisfied());

        let _ = cnf.apply_assignment(second_assignment, unit_queue);

        assert!(cnf.is_satisfied());

        let _ = cnf.undo_assignment(second_assignment);

        assert!(!cnf.is_satisfied());
    }

    #[test]
    fn test_satisfy_occurance_in_multiple_clauses() {
        let mut cnf = parse_cnf(
            "\
p cnf 3 2
1 2 0
1 3 0",
        )
        .unwrap();

        assert!(!cnf.is_satisfied());

        let unit_queue = &mut VecDeque::new();
        let assignment = &Assignment::new(1, true);

        let _ = cnf.apply_assignment(assignment, unit_queue);

        assert!(cnf.is_satisfied());

        let _ = cnf.undo_assignment(assignment);

        assert!(!cnf.is_satisfied());
    }
}
