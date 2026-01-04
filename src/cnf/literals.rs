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
}

/// A wrapper around `Vec<(VariableID, Polarity)>` to allow pretty string representation in the style of DIMCAS CNF.
/// Represents a set of literals. We do not expect very large Clauses, therefore a vec should beat
/// a HashMap.
#[derive(Clone, Debug, PartialEq)]
pub struct Literals(Vec<i32>);

impl Literals {
    pub fn new() -> Literals {
        Literals(Vec::new())
    }

    // TODO: This is potentially wrong, if a clause has a variable with positive and negative
    // polarity
    // pub fn get_polarity_for_var(&self, var: &VariableId) -> Option<&Polarity> {
    //     self.0.iter().find_map(|&x| {
    //         if x.unsigned_abs() == *var {
    //             Some(if x > 0 {
    //                 &Polarity::Positive
    //             } else {
    //                 &Polarity::Negative
    //             })
    //         } else {
    //             None
    //         }
    //     })
    // }

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
