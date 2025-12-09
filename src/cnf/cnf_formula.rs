use crate::assignment::assignment::{Assignment, AssignmentValue};
use crate::cnf::clause::{Clause, Polarity};
use crate::cnf::variable::Variable;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct CnfFormula {
    pub(crate) clauses: HashMap<usize, Clause>,
    pub(crate) variables: HashMap<usize, Variable>,
    unassigned_variables: usize,
}

impl CnfFormula {
    pub fn new(crude_clauses: Vec<Vec<i64>>) -> Self {
        let mut variables: HashMap<usize, Variable> = HashMap::new();
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
                let variable = variables.entry(identifier).or_insert(Variable::new());

                match polarity {
                    Polarity::Positive => variable.positive_occurrences.push(clause_id),
                    Polarity::Negative => variable.negative_occurrences.push(clause_id),
                }
                literals.insert(identifier, polarity);
            }
            clauses.insert(clause_id, Clause::new(literals));
        }
        CnfFormula {
            clauses,
            unassigned_variables: variables.len(),
            variables,
        }
    }

    pub fn apply_assignment(
        &mut self,
        assignment: &Assignment,
        reverse: bool,
        unit_queue: &mut VecDeque<usize>,
    ) -> Result<(), AssignException> {
        let assignee = self.variables.get_mut(&assignment.variable_id).unwrap();

        let satisfied_clause_ids: &Vec<usize>;
        let unsatisfied_clause_ids: &Vec<usize>;

        match assignment.value {
            AssignmentValue::True => {
                satisfied_clause_ids = &assignee.positive_occurrences;
                unsatisfied_clause_ids = &assignee.negative_occurrences;
            }
            AssignmentValue::False => {
                satisfied_clause_ids = &assignee.negative_occurrences;
                unsatisfied_clause_ids = &assignee.positive_occurrences;
            }
        }

        if !reverse {
            assignee.value = Some(assignment.value.clone());
            self.unassigned_variables -= 1;
        } else {
            assignee.value = None;
            unit_queue.clear();
            self.unassigned_variables += 1;
        }

        for clause_id in satisfied_clause_ids {
            let clause = self.clauses.get_mut(clause_id).unwrap();
            if !reverse {
                if matches!(clause.satisfied_by, None) {
                    clause.satisfied_by = Some(assignment.variable_id);
                }
            } else {
                if matches!(clause.satisfied_by, Some(x) if x == assignment.variable_id) {
                    clause.satisfied_by = None;
                }
            }
        }

        let mut assignment_error = false;

        for clause_id in unsatisfied_clause_ids {
            let clause = self.clauses.get_mut(clause_id).unwrap();
            if !reverse {
                if matches!(clause.satisfied_by, None) {
                    clause.unassigned_variables -= 1;
                }
                if clause.unassigned_variables == 1 {
                    unit_queue.push_back(*clause_id);
                } else if clause.unassigned_variables <= 0 {
                    assignment_error = true;
                }
            } else {
                if matches!(clause.satisfied_by, None) {
                    clause.unassigned_variables += 1;
                }
            }
        }

        if assignment_error {
            return Err(AssignException);
        }
        Ok(())
    }
}

pub struct AssignException;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assignment::assignment::AssignmentReason::Branching;

    #[test]
    fn test_formula_assign_is_reversible() {
        let mut cnf = CnfFormula::new(vec![vec![1, 2, 3], vec![4, 5, 6]]);

        let snapshot = cnf.clone();

        let mut assignment = Assignment::new(1, AssignmentValue::True, Branching);
        cnf.apply_assignment(&mut assignment, false, &mut VecDeque::new());
        cnf.apply_assignment(&mut assignment, true, &mut VecDeque::new());

        assert_eq!(snapshot, cnf);
    }

    #[test]
    fn test_unit_queue() {
        let mut cnf = CnfFormula::new(vec![vec![1, 2], vec![3, 4, 5]]);
        let mut assignment = Assignment::new(1, AssignmentValue::False, Branching);
        let mut queue: VecDeque<usize> = VecDeque::new();

        cnf.apply_assignment(&mut assignment, false, &mut queue);
        assert!(!queue.is_empty());
        cnf.apply_assignment(&mut assignment, true, &mut queue);
        assert!(queue.is_empty());
    }
}
