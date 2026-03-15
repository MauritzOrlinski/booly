use dpml::cdcl::cdcl::Cdcl;
use dpml::cdcl::cdcl::CdclStatus::{Sat, Unsat};
use dpml::parser::parse_cnf;
use std::path::Path;

fn test_satisfied(_: &Path, input: String) -> datatest_stable::Result<()> {
    let cnf = parse_cnf(input.as_str()).unwrap();
    let mut dpll = Cdcl::new(cnf);

    let result = dpll.solve();
    assert_eq!(result, Sat);
    assert!(dpll.cnf_formula.test_sat());
    Ok(())
}

fn test_unsatisfiable(_: &Path, input: String) -> datatest_stable::Result<()> {
    let cnf = parse_cnf(input.as_str()).unwrap();
    let mut dpll = Cdcl::new(cnf);

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
