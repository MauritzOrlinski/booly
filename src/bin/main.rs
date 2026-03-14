use clap::Parser;
use dpml::cli::CliArguments;
use dpml::cnf::cnf_formula::{AssignedVarsView, CnfFormula};
use dpml::parser::parse_cnf;
use std::fs;
use std::time::Instant;
use dpml::cdcl::cdcl::{Cdcl, CdclStatus};

#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

fn main() {
    let cli = CliArguments::parse();

    let cnf_formula_str: String = match fs::read_to_string(&cli.get_input_file()) {
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

    let heuristic = cli.load_heuristic();

    let mut dpll = Cdcl::new(cnf_formula, heuristic);

    let start = Instant::now();
    let result = dpll.solve();
    let elapsed = start.elapsed();

    match result {
        CdclStatus::Sat => println!(
            "\
s SATISFIABLE
v {} 0
t {:.7}",
            dpll.cnf_formula.get_assignment_view(),
            elapsed.as_secs_f64()
        ),
        CdclStatus::Unsat => println!(
            "\
s UNSATISFIABLE
t {:.7}",
            elapsed.as_secs_f64()
        ),
        CdclStatus::Incomplete | CdclStatus::Conflict => unreachable!(),
    }
}
