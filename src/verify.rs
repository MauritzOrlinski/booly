use crate::cnf::cnf_formula::CnfFormula;

/// Verifies that a cnf formula is indeed satisfied.
/// 
/// # Arguments
/// * `cnf_formula` - The CNF formula
/// 
/// # Returns
/// A boolean depending on whether the given CNF formula is satisfied.
pub fn verify_satisfied(cnf_formula: &CnfFormula) -> bool {
    cnf_formula.clauses.iter().all(|clause| {
        if let Some(satisfying_var_id) = clause.satisfied_by {
            let satisfying_var = cnf_formula.variables.get(satisfying_var_id);
            let var_literal_polarity = clause
                .literals
                .get_polarity_for_var(&satisfying_var_id)
                .unwrap();
            return var_literal_polarity.get_satisfying_assignment()
                == satisfying_var.value.unwrap();
        }
        false
    })
}
