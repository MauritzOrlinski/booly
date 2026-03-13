use clap::Parser;
use cpu_time::ProcessTime;
use dpml::dpll::dpll::Dpll;
use dpml::cdcl::heuristics;
use dpml::cdcl::heuristics::Heuristic;
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
}

impl Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            // HERE
            Label::Trivial => write!(f, "first unassigned literal"),
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
    );

    let _ = fg.save_to_png("plot.png", 1920, 1080);

    Ok(())
}
