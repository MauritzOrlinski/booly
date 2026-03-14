pub mod trivial;

use crate::cnf::cnf_formula::CnfFormula;
use crate::cdcl::assignment::Assignment;
use std::fmt::Debug;

pub trait Heuristic: Debug {
    fn chose_next_assignment(
        &mut self,
        cnf_formula: &CnfFormula,
    ) -> Assignment;
}
