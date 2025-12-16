use crate::cnf::literals::{Literals, Polarity};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, PartialEq, Clone)]
pub struct Clause {
    pub(crate) satisfied_by: Option<usize>,
    pub(crate) literals: Literals,
    pub(crate) unassigned_variables: usize,
}

impl Clause {
    pub fn new(literals: Literals) -> Self {
        Clause {
            satisfied_by: None,
            unassigned_variables: literals.len(),
            literals,
        }
    }
}

impl fmt::Display for Clause {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{} 0",
            self.literals.to_string()
        )
    }
}
