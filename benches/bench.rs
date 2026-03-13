use criterion::measurement::WallTime;
use criterion::{
    black_box, criterion_group, criterion_main, BenchmarkGroup, BenchmarkId, Criterion,
};
use dpml::cnf::cnf_formula::CnfFormula;
use dpml::dpll::dpll::Dpll;
use dpml::parser::parse_cnf;
use std::path::Path;
use std::time::Duration;
use std::{fs, io};
use dpml::cdcl::heuristics::trivial::Trivial;

fn benchmark(c: &mut Criterion<WallTime>) -> io::Result<()> {
    benchmark_all_in_directory("inputs/test/sat", c.benchmark_group("satisfiable"))?;
    benchmark_all_in_directory("inputs/test/unsat", c.benchmark_group("unsatisfiable"))?;
    Ok(())
}

fn benchmark_all_in_directory<P: AsRef<Path>>(
    path: P,
    mut group: BenchmarkGroup<WallTime>,
) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let cnf_string = fs::read_to_string(path.clone())?;
        let file_name = path.file_stem().unwrap().to_str().unwrap();
        let cnf = parse_cnf(cnf_string.as_str()).unwrap();

        group.bench_with_input(BenchmarkId::from_parameter(file_name), &cnf, |b, cnf| {
            b.iter(|| run_dpll(black_box(cnf.clone())))
        });
    }
    group.finish();
    Ok(())
}

fn run_dpll(cnf: CnfFormula) {
    let heuristic = Trivial;
    let mut dpll = Dpll::new(cnf, Box::new(heuristic));
    dpll.solve();
}

fn criterion() -> Criterion<WallTime> {
    Criterion::default()
        .warm_up_time(Duration::from_millis(3000))
        .measurement_time(Duration::from_millis(5000))
        .sample_size(200)
        .noise_threshold(0.1)
}

criterion_group! {
    name = benches;
    config = criterion();
    targets = benchmark
}

criterion_main!(benches);
