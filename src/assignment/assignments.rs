use crate::assignment::single_assignment::SingleAssignment;
use std::fmt;
use std::fmt::Formatter;
use std::slice::Iter;

#[derive(Clone, Debug)]
pub struct Assignments(Vec<SingleAssignment>);

impl Assignments {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn iter(&self) -> Iter<SingleAssignment> {
        self.0.iter()
    }

    pub fn push(&mut self, item: SingleAssignment) {
        self.0.push(item);
    }
}

impl IntoIterator for Assignments {
    type Item = SingleAssignment;
    type IntoIter = std::vec::IntoIter<SingleAssignment>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl fmt::Display for Assignments {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|assignment| assignment.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}
