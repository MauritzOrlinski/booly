# Project 2: DPLL Solver

- Grabmann, Hofstetter, Orlinski
- SAT Solving WS25/26

## Requirements

- rust
- gnuplot (For the cactus plots)
- nextest (For the 60s benchmarks with timeouts). See below for installation instructions

## Usage

To get more help on Usage run the command:

```bash
  cargo run --release -- --help
```

### Via Cargo

- Solve a SAT problem in DIMACS CNF format: `cargo run -r -- path/to/inputfile`
- Compile with timing measurements: `cargo run -r --features metadata -- path/to/inputfile`
- Speed benchmarks: `cargo bench` (will execute all example inputs from the test directory
  millions of times to get an exact measurement on performance)
- One minute benchmarks: First install nextest (`cargo install cargo-nextest --locked`), then
  execute

  ```bash
  cargo nextest run -r --color=always 2>&1 | awk '{ c=$0; gsub(/\x1b\[[0-9;]*m/,"",c); print; if (c ~ /^ *Summary /) exit }'
  ```

  (will execute the example inputs from the sat and unsat
  directories with a 60s timeout to determine how many of the problems can be solved in reasonable time).

- Cactus plots: `cargo run -r --bin plot -- TIMEOUT_IN_SECONDS`

### How to use heuristics

To use heuristics use the `--heuristic` flag. We also support combining two heuristics using the `--heuristics combined`, you can specify the `--primary` and `--secondary` heuristic and a `--decision-level-threshold` to decide when which heuristic should be used.

Another special heuristic is the `--heuristic multi-bandit` which implements a simple reinforcement learning scheme (epsilon greedy contextual bandits) and therefore has a state.
You can use the flag `--load-learned-model <FILE>` to load a .json file with a pre-trained model or save a model using the `--save-learned-model <FILE>`.

As an example there is the file `example-model.json` containing a basic model that is pre-trained on some files. We did not spend much time on parameter tuning and training so do not expect an overly precise model.

### As a compiled binary

1. Compile via `cargo build --release`. Binary is being compiled to `./target/release/main`.
2. Execute via `main path/to/inputfile`

```

```
