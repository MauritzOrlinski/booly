#[derive(Debug)]
pub enum AssignmentReason {
    Forced,
    Branching,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AssignmentValue {
    True,
    False,
}

pub struct Assignment<'a> {
    pub(crate) variable_id: usize,
    pub(crate) value: &'a AssignmentValue,
    pub(crate) reason: AssignmentReason,
}

impl<'a> Assignment<'a> {
    pub fn new(
        variable_id: usize,
        value: &'a AssignmentValue,
        reason: AssignmentReason,
    ) -> Self {
        Assignment {
            variable_id,
            value,
            reason,
        }
    }
}