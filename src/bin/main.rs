use clap::Parser;
use clap::ValueHint;
use dpml::cnf::cnf_formula::CnfFormula;
use dpml::dpll::dpll::{Dpll, DpllStatus};
use dpml::dpll::heuristics;
use dpml::parser::parse_cnf;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(name = "dpml", version)]
struct CliArguments {
    #[arg(value_name = "FILE", value_hint = ValueHint::FilePath)]
    input_file: PathBuf,
}

fn main() {
    let cli = CliArguments::parse();

    let cnf_formula_str: String = match fs::read_to_string(&cli.input_file) {
        Ok(value) => value,
        Err(_) => {
            println!("Failed to read input file.");
            return;
        }
    };

    let cnf_formula: CnfFormula = match parse_cnf(&cnf_formula_str) {
        Ok(value) => value,
        Err(_) => {
            println!("Failed to parse input file.");
            return;
        }
    };
    let heuristic = heuristics::from_shortest_clause::FromShortestClause;
    let mut dpll = Dpll::new(cnf_formula, heuristic);

    let start = Instant::now();
    let result = dpll.solve();
    let elapsed = start.elapsed();

    match result {
        DpllStatus::Sat => println!(
            "\
s SATISFIABLE
v {} 0
t {:.7}",
            dpll.cnf_formula.variables,
            elapsed.as_secs_f64()
        ),
        DpllStatus::Unsat => println!(
            "\
s UNSATISFIABLE
t {:.7}",
            elapsed.as_secs_f64()
        ),
        DpllStatus::Incomplete | DpllStatus::Conflict => unreachable!(),
    }
}
