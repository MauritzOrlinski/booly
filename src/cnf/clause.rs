use crate::cnf::literal::Literal;

#[derive(Debug)]
pub struct Clause {
    satisfied_by: Literal,
    literals: Vec<Literal>,
    unassigned_variables: u32,
}