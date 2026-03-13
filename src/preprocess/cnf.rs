use std::collections::{BTreeMap, VecDeque};

use itertools::Itertools;

use crate::cnf::{
    cnf_formula::CnfFormula,
    literals::{Literals, Polarity},
    variable::Variables,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Var {
    pub(crate) pos_occ: Vec<u32>,
    pub(crate) neg_occ: Vec<u32>,
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

    pub fn add_pos_occ(&mut self, clause_id: u32) {
        if !self.pos_occ.contains(&clause_id) {
            self.pos_occ.push(clause_id);
        }
    }

    pub fn add_neg_occ(&mut self, clause_id: u32) {
        if !self.neg_occ.contains(&clause_id) {
            self.neg_occ.push(clause_id);
        }
    }

    //TODO: refactor
    pub fn remove_clause(&mut self, clause_id: u32) {
        {
            let mut indices_to_remove: Vec<usize> = self
                .pos_occ
                .iter()
                .enumerate()
                .filter_map(|(index, &clause_id_)| {
                    if clause_id == clause_id_ {
                        Some(index)
                    } else {
                        None
                    }
                })
                .collect();
            indices_to_remove.sort_by(|a, b| b.cmp(a));
            indices_to_remove.iter().for_each(|&index| {
                self.pos_occ.remove(index);
            });
        }
        {
            let mut indices_to_remove: Vec<usize> = self
                .neg_occ
                .iter()
                .enumerate()
                .filter_map(|(index, &clause_id_)| {
                    if clause_id == clause_id_ {
                        Some(index)
                    } else {
                        None
                    }
                })
                .collect();
            indices_to_remove.sort_by(|a, b| b.cmp(a));
            indices_to_remove.iter().for_each(|&index| {
                self.neg_occ.remove(index);
            });
        }
        if self.pos_occ.len() + self.neg_occ.len() == 0 {
            self.active = false;
        }
    }
}

//TODO: perhaps implement better hash
pub fn lit_hash(lit: i32) -> u8 {
    lit as u8 & 0b111111
}

#[derive(Debug, PartialEq, Clone)]
pub struct Clause {
    pub(crate) sat_by: Option<u32>,
    pub(crate) lits: Vec<i32>,
    pub(crate) sig: u64,
    pub(crate) active: bool,
}

impl Clause {
    pub fn new(lits: Vec<i32>) -> Self {
        Clause {
            sat_by: None,
            sig: lits.iter().fold(0, |acc, &lit| acc | (1 << lit_hash(lit))),
            lits,
            active: true,
        }
    }
}

pub type Vars = BTreeMap<u32, Var>;
pub type Clauses = BTreeMap<u32, Clause>;

#[derive(Debug, PartialEq, Clone)]
pub struct CNF {
    pub(crate) clauses: Clauses,
    pub vars: Vars,
    max_clause_id: u32,
}

impl CNF {
    pub fn new(clauses: Clauses, vars: Vars) -> Self {
        CNF {
            max_clause_id: *clauses.keys().max().unwrap(),
            clauses: clauses,
            vars: vars,
        }
    }

    pub fn to_cnf_formula(&self) -> CnfFormula {
        let mut clauses: Vec<crate::cnf::clause::Clause> = Vec::new();
        let mut variables = Variables::new(self.vars.len());
        let mapping: BTreeMap<usize, u32> = self
            .clauses
            .iter()
            .filter_map(|(clause_id, clause)| if clause.active { Some(clause_id) } else { None })
            .sorted()
            .enumerate()
            .map(|(a, b)| (a, *b))
            .collect();
        for (clause_id, (_, clause)) in self
            .clauses
            .iter()
            .filter(|(_, clause)| clause.active)
            .enumerate()
        {
            let mut literals = Literals::new();
            for lit in clause.lits.iter() {
                literals.insert(lit.unsigned_abs(), {
                    match lit.signum() {
                        1 => Polarity::Positive,
                        -1 => Polarity::Negative,
                        _ => unreachable!("variable with id = 0 found!"),
                    }
                });
            }
            let clause = crate::cnf::clause::Clause::new(literals);
            let w1 = variables.get_mut(clause.watched1.unsigned_abs());
            if clause.watched1.is_positive() {
                w1.positive_watched_occurrences.push(clause_id);
            } else {
                w1.negative_watched_occurrences.push(clause_id);
            }
            if clause.watched1 != clause.watched2 {
                let w2 = variables.get_mut(clause.watched2.unsigned_abs());
                if clause.watched2.is_positive() {
                    w2.positive_watched_occurrences.push(clause_id);
                } else {
                    w2.negative_watched_occurrences.push(clause_id);
                }
            }
            clauses.push(clause);
        }
        for (var_id, var) in self.vars.iter() {
            let variable = variables.get_mut(*var_id);
            variable.positive_occurrences_count = var.pos_occ.len();
            variable.negative_occurrences_count = var.neg_occ.len();
        }
        CnfFormula::new(clauses, variables)
    }

    pub fn from_pre(pre_clauses: &Vec<Vec<i32>>, vars_count: u16) -> CNF {
        let mut clauses = Clauses::new();
        let mut vars: Vars = (1..=vars_count as u32)
            .map(|v_id| (v_id, Var::new()))
            .collect();

        for (clause_id, pre_clause) in pre_clauses.iter().enumerate() {
            let mut lits: Vec<i32> = Vec::new();

            // clause is tautology => don't add
            if pre_clause.iter().any(|&lit| pre_clause.contains(&(-lit))) {
                continue;
            }

            for &lit in pre_clause {
                let var_id = lit.unsigned_abs();
                let var = vars.get_mut(&var_id).unwrap();

                match lit.signum() {
                    1 => var.add_pos_occ(clause_id as u32 + 1),
                    -1 => var.add_neg_occ(clause_id as u32 + 1),
                    _ => unreachable!("variable with id = 0 found!"),
                }
                // don't add duplicate literals (fucks with clause deletion)
                if !lits.contains(&lit) {
                    lits.push(lit);
                }
            }
            clauses.insert(clause_id as u32 + 1, Clause::new(lits));
        }
        CNF::new(clauses, vars)
    }

    pub fn add_clause(&mut self, clause: Clause) {
        for &lit in &clause.lits {
            let var = self.vars.get_mut(&lit.unsigned_abs()).unwrap();
            match lit.signum() {
                1 => var.add_pos_occ(self.max_clause_id + 1),
                -1 => var.add_neg_occ(self.max_clause_id + 1),
                _ => unreachable!("variable with id = 0 found!"),
            }
        }
        self.clauses.insert(self.max_clause_id + 1, clause);
        self.max_clause_id += 1;
    }

    pub fn deactivate_clause(&mut self, clause_id: u32) {
        match self.clauses.get_mut(&clause_id) {
            Some(clause) => {
                clause.lits.iter().for_each(|lit| {
                    self.vars
                        .get_mut(&lit.unsigned_abs())
                        .unwrap()
                        .remove_clause(clause_id);
                });
                clause.active = false;
            }
            None => {}
        }
    }

    pub fn remove_lit(&mut self, clause_id: u32, lit: i32) {
        let clause = self.clauses.get_mut(&clause_id).unwrap();
        let var = self.vars.get_mut(&lit.unsigned_abs()).unwrap();

        clause.lits.retain(|&lit_| lit_ != lit);

        clause.sig &= !(1 << lit_hash(lit));

        match lit.signum() {
            1 => var.pos_occ.retain(|&clause_id_| clause_id_ != clause_id),
            -1 => var.neg_occ.retain(|&clause_id_| clause_id_ != clause_id),
            _ => unreachable!("variable with id = 0 found!"),
        }
    }

    pub fn unit_prop(&mut self) -> Vec<i32> {
        let mut clause_id_and_lit: VecDeque<(u32, i32)> = self
            .clauses
            .iter()
            .filter_map(|(clause_id, clause)| {
                if clause.lits.len() == 1 {
                    Some((*clause_id, clause.lits[0]))
                } else {
                    None
                }
            })
            .collect();
        while let Some((clause_id, lit)) = clause_id_and_lit.pop_front() {
            let var = self.vars.get(&lit.unsigned_abs()).unwrap().clone();
            match lit.signum() {
                1 => {
                    for &clause_id in var
                        .neg_occ
                        .iter()
                        .filter(|&&clause_id_| clause_id_ != clause_id)
                    {
                        self.remove_lit(clause_id, lit);
                        let clause = self.clauses.get(&clause_id).unwrap();
                        if clause.lits.len() == 1 {
                            clause_id_and_lit.push_back((clause_id, clause.lits[0]));
                        }
                    }
                }
                -1 => {
                    for &clause_id in var
                        .pos_occ
                        .iter()
                        .filter(|&&clause_id_| clause_id_ != clause_id)
                    {
                        self.remove_lit(clause_id, lit);
                        let clause = self.clauses.get(&clause_id).unwrap();
                        if clause.lits.len() == 1 {
                            clause_id_and_lit.push_back((clause_id, clause.lits[0]));
                        }
                    }
                }
                _ => unreachable!("variable with id = 0 found!"),
            }
        }
        clause_id_and_lit.iter().map(|(_, lit)| *lit).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;
    #[test]
    fn test_cnf_parser_skip_tautologies() {
        let cnf_pre = parse(
            "\
p cnf 2 2
1 2 0
-2 2 0
",
        )
        .unwrap();
        let cnf = CNF::from_pre(&cnf_pre.0, cnf_pre.1);
        assert_eq!(cnf.clauses.len(), 1);
    }

    #[test]
    fn test_cnf_parser_skip_duplicate_literals() {
        let cnf_pre = parse(
            "\
p cnf 3 1
1 2 2 0
",
        )
        .unwrap();
        let cnf = CNF::from_pre(&cnf_pre.0, cnf_pre.1);
        assert_eq!(cnf.clauses[&1].lits, vec![1, 2]);
    }
}
