use clap::Parser;
use dpml::cdcl::cdcl::{Cdcl, CdclStatus};
use dpml::cli::CliArguments;
use dpml::cnf::cnf_formula::CnfFormula;
use dpml::parser::{parse, parse_cnf};
use dpml::preprocess::cnf::cnf::CNF;
use dpml::preprocess::niver::recover_assigment_niver_compat;
use dpml::preprocess::preprocess::preprocess;
use std::fs;
use std::time::Instant;

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

    let cnf_formula_pre = match parse(&cnf_formula_str) {
        Ok(value) => value,
        Err(_) => {
            println!("Failed to parse input file.");
            return;
        }
    };

    let mut cnf = CNF::from_pre(&cnf_formula_pre.0, cnf_formula_pre.1);
    let heuristic = cli.load_heuristic();

    let mut dpll = Cdcl::new(cnf.to_cnf_formula(), heuristic);

    let start_pre = Instant::now();
    let niver_trace = preprocess(&mut cnf);
    let elapsed_pre = start_pre.elapsed();

    let start = Instant::now();
    let result = dpll.solve();
    let elapsed = start.elapsed();

    let assignment = recover_assigment_niver_compat(niver_trace, cnf, dpll.cnf_formula);

    match result {
        CdclStatus::Sat => println!(
            "\
s  SATISFIABLE
v  {} 0
tp {:.7}
t  {:.7}",
            assignment
                .iter()
                .map(|(key, &val)| if val {
                    key.to_string()
                } else {
                    format!("-{}", key)
                })
                .collect::<Vec<String>>()
                .join(" "),
            elapsed_pre.as_secs_f64(),
            elapsed.as_secs_f64()
        ),
        CdclStatus::Unsat => println!(
            "\
s  UNSATISFIABLE
tp {:.7}
t  {:.7}",
            elapsed_pre.as_secs_f64(),
            elapsed.as_secs_f64()
        ),
        CdclStatus::Incomplete | CdclStatus::Conflict(_) => unreachable!(),
    }
}
