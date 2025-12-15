use std::fs;
use clap::ValueHint;
use std::path::PathBuf;
use clap::Parser;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;
use tracing_tree::HierarchicalLayer;
use dpml::cnf::parser::parse_cnf;
use dpml::dpll::dpll::{Dpll, DpllResult};

#[derive(Parser, Debug)]
#[command(name = "dpml", version)]
struct CliArguments {
    #[arg(value_name = "FILE", value_hint = ValueHint::FilePath)]
    input: PathBuf,
}

fn main() {
    if cfg!(feature = "trace") {
        init_tracing();
    }
    let cli = CliArguments::parse();
    let cnf_formula_str = fs::read_to_string(&cli.input)
        .expect("Should have been able to read the file");
    let cnf_formula = parse_cnf(cnf_formula_str.as_str()).unwrap();
    let mut dpll = Dpll::new(cnf_formula);
    let result = dpll.dpll(0);
    match result {
        DpllResult::Satisfied => println!("Formula in \"{}\" is {:?}:\n{}", cli.input.to_str().unwrap().to_string(), result, dpll.cnf_formula.get_variable_assignments()),
        DpllResult::Unsatisfiable | DpllResult::Unknown => println!("Formula in \"{}\" is {:?}", cli.input.to_str().unwrap().to_string(), result),
    }
}

fn init_tracing() {
    let layer = HierarchicalLayer::default()
        .with_indent_lines(false)
        .with_indent_amount(2);
    let subscriber = Registry::default().with(layer);

    tracing::subscriber::set_global_default(subscriber).ok();
}