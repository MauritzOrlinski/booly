use crate::cnf::variable::Variable;

#[derive(Debug)]
enum Polarity {
    Positive,
    Negative,
}

#[derive(Debug)]
pub struct Literal {
    variable: Variable,
    polarity: Polarity,
}