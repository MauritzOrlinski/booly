use crate::cnf::variable::VariableId;
use std::fmt;
use std::fmt::Formatter;
use std::ops::Not;

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
}

impl Assignment {
    pub fn new(variable_id: VariableId, value: AssignmentValue) -> Self {
        Assignment { variable_id, value }
    }

    pub fn inverse(&self) -> Assignment {
        Assignment {
            variable_id: self.variable_id,
            value: self.value.not(),
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
