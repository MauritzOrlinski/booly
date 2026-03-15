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

    if !cli.get_disable_preprocess() {
        let cnf_formula_pre = match parse(&cnf_formula_str) {
            Ok(value) => value,
            Err(_) => {
                println!("Failed to parse input file.");
                return;
            }
        };

        let mut cnf = CNF::from_pre(&cnf_formula_pre.0, cnf_formula_pre.1);
        let mut cdcl = Cdcl::new(
            cnf.to_cnf_formula(),
            cli.get_disable_preprocess(),
            cli.get_restart_heuristic(),
        );

        let start = Instant::now();

        let start_pre = Instant::now();
        let niver_trace = preprocess(&mut cnf);
        let elapsed_pre = start_pre.elapsed();

        let result = cdcl.solve();

        let elapsed = start.elapsed();

        let assignment = recover_assigment_niver_compat(niver_trace, cnf, cdcl.cnf_formula);

        match result {
            CdclStatus::Sat => println!(
                "\
s  SATISFIABLE
v  {} 0
t  {:.7}
tp {:.7}",
                assignment
                    .iter()
                    .map(|(key, &val)| if val {
                        key.to_string()
                    } else {
                        format!("-{}", key)
                    })
                    .collect::<Vec<String>>()
                    .join(" "),
                elapsed.as_secs_f64(),
                elapsed_pre.as_secs_f64(),
            ),
            CdclStatus::Unsat => println!(
                "\
s  UNSATISFIABLE
t  {:.7}
tp {:.7}",
                elapsed.as_secs_f64(),
                elapsed_pre.as_secs_f64(),
            ),
            CdclStatus::Incomplete | CdclStatus::Conflict(_) => unreachable!(),
        }
    } else {
        let cnf_formula: CnfFormula = match parse_cnf(&cnf_formula_str) {
            Ok(value) => value,
            Err(_) => {
                println!("Failed to parse input file.");
                return;
            }
        };

        let mut dpll = Cdcl::new(
            cnf_formula,
            cli.get_disable_preprocess(),
            cli.get_restart_heuristic(),
        );

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
            CdclStatus::Incomplete | CdclStatus::Conflict(_) => unreachable!(),
        }
    }
}
