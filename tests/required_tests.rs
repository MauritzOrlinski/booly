use dpml::dpll::{
    dpll::Dpll, dpll::DpllResult, heuristics::from_shortest_clause::FromShortestClause,
    heuristics::mom::MOM,
};
use dpml::parser::parse_cnf;
use dpml::verify::verify_satisfied;
use std::path::Path;

fn test_satisfied(_: &Path, input: String) -> datatest_stable::Result<()> {
    let cnf = parse_cnf(input.as_str()).unwrap();
    let heuristic = MOM;
    let mut dpll = Dpll::new(cnf, heuristic);

    let result = dpll.solve();
    assert_eq!(result, DpllResult::Satisfied);
    assert!(verify_satisfied(&dpll.cnf_formula));
    Ok(())
}

fn test_unsatisfiable(_: &Path, input: String) -> datatest_stable::Result<()> {
    let cnf = parse_cnf(input.as_str()).unwrap();
    let heuristic = MOM;
    let mut dpll = Dpll::new(cnf, heuristic);

    let result = dpll.solve();
    assert_eq!(result, DpllResult::Conflict);
    Ok(())
}

datatest_stable::harness! {
    { test = test_satisfied, root = "./inputs/test/sat", pattern = r"^.*\.cnf$" },
    { test = test_satisfied, root = "./inputs/sat", pattern = r"^.*\.cnf$" },
    { test = test_unsatisfiable, root = "./inputs/test/unsat", pattern = r"^.*\.cnf$" },
}
