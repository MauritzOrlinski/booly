use crate::cnf::clause::Clause;

#[derive(Debug)]
enum VariableValue {
    True,
    False,
    Unassigned,
}

#[derive(Debug)]
enum AssignmentReason {
    Forced,
    Branching,
}

#[derive(Debug)]
pub struct Variable {
    identifier: u32,
    value: VariableValue,
    positive_occurrences: Vec<Clause>,
    negative_occurrences: Vec<Clause>,
    branching_level: u32,
    assignment_reason: AssignmentReason,
}