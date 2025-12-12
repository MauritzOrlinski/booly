use crate::assignment::single_assignment::SingleAssignment;
use crate::cnf::clause::{Clause};
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::fmt::Formatter;
use crate::cnf::literals::Polarity;
use crate::cnf::variables::Variables;

#[derive(Debug, PartialEq, Clone)]
pub struct CnfFormula {
    pub(crate) clauses: HashMap<usize, Clause>,
    pub(crate) variables: Variables,
    unsat_clauses: usize,
}

impl CnfFormula {
    pub fn new(crude_clauses: Vec<Vec<i64>>) -> Self {
        let mut variables: Variables = Variables::new();
        let mut clauses: HashMap<usize, Clause> = HashMap::new();

        for (clause_id, crude_clause) in crude_clauses.iter().enumerate() {
            let mut literals: HashMap<usize, Polarity> = HashMap::new();

            for crude_literal in crude_clause {
                let identifier = crude_literal.abs() as usize;
                let polarity = if *crude_literal > 0 {
                    Polarity::Positive
                } else {
                    Polarity::Negative
                };
                let variable = variables.get_or_create(&identifier);

                match polarity {
                    Polarity::Positive => variable.positive_occurrences.push(clause_id),
                    Polarity::Negative => variable.negative_occurrences.push(clause_id),
                }
                literals.insert(identifier, polarity);
            }
            clauses.insert(clause_id, Clause::new(literals));
        }
        CnfFormula {
            unsat_clauses: clauses.len(),
            clauses,
            variables,
        }
    }

    pub fn apply_assignment(
        &mut self,
        assignment: &SingleAssignment,
        unit_queue: &mut VecDeque<usize>,
    ) -> Result<(), AssignException> {
        let assignee = self.variables.get_mut(&assignment.variable_id).unwrap();

        assignee.value = Some(assignment.value.clone());

        let (satisfied_clause_ids, unsatisfied_clause_ids) =
            assignee.clause_id_slices_for(assignment.value);

        for clause_id in satisfied_clause_ids {
            let clause = self.clauses.get_mut(clause_id).unwrap();
            clause.unassigned_variables -= 1;

            if matches!(clause.satisfied_by, None) {
                self.unsat_clauses -= 1;
                clause.satisfied_by = Some(assignment.variable_id);
            }
        }

        let mut assignment_error = false;

        for clause_id in unsatisfied_clause_ids {
            let clause = self.clauses.get_mut(clause_id).unwrap();
            clause.unassigned_variables -= 1;

            if clause.unassigned_variables == 1 {
                unit_queue.push_back(*clause_id);
            } else if clause.unassigned_variables <= 0 && matches!(clause.satisfied_by, None) {
                assignment_error = true;
            }
        }

        if assignment_error {
            Err(AssignException)
        } else {
            Ok(())
        }
    }

    pub fn reverse_assignment(
        &mut self,
        assignment: &SingleAssignment,
    ) {
        let assignee = self.variables.get_mut(&assignment.variable_id).unwrap();

        assignee.value = None;

        let (satisfied_clause_ids, unsatisfied_clause_ids) =
            assignee.clause_id_slices_for(assignment.value);

        for clause_id in satisfied_clause_ids {
            let clause = self.clauses.get_mut(clause_id).unwrap();
            clause.unassigned_variables += 1;

            if matches!(clause.satisfied_by, Some(x) if x == assignment.variable_id) {
                self.unsat_clauses += 1;
                clause.satisfied_by = None;
            }
        }

        for clause_id in unsatisfied_clause_ids {
            let clause = self.clauses.get_mut(clause_id).unwrap();
            clause.unassigned_variables += 1;
        }
    }

    pub fn is_satisfied(&self) -> bool {
        self.unsat_clauses <= 0
    }

    pub fn get_unit_queue(&self) -> VecDeque<usize> {
        self.clauses.iter()
            .filter_map(|(clause_id, clause)| if clause.unassigned_variables == 1 { Some(*clause_id) } else { None })
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
                .map(|(_, clause)| clause.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

pub struct AssignException;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assignment::single_assignment::AssignmentReason::Branching;
    use crate::assignment::single_assignment::AssignmentValue;

    #[test]
    fn test_formula_assign_is_reversible() {
        let mut cnf = CnfFormula::new(vec![vec![1, 2, 3], vec![4, 5, 6]]);

        let snapshot = cnf.clone();

        let mut assignment = SingleAssignment::new(1, AssignmentValue::True, Branching);
        let _ = cnf.apply_assignment(&mut assignment, &mut VecDeque::new());
        let _ = cnf.reverse_assignment(&mut assignment);

        assert_eq!(snapshot, cnf);
    }

    #[test]
    fn test_unit_queue() {
        let mut cnf = CnfFormula::new(vec![vec![1, 2], vec![3, 4, 5]]);
        let mut assignment = SingleAssignment::new(1, AssignmentValue::False, Branching);
        let mut queue: VecDeque<usize> = VecDeque::new();

        let _ = cnf.apply_assignment(&mut assignment, &mut queue);
        assert!(!queue.is_empty());
    }

    #[test]
    fn test_satisfy_occurance_in_single_clause() {
        let mut cnf = CnfFormula::new(vec![vec![1, 2], vec![3, 4]]);

        assert!(!cnf.is_satisfied());

        let unit_queue = &mut VecDeque::new();

        let first_assignment = &SingleAssignment::new(1, AssignmentValue::True, Branching);
        let second_assignment = &SingleAssignment::new(3, AssignmentValue::True, Branching);

        let _ = cnf.apply_assignment(first_assignment, unit_queue);

        assert!(!cnf.is_satisfied());

        let _ = cnf.apply_assignment(second_assignment, unit_queue);

        assert!(cnf.is_satisfied());

        let _ = cnf.reverse_assignment(second_assignment);

        assert!(!cnf.is_satisfied());
    }

    #[test]
    fn test_satisfy_occurance_in_multiple_clauses() {
        let mut cnf = CnfFormula::new(vec![vec![1, 2], vec![1, 3]]);

        assert!(!cnf.is_satisfied());

        let unit_queue = &mut VecDeque::new();
        let assignment = &SingleAssignment::new(1, AssignmentValue::True, Branching);

        let _ = cnf.apply_assignment(assignment, unit_queue);

        assert!(cnf.is_satisfied());

        let _ = cnf.reverse_assignment(assignment);

        assert!(!cnf.is_satisfied());
    }
}
