use crate::{
    cnf::cnf_formula::CnfFormula,
    preprocess::cnf::{CNF, Clause},
};
use itertools::{Itertools, iproduct};
use std::collections::{BTreeMap, VecDeque};

pub fn ver(var_id: u32, cnf: &mut CNF) -> (bool, Vec<Vec<i32>>) {
    let mut resolvants: Vec<(u32, u32)> = vec![];
    let var = cnf.vars.get_mut(&var_id).unwrap();
    for (&pos_clause_id, &neg_clause_id) in iproduct!(&var.pos_occ, &var.neg_occ) {
        let pos_clause = &cnf.clauses.get(&pos_clause_id).unwrap();
        let neg_clause = &cnf.clauses.get(&neg_clause_id).unwrap();
        // checks for tautology
        let mut tautology = false;
        for &pos_lit in pos_clause.lits.iter() {
            if pos_lit.unsigned_abs() != var_id {
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
        .cloned()
        .collect();
    // size measure: number of clauses
    if !clause_ids_to_del.is_empty() && resolvants.len() <= clause_ids_to_del.len() {
        resolvants
            .iter()
            .for_each(|(pos_clause_id, neg_clause_id)| {
                let pos_clause = cnf.clauses.get(pos_clause_id).unwrap();
                let neg_clause = cnf.clauses.get(neg_clause_id).unwrap();
                let lits: Vec<i32> = pos_clause
                    .lits
                    .iter()
                    .chain(neg_clause.lits.iter())
                    .filter(|lit| lit.unsigned_abs() != var_id)
                    .cloned()
                    .unique()
                    .collect();
                cnf.add_clause(Clause::new(lits));
            });
        let niver_trace: Vec<Vec<i32>> = clause_ids_to_del
            .iter()
            .map(|&clause_id| cnf.remove_clause(clause_id).unwrap().lits)
            .collect();
        return (true, niver_trace);
    }
    (false, vec![])
}

pub fn niver(cnf: &mut CNF) -> VecDeque<(u32, Vec<Vec<i32>>)> {
    let mut change = true;
    let mut niver_trace: VecDeque<(u32, Vec<Vec<i32>>)> = VecDeque::new();
    while change {
        change = false;
        let var_ids: Vec<u32> = cnf.vars.keys().cloned().collect();
        for var_id in var_ids {
            if cnf.vars.get(&var_id).unwrap().active {
                let (change_, niver_trace_) = ver(var_id, cnf);
                change |= change_;
                niver_trace.push_back((var_id, niver_trace_));
            }
        }
    }
    niver_trace
}

pub fn recover_assigment_niver(
    mut niver_trace: VecDeque<(u32, Vec<Vec<i32>>)>,
    mut clauses: Vec<Vec<i32>>,
    mut assignment: BTreeMap<u32, bool>,
) -> BTreeMap<u32, bool> {
    while let Some((var_id, mut niver_clauses)) = niver_trace.pop_front() {
        clauses.append(&mut niver_clauses);
        assignment.insert(var_id, true);
        if !clauses.iter().all(|clause| {
            clause.iter().any(|lit| {
                if lit.signum() == 1 {
                    *assignment.get(&lit.unsigned_abs()).unwrap()
                } else {
                    !*assignment.get(&lit.unsigned_abs()).unwrap()
                }
            })
        }) {
            assignment.insert(var_id, false);
        }
    }
    assignment
}

pub fn recover_assigment_niver_compat(
    niver_trace: VecDeque<(u32, Vec<Vec<i32>>)>,
    cnf_formula: CnfFormula,
) -> BTreeMap<u32, bool> {
    let mut clauses: Vec<Vec<i32>> = Vec::new();
    for c in cnf_formula.clauses {
        clauses.push(c.literals.0.iter().cloned().collect());
    }
    let mut assignment: BTreeMap<u32, bool> = BTreeMap::new();
    for (id, v) in cnf_formula.variables.iter().enumerate() {
        assignment.insert(id as u32 + 1, v.value.unwrap_or(true));
    }
    recover_assigment_niver(niver_trace, clauses, assignment)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        dpll::{
            dpll::{Dpll, DpllStatus},
            heuristics::trivial::Trivial,
        },
        parser::parse,
    };

    #[test]
    fn niver_test_sat() {
        let cnf_pre = parse(include_str!("../../inputs/sat/aim-100-3_4-yes1-1.cnf")).unwrap();
        let mut cnf = CNF::from_pre(&cnf_pre.0, cnf_pre.1);
        let niver_trace = niver(&mut cnf);

        let heuristic = Box::new(Trivial);
        let mut dpll = Dpll::new(cnf.to_cnf_formula(), heuristic);
        let status = dpll.solve();

        assert_eq!(status, DpllStatus::Sat);

        let assignment = recover_assigment_niver_compat(niver_trace, dpll.cnf_formula);
        let cnf = CNF::from_pre(&cnf_pre.0, cnf_pre.1);
        assert!(cnf.clauses.iter().all(|(_, clause)| {
            clause.lits.iter().any(|lit| {
                if lit.signum() == 1 {
                    *assignment.get(&lit.unsigned_abs()).unwrap()
                } else {
                    !*assignment.get(&lit.unsigned_abs()).unwrap()
                }
            })
        }))
    }

    #[test]
    fn ver_does_unit_prop() {
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

    #[test]
    fn cnf_parser_skip_tautologies() {
        let cnf_pre = parse(
            "\
p cnf 3 3
1 2 0
-1 3 0
-2 2 0
",
        )
        .unwrap();
        let mut cnf = CNF::from_pre(&cnf_pre.0, cnf_pre.1);
        ver(1, &mut cnf);
        assert_eq!(cnf.clauses[&3].lits, vec![2, 3]);
    }

    #[test]
    fn cnf_parser_skip_duplicate_literals() {
        let cnf_pre = parse(
            "\
p cnf 3 2
1 2 2 0
-1 3 0
",
        )
        .unwrap();
        let mut cnf = CNF::from_pre(&cnf_pre.0, cnf_pre.1);
        ver(1, &mut cnf);
        assert_eq!(cnf.clauses[&3].lits, vec![2, 3]);
    }
}
