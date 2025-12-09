#[derive(Debug, Clone)]
pub enum AssignmentReason {
    Forced,
    Branching,
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Clone)]
pub struct Assignment {
    pub(crate) variable_id: usize,
    pub(crate) value: AssignmentValue,
    pub(crate) reason: AssignmentReason,
}

impl Assignment {
    pub fn new(
        variable_id: usize,
        value: AssignmentValue,
        reason: AssignmentReason,
    ) -> Self {
        Assignment {
            variable_id,
            value,
            reason,
        }
    }
}