use cpu_time::ProcessTime;
use dpml::dpll::dpll::Dpll;
use dpml::dpll::heuristics;
use dpml::parser::parse_cnf;
use gnuplot::AxesCommon;
use gnuplot::Figure;
use rayon::prelude::*;
use std::fs;
use std::io;
use walkdir::WalkDir;

fn measure_dpll(cnf_string: &String) -> f64 {
    let heuristic = heuristics::dlcs1::DLCS1;
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
    let cnf_strings: Vec<String> = WalkDir::new("inputs")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| fs::read_to_string(&entry.path().to_path_buf()).unwrap())
        .collect();

    let mut results: Vec<f64> = cnf_strings
        .par_iter()
        .map(|cnf_string| measure_dpll(&cnf_string))
        .collect();

    results.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut fg = Figure::new();
    fg.axes2d()
        .lines_points(0..results.len(), &results, &[])
        .set_x_label("number of solved instances", &[])
        .set_y_label("CPU Times(s)", &[]);
    let _ = fg.save_to_png("plots/plot.png", 1920, 1080);

    Ok(())
}
