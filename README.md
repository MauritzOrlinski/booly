# Project 2: DPLL Solver

- Grabmann, Hofstetter, Orlinski
- SAT Solving WS25/26

## Requirements

- rust
- gnuplot (For the cactus plots)
- nextest (For the 60s benchmarks with timeouts)

## Usage

### Via Cargo

- Solve a SAT problem in DIMACS CNF format: `cargo run -r -- path/to/inputfile`
- Compile with timing measurements: `cargo run -r --features metadata -- path/to/inputfile`
- Speed benchmarks: `cargo bench` (will execute all example inputs from the test directory
  millions of times to get an exact measurement on performance)
- One minute benchmarks: First install nextest (`cargo install cargo-nextest --locked`), then
  execute `cargonextest run --no-fail-fast` (will execute the example inputs from the sat and unsat
  directories with a 60s timeout to determine how many of problems can be solved in reasonable time)
- Cactus plots: `cargo run -r --bin plot -- TIMEOUT_IN_SECONDS`

### As a compiled binary

1. Compile via `cargo build --release`. Binary is being compiled to `./target/release/main`.
2. Execute via `main path/to/inputfile`
