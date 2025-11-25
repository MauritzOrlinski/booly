use super::clauses::Clauses;
use super::variables::{Variable, Variables};

trait CNF<VARS: Variables, CLAUSE: Clauses, V: Variable + Copy> {
    fn simplify(self);
    fn pick(self) -> V;
    fn set(self, branching_variable: &V);
    fn unit_prop(self);
    fn pure_literal(self);
    fn dpll<S: Iterator, Q: Iterator>(self);
}
