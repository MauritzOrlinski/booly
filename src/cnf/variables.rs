use crate::cnf::assignment::AssignmentValue;
use crate::cnf::variable::Variable;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

#[derive(Clone, Debug, PartialEq)]
pub struct Variables(HashMap<usize, Variable>);

impl Variables {
    pub fn new() -> Variables {
        Variables(HashMap::new())
    }

    pub(crate) fn get(&self, variable_id: &usize) -> Option<&Variable> {
        self.0.get(variable_id)
    }

    pub fn get_or_create(&mut self, variable_id: &usize) -> &mut Variable {
        self.0.entry(*variable_id).or_insert(Variable::new())
    }

    pub(crate) fn get_mut(&mut self, variable_id: &usize) -> Option<&mut Variable> {
        self.0.get_mut(variable_id)
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }
}

impl fmt::Display for Variables {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .filter_map(|(variable_id, variable)| variable
                    .value
                    .map(|value| (variable_id, value)))
                .map(|(var_id, variable_value)| format!(
                    "{}{}",
                    match variable_value {
                        AssignmentValue::True => "",
                        AssignmentValue::False => "-",
                    },
                    var_id
                ))
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}
