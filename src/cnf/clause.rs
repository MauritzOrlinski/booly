use crate::cnf::literals::{to_lit, Literal, Literals, Polarity};
use crate::cnf::variable::{VariableId, Variables};
use std::fmt;
use std::fmt::Formatter;

pub type ClauseID = usize;

/// The clause implementation
///
/// # Fields
/// * `satisfied_by` - The variable this clause has first been satisfied by. None, if the clause is not yet satisfied.
/// * `literals` - The set of literals in this clause
/// * `unassigned_variables` - The count of how many variables occur in this clause that have not yet been assigned a value.
/// * `watched1` and `watched2` - The currently watched literals, which are always unassigned
#[derive(Debug, PartialEq, Clone)]
pub struct Clause {
    pub(crate) satisfied_by: Option<VariableId>,
    pub(crate) literals: Literals,
    pub(crate) watched1: Literal,
    pub(crate) watched2: Literal,
}

impl Clause {
    pub fn new(literals: Literals, variables: &mut Variables, clause_id: usize) -> Self {
        // Safety note: Assumes at least one element
        let w1 = literals.iter().last().unwrap();
        let w2 = literals.iter().find(|x| x.0 != w1.0).unwrap_or(w1); // picks

        let w1_var = variables.get_mut(w1.0);

        if w1.1.is_positive() {
            w1_var.positive_watched_occurrences.push(clause_id);
        } else {
            w1_var.negative_watched_occurrences.push(clause_id);
        }
        if w1 != w2 {
            let w2_var = variables.get_mut(w2.0);
            if w2.1.is_positive() {
                w2_var.positive_watched_occurrences.push(clause_id);
            } else {
                w2_var.negative_watched_occurrences.push(clause_id);
            }
        }

        // first different lit
        Clause {
            satisfied_by: None,
            literals,
            watched1: to_lit(&w1),
            watched2: to_lit(&w2),
        }
    }

    pub fn is_watched(&self, var: i32) -> bool {
        var == self.watched1 || var == self.watched2
    }

    pub fn is_unit(&self, assignments: &[Option<bool>]) -> bool {
        let t = Some(self.watched2.is_negative());
        assignments[self.watched2.unsigned_abs() as usize - 1] == t
    }

    pub fn is_satisfied_by_watched(&self, assignments: &[Option<bool>]) -> bool {
        assignments[self.watched1.unsigned_abs() as usize - 1] == Some(self.watched1.is_positive())
            || assignments[self.watched2.unsigned_abs() as usize - 1]
                == Some(self.watched2.is_positive())
    }

    pub fn is_conflict_clause(&self, assignments: &[Option<bool>]) -> bool {
        assignments[self.watched1.unsigned_abs() as usize - 1] == Some(self.watched1.is_negative())
            && assignments[self.watched2.unsigned_abs() as usize - 1]
                == Some(self.watched2.is_negative())
    }
}

impl fmt::Display for Clause {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} 0", self.literals)
    }
}
