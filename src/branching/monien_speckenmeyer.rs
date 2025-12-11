use crate::assignment::assignments::Assignments;
use crate::assignment::single_assignment::{SingleAssignment, AssignmentReason};
use crate::branching::chose_next_assignment::{Branching, MultiAssignment};
use crate::cnf::cnf_formula::CnfFormula;

pub(crate) struct MonienSpeckenmeyer;

impl Branching for MonienSpeckenmeyer {
    fn chose_branches(cnf_formula: &CnfFormula) -> Vec<Assignments> {
        let shortest_clause = cnf_formula
            .clauses
            .iter()
            .map(|(_, clause)| clause)
            .filter(|clause| matches!(clause.satisfied_by, None))
            .min_by(|a, b| (&a.unassigned_variables).cmp(&b.unassigned_variables))
            .unwrap();

        let mut branches: Vec<Assignments> = vec![];
        let mut unsat_assignments = Assignments::new();

        for (satisfying_variable_id, polarity) in shortest_clause.literals.iter() {

            let variable = cnf_formula.variables.get(satisfying_variable_id).unwrap();
            if matches!(variable.value, Some(_)) {
                continue;
            }

            let sat_assignment = SingleAssignment::new(
                *satisfying_variable_id,
                polarity.get_satisfying_assignment(),
                AssignmentReason::Branching,
            );

            let mut branch = unsat_assignments.clone();
            branch.push(sat_assignment);
            branches.push(branch);

            let unsat_assignment = SingleAssignment::new(
                *satisfying_variable_id,
                polarity.get_satisfying_assignment().get_inverse(),
                AssignmentReason::Branching,
            );

            unsat_assignments.push(unsat_assignment);
        }
        branches.push(unsat_assignments);
        branches
    }
}
