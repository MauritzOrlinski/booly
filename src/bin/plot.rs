use clap::Parser;
use cpu_time::ProcessTime;
use dpml::dpll::dpll::Dpll;
use dpml::dpll::heuristics;
use dpml::dpll::heuristics::Heuristic;
use dpml::parser::parse_cnf;
use gnuplot::AxesCommon;
use gnuplot::Coordinate::Graph;
use gnuplot::Figure;
use gnuplot::LegendOption::Placement;
use gnuplot::PlotOption::Caption;
use itertools::Itertools;
use rayon::prelude::*;
use std::fmt::Display;
use std::fs;
use std::io;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;
use walkdir::WalkDir;

// INFO: In order to add a heuristic, extend the code marked with "HERE"

#[derive(Parser)]
struct Args {
    /// timeout per instance in seconds
    timeout: u64,
}

macro_rules! plot_for_labels {
    ($axes:expr, $results:expr, $( $label:path ),+ $(,)?) => {{
        let mut axes = $axes;
        $(
            axes = axes.lines_points(
                1..$results
                    .iter()
                    .filter(|(_, l)| *l == $label)
                    .count()+1,
                $results
                    .iter()
                    .filter_map(|(f, l)| { if *l == $label { Some(f) } else { None } })
                    .scan(0.0, |state, x| { *state += x; Some(*state)}),
                &[Caption(&format!("{}", $label))],
            );
        )+
        axes
    }};
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Label {
    // HERE
    Trivial,
    FSC,
    DLCS,
    DLIS,
    JW,
    MOM,
    STATICJW,
    MOMDLCS,
    MOMDLCSTrivial,
    MBL,
}

impl Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            // HERE
            Label::Trivial => write!(f, "first unassigned literal"),
            Label::FSC => write!(f, "from shortest clause"),
            Label::DLCS => write!(f, "dlcs"),
            Label::DLIS => write!(f, "dlis"),
            Label::MOM => write!(f, "mom"),
            Label::JW => write!(f, "jeroslaw wang"),
            Label::MOMDLCS => write!(f, "mom+dlcs"),
            Label::MBL => write!(f, "multi bandits leaning"),
            Label::STATICJW => write!(f, "static jeroslaw wang"),
            Label::MOMDLCSTrivial => write!(f, "mom+dlcs+trivial"),
        }
    }
}

fn measure_dpll(cnf_string: &String, heuristic: Box<dyn Heuristic>) -> Option<f64> {
    let mut dpll = Dpll::new(parse_cnf(cnf_string).unwrap(), heuristic);
    let timeout = Args::parse().timeout;

    let cancel_flag = Arc::new(AtomicBool::new(false));

    {
        let cancel_flag = Arc::clone(&cancel_flag);
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(timeout));
            cancel_flag.store(true, Ordering::Relaxed);
        });
    }

    let start = ProcessTime::now();
    let result = dpll.solve_interruptable(&cancel_flag);
    let elapsed = start.elapsed();

    match result {
        dpml::dpll::dpll::DpllStatus::Sat => Some(elapsed.as_secs_f64()),
        dpml::dpll::dpll::DpllStatus::Unsat => Some(elapsed.as_secs_f64()),
        _ => None,
    }
}

fn main() -> io::Result<()> {
    let _ = Args::parse();
    let mut fg = Figure::new();

    let measures: Vec<fn(&String) -> Option<(f64, Label)>> = vec![
        // HERE
        |cnf| {
            measure_dpll(cnf, Box::new(heuristics::trivial::Trivial)).map(|f| (f, Label::Trivial))
        },
        |cnf| {
            measure_dpll(
                cnf,
                Box::new(heuristics::from_shortest_clause::FromShortestClause),
            )
            .map(|f| (f, Label::FSC))
        },
        |cnf| measure_dpll(cnf, Box::new(heuristics::dlcs::DLCS)).map(|f| (f, Label::DLCS)),
        |cnf| measure_dpll(cnf, Box::new(heuristics::dlis::DLIS)).map(|f| (f, Label::DLIS)),
        |cnf| {
            measure_dpll(cnf, Box::new(heuristics::jeroslaw_wang::JeroslawWang))
                .map(|f| (f, Label::JW))
        },
        |cnf| measure_dpll(cnf, Box::new(heuristics::mom::MOM)).map(|f| (f, Label::MOM)),
        |cnf| {
            measure_dpll(
                cnf,
                Box::new(heuristics::combined_heuristic::CombinedHeuristic::new(
                    Box::new(heuristics::mom::MOM),
                    Box::new(heuristics::dlcs::DLCS),
                    50,
                )),
            )
            .map(|f| (f, Label::MOMDLCS))
        },
        |cnf| {
            measure_dpll(
                cnf,
                Box::new(
                    heuristics::multi_bandits_learning::ContextualBandits::load_or_new(
                        "./model.json",
                    ),
                ),
            )
            .map(|f| (f, Label::MBL))
        },
        |cnf| {
            measure_dpll(cnf, Box::new(heuristics::static_jw::StaticJW::new()))
                .map(|f| (f, Label::STATICJW))
        },
        |cnf| {
            measure_dpll(
                cnf,
                Box::new(heuristics::combined_heuristic::CombinedHeuristic::new(
                    Box::new(heuristics::mom::MOM),
                    Box::new(
                        heuristics::combined_heuristic_reverse::CombinedHeuristicReverse::new(
                            Box::new(heuristics::dlcs::DLCS),
                            Box::new(heuristics::trivial::Trivial),
                            20,
                        ),
                    ),
                    50,
                )),
            )
            .map(|f| (f, Label::MOMDLCSTrivial))
        },
    ];

    let cnf_strings: Vec<String> = WalkDir::new("inputs")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .filter_map(|entry| {
            let bytes = fs::read(entry.path()).ok()?;
            match String::from_utf8(bytes) {
                Ok(string) => Some(string),
                Err(_) => {
                    println!("{:?}", entry);
                    None
                }
            }
        })
        .collect();

    let mut results: Vec<(f64, Label)> = cnf_strings
        .iter()
        .cartesian_product(measures)
        .par_bridge()
        .filter_map(|(cnf_string, measure)| measure(&cnf_string))
        .collect();

    results.sort_by(|a, b| a.partial_cmp(b).unwrap());

    plot_for_labels!(
        fg.axes2d()
            .set_legend(
                Graph(0.01),
                Graph(0.99),
                &[Placement(
                    gnuplot::AlignType::AlignLeft,
                    gnuplot::AlignType::AlignTop,
                )],
                &[],
            )
            .set_x_label("number of solved instances", &[])
            .set_y_label("CPU Times(s)", &[]),
        results,
        // HERE
        Label::Trivial,
        Label::FSC,
        Label::DLCS,
        Label::DLIS,
        Label::MOM,
        Label::JW,
        Label::MOMDLCS,
        Label::MOMDLCSTrivial,
        Label::MBL,
        Label::STATICJW
    );

    let _ = fg.save_to_png("plot.png", 1920, 1080);

    Ok(())
}
