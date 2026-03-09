use crate::preprocess::cnf::{CNF, Clause};
use itertools::{Itertools, iproduct};

//TODO: recover assigment from nivered assigment
//TODO: think about what information to save

fn ver(var_id: u32, cnf: &mut CNF) -> (bool, Vec<Vec<i64>>) {
    let mut resolvants: Vec<(u32, u32)> = vec![];
    let var = cnf.vars.get_mut(&var_id).unwrap();
    for (&pos_clause_id, &neg_clause_id) in iproduct!(&var.pos_occ, &var.neg_occ) {
        // TODO: Here something breaks
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
    fn foo() {
        let cnf_pre = parse(
            "\
c description: Graph 3-Colorability of graph: Random 2-regular graph of 10 vertices + planted 3-clique
c generator: CNFgen (0.9.0)
c copyright: (C) 2012-2020 Massimo Lauria <massimo.lauria@uniroma1.it>
c url: https://massimolauria.net/cnfgen
c command line: cnfgen kcolor 3 gnd 10 2 plantclique 3
c
p cnf 30 79
1 2 3 0
-1 -2 0
-1 -3 0
-2 -3 0
4 5 6 0
-4 -5 0
-4 -6 0
-5 -6 0
7 8 9 0
-7 -8 0
-7 -9 0
-8 -9 0
10 11 12 0
-10 -11 0
-10 -12 0
-11 -12 0
13 14 15 0
-13 -14 0
-13 -15 0
-14 -15 0
16 17 18 0
-16 -17 0
-16 -18 0
-17 -18 0
19 20 21 0
-19 -20 0
-19 -21 0
-20 -21 0
22 23 24 0
-22 -23 0
-22 -24 0
-23 -24 0
25 26 27 0
-25 -26 0
-25 -27 0
-26 -27 0
28 29 30 0
-28 -29 0
-28 -30 0
-29 -30 0
-1 -7 0
-2 -8 0
-3 -9 0
-1 -22 0
-2 -23 0
-3 -24 0
-4 -16 0
-5 -17 0
-6 -18 0
-4 -25 0
-5 -26 0
-6 -27 0
-13 -10 0
-14 -11 0
-15 -12 0
-13 -25 0
-14 -26 0
-15 -27 0
-13 -28 0
-14 -29 0
-15 -30 0
-16 -7 0
-17 -8 0
-18 -9 0
-16 -10 0
-17 -11 0
-18 -12 0
-16 -13 0
-17 -14 0
-18 -15 0
-19 -10 0
-20 -11 0
-21 -12 0
-22 -19 0
-23 -20 0
-24 -21 0
-28 -10 0
-29 -11 0
-30 -12 0
",
        )
        .unwrap();
        let mut cnf = CNF::from_pre(&cnf_pre.0, cnf_pre.1);
        println!("trace {:?}", niver(&mut cnf));
        println!("niver: {:?}", cnf);
    }
}
