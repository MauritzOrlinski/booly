use crate::{
    cnf::{cnf_formula, variable::Variable},
    dpll::{
        assignment::Assignment,
        heuristics::{Heuristic, Stats},
    },
};
use rand::{Rng, seq::IteratorRandom};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct LastContext {
    pos_len: f32,
    neg_len: f32,
    unsat_clauses: f32,
    dec_level: f32,
}

const EPSILON_INITIAL: f64 = 0.5;
const EPSILON_MIN: f64 = 0.001;

#[derive(Debug, Serialize, Deserialize)]
pub struct ContextualBandits {
    weight_pos_len: f32,
    weight_neg_len: f32,
    weight_unsat_clauses: f32,
    weight_dec_level: f32,
    epsilon: f64,
    epsilon_decay: f64,
    learning_rate: f32,
    last_branch: LastContext,
}

impl ContextualBandits {
    pub fn new() -> Self {
        Self {
            weight_pos_len: 1.0,
            weight_neg_len: 1.0,
            weight_unsat_clauses: 1.0,
            weight_dec_level: 0.0,
            epsilon: EPSILON_INITIAL,
            epsilon_decay: 0.001,
            learning_rate: 0.1,
            last_branch: LastContext {
                pos_len: 0.0,
                neg_len: 0.0,
                unsat_clauses: 0.0,
                dec_level: 0.0,
            },
        }
    }
    pub fn load_or_new(path: &str) -> Self {
        if Path::new(path).exists() {
            match fs::read_to_string(path) {
                Ok(contents) => match serde_json::from_str::<ContextualBandits>(&contents) {
                    Ok(model) => {
                        println!("Loaded model from {}", path);
                        return model;
                    }
                    Err(_) => {
                        eprintln!("No valid Model found. Creating new model.");
                    }
                },
                Err(_) => {
                    eprintln!("No valid Model found. Creating new model.");
                }
            }
        }
        println!("No saved model found. Creating new one.");
        Self::new()
    }
    /// Evaluation function using the weights that we try to optimize
    fn eval(
        &self,
        v: &Variable,
        cnf_formula: &crate::cnf::cnf_formula::CnfFormula,
        decision_level: i32,
    ) -> f32 {
        self.weight_pos_len * (v.positive_occurrences.len() as f32)
            + self.weight_neg_len * (v.negative_occurrences.len() as f32)
            + self.weight_unsat_clauses * (cnf_formula.unsat_clauses as f32)
            + self.weight_dec_level * (decision_level as f32)
    }
    fn update_epsilon(&mut self) {
        self.epsilon = EPSILON_MIN + (self.epsilon - EPSILON_MIN) * (-self.epsilon_decay).exp();
    }
}

impl Default for ContextualBandits {
    fn default() -> Self {
        Self::new()
    }
}

impl Heuristic for ContextualBandits {
    fn chose_next_assignment(
        &mut self,
        cnf_formula: &crate::cnf::cnf_formula::CnfFormula,
        decision_level: i32,
    ) -> crate::dpll::assignment::Assignment {
        let r = rand::rng().random_bool(self.epsilon);
        // eqsilon greedy explore/exploit
        if r {
            let (choice, v) = cnf_formula
                .variables
                .iter()
                .enumerate()
                .filter(|&(_, v)| v.value.is_none())
                .choose(&mut rand::rng())
                .unwrap();
            self.last_branch = LastContext {
                pos_len: v.positive_occurrences.len() as f32,
                neg_len: v.negative_occurrences.len() as f32,
                unsat_clauses: cnf_formula.unsat_clauses as f32,
                dec_level: decision_level as f32,
            };
            Assignment {
                variable_id: choice as u32 + 1,
                value: v.positive_occurrences.len() > v.negative_occurrences.len(),
            }
        } else {
            let (choice, v) = cnf_formula
                .variables
                .iter()
                .enumerate()
                .filter(|&(_, v)| v.value.is_none())
                .max_by(|&(_, v1), &(_, v2)| {
                    self.eval(v1, cnf_formula, decision_level)
                        .partial_cmp(&self.eval(v2, cnf_formula, decision_level))
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .unwrap();
            self.last_branch = LastContext {
                pos_len: v.positive_occurrences.len() as f32,
                neg_len: v.negative_occurrences.len() as f32,
                unsat_clauses: cnf_formula.unsat_clauses as f32,
                dec_level: decision_level as f32,
            };
            Assignment {
                variable_id: choice as u32 + 1,
                value: v.positive_occurrences.len() > v.negative_occurrences.len(),
            }
        }
    }

    fn feedback(&mut self, feedback: Stats) {
        let reward = (feedback.unit_clauses as f32).ln_1p();
        let predicted = self.weight_pos_len * self.last_branch.pos_len
            + self.weight_neg_len * self.last_branch.neg_len
            + self.weight_unsat_clauses * self.last_branch.unsat_clauses
            + self.weight_dec_level * self.last_branch.dec_level;
        let error = reward - predicted;
        let improvement = self.learning_rate * error;
        self.weight_pos_len += improvement * self.last_branch.pos_len;
        self.weight_neg_len += improvement * self.last_branch.neg_len;
        self.weight_unsat_clauses += improvement * self.last_branch.unsat_clauses;
        self.weight_dec_level += improvement * self.last_branch.dec_level;
        self.update_epsilon();
    }

    fn is_learning(&self) -> bool {
        true
    }

    fn save(&self, path: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        fs::write(path, json)
    }
}
