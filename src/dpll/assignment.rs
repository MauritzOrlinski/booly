use std::fmt;
use std::fmt::Formatter;
use crate::cnf::variable::VariableId;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AssignmentValue {
    True,
    False,
}

impl AssignmentValue {
    pub fn get_inverse(&self) -> AssignmentValue {
        match self {
            AssignmentValue::True => AssignmentValue::False,
            AssignmentValue::False => AssignmentValue::True,
        }
    }
}

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
        Assignment {
            variable_id,
            value,
        }
    }

    pub fn inverse(&self) -> Assignment {
        Assignment {
            variable_id: self.variable_id,
            value: self.value.get_inverse(),
        }
    }
}

impl fmt::Display for Assignment {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            match self.value {
                AssignmentValue::True => "",
                AssignmentValue::False => "-",
            },
            self.variable_id
        )
    }
}
