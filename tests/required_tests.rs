use dpml::dpll::dpll::Dpll;
use dpml::dpll::dpll::DpllStatus;
use dpml::dpll::heuristics::trivial::Trivial;
use dpml::parser::parse;
use dpml::parser::parse_cnf;
use dpml::preprocess::cnf::cnf::CNF;
use dpml::preprocess::niver::recover_assigment_niver_compat;
use dpml::preprocess::preprocess::preprocess;
use dpml::verify::verify_satisfied;
use std::path::Path;

fn test_satisfied(_: &Path, input: String) -> datatest_stable::Result<()> {
    let cnf = parse_cnf(input.as_str()).unwrap();
    let heuristic = Trivial;
    let mut dpll = Dpll::new(cnf, Box::new(heuristic));

    let result = dpll.solve();
    assert_eq!(result, DpllStatus::Sat);
    assert!(verify_satisfied(&dpll.cnf_formula));
    Ok(())
}

fn test_unsatisfiable(_: &Path, input: String) -> datatest_stable::Result<()> {
    let cnf = parse_cnf(input.as_str()).unwrap();
    let heuristic = Trivial;
    let mut dpll = Dpll::new(cnf, Box::new(heuristic));

    let result = dpll.solve();
    assert_eq!(result, DpllStatus::Unsat);
    Ok(())
}

fn test_preprocess_sat(_: &Path, input: String) -> datatest_stable::Result<()> {
    let cnf_pre = parse(input.as_str()).unwrap();

    let mut cnf = CNF::from_pre(&cnf_pre.0, cnf_pre.1);
    let niver_trace = preprocess(&mut cnf);

    let heuristic = Box::new(Trivial);
    let mut dpll = Dpll::new(cnf.to_cnf_formula(), heuristic);
    let status = dpll.solve();

    assert_eq!(status, DpllStatus::Sat);

    let assignment = recover_assigment_niver_compat(niver_trace, cnf, dpll.cnf_formula);
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

    let heuristic = Box::new(Trivial);
    let mut dpll = Dpll::new(cnf.to_cnf_formula(), heuristic);
    let status = dpll.solve();

    assert_eq!(status, DpllStatus::Unsat);
    Ok(())
}

datatest_stable::harness! {
    { test = test_preprocess_sat, root = "./inputs/test/sat", pattern = r"^.*\.cnf$" },
    { test = test_preprocess_unsat, root = "./inputs/test/unsat", pattern = r"^.*\.cnf$" },
    { test = test_satisfied, root = "./inputs/test/sat", pattern = r"^.*\.cnf$" },
    { test = test_unsatisfiable, root = "./inputs/test/unsat", pattern = r"^.*\.cnf$" },
}
