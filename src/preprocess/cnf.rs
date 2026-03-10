use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Var {
    pub(crate) val: Option<bool>,
    pub(crate) pos_occ: Vec<u32>,
    pub(crate) neg_occ: Vec<u32>,
    pub(crate) active: bool,
}

impl Var {
    pub(crate) fn new() -> Var {
        Var {
            val: None,
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
fn lit_hash(lit: i64) -> u8 {
    lit as u8 & 0b111111
}

#[derive(Debug, PartialEq, Clone)]
pub struct Clause {
    pub(crate) sat_by: Option<u32>,
    pub(crate) lits: Vec<i64>,
    pub(crate) act: u8,
    pub(crate) sig: u64,
}

impl Clause {
    pub fn new(lits: Vec<i64>) -> Self {
        Clause {
            sat_by: None,
            act: lits.len() as u8,
            sig: lits.iter().fold(0, |acc, &lit| acc | (1 << lit_hash(lit))),
            lits,
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

    pub fn from_pre(pre_clauses: &Vec<Vec<i32>>, vars_count: u16) -> CNF {
        let mut clauses = Clauses::new();
        let mut vars: Vars = (1..=vars_count as u32)
            .map(|v_id| (v_id, Var::new()))
            .collect();

        for (clause_id, pre_clause) in pre_clauses.iter().enumerate() {
            let mut lits: Vec<i64> = Vec::new();

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
                if !lits.contains(&(lit as i64)) {
                    lits.push(lit as i64);
                }
            }
            clauses.insert(clause_id as u32 + 1, Clause::new(lits));
        }
        CNF::new(clauses, vars)
    }

    pub fn add_clause(&mut self, clause: Clause) {
        for &lit in &clause.lits {
            let var = self.vars.get_mut(&(lit.unsigned_abs() as u32)).unwrap();
            match lit.signum() {
                1 => var.add_pos_occ(self.max_clause_id + 1),
                -1 => var.add_neg_occ(self.max_clause_id + 1),
                _ => unreachable!("variable with id = 0 found!"),
            }
        }
        self.clauses.insert(self.max_clause_id + 1, clause);
        self.max_clause_id += 1;
    }

    pub fn remove_clause(&mut self, clause_id: u32) -> Option<Clause> {
        match self.clauses.get(&clause_id) {
            Some(clause) => clause.lits.iter().for_each(|lit| {
                self.vars
                    .get_mut(&(lit.unsigned_abs() as u32))
                    .unwrap()
                    .remove_clause(clause_id);
            }),
            None => {}
        }
        self.clauses.remove(&clause_id)
    }
}
