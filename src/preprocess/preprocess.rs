use std::collections::{BTreeMap, VecDeque};

use itertools::Itertools;

use crate::preprocess::{
    cnf::{clause::ClauseID, cnf::CNF, var::VarId},
    niver::niver,
    selfsubsume::selfsubsumes,
    subsume::subsumed,
};

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

// SatELite Preprocessor of the lecture slides
pub fn preprocess(cnf: &mut CNF) -> VecDeque<(VarId, Vec<ClauseID>)> {
    let mut round_traces: BTreeMap<ClauseID, RoundTrace> = cnf
        .clauses
        .keys()
        .map(|&clause_id| (clause_id, RoundTrace::new(Reason::Add, Round::Prev)))
        .collect();
    let mut niver_trace: VecDeque<(VarId, Vec<ClauseID>)> = VecDeque::new();
    let mut niver_vars: VecDeque<VarId> = VecDeque::new();
    let mut change: u8 = 0b11;

    // repeat until either no more (self)-subsumption happens or NiVER changes nothing
    while change == 0b11 {
        change = 0;

        let mut clauses: VecDeque<ClauseID> = round_traces
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

        // self-subsumption
        while let Some(clause_id) = clauses.pop_front() {
            if selfsubsumes(clause_id, cnf) {
                change |= 0b10;
                clauses.push_back(clause_id);
                let round_trace = round_traces.get_mut(&clause_id).unwrap();
                round_trace.reason = Reason::Stren;
                round_trace.round = Round::Curr;
                cnf.unit_prop().iter().for_each(|&clause_id| {
                    round_traces.insert(clause_id, RoundTrace::new(Reason::Del, Round::Curr));
                });
            }
        }

        // subsumption
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

        // NiVER
        for (clause_id, round_trace) in round_traces.iter() {
            if round_trace.recently() {
                niver_vars.append(
                    &mut cnf
                        .clauses
                        .get(&clause_id)
                        .unwrap()
                        .lits
                        .iter()
                        .map(|&lit| lit.var_id())
                        .collect(),
                );
            }
        }
        niver_vars = niver_vars.iter().unique().copied().collect();
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
