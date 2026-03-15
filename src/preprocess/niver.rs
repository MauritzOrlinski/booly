use std::collections::VecDeque;

use crate::{
    cnf::cnf_formula::CnfFormula,
    preprocess::cnf::{
        clause::{Clause, ClauseID},
        cnf::CNF,
        lit::Lit,
        var::VarId,
    },
};
use itertools::{Itertools, iproduct};
use rustc_hash::FxHashMap;

fn pre_resolvant_tautology(
    clause_id_1: ClauseID,
    clause_id_2: ClauseID,
    var_id: VarId,
    cnf: &CNF,
) -> bool {
    let clause_1 = cnf.clauses.get(&clause_id_1).unwrap();
    let clause_2 = cnf.clauses.get(&clause_id_2).unwrap();
    clause_1
        .lits
        .iter()
        .any(|&lit| lit.var_id() != var_id && clause_2.lits.iter().contains(&lit.neg()))
}

pub fn niver(var_id: VarId, cnf: &mut CNF) -> (bool, Vec<ClauseID>) {
    let mut resolvants: Vec<(ClauseID, ClauseID)> = Vec::new();
    let var = cnf.vars.get(&var_id).unwrap();
    let mut niver_trace: Vec<ClauseID> = Vec::new();
    for (&pos_clause_id, &neg_clause_id) in iproduct!(&var.pos_occ, &var.neg_occ) {
        if !pre_resolvant_tautology(pos_clause_id, neg_clause_id, var_id, cnf) {
            resolvants.push((pos_clause_id, neg_clause_id));
        }
    }

    // union of pos_occ and neg_occ without duplicates
    let clause_ids_to_del: Vec<ClauseID> = var
        .pos_occ
        .iter()
        .chain(var.neg_occ.iter())
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
                let lits: Vec<Lit> = pos_clause
                    .lits
                    .iter()
                    .chain(neg_clause.lits.iter())
                    .unique()
                    .filter(|lit| lit.var_id() != var_id)
                    .cloned()
                    .collect();
                cnf.add_clause(Clause::new(lits));
            });
        for clause_id in clause_ids_to_del {
            cnf.deactivate_clause(clause_id);
            niver_trace.push(clause_id);
        }
        return (true, niver_trace);
    }
    (false, Vec::new())
}

pub fn niver_all(cnf: &mut CNF) -> VecDeque<(VarId, Vec<ClauseID>)> {
    let mut change = true;
    let mut niver_trace: VecDeque<(VarId, Vec<ClauseID>)> = VecDeque::new();
    while change {
        change = false;
        let var_ids: Vec<VarId> = cnf.vars.keys().cloned().collect();
        for var_id in var_ids {
            if cnf.vars.get(&var_id).unwrap().active {
                let (change_, niver_trace_) = niver(var_id, cnf);
                if change_ {
                    change = true;
                    niver_trace.push_back((var_id, niver_trace_));
                }
            }
        }
    }
    niver_trace
}

pub fn recover_assigment_niver(
    mut niver_trace: VecDeque<(VarId, Vec<ClauseID>)>,
    mut cnf: CNF,
    mut assignment: FxHashMap<VarId, bool>,
) -> FxHashMap<VarId, bool> {
    while let Some((var_id, clause_ids)) = niver_trace.pop_back() {
        for clause_id in clause_ids {
            cnf.clauses.get_mut(&clause_id).unwrap().active = true;
        }
        assignment.insert(var_id, true);
        if !cnf
            .clauses
            .iter()
            .filter(|(_, clause)| clause.active)
            .all(|(_, clause)| {
                clause.lits.iter().any(|lit| {
                    if lit.pos() {
                        *assignment.get(&lit.var_id()).unwrap()
                    } else {
                        !*assignment.get(&lit.var_id()).unwrap()
                    }
                })
            })
        {
            assignment.insert(var_id, false);
        }
    }
    assignment
}

pub fn recover_assigment_niver_compat(
    niver_trace: VecDeque<(VarId, Vec<ClauseID>)>,
    cnf: CNF,
    cnf_formula: CnfFormula,
) -> FxHashMap<VarId, bool> {
    let mut assignment: FxHashMap<VarId, bool> =
        FxHashMap::with_capacity_and_hasher(cnf_formula.assignments.len(), Default::default());
    for (id, v) in cnf_formula.assignments.iter().enumerate() {
        assignment.insert(id as u32 + 1, v.unwrap_or(true));
    }
    recover_assigment_niver(niver_trace, cnf, assignment)
}
