use clap::ValueHint;
use std::path::PathBuf;
use clap::{Parser, ValueEnum};
use crate::dpll::heuristics::combined_heuristic::CombinedHeuristic;
use crate::dpll::heuristics::dlcs::DLCS;
use crate::dpll::heuristics::dlis1::DLIS1;
use crate::dpll::heuristics::dlis::DLIS;
use crate::dpll::heuristics::from_shortest_clause::FromShortestClause;
use crate::dpll::heuristics::Heuristic;
use crate::dpll::heuristics::mom::MOM;
use crate::dpll::heuristics::trivial::Trivial;

#[derive(ValueEnum, Debug, Clone, Copy)]
enum SimpleHeuristicCliArgument {
    DLCS,
    DLCS1,
    DLIS,
    DLIS1,
    FromShortestClause,
    Mom,
    Trivial
}

impl SimpleHeuristicCliArgument {
    fn get(&self) -> Box<dyn Heuristic>{
        match self {
            SimpleHeuristicCliArgument::DLCS => Box::new(DLCS),
            SimpleHeuristicCliArgument::DLCS1 => Box::new(DLCS),
            SimpleHeuristicCliArgument::DLIS => Box::new(DLIS),
            SimpleHeuristicCliArgument::DLIS1 => Box::new(DLIS1),
            SimpleHeuristicCliArgument::FromShortestClause => Box::new(FromShortestClause),
            SimpleHeuristicCliArgument::Mom => Box::new(MOM),
            SimpleHeuristicCliArgument::Trivial => Box::new(Trivial),
        }
    }
}

#[derive(ValueEnum, Debug, Clone, Copy)]
enum CompositeHeuristicCliArgument {
    DLCS,
    DLCS1,
    DLIS,
    DLIS1,
    FromShortestClause,
    Mom,
    Trivial,
    Combined
}

#[derive(Parser, Debug)]
#[command(name = "dpml", version)]
pub struct CliArguments {
    /// The input file to solve in DIMACS CNF format
    #[arg(value_name = "FILE", value_hint = ValueHint::FilePath)]
    input_file: PathBuf,

    /// The heuristic to use when chosing the next branching variable
    #[arg(long, value_enum, default_value = "from-shortest-clause")]
    heuristic: CompositeHeuristicCliArgument,

    #[arg(
        long,
        required_if_eq("heuristic", "combined"),
    )]
    /// The primary heuristic to use when `--heuristic` is set to `combined`. Must not be `combined`.
    ///
    /// Required only if `--heuristic combined` is chosen.
    primary: Option<SimpleHeuristicCliArgument>,

    #[arg(
        long,
        required_if_eq("heuristic", "combined"),
    )]
    /// The secondary heuristic to use when `--heuristic` is set to `combined`. Must not be `combined`.
    ///
    /// Required only if `--heuristic combined` is chosen.
    secondary: Option<SimpleHeuristicCliArgument>,

    #[arg(
        long,
        required_if_eq("heuristic", "combined"),
    )]
    /// The decision level threshold at which to switch from the primary to the secondary heuristic
    ///
    /// Required only if `--heuristic combined` is chosen.
    decision_level_threshold: Option<i32>,
}

impl CliArguments {
    pub fn get_heuristic(&self) -> Box<dyn Heuristic> {
        match self.heuristic {
            CompositeHeuristicCliArgument::DLCS => Box::new(DLCS),
            CompositeHeuristicCliArgument::DLCS1 => Box::new(DLCS),
            CompositeHeuristicCliArgument::DLIS => Box::new(DLIS),
            CompositeHeuristicCliArgument::DLIS1 => Box::new(DLIS1),
            CompositeHeuristicCliArgument::FromShortestClause => Box::new(FromShortestClause),
            CompositeHeuristicCliArgument::Mom => Box::new(MOM),
            CompositeHeuristicCliArgument::Trivial => Box::new(Trivial),
            CompositeHeuristicCliArgument::Combined => {
                let primary = self.primary.unwrap().get();
                let secondary = self.secondary.unwrap().get();
                let decision_level_threshold = self.decision_level_threshold.unwrap();
                Box::new(
                    CombinedHeuristic::new(primary, secondary, decision_level_threshold)
                )
            }
        }
    }

    pub fn get_input_file(&self) -> PathBuf {
        self.input_file.clone()
    }
}
