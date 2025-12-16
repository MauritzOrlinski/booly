use crate::cnf::assignment::AssignmentValue;
use crate::cnf::variable::Variable;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

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
                    var_id
                ))
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}
