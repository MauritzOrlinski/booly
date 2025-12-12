use crate::assignment::assignments::Assignments;
use crate::assignment::single_assignment::AssignmentReason::Forced;
use crate::assignment::single_assignment::{AssignmentValue, SingleAssignment};
use crate::branching::chose_next_assignment::{Branching};
use crate::branching::monien_speckenmeyer::MonienSpeckenmeyer;
use crate::cnf::cnf_formula::{AssignException, CnfFormula};
use std::collections::VecDeque;
use tracing::{instrument, trace};
use crate::branching::chose_next_variable::{ChooseNextVariable, TrivialChooseNextVariable};
use crate::dpll::dpll::DpllResult::{Unknown, Unsatisfiable, Satisfied};

#[derive(Debug, PartialEq)]
pub enum DpllResult {
    Satisfied,
    Unknown,
    Unsatisfiable,
}

impl DpllResult {
    pub fn as_str(&self) -> &str {
        match self {
            Satisfied => "SATISFIABLE",
            Unknown => "UNKNOWN",
            Unsatisfiable => "UNSATISFIABLE",
        }
    }
}

#[derive(Debug)]
pub struct Dpll {
    unit_queue: VecDeque<usize>,
    cnf_formula: CnfFormula,
    assignment_stack: Vec<(u32, SingleAssignment)>,
}

impl Dpll {
    pub fn new(cnf_formula: CnfFormula) -> Dpll {
        Dpll {
            cnf_formula,
            unit_queue: VecDeque::new(),
            assignment_stack: Vec::new(),
        }
    }

    #[instrument(
        skip_all,
        fields(variables = %self.cnf_formula.variables)
    )]
    pub fn dpll(&mut self, depth: u32) -> DpllResult {
        if self.cnf_formula.is_satisfied() {
            return Satisfied;
        }

        let unit_propagation_result = self.propagate_unit_clauses(depth);
        match unit_propagation_result {
            Satisfied => return Satisfied,
            Unsatisfiable => return Unsatisfiable,
            _ => (),
        }

        let branch_a = TrivialChooseNextVariable::chose(&self.cnf_formula);
        let branch_b = branch_a.inverse();

        match self.handle_assign_single_branch(branch_a, depth) {
            Satisfied => return Satisfied,
            Unknown => panic!("This should not happen."),
            Unsatisfiable => ()
        }

        match self.handle_assign_single_branch(branch_b, depth) {
            Satisfied => Satisfied,
            Unknown => panic!("This should not happen."),
            Unsatisfiable => Unsatisfiable
        }

    }

    #[instrument(
        skip_all,
        fields(assignment = %assignment)
    )]
    fn handle_assign_single_branch(&mut self, assignment: SingleAssignment, depth: u32) -> DpllResult {
        let assignment_result = self
            .cnf_formula
            .apply_assignment(&assignment, &mut self.unit_queue);

        match assignment_result {
            Ok(_) => {
                self.assignment_stack.push((depth, assignment));
                let branch_result = self.dpll(depth + 1);
                match branch_result {
                    Satisfied => Satisfied,
                    Unknown => panic!("This should not happen."),
                    Unsatisfiable => {
                        let (_, assignment) = self.assignment_stack.pop().unwrap();
                        self.cnf_formula.reverse_assignment(&assignment);
                        Unsatisfiable
                    }
                }
            }
            Err(_) => {
                self.cnf_formula.reverse_assignment(&assignment);
                Unsatisfiable
            }
        }
    }

    #[instrument(
        skip_all,
        fields(unit_queue = ?self.unit_queue),
    )]
    fn propagate_unit_clauses(&mut self, depth: u32) -> DpllResult {
        while let Some(unit_clause_id) = self.unit_queue.pop_front() {
            let unit_clause = self.cnf_formula.clauses.get(&unit_clause_id).unwrap();
            let satisfying_assignment = unit_clause
                .literals
                .iter()
                .find_map(|(variable_id, polarity)| {
                    let variable = self.cnf_formula.variables.get(variable_id).unwrap();
                    match variable.value {
                        None => Some(SingleAssignment::new(
                            *variable_id,
                            polarity.get_satisfying_assignment(),
                            Forced,
                        )),
                        Some(_) => None
                    }
                })
                .unwrap();

            let assignment_result = self
                .cnf_formula
                .apply_assignment(&satisfying_assignment, &mut self.unit_queue);
            self.assignment_stack.push((depth, satisfying_assignment));

            if matches!(assignment_result, Err(_)) {
                trace!("Could not assign unit clause. Branch is unsatisfiable.");
                self.undo_assignment_stack(depth);
                return Unsatisfiable;
            }
            if self.cnf_formula.is_satisfied() {
                trace!("Branch has been satisfied while doing unit propagation.");
                return Satisfied;
            }
        }
        Unknown
    }

    #[instrument(
        skip_all,
        fields(depth = depth),
    )]
    pub fn undo_assignment_stack(&mut self, depth: u32) {
        while let Some((stack_depth, assignment)) = self.assignment_stack.pop() {
            if stack_depth >= depth {
                self.cnf_formula.reverse_assignment(&assignment);
            } else {
                self.assignment_stack.push((stack_depth, assignment));
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::check::check::check_satisfied;
    use crate::parser::parser::parse_cnf;
    use tracing_subscriber::Registry;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_tree::HierarchicalLayer;

    fn init_tracing() {
        let layer = HierarchicalLayer::default()
            .with_indent_lines(false)
            .with_indent_amount(2);
        let subscriber = Registry::default().with(layer);

        tracing::subscriber::set_global_default(subscriber).ok();
    }

    fn test_from_file(cnf_string: &str, sat: DpllResult) {
        init_tracing();
        let cnf = parse_cnf(cnf_string).unwrap();
        let mut dpll = Dpll::new(cnf);

        let result = dpll.dpll(0);
        assert_eq!(result, sat);
        assert!(check_satisfied(&dpll.cnf_formula))
    }

    #[test]
    fn test_dpll_xor() {
        test_from_file(
            include_str!("../../inputs/test/sat/xor.cnf"),
            DpllResult::Satisfied,
        );
    }

    #[test]
    fn test_dpll_unit() {
        test_from_file(
            include_str!("../../inputs/test/sat/unit.cnf"),
            DpllResult::Satisfied,
        );
    }

    #[test]
    fn test_dpll_unique() {
        test_from_file(
            include_str!("../../inputs/test/sat/unique.cnf"),
            DpllResult::Satisfied,
        );
    }
}
