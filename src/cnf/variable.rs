use crate::cnf::assignment::AssignmentValue;

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub(crate) value: Option<AssignmentValue>,
    pub(crate) positive_occurrences: Vec<usize>,
    pub(crate) negative_occurrences: Vec<usize>,
}

impl Variable {
    pub(crate) fn new() -> Variable {
        Variable {
            value: None,
            positive_occurrences: vec![],
            negative_occurrences: vec![],
        }
    }

    /// Chooses the appropriate clause slices depending on an assignment value
    ///
    /// # Arguments
    /// * `value` - The assignment value
    ///
    /// # Returns
    /// A tuple `(satisfied_clause_ids, unsatisfied_clause_ids)`, where `satisfied_clause_ids` is a slice of clauses that will become satisfied by the
    /// given assignment and `unsatisfied_clause_ids` is a slice of clauses that contain this variable, but wont be satisifed by this assignment
    pub(crate) fn associated_clauses(&self, value: AssignmentValue) -> (&[usize], &[usize]) {
        match value {
            AssignmentValue::True => (&self.positive_occurrences, &self.negative_occurrences),
            AssignmentValue::False => (&self.negative_occurrences, &self.positive_occurrences),
        }
    }
}
