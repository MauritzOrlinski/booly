use crate::cnf::cnf_formula::CnfFormula;

pub fn parse_cnf(cnf_string: &str) -> Result<CnfFormula, ParseError> {
    let cleaned_up_source = cnf_string
        .lines()
        .filter(|line| !line.starts_with("c") && !line.starts_with("p"))
        .map(|line| line.trim())
        .fold(String::new(), |acc, line| acc + " " + line);

    let clauses = cleaned_up_source
        .split(" 0")
        .filter(|clause| !clause.is_empty())
        .map(parse_clause)
        .collect::<Result<Vec<Vec<i64>>, ParseError>>()?;

    Ok(CnfFormula::new(clauses))
}

fn parse_clause(clause_string: &str) -> Result<Vec<i64>, ParseError> {
    clause_string
        .split(' ')
        .filter(|clause| !clause.is_empty())
        .map(parse_literal)
        .collect::<Result<Vec<i64>, ParseError>>()
}

fn parse_literal(literal_string: &str) -> Result<i64, ParseError> {
    let literal_integer = literal_string
        .parse::<i64>()
        .map_err(|_| ParseError {
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
    use crate::parser::parser::parse_cnf;

    #[test]
    fn test_parse_cnf() {
        let parse_input = "\
p cnf 4 2
1 2 0
3 4 0";
        let cnf = parse_cnf(parse_input).unwrap();
    }
}