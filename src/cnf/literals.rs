use smallvec::{SmallVec, smallvec};

use crate::cdcl::assignment::AssignmentValue;
use crate::cnf::variable::VariableId;
use std::fmt;
use std::fmt::Formatter;

pub type Literal = i32;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Polarity {
    Positive,
    Negative,
}

impl Polarity {
    /// Returns the satisfying assignment value for a literals.
    /// E.g. for a literal `-3`, the variable `3` needs to be assigned `False` to satisfy the literal.
    pub fn get_satisfying_assignment(&self) -> AssignmentValue {
        self.is_positive()
    }

    pub fn reverse(&self) -> Polarity {
        match self {
            Polarity::Positive => Polarity::Negative,
            Polarity::Negative => Polarity::Positive,
        }
    }

    pub fn is_positive(&self) -> bool {
        match self {
            Polarity::Positive => true,
            Polarity::Negative => false,
        }
    }

    pub fn is_negative(&self) -> bool {
        !self.is_positive()
    }
}

pub fn to_lit((value, pol): &(VariableId, Polarity)) -> Literal {
    match pol {
        Polarity::Positive => *value as i32,
        Polarity::Negative => -(*value as i32),
    }
}

/// A wrapper around `Vec<(VariableID, Polarity)>` to allow pretty string representation in the style of DIMCAS CNF.
/// Represents a set of literals. We do not expect very large Clauses, therefore a vec should beat
/// a HashMap.
#[derive(Clone, Debug, PartialEq)]
pub struct Literals(pub SmallVec<[Literal; 6]>);

impl Literals {
    pub fn new() -> Literals {
        Literals(SmallVec::new())
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

    pub fn remove(&mut self, var: VariableId) {
        self.0.retain(|literal| literal.unsigned_abs() != var);
    }

    pub fn combine(a: &Literals, b: &Literals) -> Literals {
        let mut combined: SmallVec<[Literal; 6]> = smallvec![];
        combined.extend(a.0.iter().cloned());
        combined.extend(b.0.iter().cloned());
        combined.sort();
        combined.dedup();
        Literals(combined)
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
