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
        clause
            .literals
            .iter()
            .any(|(variable_id, var_literal_polarity)| {
                let var = cnf_formula.variables.get(variable_id);
                var.value == Some(var_literal_polarity.get_satisfying_assignment())
            })
    })
}
