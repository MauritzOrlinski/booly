pub mod restart;
pub mod trivial;

use crate::cdcl::assignment::Assignment;
use crate::cnf::cnf_formula::CnfFormula;
use std::fmt::Debug;

#[derive(Debug)]
pub struct SolverStats {
    pub conflict_count: usize,
    pub number_of_restarts: usize,
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

/// Names accepted by [`restart_heuristic_by_name`], in the order the UIs list them.
pub const RESTART_HEURISTIC_NAMES: [&str; 4] = ["luby", "geometric", "fixed-interval", "never"];

/// Builds a restart heuristic from its name; shared by the CLI and the browser bindings.
pub fn restart_heuristic_by_name(name: &str) -> Option<Box<dyn RestartHeuristic>> {
    use restart::{FixedIntervalHeuristic, GeometricHeuristic, LubyHeuristic, Never};
    Some(match name {
        "never" => Box::new(Never {}),
        "fixed-interval" => Box::new(FixedIntervalHeuristic {
            fixed_restart_policy: 700,
            max_restarts: 10,
        }),
        "geometric" => Box::new(GeometricHeuristic {
            max_restarts: 10,
            threshold: 400,
            factor: 1.5,
        }),
        "luby" => Box::new(LubyHeuristic::new(10)),
        _ => return None,
    })
}
