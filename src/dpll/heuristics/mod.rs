pub mod combined_heuristic;
pub mod combined_heuristic_reverse;
pub mod dlcs;
pub mod dlis;
pub mod from_shortest_clause;
pub mod jeroslaw_wang;
pub mod mom;
pub mod multi_bandits_learning;
pub mod static_jw;
pub mod trivial;

use crate::cnf::cnf_formula::CnfFormula;
use crate::dpll::assignment::Assignment;
use std::fmt::Debug;

pub struct Stats {
    pub(crate) unit_clauses: usize,
}

pub trait Heuristic: Debug {
    fn chose_next_assignment(
        &mut self,
        cnf_formula: &CnfFormula,
        decision_level: i32,
    ) -> Assignment;

    /// these give our heuristic the possiblity to implement rudimentary
    fn feedback(&mut self, feedback: Stats);

    fn is_learning(&self) -> bool;
    /// saves a trained model
    fn save(&self, path: &str) -> std::io::Result<()>;
}
