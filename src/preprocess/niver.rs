use crate::preprocess::cnf::{CNF, Clause};
use itertools::{Itertools, iproduct};

//TODO: recover assigment from nivered assigment
//TODO: think about what information to save

fn ver(var_id: u32, cnf: &mut CNF) -> (bool, Vec<Vec<i64>>) {
    let mut resolvants: Vec<(u32, u32)> = vec![];
    let var = cnf.vars.get_mut(&var_id).unwrap();
    for (&pos_clause_id, &neg_clause_id) in iproduct!(&var.pos_occ, &var.neg_occ) {
        let pos_clause = &cnf.clauses.get(&pos_clause_id).unwrap();
        let neg_clause = &cnf.clauses.get(&neg_clause_id).unwrap();
        // checks for tautology
        let mut tautology = false;
        for &pos_lit in pos_clause.lits.iter() {
            if pos_lit.unsigned_abs() != var_id as u64 {
                tautology |= {
                    neg_clause
                        .lits
                        .iter()
                        .find(|&&neg_lit| pos_lit == -neg_lit)
                        .is_some()
                }
            }
        }
        if !tautology {
            resolvants.push((pos_clause_id, neg_clause_id));
        }
    }

    // union of pos_occ and neg_occ without duplicates
    let clause_ids_to_del: Vec<u32> = var
        .pos_occ
        .iter()
        .chain(var.neg_occ.iter())
        .clone()
        .unique()
        .map(|clause_id| *clause_id)
        .collect();
    // size measure: number of clauses
    if !clause_ids_to_del.is_empty() && resolvants.len() <= clause_ids_to_del.len() {
        resolvants
            .iter()
            .for_each(|(pos_clause_id, neg_clause_id)| {
                let pos_clause = cnf.clauses.get(pos_clause_id).unwrap();
                let neg_clause = cnf.clauses.get(neg_clause_id).unwrap();
                let lits: Vec<i64> = pos_clause
                    .lits
                    .iter()
                    .chain(neg_clause.lits.iter())
                    .filter_map(|lit| {
                        if lit.unsigned_abs() as u32 != var_id {
                            Some(*lit)
                        } else {
                            None
                        }
                    })
                    .unique()
                    .collect();
                cnf.add_clause(Clause::new(lits));
            });
        let niver_trace: Vec<Vec<i64>> = clause_ids_to_del
            .iter()
            .map(|&clause_id| cnf.remove_clause(clause_id).unwrap().lits)
            .collect();
        return (true, niver_trace);
    }
    (false, vec![])
}

pub fn niver(cnf: &mut CNF) -> Vec<(u32, Vec<Vec<i64>>)> {
    let mut change = true;
    let mut niver_trace: Vec<(u32, Vec<Vec<i64>>)> = Vec::new();
    while change {
        change = false;
        let var_ids: Vec<u32> = cnf.vars.keys().map(|var_id| *var_id).collect();
        for var_id in var_ids {
            if cnf.vars.get(&var_id).unwrap().active {
                let (change_, niver_trace_) = ver(var_id, cnf);
                change |= change_;
                niver_trace.push((var_id, niver_trace_));
            }
        }
    }
    niver_trace
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    #[test]
    fn ver_unit_prop() {
        let cnf_pre = parse(
            "\
p cnf 3 2
1 2 0
-1 3 0
",
        )
        .unwrap();
        let mut cnf = CNF::from_pre(&cnf_pre.0, cnf_pre.1);
        ver(1, &mut cnf);
        assert_eq!(cnf.clauses[&3].lits, vec![2, 3]);
    }
}
