use crate::preprocess::cnf::CNF;

fn subsumes(clause_id_1: u32, clause_id_2: u32, cnf: &CNF) -> bool {
    let maybe_ret = || -> Option<bool> {
        {
            let clause_1 = cnf.clauses.get(&clause_id_1)?;
            let clause_2 = cnf.clauses.get(&clause_id_2)?;
            if clause_1.sig & !clause_2.sig != 0 {
                Some(false)
            } else {
                Some(clause_1.lits.iter().all(|lit| clause_2.lits.contains(lit)))
            }
        }
    };
    maybe_ret().unwrap_or(false)
}

pub fn subsumed(clause_id: u32, cnf: &CNF) -> bool {
    cnf.clauses
        .iter()
        .any(|(&clause_id_, _)| clause_id_ != clause_id && subsumes(clause_id, clause_id_, cnf))
}
