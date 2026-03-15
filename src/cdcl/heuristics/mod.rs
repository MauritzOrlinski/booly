pub mod restart;
pub mod trivial;

use crate::cdcl::assignment::Assignment;
use crate::cnf::cnf_formula::CnfFormula;
use std::fmt::Debug;

#[derive(Debug)]
pub struct SolverStats {
    pub(crate) conflict_count: usize,
    pub(crate) number_of_restarts: usize,
}

impl SolverStats {
    pub fn new() -> Self {
        SolverStats {
            conflict_count: 0,
            number_of_restarts: 0,
        }
    }
}

pub trait Heuristic: Debug {
    fn chose_next_assignment(&mut self, cnf_formula: &CnfFormula) -> Assignment;
}

pub trait RestartHeuristic: Debug {
    fn should_restart(&mut self, stats: &SolverStats) -> bool;
}
