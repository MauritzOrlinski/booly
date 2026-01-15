use crate::{
    cnf::variable::Variable,
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
const COV_DIAG: f32 = 1000.0;

#[derive(Debug, Serialize, Deserialize)]
pub struct ContextualBandits {
    weights: [f32; 5],
    p: [[f32; 5]; 5],
    epsilon: f64,
    epsilon_decay: f64,
    learn: bool,
    last_branch: LastContext,
}

impl ContextualBandits {
    pub fn new() -> Self {
        let mut p = [[0.0; 5]; 5];
        for i in 0..5 {
            p[i][i] = COV_DIAG;
        }
        Self {
            weights: [0.0; 5],
            p,
            epsilon: EPSILON_INITIAL,
            epsilon_decay: 0.001,
            learn: true,
            last_branch: LastContext {
                pos_len: 1.0,
                neg_len: 1.0,
                unsat_clauses: 1.0,
                dec_level: 1.0,
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
        self.weights[0]
            + self.weights[1] * (v.positive_occurrences.len() as f32)
            + self.weights[2] * (v.negative_occurrences.len() as f32)
            + self.weights[3] * (cnf_formula.unsat_clauses as f32)
            + self.weights[4] * (decision_level as f32)
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
        // epsilon greedy explore/exploit
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

    /// A Recursive Least Square implementation to update the regression continuously, compare to: https://www.geeksforgeeks.org/machine-learning/recursive-least-square-algorithm/
    fn feedback(&mut self, feedback: Stats) {
        let reward = feedback.unit_clauses as f32;

        let feature = [
            1.0,
            self.last_branch.pos_len,
            self.last_branch.neg_len,
            self.last_branch.unsat_clauses,
            self.last_branch.dec_level,
        ];

        let mut prediction = 0.0;
        for i in 0..5 {
            prediction += self.weights[i] * feature[i];
        }

        let error = reward - prediction;

        let mut px = [0.0; 5];
        for i in 0..5 {
            for j in 0..5 {
                px[i] += self.p[i][j] * feature[j];
            }
        }

        let mut denom = 1.0;
        for i in 0..5 {
            denom += feature[i] * px[i];
        }

        let mut k = [0.0; 5];
        for i in 0..5 {
            k[i] = px[i] / denom;
        }

        for i in 0..5 {
            self.weights[i] += k[i] * error;
        }

        let mut new_p = self.p;
        for i in 0..5 {
            for j in 0..5 {
                new_p[i][j] -= k[i] * px[j];
            }
        }
        self.p = new_p;

        self.update_epsilon();
    }

    fn is_learning(&self) -> bool {
        self.learn
    }

    fn save(&self, path: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        fs::write(path, json)
    }
}
