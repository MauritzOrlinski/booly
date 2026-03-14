use crate::preprocess::cnf::{clause::ClauseID, misc::remove_element};

pub type VarId = u32;

#[derive(Debug, PartialEq, Clone)]
pub struct Var {
    pub(crate) pos_occ: Vec<ClauseID>,
    pub(crate) neg_occ: Vec<ClauseID>,
    pub(crate) active: bool,
}

impl Var {
    pub(crate) fn new() -> Var {
        Var {
            pos_occ: vec![],
            neg_occ: vec![],
            active: true,
        }
    }

    pub fn add_pos_occ(&mut self, clause_id: ClauseID) {
        if !self.pos_occ.contains(&clause_id) {
            self.pos_occ.push(clause_id);
        }
    }

    pub fn add_neg_occ(&mut self, clause_id: ClauseID) {
        if !self.neg_occ.contains(&clause_id) {
            self.neg_occ.push(clause_id);
        }
    }

    pub fn remove_clause(&mut self, clause_id: ClauseID) {
        remove_element(&mut self.pos_occ, clause_id);
        remove_element(&mut self.neg_occ, clause_id);
        if self.pos_occ.len() + self.neg_occ.len() == 0 {
            self.active = false;
        }
    }
}
