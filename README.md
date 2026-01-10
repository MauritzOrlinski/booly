# Project 2: DPLL Solver
- Grabmann, Hofstetter, Orlinski
- SAT Solving WS25/26

## Requirements
- Rust
- gnuplot (For the cactus plots)

## Usage
### Via Cargo

- Solve a SAT problem in DIMACS CNF format:  `cargo run path/to/inputfile`
- Compile with timing measurements: `cargo run --features metadata -- path/to/inputfile`
- Speed benchmarks: `cargo bench` (will execute all example inputs from the test directory 
  millions of times to get an exact measurement on performance)
- One minute benchmarks: `cargo nextest run --no-fail-fast` (will execute the example inputs 
  from the sat and unsat directories with a 60s timeout to determine how many of problems 
  can be solved in reasonable time)
- Cactus plots: TODO!

### As a compiled binary

1. Compile via `cargo build --release`. Binary is being compiled to `./target/release/main`.
2. Execute via `main path/to/inputfile`

