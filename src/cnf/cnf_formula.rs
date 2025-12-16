use crate::cnf::assignment::{Assignment, AssignmentValue};
use crate::cnf::clause::Clause;
use crate::cnf::literals::{Literals, Polarity};
use crate::cnf::variable::{Variable, Variables};
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::fmt::Formatter;
use tracing::{instrument, trace};

#[derive(Debug, PartialEq, Clone)]
pub struct CnfFormula {
    pub(crate) clauses: Vec<Clause>,
    pub(crate) variables: Variables,
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

    #[cfg_attr(feature = "trace", instrument(
        skip_all,
        fields(assignment = %assignment),
    ))]
    pub fn apply_assignment(
        &mut self,
        assignment: &Assignment,
        unit_queue: &mut VecDeque<usize>,
    ) -> Result<(), AssignException> {
        let assignee = self.variables.get_mut(assignment.variable_id);

        assignee.value = Some(assignment.value);

        let (satisfied_clause_ids, unsatisfied_clause_ids) =
            assignee.associated_clauses(assignment.value);

        for clause_id in satisfied_clause_ids {
            let clause = self.clauses.get_mut(*clause_id).unwrap();
            if matches!(clause.satisfied_by, None) {
                self.unsat_clauses -= 1;
                trace!("Clause {}({}) satisfied by {}", clause_id, clause.literals, assignment);
                clause.satisfied_by = Some(assignment.variable_id);
            }
        }

        let mut assignment_error = false;

        for clause_id in unsatisfied_clause_ids {
            let clause = self.clauses.get_mut(*clause_id).unwrap();
            clause.unassigned_variables -= 1;
            if matches!(clause.satisfied_by, Some(_)) {
                continue;
            }
            if clause.unassigned_variables == 1 {
                trace!("Adding unit clause {} ({}) to unit queue",clause_id, clause);
                unit_queue.push_back(*clause_id);
            } else if clause.unassigned_variables <= 0 {
                assignment_error = true;
            }
        }

        if assignment_error {
            Err(AssignException)
        } else {
            Ok(())
        }
    }

    #[cfg_attr(feature = "trace", instrument(
        skip_all,
        fields(assignment = %assignment),
    ))]
    pub fn reverse_assignment(
        &mut self,
        assignment: &Assignment,
    ) {
        let assignee = self.variables.get_mut(assignment.variable_id);

        assignee.value = None;

        let (satisfied_clause_ids, unsatisfied_clause_ids) =
            assignee.associated_clauses(assignment.value);

        for clause_id in satisfied_clause_ids {
            let clause = self.clauses.get_mut(*clause_id).unwrap();
            if matches!(clause.satisfied_by, Some(x) if x == assignment.variable_id) {
                self.unsat_clauses += 1;
                trace!("Clause {}({}) is no longer satisfied by {}", clause_id, clause.literals, assignment);
                clause.satisfied_by = None;
            }
        }

        for clause_id in unsatisfied_clause_ids {
            let clause = self.clauses.get_mut(*clause_id).unwrap();
            clause.unassigned_variables += 1;
        }
    }

    pub fn is_satisfied(&self) -> bool {
        self.unsat_clauses <= 0
    }

    pub fn generate_unit_queue(&self) -> VecDeque<usize> {
        self.clauses.iter()
            .enumerate()
            .filter_map(|(clause_id, clause)| if clause.unassigned_variables == 1 { Some(clause_id) } else { None })
            .collect()
    }
    
    pub fn get_variable_assignments(&self) -> String {
        self.variables.to_string()
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

pub struct AssignException;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cnf::assignment::AssignmentReason::Branching;
    use crate::cnf::assignment::AssignmentValue;
    use crate::cnf::parser::parse_cnf;

    #[test]
    fn test_formula_assign_is_reversible() {
        let mut cnf = parse_cnf("\
p cnf 6 2
1 2 3 0
4 5 6 0").unwrap();

        let snapshot = cnf.clone();

        let mut assignment = Assignment::new(1, AssignmentValue::True, Branching);
        let _ = cnf.apply_assignment(&mut assignment, &mut VecDeque::new());
        let _ = cnf.reverse_assignment(&mut assignment);

        assert_eq!(snapshot, cnf);
    }

    #[test]
    fn test_unit_queue() {
        let mut cnf = parse_cnf("\
p cnf 5 2
1 2 0
3 4 5 0").unwrap();

        let mut assignment = Assignment::new(1, AssignmentValue::False, Branching);
        let mut queue: VecDeque<usize> = VecDeque::new();

        let _ = cnf.apply_assignment(&mut assignment, &mut queue);
        assert!(!queue.is_empty());
    }

    #[test]
    fn test_satisfy_occurance_in_single_clause() {
        let mut cnf = parse_cnf("\
p cnf 4 2
1 2 0
3 4 0").unwrap();

        assert!(!cnf.is_satisfied());

        let unit_queue = &mut VecDeque::new();

        let first_assignment = &Assignment::new(1, AssignmentValue::True, Branching);
        let second_assignment = &Assignment::new(3, AssignmentValue::True, Branching);

        let _ = cnf.apply_assignment(first_assignment, unit_queue);

        assert!(!cnf.is_satisfied());

        let _ = cnf.apply_assignment(second_assignment, unit_queue);

        assert!(cnf.is_satisfied());

        let _ = cnf.reverse_assignment(second_assignment);

        assert!(!cnf.is_satisfied());
    }

    #[test]
    fn test_satisfy_occurance_in_multiple_clauses() {
        let mut cnf = parse_cnf("\
p cnf 3 2
1 2 0
1 3 0").unwrap();

        assert!(!cnf.is_satisfied());

        let unit_queue = &mut VecDeque::new();
        let assignment = &Assignment::new(1, AssignmentValue::True, Branching);

        let _ = cnf.apply_assignment(assignment, unit_queue);

        assert!(cnf.is_satisfied());

        let _ = cnf.reverse_assignment(assignment);

        assert!(!cnf.is_satisfied());
    }
}
