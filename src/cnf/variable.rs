use crate::cnf::clause::ClauseID;
use crate::dpll::assignment::AssignmentValue;
use std::fmt;
use std::fmt::Formatter;
use std::slice::Iter;

pub type VariableId = u32;

/// The variable implementation
///
/// # Fields
/// * `value` - The currently assigned value
/// * `positive_occurrences` - The IDs of the clauses in which this variable occurs as a positive literal
/// * `negative_occurrences` - The IDs of the clauses in which this variable occurs as a positive literal
#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    // pub(crate) value: Option<AssignmentValue>,
    pub(crate) positive_watched_occurrences: Vec<ClauseID>,
    pub(crate) negative_watched_occurrences: Vec<ClauseID>,
    pub(crate) positive_occurrences_count: usize,
    pub(crate) negative_occurrences_count: usize,
}

impl Variable {
    pub(crate) fn new() -> Variable {
        Variable {
            positive_watched_occurrences: vec![],
            negative_watched_occurrences: vec![],
            positive_occurrences_count: 0,
            negative_occurrences_count: 0,
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
            true => (
                &self.positive_watched_occurrences,
                &self.negative_watched_occurrences,
            ),
            false => (
                &self.negative_watched_occurrences,
                &self.positive_watched_occurrences,
            ),
        }
    }
}

/// A wrapper around `Vec<Variable>` to allow pretty string representation in the style of DIMCAS CNF.
#[derive(Clone, Debug, PartialEq)]
pub struct Variables(pub Vec<Variable>);

impl Variables {
    pub fn new(variable_count: usize) -> Variables {
        Variables(vec![Variable::new(); variable_count])
    }

    pub(crate) fn get(&self, variable_id: VariableId) -> &Variable {
        self.0.get((variable_id - 1) as usize).unwrap()
    }

    pub(crate) fn get_mut(&mut self, variable_id: VariableId) -> &mut Variable {
        self.0.get_mut((variable_id - 1) as usize).unwrap()
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
    #[allow(dead_code)]
    pub(crate) fn find_unassigned(&self, assignments: &[Option<AssignmentValue>]) -> VariableId {
        self.0
            .iter()
            .enumerate()
            .find_map(|(id, _)| match assignments[id] {
                Some(_) => None,
                None => Some(id as u32),
            })
            .unwrap()
            + 1
    }

    pub(crate) fn find_all_unassigned(
        &self,
        assignments: &[Option<AssignmentValue>],
    ) -> Vec<VariableId> {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(id, _)| match assignments[id] {
                Some(_) => None,
                None => Some(id as u32 + 1),
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
                // .filter_map(|(variable_id, _)| variable.value.map(|value| (variable_id, value)))
                .map(|(var_id, _)| format!("{}", var_id + 1))
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}
