use clap::ValueHint;
use clap::{Parser, ValueEnum};
use std::path::PathBuf;

use crate::cdcl::heuristics::{RestartHeuristic, restart_heuristic_by_name};

#[derive(ValueEnum, Debug, Clone, Copy)]
enum HeuristicCliArgument {
    Never,
    FixedInterval,
    Geometric,
    Luby,
}

impl HeuristicCliArgument {
    fn get(&self) -> Box<dyn RestartHeuristic> {
        let name = match self {
            HeuristicCliArgument::Never => "never",
            HeuristicCliArgument::FixedInterval => "fixed-interval",
            HeuristicCliArgument::Geometric => "geometric",
            HeuristicCliArgument::Luby => "luby",
        };
        restart_heuristic_by_name(name).unwrap()
    }
}

#[derive(Parser, Debug)]
#[command(name = "dpml", version)]
pub struct CliArguments {
    /// The input file to solve in DIMACS CNF format
    #[arg(value_name = "FILE", value_hint = ValueHint::FilePath)]
    input_file: PathBuf,

    /// Disable the preprocessor
    #[arg(long, help = "disable the preprocessor")]
    disable_preprocess: bool,

    /// Enable phase saving
    #[arg(long, help = "enable phase saving")]
    phase_saving: bool,

    /// Enable proof logging (disables preprocessing)
    #[arg(long, help = "enable proof logging", requires = "disable_preprocess")]
    proof_logging: bool,

    /// The heuristic to use for restarts
    #[arg(long, value_enum, default_value = "luby")]
    restart_heuristic: HeuristicCliArgument,
}

impl CliArguments {
    pub fn get_input_file(&self) -> PathBuf {
        self.input_file.clone()
    }

    pub fn get_disable_preprocess(&self) -> bool {
        self.disable_preprocess
    }

    pub fn get_proof_logging(&self) -> bool {
        self.proof_logging
    }

    pub fn get_phase_saving(&self) -> bool {
        self.phase_saving
    }

    pub fn get_restart_heuristic(&self) -> Box<dyn RestartHeuristic> {
        self.restart_heuristic.get()
    }
}
