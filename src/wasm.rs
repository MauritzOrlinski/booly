//! Browser bindings for the solver, compiled with `--no-default-features --features wasm`.
//!
//! The single entry point is [`solve`]: it takes a DIMACS CNF string plus a small options
//! object and returns a plain JavaScript object describing the outcome. Everything that
//! needs the file system or the clock on the native side (proof file, timing) is either
//! kept in memory here or left to the calling JavaScript.

use std::cell::RefCell;
use std::io::{self, Write};
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::cdcl::cdcl::{Cdcl, CdclStatus};
use crate::cdcl::heuristics::{RESTART_HEURISTIC_NAMES, restart_heuristic_by_name};
use crate::parser::{parse, parse_cnf};
use crate::preprocess::cnf::cnf::CNF;
use crate::preprocess::niver::recover_assigment_niver_compat;
use crate::preprocess::preprocess::preprocess;
use crate::proof_logger::ProofLogger;

/// Options accepted by [`solve`]; every field is optional on the JavaScript side.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct SolveOptions {
    /// Run the NiVER / subsumption preprocessor first.
    pub preprocess: bool,
    /// Reuse the last value of a variable when deciding on it again.
    pub phase_saving: bool,
    /// One of [`RESTART_HEURISTIC_NAMES`].
    pub restart: String,
    /// Produce a DRAT proof. Forces `preprocess` off, as on the command line.
    pub proof: bool,
}

impl Default for SolveOptions {
    fn default() -> Self {
        SolveOptions {
            preprocess: true,
            phase_saving: false,
            restart: "luby".to_string(),
            proof: false,
        }
    }
}

/// Result object handed back to JavaScript.
#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SolveOutput {
    /// `"sat"`, `"unsat"` or `"error"`.
    pub status: String,
    /// Human readable message when `status == "error"`.
    pub error: Option<String>,
    /// Satisfying assignment as signed literals, one per variable, ascending.
    pub model: Vec<i32>,
    /// `true` when the model was checked against every original clause.
    pub verified: bool,
    pub variables: u32,
    pub clauses: u32,
    pub conflicts: usize,
    pub restarts: usize,
    /// Whether the preprocessor actually ran.
    pub preprocessed: bool,
    /// DRAT proof text when requested.
    pub proof: Option<String>,
}

/// In-memory sink for the proof logger that can be read back after solving.
#[derive(Clone, Default)]
struct SharedBuffer(Rc<RefCell<Vec<u8>>>);

impl Write for SharedBuffer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.borrow_mut().extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

/// Names of the available restart heuristics, for populating a dropdown.
#[wasm_bindgen]
pub fn restart_heuristics() -> Vec<JsValue> {
    RESTART_HEURISTIC_NAMES
        .iter()
        .map(|n| JsValue::from_str(n))
        .collect()
}

/// Solve a DIMACS CNF formula. `options` is a plain object matching [`SolveOptions`].
#[wasm_bindgen]
pub fn solve(dimacs: &str, options: JsValue) -> Result<JsValue, JsValue> {
    let options: SolveOptions = if options.is_undefined() || options.is_null() {
        SolveOptions::default()
    } else {
        serde_wasm_bindgen::from_value(options).map_err(|e| JsValue::from_str(&e.to_string()))?
    };
    let output = solve_to_output(dimacs, options);
    serde_wasm_bindgen::to_value(&output).map_err(|e| JsValue::from_str(&e.to_string()))
}

fn error(message: impl Into<String>) -> SolveOutput {
    SolveOutput {
        status: "error".to_string(),
        error: Some(message.into()),
        ..Default::default()
    }
}

/// Pure-Rust core of [`solve`], also usable from native tests.
pub fn solve_to_output(dimacs: &str, options: SolveOptions) -> SolveOutput {
    let Some(restart_heuristic) = restart_heuristic_by_name(&options.restart) else {
        return error(format!(
            "unknown restart heuristic '{}', expected one of {}",
            options.restart,
            RESTART_HEURISTIC_NAMES.join(", ")
        ));
    };

    // Parse once to get the original clauses for the final model check.
    let (original_clauses, variable_count, clause_count) = match parse(dimacs) {
        Ok(parsed) => parsed,
        Err(e) => {
            let rest: String = e.input.chars().take(40).collect();
            return error(if rest.is_empty() {
                "could not parse DIMACS input: unexpected end of input (does the header's clause count match?)".to_string()
            } else {
                format!("could not parse DIMACS input near: {rest:?}")
            });
        }
    };

    let proof_buffer = SharedBuffer::default();
    let proof_logger = options.proof.then(|| {
        let sink: Box<dyn Write> = Box::new(proof_buffer.clone());
        ProofLogger::new(sink)
    });
    // Proof logging only covers the CDCL loop, so it disables preprocessing (as the CLI does).
    let preprocessed = options.preprocess && !options.proof;

    // A formula whose clauses are all tautologies (or that has none) is trivially satisfiable;
    // the preprocessor cannot represent an empty clause set, so answer directly.
    let all_tautologies = original_clauses
        .iter()
        .all(|clause| clause.iter().any(|lit| clause.contains(&-lit)));
    if all_tautologies {
        return SolveOutput {
            status: "sat".to_string(),
            model: (1..=variable_count as i32).collect(),
            verified: true,
            variables: variable_count as u32,
            clauses: clause_count as u32,
            preprocessed,
            ..Default::default()
        };
    }

    let (status, model, conflicts, restarts) = if preprocessed {
        let mut cnf = CNF::from_pre(&original_clauses, variable_count);
        let niver_trace = preprocess(&mut cnf);
        // The preprocessor may already derive the empty clause; the solver cannot represent it.
        if cnf
            .clauses
            .iter()
            .any(|(_, clause)| clause.active && clause.lits.is_empty())
        {
            return SolveOutput {
                status: "unsat".to_string(),
                variables: variable_count as u32,
                clauses: clause_count as u32,
                preprocessed: true,
                ..Default::default()
            };
        }
        let mut cdcl = Cdcl::new(
            cnf.to_cnf_formula(),
            options.phase_saving,
            restart_heuristic,
            proof_logger,
        );
        let status = cdcl.solve();
        let (conflicts, restarts) = (cdcl.stats.conflict_count, cdcl.stats.number_of_restarts);
        let model = match status {
            CdclStatus::Sat => {
                let assignment = recover_assigment_niver_compat(niver_trace, cnf, cdcl.cnf_formula);
                (1..=variable_count as u32)
                    .map(|v| {
                        if *assignment.get(&v).unwrap_or(&true) {
                            v as i32
                        } else {
                            -(v as i32)
                        }
                    })
                    .collect()
            }
            _ => Vec::new(),
        };
        (status, model, conflicts, restarts)
    } else {
        let cnf_formula = match parse_cnf(dimacs) {
            Ok(value) => value,
            Err(_) => return error("could not parse DIMACS input"),
        };
        let mut cdcl = Cdcl::new(
            cnf_formula,
            options.phase_saving,
            restart_heuristic,
            proof_logger,
        );
        let status = cdcl.solve();
        if let Some(logger) = cdcl.proof_logger.as_mut() {
            logger.flush_to_file().ok();
        }
        let model = match status {
            CdclStatus::Sat => cdcl
                .cnf_formula
                .assignments
                .iter()
                .enumerate()
                .map(|(i, value)| {
                    let v = i as i32 + 1;
                    if value.unwrap_or(true) { v } else { -v }
                })
                .collect(),
            _ => Vec::new(),
        };
        (
            status,
            model,
            cdcl.stats.conflict_count,
            cdcl.stats.number_of_restarts,
        )
    };

    let verified = status == CdclStatus::Sat
        && original_clauses
            .iter()
            .all(|clause| clause.iter().any(|lit| model.contains(lit)));

    let proof = options
        .proof
        .then(|| String::from_utf8_lossy(&proof_buffer.0.borrow()).into_owned());

    SolveOutput {
        status: match status {
            CdclStatus::Sat => "sat",
            CdclStatus::Unsat => "unsat",
            CdclStatus::Incomplete | CdclStatus::Conflict(_) => "error",
        }
        .to_string(),
        error: matches!(status, CdclStatus::Incomplete | CdclStatus::Conflict(_))
            .then(|| "solver stopped without a verdict".to_string()),
        model,
        verified,
        variables: variable_count as u32,
        clauses: clause_count as u32,
        conflicts,
        restarts,
        preprocessed,
        proof,
    }
}
