use crate::assignment::assignment::Assignment;
use crate::chose_next_assignment::chose_next_assignment::ChooseNextAssignment;
use crate::cnf::cnf_formula::CnfFormula;

struct MonienSpeckenmeyer {}

impl ChooseNextAssignment for MonienSpeckenmeyer {
    fn choose_next_assignment(cnf_formula: CnfFormula) -> Vec<Assignment> {
        todo!()
    }
}