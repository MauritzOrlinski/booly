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

pub fn subsumed(clause_id: u32, cnf: &CNF) -> bool {
    cnf.clauses
        .iter()
        .any(|(&clause_id_, _)| clause_id_ != clause_id && subsumes(clause_id, clause_id_, cnf))
}

#[cfg(test)]
mod tests {
    use crate::{
        parser::parse,
        preprocess::{cnf::CNF, subsume::subsumed},
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
