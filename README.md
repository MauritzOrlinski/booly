# booly: A modern CDCL SAT Solver

> A conflict driven clause learning (CDCL) SAT Solver written in Rust.

## Requirements

For running this project a working Rust environment is necessary. To install rust consider following [this guide](https://doc.rust-lang.org/book/ch01-01-installation.html).

## Usage

**Via Cargo**

- Solve a SAT problem in DIMACS CNF format: `cargo run -r -- <PATH_TO_INPUTFILE>`.

**As a compiled binary**

1. Compile via `cargo build -r`. Binary is being compiled to `./target/release/main`.
2. Execute via `main <PATH_TO_INPUTFILE>`.

To get more help on usage, run the command:

```bash
  cargo run -r -- --help
```

### Proof Logging

To use proof logging in DRAT-Trim format, use

```bash
  cargo run -r -- --proof-logging --disable-preprocess <PATH_TO_INPUTFILE>
```

This will create a proof certificate `proof.drat`.

### Preprocessing

Preprocessing is enabled by default. To disable it, use

```bash
  cargo run -r -- --disable-preprocess <PATH_TO_INPUTFILE>
```

### Restart Heuristics

We implemented multiple restart heuristics. Luby is chosen by default.

```bash
  cargo run -r -- --restart-heuristic <RESTART_HEURISTIC> <PATH_TO_INPUTFILE>
```

Possible restart heuristics are:

- never
- fixed-interval
- geometric
- luby

### Phase Saving

To enable phase saving, use

```bash
  cargo run -r -- --phase-saving <PATH_TO_INPUTFILE>
```
