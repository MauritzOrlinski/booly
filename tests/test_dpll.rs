use dpml::check::check::check_satisfied;
use dpml::dpll::dpll::{Dpll, DpllResult};
use dpml::parser::parser::parse_cnf;
use dpml::dpll::dpll::DpllResult::{Satisfied, Unsatisfiable};

fn test_from_file(cnf_string: &str, sat: DpllResult) {
    let cnf = parse_cnf(cnf_string).unwrap();
    let mut dpll = Dpll::new(cnf);

    let result = dpll.dpll(0);
    assert_eq!(result, sat);
    if sat == Satisfied {
        assert!(check_satisfied(&dpll.cnf_formula))
    }
}

macro_rules! test {
     ($name:ident, $path:expr, $expected:expr $(,)?) => {
            #[test]
            fn $name() {
                test_from_file(include_str!($path), $expected);
            }
        };
}

test!(test_dpll_count4_2, "../inputs/test/sat/count4_2.cnf", Satisfied);
test!(test_dpll_count6_2, "../inputs/test/sat/count6_2.cnf", Satisfied);
test!(test_dpll_count6_3, "../inputs/test/sat/count6_3.cnf", Satisfied);
test!(test_dpll_hole3_3, "../inputs/test/sat/hole3_3.cnf", Satisfied);
test!(test_dpll_kcolor, "../inputs/test/sat/kcolor.cnf", Satisfied);
test!(test_dpll_matching4_3, "../inputs/test/sat/matching4_3.cnf", Satisfied);
test!(test_dpll_matching6_4, "../inputs/test/sat/matching6_4.cnf", Satisfied);
test!(test_dpll_parity4, "../inputs/test/sat/parity4.cnf", Satisfied);
test!(test_dpll_parity6, "../inputs/test/sat/parity6.cnf", Satisfied);
test!(test_dpll_rand6, "../inputs/test/sat/rand6.cnf", Satisfied);
test!(test_dpll_tent4_4, "../inputs/test/sat/tent4_4.cnf", Satisfied);
test!(test_dpll_unique, "../inputs/test/sat/unique.cnf", Satisfied);
test!(test_dpll_unit, "../inputs/test/sat/unit.cnf", Satisfied);
test!(test_dpll_xor, "../inputs/test/sat/xor.cnf", Satisfied);

test!(test_dpll_count7_2, "../inputs/test/unsat/count7_2.cnf", Unsatisfiable);
test!(test_dpll_hole2, "../inputs/test/unsat/hole2.cnf", Unsatisfiable);
test!(test_dpll_hole5, "../inputs/test/unsat/hole5.cnf", Unsatisfiable);
test!(test_dpll_nop, "../inputs/test/unsat/nop.cnf", Unsatisfiable);
test!(test_dpll_nop2, "../inputs/test/unsat/nop2.cnf", Unsatisfiable);
test!(test_dpll_op5, "../inputs/test/unsat/op5.cnf", Unsatisfiable);
test!(test_dpll_op7, "../inputs/test/unsat/op7.cnf", Unsatisfiable);
test!(test_dpll_parity5, "../inputs/test/unsat/parity5.cnf", Unsatisfiable);
test!(test_dpll_rand3, "../inputs/test/unsat/rand3.cnf", Unsatisfiable);
test!(test_dpll_rand3_2, "../inputs/test/unsat/rand3_2.cnf", Unsatisfiable);
test!(test_dpll_stone_pyramid4_2, "../inputs/test/unsat/stone_pyramid4_2.cnf", Unsatisfiable);
test!(test_dpll_subset6, "../inputs/test/unsat/subset6.cnf", Unsatisfiable);
test!(test_dpll_tent2_2, "../inputs/test/unsat/tent2_2.cnf", Unsatisfiable);
test!(test_dpll_tree5, "../inputs/test/unsat/tree5.cnf", Unsatisfiable);
test!(test_dpll_tseitin, "../inputs/test/unsat/tseitin.cnf", Unsatisfiable);
