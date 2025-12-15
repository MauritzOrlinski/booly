use crate::assignment::single_assignment::AssignmentValue;
use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, PartialEq, Clone)]
pub enum Polarity {
    Positive,
    Negative,
}

impl Polarity {
    pub fn get_satisfying_assignment(&self) -> AssignmentValue {
        match self {
            Polarity::Positive => AssignmentValue::True,
            Polarity::Negative => AssignmentValue::False
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Literals(HashMap<usize, Polarity>);

impl Literals {
    pub fn new(source: HashMap<usize, Polarity>) -> Literals {
        Literals(source)
    }

    pub fn get_polarity_for_var(&self, var: &usize) -> Option<&Polarity> {
        self.0.get(var)
    }

    pub fn iter(&self) -> Iter<usize, Polarity> {
        self.0.iter()
    }
}

impl fmt::Display for Literals {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|(var_id, polarity)| match polarity {
                    Polarity::Positive => format!("{}", var_id),
                    Polarity::Negative => format!("-{}", var_id),
                })
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}