use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use crate::cnf::literals::{Literals, Polarity};

#[derive(Debug, PartialEq, Clone)]
pub struct Clause {
    pub(crate) satisfied_by: Option<usize>,
    pub(crate) literals: Literals,
    pub(crate) unassigned_variables: usize,
}

impl Clause {
    pub fn new(literals: HashMap<usize, Polarity>) -> Self {
        Clause {
            satisfied_by: None,
            unassigned_variables: literals.len(),
            literals: Literals::new(literals)
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
