use dpml::cdcl::cdcl::{Cdcl, CdclStatus};
use dpml::parser::{parse, parse_cnf};
use dpml::preprocess::cnf::cnf::CNF;
use dpml::preprocess::niver::recover_assigment_niver_compat;
use dpml::preprocess::preprocess::preprocess;
use dpml::verify::verify_satisfied;
use std::path::Path;

fn test_satisfied(_: &Path, input: String) -> datatest_stable::Result<()> {
    let cnf = parse_cnf(input.as_str()).unwrap();
    let mut dpll = Cdcl::new(cnf);

    let result = dpll.solve();
    assert_eq!(result, CdclStatus::Sat);
    assert!(verify_satisfied(&dpll.cnf_formula));

    Ok(())
}

fn test_unsatisfiable(_: &Path, input: String) -> datatest_stable::Result<()> {
    let cnf = parse_cnf(input.as_str()).unwrap();
    let mut dpll = Cdcl::new(cnf);

    let result = dpll.solve();
    assert_eq!(result, CdclStatus::Unsat);
    Ok(())
}

fn test_preprocess_sat(_: &Path, input: String) -> datatest_stable::Result<()> {
    let cnf_pre = parse(input.as_str()).unwrap();

    let mut cnf = CNF::from_pre(&cnf_pre.0, cnf_pre.1);
    let niver_trace = preprocess(&mut cnf);

    let mut cdcl = Cdcl::new(cnf.to_cnf_formula());
    let status = cdcl.solve();

    assert_eq!(status, CdclStatus::Sat);

    let assignment = recover_assigment_niver_compat(niver_trace, cnf, cdcl.cnf_formula);
    let cnf = CNF::from_pre(&cnf_pre.0, cnf_pre.1);
    assert!(cnf.clauses.iter().all(|(_, clause)| {
        clause.lits.iter().any(|lit| {
            if lit.pos() {
                *assignment.get(&lit.var_id()).unwrap()
            } else {
                !*assignment.get(&lit.var_id()).unwrap()
            }
        })
    }));
    Ok(())
}

fn test_preprocess_unsat(_: &Path, input: String) -> datatest_stable::Result<()> {
    let cnf_pre = parse(input.as_str()).unwrap();

    let mut cnf = CNF::from_pre(&cnf_pre.0, cnf_pre.1);
    preprocess(&mut cnf);

    if cnf
        .clauses
        .iter()
        .any(|(_, clause)| clause.active && clause.lits.len() == 0)
    {
        return Ok(());
    }

    let mut cdcl = Cdcl::new(cnf.to_cnf_formula());
    let status = cdcl.solve();

    assert_eq!(status, CdclStatus::Unsat);
    Ok(())
}

datatest_stable::harness! {
    { test = test_preprocess_sat, root = "./inputs/sat", pattern = r"^.*\.cnf$" },
    { test = test_preprocess_unsat, root = "./inputs/unsat", pattern = r"^.*\.cnf$" },
    { test = test_satisfied, root = "./inputs/test/sat", pattern = r"^.*\.cnf$" },
    { test = test_unsatisfiable, root = "./inputs/test/unsat", pattern = r"^.*\.cnf$" },
}
