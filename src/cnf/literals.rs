use crate::cnf::variable::VariableId;
use crate::dpll::assignment::AssignmentValue;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, PartialEq, Clone)]
pub enum Polarity {
    Positive,
    Negative,
}

impl Polarity {
    /// Returns the satisfying assignment value for a literals.
    /// E.g. for a literal `-3`, the variable `3` needs to be assigned `False` to satisfy the literal.
    pub fn get_satisfying_assignment(&self) -> AssignmentValue {
        match self {
            Polarity::Positive => true,
            Polarity::Negative => false,
        }
    }

    pub fn reverse(&self) -> Polarity {
        match self {
            Polarity::Positive => Polarity::Negative,
            Polarity::Negative => Polarity::Positive,
        }
    }
}

/// A wrapper around `Vec<(VariableID, Polarity)>` to allow pretty string representation in the style of DIMCAS CNF.
/// Represents a set of literals. We do not expect very large Clauses, therefore a vec should beat
/// a HashMap.
#[derive(Clone, Debug, PartialEq)]
pub struct Literals(pub Vec<i32>);

impl Literals {
    pub fn new() -> Literals {
        Literals(Vec::new())
    }

    pub fn iter(&'_ self) -> impl Iterator<Item = (VariableId, Polarity)> + '_ {
        self.0.iter().map(|&x| {
            if x > 0 {
                (x.unsigned_abs(), Polarity::Positive)
            } else {
                (x.unsigned_abs(), Polarity::Negative)
            }
        })
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, var: VariableId, polarity: Polarity) {
        let var = var as i32;
        let var = match polarity {
            Polarity::Positive => var,
            Polarity::Negative => -var,
        };
        self.0.push(var);
    }
}

impl Default for Literals {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Literals {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|x| format!("{}", x),)
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}
