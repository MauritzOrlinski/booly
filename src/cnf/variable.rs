use crate::cnf::clause::ClauseID;
use crate::dpll::assignment::AssignmentValue;
use std::fmt;
use std::fmt::Formatter;
use std::slice::Iter;

pub type VariableId = usize;

/// The variable implementation
///
/// # Fields
/// * `value` - The currently assigned value
/// * `positive_occurrences` - The IDs of the clauses in which this variable occurs as a positive literal
/// * `negative_occurrences` - The IDs of the clauses in which this variable occurs as a positive literal
#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub(crate) value: Option<AssignmentValue>,
    pub(crate) positive_occurrences: Vec<ClauseID>,
    pub(crate) negative_occurrences: Vec<ClauseID>,
}

impl Variable {
    pub(crate) fn new() -> Variable {
        Variable {
            value: None,
            positive_occurrences: vec![],
            negative_occurrences: vec![],
        }
    }

    /// Chooses the appropriate clause slices depending on an assignment value.
    ///
    /// # Arguments
    /// * `value` - The assignment value
    ///
    /// # Returns
    /// A tuple `(satisfied_clause_ids, unsatisfied_clause_ids)`, where `satisfied_clause_ids` is a slice of clauses that will become satisfied by the
    /// given assignment and `unsatisfied_clause_ids` is a slice of clauses that contain this variable, but wont be satisifed by this assignment
    pub(crate) fn associated_clauses(&self, value: AssignmentValue) -> (&[ClauseID], &[ClauseID]) {
        match value {
            AssignmentValue::True => (&self.positive_occurrences, &self.negative_occurrences),
            AssignmentValue::False => (&self.negative_occurrences, &self.positive_occurrences),
        }
    }
}

/// A wrapper around `Vec<Variable>` to allow pretty string representation in the style of DIMCAS CNF.
#[derive(Clone, Debug, PartialEq)]
pub struct Variables(Vec<Variable>);

impl Variables {
    pub fn new(variable_count: usize) -> Variables {
        Variables(vec![Variable::new(); variable_count])
    }

    pub(crate) fn get(&self, variable_id: VariableId) -> &Variable {
        self.0.get(variable_id - 1).unwrap()
    }

    pub(crate) fn get_mut(&mut self, variable_id: VariableId) -> &mut Variable {
        self.0.get_mut(variable_id - 1).unwrap()
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&'_ self) -> Iter<'_, Variable> {
        self.0.iter()
    }

    /// Find any unassigned variable. Expects there to be at least one.
    ///
    /// # Returns
    /// The unassigned variable ID.
    ///
    /// # Panics
    /// If there is none.
    pub(crate) fn find_unassigned(&self) -> VariableId {
        self.0
            .iter()
            .enumerate()
            .find_map(|(id, variable)| match variable.value {
                Some(_) => None,
                None => Some(id),
            })
            .unwrap()
            + 1
    }

    pub(crate) fn find_all_unassigned(&self) -> Vec<VariableId> {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(id, variable)| match variable.value {
                Some(_) => None,
                None => Some(id + 1),
            })
            .collect::<Vec<VariableId>>()
    }
}

impl fmt::Display for Variables {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .enumerate()
                .filter_map(|(variable_id, variable)| variable
                    .value
                    .map(|value| (variable_id, value)))
                .map(|(var_id, variable_value)| format!(
                    "{}{}",
                    match variable_value {
                        AssignmentValue::True => "",
                        AssignmentValue::False => "-",
                    },
                    var_id + 1
                ))
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}
