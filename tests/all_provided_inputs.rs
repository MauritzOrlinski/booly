use dpml::cdcl::cdcl::CdclStatus::{Sat, Unsat};
use dpml::cdcl::heuristics::trivial::Trivial;
use dpml::parser::parse_cnf;
use std::path::Path;
use dpml::cdcl::cdcl::Cdcl;

fn test_satisfied(_: &Path, input: String) -> datatest_stable::Result<()> {
    let cnf = parse_cnf(input.as_str()).unwrap();
    let heuristic = Trivial;
    let mut dpll = Cdcl::new(cnf, Box::new(heuristic));

    let result = dpll.solve();
    assert_eq!(result, Sat);
    Ok(())
}

fn test_unsatisfiable(_: &Path, input: String) -> datatest_stable::Result<()> {
    let cnf = parse_cnf(input.as_str()).unwrap();
    let heuristic = Trivial;
    let mut dpll = Cdcl::new(cnf, Box::new(heuristic));

    let result = dpll.solve();
    assert_eq!(result, Unsat);
    Ok(())
}

datatest_stable::harness! {
    { test = test_satisfied, root = "./inputs/test/sat", pattern = r"^.*\.cnf$" },
    { test = test_satisfied, root = "./inputs/sat", pattern = r"^.*\.cnf$" },
    { test = test_unsatisfiable, root = "./inputs/test/unsat", pattern = r"^.*\.cnf$" },
    { test = test_unsatisfiable, root = "./inputs/unsat", pattern = r"^.*\.cnf$" },
}
