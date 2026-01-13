use itertools::Itertools;

use crate::cnf::literals::Literals;
use crate::cnf::variable::VariableId;
use std::fmt;
use std::fmt::Formatter;

pub type ClauseID = usize;

/// The clause implementation
///
/// # Fields
/// * `satisfied_by` - The variable this clause has first been satisfied by. None, if the clause is not yet satisfied.
/// * `literals` - The set of literals in this clause
/// * `unassigned_variables` - The count of how many variables occur in this clause that have not yet been assigned a value.
#[derive(Debug, PartialEq, Clone)]
pub struct Clause {
    pub(crate) satisfied_by: Option<VariableId>,
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

    pub fn is_tautology(&self) -> bool {
        self.literals.iter().any(|(variable_id, polarity)| {
            self.literals
                .iter()
                .contains(&(variable_id, polarity.reverse()))
        })
    }
}

impl fmt::Display for Clause {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} 0", self.literals)
    }
}
