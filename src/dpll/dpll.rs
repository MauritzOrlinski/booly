use crate::assignment::assignment::Assignment;
use crate::branching::chose_next_assignment::Branching;
use crate::branching::monien_speckenmeyer::MonienSpeckenmeyer;
use crate::cnf::cnf_formula::CnfFormula;
use std::collections::VecDeque;

pub fn dpll(cnf_formula: &mut CnfFormula) {
    let mut assignment_stack: Vec<Assignment> = Vec::new();

    let branches = MonienSpeckenmeyer::chose_branches(cnf_formula);
    let mut unit_queue: VecDeque<usize> = VecDeque::new();

    for branch in branches {
        'single_assignment: for single_assignment in branch {
            let assignment_result = cnf_formula.apply_assignment(&single_assignment, &mut unit_queue);
            match assignment_result {
                Ok(_) => continue,
                Err(_) => {
                    while let Some(assignment) = assignment_stack.pop() {
                        let assignment_result = cnf_formula.reverse_assignment(&single_assignment, &mut unit_queue);
                    }
                }
            }
        }
    }
}
