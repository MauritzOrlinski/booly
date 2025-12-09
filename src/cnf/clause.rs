use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Polarity {
    Positive,
    Negative,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Clause {
    pub(crate) satisfied_by: Option<usize>,
    pub(crate) literals: HashMap<usize, Polarity>,
    pub(crate) unassigned_variables: usize,
}

impl Clause {
    pub fn new(literals: HashMap<usize, Polarity>) -> Self {
        Clause {
            satisfied_by: None,
            unassigned_variables: literals.len(),
            literals,
        }
    }
}
