use crate::cnf::cnf_formula::CnfFormula;

pub fn verify_satisfied(cnf_formula: &CnfFormula) -> bool {
    cnf_formula.clauses.iter().all(|(_, clause)| {
        if let Some(satisfying_var_id) = clause.satisfied_by {
            let satisfying_var = cnf_formula.variables.get(&satisfying_var_id).unwrap();
            let var_literal_polarity = clause.literals.get_polarity_for_var(&satisfying_var_id).unwrap();
            return var_literal_polarity.get_satisfying_assignment() == satisfying_var.value.unwrap();
        }
        false
    })
}