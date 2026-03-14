use std::collections::{BTreeMap, VecDeque};

use crate::{
    cnf::{
        cnf_formula::CnfFormula,
        literals::{Literals, Polarity},
        variable::Variables,
    },
    preprocess::cnf::{
        clause::{Clause, ClauseID},
        lit::Lit,
        misc::remove_element,
        var::{Var, VarId},
    },
};

pub type Vars = BTreeMap<VarId, Var>;
pub type Clauses = BTreeMap<ClauseID, Clause>;

#[derive(Debug, PartialEq, Clone)]
pub struct CNF {
    pub clauses: Clauses,
    pub vars: Vars,
    max_clause_id: ClauseID,
}

impl CNF {
    pub fn new(clauses: Clauses, vars: Vars) -> Self {
        CNF {
            max_clause_id: *clauses.keys().max().unwrap(),
            clauses: clauses,
            vars: vars,
        }
    }

    pub fn add_clause(&mut self, clause: Clause) {
        for lit in clause.lits.iter() {
            let var = self.vars.get_mut(&lit.var_id()).unwrap();
            if lit.pos() {
                var.add_pos_occ(self.max_clause_id + 1);
            } else {
                var.add_neg_occ(self.max_clause_id + 1);
            }
        }
        self.clauses.insert(self.max_clause_id + 1, clause);
        self.max_clause_id += 1;
    }

    pub fn deactivate_clause(&mut self, clause_id: ClauseID) {
        let clause = self.clauses.get_mut(&clause_id).unwrap();
        clause.lits.iter().for_each(|lit| {
            self.vars
                .get_mut(&lit.var_id())
                .unwrap()
                .remove_clause(clause_id);
        });
        clause.active = false;
    }

    pub fn remove_lit(&mut self, clause_id: ClauseID, lit: Lit) {
        let clause = self.clauses.get_mut(&clause_id).unwrap();
        let var = self.vars.get_mut(&lit.var_id()).unwrap();

        remove_element(&mut clause.lits, lit);

        clause.sig &= !(1 << lit.hash());

        if lit.pos() {
            remove_element(&mut var.pos_occ, clause_id);
        } else {
            remove_element(&mut var.neg_occ, clause_id);
        }
    }

    //TODO: find other units
    pub fn unit_prop(&mut self) -> Vec<Lit> {
        let mut clause_id_and_lit: VecDeque<(ClauseID, Lit)> = self
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
            let var = self.vars.get(&lit.var_id()).unwrap().clone();
            let occ = if lit.pos() {
                &var.neg_occ
            } else {
                &var.pos_occ
            };
            for &clause_id in occ.iter().filter(|&&clause_id_| clause_id_ != clause_id) {
                self.remove_lit(clause_id, lit);
                let clause = self.clauses.get(&clause_id).unwrap();
                if clause.lits.len() == 1 {
                    clause_id_and_lit.push_back((clause_id, clause.lits[0]));
                }
            }
        }
        clause_id_and_lit.iter().map(|(_, lit)| *lit).collect()
    }

    pub fn to_cnf_formula(&self) -> CnfFormula {
        let mut clauses: Vec<crate::cnf::clause::Clause> = Vec::new();
        let mut variables = Variables::new(self.vars.len());
        for (clause_id, (_, clause)) in self
            .clauses
            .iter()
            .filter(|(_, clause)| clause.active)
            .enumerate()
        {
            let mut literals = Literals::new();
            for lit in clause.lits.iter() {
                literals.insert(lit.var_id(), {
                    if lit.pos() {
                        Polarity::Positive
                    } else {
                        Polarity::Negative
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
        CnfFormula::new(clauses, variables)
    }

    pub fn from_pre(pre_clauses: &Vec<Vec<i32>>, vars_count: u16) -> CNF {
        let mut clauses = Clauses::new();
        let mut vars: Vars = (1..=vars_count as u32)
            .map(|v_id| (v_id, Var::new()))
            .collect();

        for (clause_id, pre_clause) in pre_clauses.iter().enumerate() {
            let mut lits: Vec<Lit> = Vec::new();

            // clause is tautology => don't add
            if pre_clause.iter().any(|&lit| pre_clause.contains(&(-lit))) {
                continue;
            }

            for &pre_lit in pre_clause {
                let var_id = pre_lit.unsigned_abs();
                let var = vars.get_mut(&var_id).unwrap();

                match pre_lit.signum() {
                    1 => var.add_pos_occ(clause_id as u32 + 1),
                    -1 => var.add_neg_occ(clause_id as u32 + 1),
                    _ => unreachable!("variable with id = 0 found!"),
                }
                // don't add duplicate literals (fucks with clause deletion)
                if !lits.contains(&Lit::new(pre_lit)) {
                    lits.push(Lit::new(pre_lit));
                }
            }
            clauses.insert(clause_id as u32 + 1, Clause::new(lits));
        }
        CNF::new(clauses, vars)
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
        assert_eq!(cnf.clauses[&1].lits, vec![Lit::new(1), Lit::new(2)]);
    }
}
