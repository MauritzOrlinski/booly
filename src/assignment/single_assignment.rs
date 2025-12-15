use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone, Copy)]
pub enum AssignmentReason {
    Forced,
    Branching,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AssignmentValue {
    True,
    False,
}

impl AssignmentValue {
    pub fn get_inverse(&self) -> AssignmentValue {
        match self {
            AssignmentValue::True => AssignmentValue::False,
            AssignmentValue::False => AssignmentValue::True
        }
    }
}

#[derive(Clone, Debug)]
pub struct SingleAssignment {
    pub(crate) variable_id: usize,
    pub(crate) value: AssignmentValue,
    pub(crate) reason: AssignmentReason,
}

impl SingleAssignment {
    pub fn new(
        variable_id: usize,
        value: AssignmentValue,
        reason: AssignmentReason,
    ) -> Self {
        SingleAssignment {
            variable_id,
            value,
            reason,
        }
    }

    pub fn inverse(&self) -> SingleAssignment {
        SingleAssignment {
            variable_id: self.variable_id,
            value: self.value.get_inverse(),
            reason: self.reason,
        }
    }
}

impl fmt::Display for SingleAssignment {
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