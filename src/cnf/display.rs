use crate::cnf::clause::{Clause, Polarity};
use crate::cnf::cnf_formula::CnfFormula;
use std::fmt;
use std::fmt::Formatter;

impl fmt::Display for CnfFormula {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "p cnf {} {}\n{}",
            self.variables.len(),
            self.clauses.len(),
            self.clauses
                .iter()
                .map(|(_, clause)| clause.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl fmt::Display for Clause {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{} 0",
            self.literals
                .iter()
                .map(|(var_id, polarity)| match polarity {
                    Polarity::Positive => format!("{}", var_id),
                    Polarity::Negative => format!("-{}", var_id),
                })
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}
