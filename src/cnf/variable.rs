use crate::assignment::single_assignment::AssignmentValue;
use crate::cnf::cnf_formula::CnfFormula;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

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

    pub(crate) fn clause_id_slices_for(&self, value: AssignmentValue) -> (&[usize], &[usize]) {
        match value {
            AssignmentValue::True => (&self.positive_occurrences, &self.negative_occurrences),
            AssignmentValue::False => (&self.negative_occurrences, &self.positive_occurrences),
        }
    }
}