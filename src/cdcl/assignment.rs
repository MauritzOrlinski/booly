use crate::cnf::clause::ClauseID;
use crate::cnf::variable::VariableId;
use std::fmt;
use std::fmt::Formatter;

pub type AssignmentValue = bool;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AssignmentResult {
    Success,
    Conflict,
}

#[derive(Clone, Debug)]
pub struct Assignment {
    pub(crate) variable_id: VariableId,
    pub(crate) value: AssignmentValue,
    pub(crate) reason: Option<ClauseID>,
}

impl Assignment {
    pub fn new(variable_id: VariableId, value: AssignmentValue, reason: Option<ClauseID>) -> Self {
        Assignment {
            variable_id,
            value,
            reason,
        }
    }
}

impl fmt::Display for Assignment {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            match self.value {
                true => "",
                false => "-",
            },
            self.variable_id
        )
    }
}
