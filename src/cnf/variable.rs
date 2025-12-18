use crate::dpll::assignment::AssignmentValue;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub(crate) value: Option<AssignmentValue>,
    pub(crate) positive_occurrences: Vec<usize>,
    pub(crate) negative_occurrences: Vec<usize>,
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
    pub(crate) fn associated_clauses(&self, value: AssignmentValue) -> (&[usize], &[usize]) {
        match value {
            AssignmentValue::True => (&self.positive_occurrences, &self.negative_occurrences),
            AssignmentValue::False => (&self.negative_occurrences, &self.positive_occurrences),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Variables(Vec<Variable>);

impl Variables {
    pub fn new(variable_count: usize) -> Variables {
        Variables(vec![Variable::new(); variable_count])
    }

    pub(crate) fn get(&self, variable_id: usize) -> &Variable {
        self.0.get(variable_id - 1).unwrap()
    }

    pub(crate) fn get_mut(&mut self, variable_id: usize) -> &mut Variable {
        self.0.get_mut(variable_id - 1).unwrap()
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn find_unassigned(&self) -> usize {
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
