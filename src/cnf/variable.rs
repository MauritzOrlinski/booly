use crate::assignment::assignment::AssignmentValue;

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub(crate) value: Option<AssignmentValue>,
    pub(crate) positive_occurrences: Vec<usize>,
    pub(crate) negative_occurrences: Vec<usize>
}

impl Variable {
    pub(crate) fn new() -> Variable {
        Variable {
            value: None,
            positive_occurrences: vec![],
            negative_occurrences: vec![],
        }
    }
}