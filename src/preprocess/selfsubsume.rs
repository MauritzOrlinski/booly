use crate::preprocess::cnf::{CNF, lit_hash};

// C_1 \ -a subset C_2 \ a
fn selfsubsumes_aux(clause_id_1: u32, clause_id_2: u32, lit: i32, cnf: &CNF) -> bool {
    let clause_1 = cnf.clauses.get(&clause_id_1).unwrap();
    let clause_2 = cnf.clauses.get(&clause_id_2).unwrap();

    if !clause_1.lits.contains(&-lit) {
        return false;
    }

    if (clause_1.sig & !(1 << lit_hash(-lit))) & !(clause_2.sig & !(1 << lit_hash(lit))) != 0 {
        false
    } else {
        clause_1
            .lits
            .iter()
            .all(|lit_| -lit_ == lit || clause_2.lits.contains(lit_))
    }
}

pub fn selfsubsumes(clause_id: u32, cnf: &mut CNF) -> bool {
    if let Some(clause) = cnf.clauses.get(&clause_id) {
        let sub_clause_and_lit = clause.lits.iter().find_map(|&lit| {
            cnf.clauses.iter().find_map(|(&clause_id_, _)| {
                if clause_id_ != clause_id && selfsubsumes_aux(clause_id_, clause_id, lit, cnf) {
                    Some((clause_id, lit))
                } else {
                    None
                }
            })
        });
        if let Some((sub_clause, sub_lit)) = sub_clause_and_lit {
            cnf.remove_lit(sub_clause, sub_lit);
            true
        } else {
            false
        }
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parser::parse, preprocess::selfsubsume::selfsubsumes};

    #[test]
    fn test_selfsubsumes() {
        let cnf_pre = parse(
            "\
p cnf 3 2
1 2 3 0
-1 2 0
",
        )
        .unwrap();
        let mut cnf = CNF::from_pre(&cnf_pre.0, cnf_pre.1);
        selfsubsumes(1, &mut cnf);
        assert_eq!(cnf.clauses[&1].lits, vec![2, 3]);
    }
}
