use clap::{Parser, ValueEnum};
use clap::ValueHint;
use dpml::cnf::cnf_formula::CnfFormula;
use dpml::dpll::dpll::{Dpll, DpllStatus};
use dpml::parser::parse_cnf;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;
use dpml::dpll::heuristics::dlcs::DLCS;
use dpml::dpll::heuristics::dlis1::DLIS1;
use dpml::dpll::heuristics::dlis::DLIS;
use dpml::dpll::heuristics::from_shortest_clause::FromShortestClause;
use dpml::dpll::heuristics::Heuristic;
use dpml::dpll::heuristics::mom::MOM;
use dpml::dpll::heuristics::trivial::Trivial;

#[derive(ValueEnum, Debug, Clone)]
enum HeuristicCliArgument {
    DLCS,
    DLCS1,
    DLIS,
    DLIS1,
    FromShortestClause,
    Mom,
    Trivial
}

impl HeuristicCliArgument {
    fn get(&self) -> Box<dyn Heuristic> {
        match self {
            HeuristicCliArgument::DLCS => Box::new(DLCS),
            HeuristicCliArgument::DLCS1 => Box::new(DLCS),
            HeuristicCliArgument::DLIS => Box::new(DLIS),
            HeuristicCliArgument::DLIS1 => Box::new(DLIS1),
            HeuristicCliArgument::FromShortestClause => Box::new(FromShortestClause),
            HeuristicCliArgument::Mom => Box::new(MOM),
            HeuristicCliArgument::Trivial => Box::new(Trivial),
        }
    }
}

#[derive(Parser, Debug)]
#[command(name = "dpml", version)]
struct CliArguments {
    /// The input file to solve in DIMACS CNF format
    #[arg(value_name = "FILE", value_hint = ValueHint::FilePath)]
    input_file: PathBuf,

    /// The heuristic to use when chosing the next branching variable
    #[arg(long, value_enum, default_value = "from-shortest-clause")]
    heuristic: HeuristicCliArgument
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

    let heuristic = cli.heuristic.get();

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
