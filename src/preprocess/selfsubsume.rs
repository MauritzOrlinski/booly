use crate::preprocess::cnf::{clause::ClauseID, cnf::CNF, lit::Lit};

fn bloom_check(clause_sig1: u128, clause_sig2: u128, lit: Lit) -> bool {
    (clause_sig1 & !(1u128 << lit.neg().hash1())) & !(clause_sig2 & !(1u128 << lit.hash1())) != 0
}

// C_1 \ -a subset C_2 \ a
fn selfsubsumes_aux(clause_id_1: ClauseID, clause_id_2: ClauseID, lit: Lit, cnf: &CNF) -> bool {
    let clause_1 = cnf.clauses.get(&clause_id_1).unwrap();
    let clause_2 = cnf.clauses.get(&clause_id_2).unwrap();

    if bloom_check(clause_1.sig, clause_2.sig, lit) {
        false
    } else {
        clause_1
            .lits
            .iter()
            .all(|lit_| lit_.neg() == lit || clause_2.lits.contains(lit_))
    }
}

pub fn selfsubsumes(clause_id: ClauseID, cnf: &mut CNF) -> bool {
    let clause = cnf.clauses.get(&clause_id).unwrap();
    let sub_clause_and_lit = clause.lits.iter().find_map(|&lit| {
        let var = cnf.vars.get(&lit.var_id()).unwrap();
        let occ = if lit.pos() {
            &var.neg_occ
        } else {
            &var.pos_occ
        };
        occ.iter().find_map(|&clause_id_| {
            if selfsubsumes_aux(clause_id_, clause_id, lit, cnf) {
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
}

#[cfg(test)]
mod tests {
    use crate::{
        parser::parse,
        preprocess::{cnf::cnf::CNF, cnf::lit::Lit, selfsubsume::selfsubsumes},
    };

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
        assert_eq!(cnf.clauses[&1].lits, vec![Lit::new(3), Lit::new(2)]);
    }
}
