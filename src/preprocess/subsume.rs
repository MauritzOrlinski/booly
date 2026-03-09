use crate::preprocess::cnf::CNF;

fn subsumes(clause_id_1: u32, clause_id_2: u32, cnf: &CNF) -> bool {
    let clause_1 = cnf.clauses.get(&clause_id_1).unwrap();
    let clause_2 = cnf.clauses.get(&clause_id_2).unwrap();
    if clause_1.sig & !clause_2.sig != 0 {
        false
    } else {
        clause_1.lits.iter().all(|lit| clause_2.lits.contains(lit))
    }
}
