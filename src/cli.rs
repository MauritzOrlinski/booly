use clap::Parser;
use clap::ValueHint;
use std::path::PathBuf;

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
