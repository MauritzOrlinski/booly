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
use rayon::prelude::*;
use std::fs;
use std::io;
use walkdir::WalkDir;

fn measure_dpll(cnf_string: &String, heuristic: impl Heuristic) -> f64 {
    let mut dpll = Dpll::new(parse_cnf(cnf_string).unwrap(), heuristic);

    let start = ProcessTime::now();
    let result = dpll.solve();
    let elapsed = start.elapsed();

    match result {
        dpml::dpll::dpll::DpllStatus::Sat => elapsed.as_secs_f64(),
        dpml::dpll::dpll::DpllStatus::Unsat => elapsed.as_secs_f64(),
        _ => unreachable!(""),
    }
}

fn main() -> io::Result<()> {
    let mut fg = Figure::new();
    let mut c = Vec::new();

    for i in 0..7 {
        let cnf_strings: Vec<String> = WalkDir::new("inputs")
            .into_iter()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_file())
            .map(|entry| fs::read_to_string(&entry.path().to_path_buf()).unwrap())
            .collect();

        let mut results: Vec<f64> = cnf_strings
            .par_iter()
            .map(|cnf_string| match i {
                1 => measure_dpll(&cnf_string, heuristics::dlcs::DLCS),
                2 => measure_dpll(&cnf_string, heuristics::dlcs1::DLCS1),
                3 => measure_dpll(&cnf_string, heuristics::dlis::DLIS),
                4 => measure_dpll(&cnf_string, heuristics::dlis1::DLIS1),
                5 => measure_dpll(&cnf_string, heuristics::mom::MOM),
                _ => measure_dpll(
                    &cnf_string,
                    heuristics::from_shortest_clause::FromShortestClause,
                ),
            })
            .collect();

        results.sort_by(|a, b| a.partial_cmp(b).unwrap());
        c.push(results);
    }

    fg.axes2d()
        .lines_points(0..c[0].len(), &c[0], &[Caption("dlcs")])
        .lines_points(0..c[1].len(), &c[1], &[Caption("dlcs1")])
        .lines_points(0..c[2].len(), &c[2], &[Caption("dlis")])
        .lines_points(0..c[3].len(), &c[3], &[Caption("dlis1")])
        .lines_points(0..c[4].len(), &c[4], &[Caption("mom")])
        .lines_points(0..c[5].len(), &c[5], &[Caption("fsc")])
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
        .set_y_label("CPU Times(s)", &[]);
    let _ = fg.save_to_png("plots/plot.png", 1920, 1080);

    Ok(())
}
