use std::collections::{BTreeMap, VecDeque};

use itertools::Itertools;

use crate::preprocess::{cnf::CNF, niver::niver, selfsubsume::selfsubsumes, subsume::subsumed};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Reason {
    Add,
    Del,
    Stren,
}

#[derive(Debug, PartialEq, Clone, Eq, Hash, Copy)]
enum Round {
    Prev,
    Curr,
    Never,
}

#[derive(Debug, PartialEq, Clone, Copy)]
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

    fn recently(self) -> bool {
        self.round != Round::Never
    }
}

pub fn preprocess(cnf: &mut CNF) -> VecDeque<(u32, Vec<u32>)> {
    let mut round_traces: BTreeMap<u32, RoundTrace> = cnf
        .clauses
        .keys()
        .map(|&clause_id| (clause_id, RoundTrace::new(Reason::Add, Round::Prev)))
        .collect();
    let mut niver_trace: VecDeque<(u32, Vec<u32>)> = VecDeque::new();
    let mut niver_vars: VecDeque<u32> = VecDeque::new();
    let mut change: u8 = 0b11;

    while change == 0b11 {
        change = 0;

        let mut clauses: VecDeque<u32> = round_traces
            .iter()
            .filter_map(|(clause_id, round_trace)| {
                if round_trace.recently() {
                    Some(clause_id)
                } else {
                    None
                }
            })
            .copied()
            .collect();

        while let Some(clause_id) = clauses.pop_front() {
            if selfsubsumes(clause_id, cnf) {
                change |= 0b10;
                clauses.push_back(clause_id);
                let round_trace = round_traces.get_mut(&clause_id).unwrap();
                round_trace.reason = Reason::Stren;
                round_trace.round = Round::Curr;
                niver_vars.append(
                    &mut cnf
                        .unit_prop()
                        .iter()
                        .map(|lit| lit.unsigned_abs())
                        .collect(),
                );
            }
        }

        for (&clause_id, round_trace) in round_traces.iter_mut() {
            if round_trace.reason == Reason::Add
                && round_trace.round == Round::Prev
                && subsumed(clause_id, cnf)
            {
                change |= 0b10;
                round_trace.round = Round::Curr;
                round_trace.reason = Reason::Del;
            }
        }

        for (clause_id, round_trace) in round_traces.iter() {
            if round_trace.recently() {
                niver_vars.append(
                    &mut cnf
                        .clauses
                        .get(&clause_id)
                        .unwrap()
                        .lits
                        .iter()
                        .map(|&lit| lit.unsigned_abs())
                        .collect(),
                );
            }
        }
        //TODO: remove duplicates in niver_vars?
        while let Some(var_id) = niver_vars.pop_front() {
            let var = cnf.vars.get(&var_id).unwrap();
            if std::cmp::min(var.pos_occ.len(), var.neg_occ.len()) > 10 {
                continue;
            }
            let (niver_change, niver_trace_) = niver(var_id, cnf);
            if niver_change {
                for clause_id in niver_trace_.iter() {
                    round_traces.insert(*clause_id, RoundTrace::new(Reason::Add, Round::Curr));
                }
                niver_trace.push_back((var_id, niver_trace_));
                niver_vars.push_back(var_id);
                change |= 0b01;
            }
        }

        for (_, round_trace) in round_traces.iter_mut() {
            match round_trace.round {
                Round::Curr => round_trace.round = Round::Prev,
                _ => round_trace.round = Round::Never,
            }
        }
    }
    niver_trace
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
        preprocess::niver::recover_assigment_niver_compat,
    };

    #[test]
    fn test_preprocess_sat() {
        let cnf_pre = parse(include_str!("../../inputs/sat/aim-200-2_0-yes1-2.cnf")).unwrap();

        let mut cnf = CNF::from_pre(&cnf_pre.0, cnf_pre.1);
        let niver_trace = preprocess(&mut cnf);

        let heuristic = Box::new(Trivial);
        let mut dpll = Dpll::new(cnf.to_cnf_formula(), heuristic);
        let status = dpll.solve();

        assert_eq!(status, DpllStatus::Sat);

        let assignment = recover_assigment_niver_compat(niver_trace, cnf, dpll.cnf_formula);
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
}
