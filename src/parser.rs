use crate::cnf::clause::Clause;
use crate::cnf::cnf_formula::CnfFormula;
use crate::cnf::literals::{Literals, Polarity};
use crate::cnf::variable::Variables;

/// Parses a string in DIMACS CNF format to the CNF data structure.
/// See (https://people.sc.fsu.edu/~jburkardt/data/cnf/cnf.html) for a specification of the file format.
///
/// # Arguments
/// * `cnf_string` - A string in DIMACS CNF form
///
/// # Returns
/// The parsed [`CnfFormula`](CnfFormula)
///
/// # Errors
/// Returns an error, if unable to parse given string.
pub fn parse_cnf(cnf_string: &str) -> Result<CnfFormula, ParseError> {
    let header_line_string = cnf_string
        .lines()
        .find(|line| line.starts_with("p"))
        .ok_or(ParseError {
            reason: "Unable to find header line".to_string(),
        })?;

    let variable_count = header_line_string
        .split_whitespace()
        .nth(2)
        .ok_or(ParseError {
            reason: "Unable to parse header".to_string(),
        })?
        .parse()
        .map_err(|_| ParseError {
            reason: "Unable to parse header".to_string(),
        })?;

    let cleaned_up_source = cnf_string
        .lines()
        .filter(|line| !line.starts_with("c") && !line.starts_with("p"))
        .map(|line| line.trim())
        .fold(String::new(), |acc, line| acc + " " + line);

    let crude_clauses = cleaned_up_source
        .split(" 0")
        .filter(|clause| !clause.is_empty())
        .map(parse_clause)
        .collect::<Result<Vec<Vec<i64>>, ParseError>>()?;

    let mut variables: Variables = Variables::new(variable_count);
    let mut clauses: Vec<Clause> = Vec::new();

    for (clause_id, crude_clause) in crude_clauses.iter().enumerate() {
        let mut literals = Literals::new();

        for crude_literal in crude_clause {
            let variable_id = crude_literal.abs() as usize;
            let polarity = if *crude_literal > 0 {
                Polarity::Positive
            } else {
                Polarity::Negative
            };
            let variable = variables.get_mut(variable_id);

            match polarity {
                Polarity::Positive => variable.positive_occurrences.push(clause_id),
                Polarity::Negative => variable.negative_occurrences.push(clause_id),
            }
            literals.insert(variable_id, polarity);
        }

        clauses.push(Clause::new(literals));
    }

    Ok(CnfFormula::new(clauses, variables))
}

fn parse_clause(clause_string: &str) -> Result<Vec<i64>, ParseError> {
    clause_string
        .split(' ')
        .filter(|clause| !clause.is_empty())
        .map(parse_literal)
        .collect::<Result<Vec<i64>, ParseError>>()
}

fn parse_literal(literal_string: &str) -> Result<i64, ParseError> {
    let literal_integer = literal_string.parse::<i64>().map_err(|_| ParseError {
        reason: format!("Unable to parse supposed literal: \"{literal_string}\""),
    })?;
    Ok(literal_integer)
}

#[derive(Debug)]
pub struct ParseError {
    pub reason: String,
}

#[cfg(test)]
mod tests {
    use crate::parser::parse_cnf;

    #[test]
    fn test_parse_cnf() {
        let parse_input = "\
p cnf 4 2
1 2 0
3 4 0";
        let _ = parse_cnf(parse_input).unwrap();
    }
}
