use nom::{
    Finish, IResult, Parser,
    branch::alt,
    bytes::tag,
    character::complete::{char, digit1, line_ending, multispace1, not_line_ending},
    combinator::{map, map_res, opt, value, verify},
    error::Error,
    multi::{count, many_m_n, many0, many1},
    sequence::{preceded, separated_pair, terminated},
};
use std::str::FromStr;

use crate::cnf::{
    clause::Clause,
    cnf_formula::CnfFormula,
    literals::{Literals, Polarity},
    variable::Variables,
};

fn peol_comment(i: &str) -> IResult<&str, ()> {
    value((), (char('c'), not_line_ending, line_ending)).parse(i)
}

fn ppos_number(i: &str) -> IResult<&str, u16> {
    map_res(
        verify(digit1, |s: &str| s.chars().next() != Some('0')),
        u16::from_str,
    )
    .parse(i)
}

fn pnumber(i: &str) -> IResult<&str, i32> {
    map((opt(char('-')), ppos_number), |(sign, digits)| {
        if sign.is_some() {
            -i32::from(digits)
        } else {
            i32::from(digits)
        }
    })
    .parse(i)
}

fn pvariable(i: &str) -> IResult<&str, i32> {
    let (i, (_, v, _)) = (
        many0(alt((tag(" "), line_ending))),
        pnumber,
        many1(alt((tag(" "), line_ending))),
    )
        .parse(i)?;

    Ok((i, v))
}

fn pheader(i: &str) -> IResult<&str, (usize, usize)> {
    let (i, (n, m)) = preceded(
        (tag("p cnf"), multispace1),
        separated_pair(
            map_res(ppos_number, usize::try_from),
            multispace1,
            map_res(ppos_number, usize::try_from),
        ),
    )
    .parse(i)?;

    Ok((i, (n, m)))
}

fn pclauses(n: usize, i: &str) -> IResult<&str, Vec<i32>> {
    terminated(many_m_n(1, n, pvariable), char('0')).parse(i)
}

fn pdimacs(i: &str) -> IResult<&str, (Vec<Vec<i32>>, u16, u16)> {
    let (i, _) = many0(peol_comment).parse(i)?;
    let (i, (n, m)) = pheader.parse(i)?;
    let (i, cs) = count(|i| pclauses(n, i), m).parse(i)?;

    Ok((i, (cs, n as u16, m as u16)))
}

pub fn parse(i: &str) -> Result<(Vec<Vec<i32>>, u16, u16), Error<&str>> {
    pdimacs(i).finish().map(|t| t.1)
}

pub fn parse_cnf(cnf_string: &str) -> Result<CnfFormula, Error<&str>> {
    let (mut crude_clauses, variable_count, _) = parse(cnf_string)?;

    // remove clauses which are tautologies
    {
        let mut tautological_crude_clause_ids = Vec::new();

        crude_clauses
            .iter()
            .enumerate()
            .filter(|(_, crude_clause)| {
                crude_clause
                    .iter()
                    .any(|&crude_literal| crude_clause.contains(&-crude_literal))
            })
            .for_each(|(crude_clause_id, _)| tautological_crude_clause_ids.push(crude_clause_id));

        tautological_crude_clause_ids.sort_by(|a, b| b.cmp(a));

        tautological_crude_clause_ids
            .iter()
            .for_each(|&crude_clause_id| {
                crude_clauses.remove(crude_clause_id);
            });
    }

    let mut clauses: Vec<Clause> = Vec::new();
    let mut variables = Variables::new(variable_count as usize);

    for (clause_id, crude_clause) in crude_clauses.iter().enumerate() {
        let mut literals = Literals::new();

        for &crude_literal in crude_clause {
            let variable_id = crude_literal.unsigned_abs();
            let polarity = if crude_literal > 0 {
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
        clauses.push(Clause::new(literals))
    }

    Ok(CnfFormula::new(clauses, variables))
}
