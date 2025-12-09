use crate::assignment::assignment::{Assignment, AssignmentReason};
use crate::branching::chose_next_assignment::{Branching, MultiAssignment};
use crate::cnf::cnf_formula::CnfFormula;

pub(crate) struct MonienSpeckenmeyer;

impl Branching for MonienSpeckenmeyer {
    fn chose_branches(cnf_formula: &CnfFormula) -> Vec<MultiAssignment> {
        let shortest_clause = cnf_formula
            .clauses
            .iter()
            .map(|(_, clause)| clause)
            .min_by(|a, b| (&a.unassigned_variables).cmp(&b.unassigned_variables))
            .unwrap();

        let mut branches: Vec<MultiAssignment> = vec![];
        let mut unsat_assignments = MultiAssignment::new();

        for (satisfying_variable_id, polarity) in &shortest_clause.literals {
            let sat_assignment = Assignment::new(
                *satisfying_variable_id,
                polarity.get_satisfying_assignment(),
                AssignmentReason::Branching,
            );

            let mut branch = unsat_assignments.clone();
            branch.push(sat_assignment);
            branches.push(branch);

            let unsat_assignment = Assignment::new(
                *satisfying_variable_id,
                polarity.get_satisfying_assignment().get_inverse(),
                AssignmentReason::Branching,
            );

            unsat_assignments.push(unsat_assignment);
        }
        branches
    }
}
