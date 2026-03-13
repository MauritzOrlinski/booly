use std::collections::VecDeque;

use itertools::Itertools;

use crate::preprocess::{cnf::CNF, niver::ver, selfsubsume::selfsubsumes, subsume::subsumed};

#[derive(Debug, PartialEq, Clone)]
enum Reason {
    Add,
    Strengthen,
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
enum Round {
    Prev,
    Curr,
}

#[derive(Debug, PartialEq, Clone)]
struct RoundTrace {
    reason: Reason,
    round: Round,
}

impl RoundTrace {
    fn new(reason: Reason, round: Round) -> RoundTrace {
        RoundTrace {
            reason: reason,
            round: round,
        }
    }
}

pub fn preprocess(cnf: &mut CNF) {
    let mut round_traces: VecDeque<RoundTrace> = cnf
        .clauses
        .keys()
        .map(|&clause_id| RoundTrace::new(clause_id, Reason::Add, Round::Prev))
        .collect();
    let mut niver_vars: VecDeque<(u32, Round)> = VecDeque::new();
    let mut niver_trace: VecDeque<(u32, Vec<Vec<i32>>)> = VecDeque::new();
    let mut change: u8 = 0b11;

    while change == 0b11 {
        change = 0;
        let mut clauses: VecDeque<u32> = round_traces
            .iter()
            .map(|round_trace| round_trace.clause_id)
            .collect();
        while let Some(clause_id) = clauses.pop_front() {
            if selfsubsumes(clause_id, cnf) {
                println!("SELFSUBSUME");
                change |= 0b10;
                clauses.push_back(clause_id);
                round_traces.push_back(RoundTrace::new(clause_id, Reason::Strengthen, Round::Curr));
                //TODO: unit prpop should return the literals of removed clauses
                cnf.unit_prop();
            }
        }

        for round_trace in round_traces.iter() {
            if round_trace.reason == Reason::Add
                && round_trace.round == Round::Prev
                && subsumed(round_trace.clause_id, cnf)
            {
                println!("SUBSUME");
                change |= 0b10;
                niver_vars.append(
                    &mut cnf
                        .remove_clause(round_trace.clause_id)
                        .unwrap()
                        .lits
                        .iter()
                        .map(|&lit| (lit.unsigned_abs(), Round::Curr))
                        .collect(),
                );
            }
        }

        round_traces.iter().for_each(|round_trace| {
            niver_vars.append(
                &mut cnf
                    .clauses
                    .get(&round_trace.clause_id)
                    .unwrap()
                    .lits
                    .iter()
                    .map(|&lit| (lit.unsigned_abs(), round_trace.round.clone()))
                    .collect(),
            );
        });

        for (var_id, _) in niver_vars.iter().unique_by(|(var_id, _)| var_id) {
            let var = cnf.vars.get(var_id).unwrap();
            if std::cmp::min(var.pos_occ.len(), var.neg_occ.len()) > 10 {
                continue;
            }
            let ver_res = ver(*var_id, cnf);
            niver_trace.push_back((*var_id, ver_res.1));
            if ver_res.0 {
                println!("NIVER");
                change |= 0b01;
            }
        }

        round_traces.retain(|round_trace| round_trace.round == Round::Curr);
        for round_trace in round_traces.iter_mut() {
            round_trace.round = Round::Prev;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    #[test]
    fn foo() {
        let cnf_pre = parse(include_str!("../../inputs/sat/aim-100-1_6-yes1-1.cnf")).unwrap();
        let mut cnf = CNF::from_pre(&cnf_pre.0, cnf_pre.1);
        preprocess(&mut cnf);
        // for (_, clause) in cnf.clauses {
        //     for lit in clause.lits {
        //         print!("{} ", lit);
        //     }
        //     println!("0");
        // }
    }
}
