use crate::cnf::cnf_formula::CnfFormula;
use crate::dpll::assignment::Assignment;
use crate::dpll::heuristics::{Heuristic, Stats};

#[derive(Debug)]
pub struct StaticJW {
    order: Vec<i32>,
    computed: bool,
    last_assignment: (usize, i32),
}
impl StaticJW {
    pub fn new() -> Self {
        Self {
            order: Vec::new(),
            computed: false,
            last_assignment: (0, 0),
        }
    }
}

impl Default for StaticJW {
    fn default() -> Self {
        Self::new()
    }
}
impl Heuristic for StaticJW {
    fn chose_next_assignment(
        &mut self,
        cnf_formula: &CnfFormula,
        decision_level: i32,
    ) -> Assignment {
        if self.computed {
            let (i, &lit) = self
                .order
                .iter()
                .enumerate()
                .find(|&(_, x)| cnf_formula.variables.get(x.unsigned_abs()).value.is_none())
                .unwrap();
            self.last_assignment = (i, decision_level);
            Assignment {
                variable_id: lit.unsigned_abs(),
                value: lit > 0,
            }
        } else {
            let mut jw_weights: Vec<(i32, f64)> = cnf_formula
                .variables
                .iter()
                .enumerate()
                .map(|(vid, _)| (vid as i32 + 1, 0.0))
                .collect();

            for clause in &cnf_formula.clauses {
                let weight = 2f64.powf(-(clause.literals.len() as f64));
                for (var, _) in clause.literals.iter() {
                    if let Some((_v, w)) = jw_weights.iter_mut().find(|(v, _)| *v == var as i32) {
                        *w += weight;
                    }
                }
            }

            jw_weights.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

            self.order = jw_weights.iter().map(|(v, _)| *v).collect();

            self.computed = true;

            self.chose_next_assignment(cnf_formula, decision_level)
        }
    }

    #[allow(unused_variables)]
    fn feedback(&mut self, feedback: Stats) {}

    fn is_learning(&self) -> bool {
        false
    }
    #[allow(unused_variables)]
    fn save(&self, path: &str) -> std::io::Result<()> {
        Result::Ok(())
    }
}
