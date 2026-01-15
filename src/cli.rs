use crate::dpll::heuristics::Heuristic;
use crate::dpll::heuristics::combined_heuristic::CombinedHeuristic;
use crate::dpll::heuristics::combined_heuristic_reverse::CombinedHeuristicReverse;
use crate::dpll::heuristics::dlcs::DLCS;
use crate::dpll::heuristics::dlis::DLIS;
use crate::dpll::heuristics::from_shortest_clause::FromShortestClause;
use crate::dpll::heuristics::jeroslaw_wang::JeroslawWang;
use crate::dpll::heuristics::mom::MOM;
use crate::dpll::heuristics::multi_bandits_learning::ContextualBandits;
use crate::dpll::heuristics::trivial::Trivial;
use clap::ValueHint;
use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(ValueEnum, Debug, Clone, Copy)]
enum SimpleHeuristicCliArgument {
    DLCS,
    DLIS,
    FromShortestClause,
    Mom,
    Trivial,
    JW,
    MultiBandit,
}

impl SimpleHeuristicCliArgument {
    fn get(&self) -> Box<dyn Heuristic> {
        match self {
            SimpleHeuristicCliArgument::DLCS => Box::new(DLCS),
            SimpleHeuristicCliArgument::DLIS => Box::new(DLIS),
            SimpleHeuristicCliArgument::FromShortestClause => Box::new(FromShortestClause),
            SimpleHeuristicCliArgument::Mom => Box::new(MOM),
            SimpleHeuristicCliArgument::Trivial => Box::new(Trivial),
            SimpleHeuristicCliArgument::JW => Box::new(JeroslawWang),
            SimpleHeuristicCliArgument::MultiBandit => Box::new(ContextualBandits::new()),
        }
    }
}

#[derive(ValueEnum, Debug, Clone, Copy)]
enum LearningModelCliArgument {
    MultiBandit,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
enum CompositeHeuristicCliArgument {
    DLCS,
    DLIS,
    FromShortestClause,
    Mom,
    Trivial,
    JW,
    MultiBandit,
    Combined,
    CombinedReverse,
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
        required_if_eq("heuristic", "combined_reverse")
    )]
    /// The primary heuristic to use when `--heuristic` is set to `combined`. Must not be `combined`.
    ///
    /// Required only if `--heuristic combined` is chosen.
    primary: Option<SimpleHeuristicCliArgument>,

    #[arg(
        long,
        required_if_eq("heuristic", "combined"),
        required_if_eq("heuristic", "combined_reverse")
    )]
    /// The secondary heuristic to use when `--heuristic` is set to `combined`. Must not be `combined`.
    ///
    /// Required only if `--heuristic combined` is chosen.
    secondary: Option<SimpleHeuristicCliArgument>,

    #[arg(
        long,
        required_if_eq("heuristic", "combined"),
        required_if_eq("heuristic", "combined_reverse")
    )]
    /// The decision level threshold at which to switch from the primary to the secondary heuristic
    ///
    /// Required only if `--heuristic combined` is chosen.
    decision_level_threshold: Option<i32>,

    #[arg(long)]
    pub save_learned_model: Option<String>,
    #[arg(long)]
    pub load_learned_model: Option<String>,
}

impl CliArguments {
    pub fn get_heuristic(&self) -> Box<dyn Heuristic> {
        match self.heuristic {
            CompositeHeuristicCliArgument::DLCS => Box::new(DLCS),
            CompositeHeuristicCliArgument::DLIS => Box::new(DLIS),
            CompositeHeuristicCliArgument::FromShortestClause => Box::new(FromShortestClause),
            CompositeHeuristicCliArgument::Mom => Box::new(MOM),
            CompositeHeuristicCliArgument::JW => Box::new(JeroslawWang),
            CompositeHeuristicCliArgument::Trivial => Box::new(Trivial),
            CompositeHeuristicCliArgument::Combined => {
                let primary = self.primary.unwrap().get();
                let secondary = self.secondary.unwrap().get();
                let decision_level_threshold = self.decision_level_threshold.unwrap();
                Box::new(CombinedHeuristic::new(
                    primary,
                    secondary,
                    decision_level_threshold,
                ))
            }
            CompositeHeuristicCliArgument::CombinedReverse => {
                let primary = self.primary.unwrap().get();
                let secondary = self.secondary.unwrap().get();
                let decision_level_threshold = self.decision_level_threshold.unwrap();
                Box::new(CombinedHeuristicReverse::new(
                    primary,
                    secondary,
                    decision_level_threshold,
                ))
            }
            CompositeHeuristicCliArgument::MultiBandit => Box::new(ContextualBandits::new()),
        }
    }
    pub fn load_heuristic(&self) -> Box<dyn Heuristic> {
        match self.heuristic {
            CompositeHeuristicCliArgument::MultiBandit => {
                if let Some(path) = &self.load_learned_model {
                    Box::new(ContextualBandits::load_or_new(path.as_str()))
                } else {
                    Box::new(ContextualBandits::new())
                }
            }
            _ => self.get_heuristic(),
        }
    }

    pub fn get_input_file(&self) -> PathBuf {
        self.input_file.clone()
    }
}
