use clap::ValueHint;
use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(ValueEnum, Debug, Clone, Copy)]
enum SimpleHeuristicCliArgument {
    Trivial,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
enum CompositeHeuristicCliArgument {
    Trivial,
}

#[derive(Parser, Debug)]
#[command(name = "dpml", version)]
pub struct CliArguments {
    /// The input file to solve in DIMACS CNF format
    #[arg(value_name = "FILE", value_hint = ValueHint::FilePath)]
    input_file: PathBuf,
}

impl CliArguments {
    pub fn get_input_file(&self) -> PathBuf {
        self.input_file.clone()
    }
}
