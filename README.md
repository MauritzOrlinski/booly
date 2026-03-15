# Project 3: CDCL Solver

- Grabmann, Hofstetter, Orlinski
- SAT Solving WS25/26

## Requirements

- rust

## Usage

To get more help on usage, run the command:

```bash
  cargo run -r -- --help
```

### Via Cargo

- Solve a SAT problem in DIMACS CNF format: `cargo run -r -- path/to/inputfile`.

### As a compiled binary

1. Compile via `cargo build -r`. Binary is being compiled to `./target/release/main`.
2. Execute via `main path/to/inputfile`.
3. Optionally: Disable preprocessing with the `-d` flag.
