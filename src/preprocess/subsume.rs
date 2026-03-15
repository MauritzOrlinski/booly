use crate::preprocess::cnf::{clause::ClauseID, cnf::CNF};

fn subsumes(clause_id_1: ClauseID, clause_id_2: ClauseID, cnf: &CNF) -> bool {
    let clause_1 = cnf.clauses.get(&clause_id_1).unwrap();
    let clause_2 = cnf.clauses.get(&clause_id_2).unwrap();
    if clause_1.sig & !clause_2.sig != 0 {
        false
    } else {
        clause_1.lits.iter().all(|lit| clause_2.lits.contains(lit))
    }
}

pub fn subsumed(clause_id: ClauseID, cnf: &CNF) -> bool {
    let clause = cnf.clauses.get(&clause_id).unwrap();
    if clause.lits.is_empty() {
        return false;
    }
    let clause_ids = if clause.lits[0].pos() {
        &cnf.vars.get(&clause.lits[0].var_id()).unwrap().pos_occ
    } else {
        &cnf.vars.get(&clause.lits[0].var_id()).unwrap().neg_occ
    };
    clause_ids
        .iter()
        .any(|&clause_id_| clause_id_ != clause_id && subsumes(clause_id, clause_id_, cnf))
}

#[cfg(test)]
mod tests {
    use crate::{
        parser::parse,
        preprocess::{cnf::cnf::CNF, subsume::subsumed},
    };

    #[test]
    fn test_subsumed() {
        let cnf_pre = parse(
            "\
p cnf 3 2
1 2 0
1 3 2 0
",
        )
        .unwrap();
        let cnf = CNF::from_pre(&cnf_pre.0, cnf_pre.1);
        assert!(subsumed(1, &cnf));
    }
}
