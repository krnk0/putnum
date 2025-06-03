# Putnam

*A tiny Rust SAT solver built around a minimal DPLL core.*

---

## Features (current)

| Area                               | Status | Notes                                                               |
| ---------------------------------- | ------ | ------------------------------------------------------------------- |
| DIMACS CNF parser                  | ✅      | Handles comments, empty clauses, declaration checks                 |
| Core types (`Var`, `Lit`, `Model`) | ✅      | Simple `Vec`‑backed model + trail                                   |
| Unit Propagation                   | ✅      | Queue‑driven, retains only `Undef` literals, detects contradictions |
| DPLL search                        | ✅      | Complete recursive back‑tracking solver with variable selection     |
| CLI (`putnam`)                     | ✅      | File I/O, SAT/UNSAT output, optional model dump                     |

---

## Folder structure

```text
putnam/
├── Cargo.toml            # crate metadata
├── docs/                 # high-level documentation
│   ├── architecture.md   # system design & philosophy
│   ├── tutorial.md       # beginner-friendly guide
│   └── CLAUDE.md         # developer notes
├── src/
│   ├── lib.rs            # public re‑exports & docs
│   ├── types.rs          # core data structures
│   ├── parser.rs         # DIMACS I/O
│   └── solver/           # algorithms
│       ├── mod.rs        # solver namespace
│       ├── unit.rs       # unit_propagate()
│       └── dpll.rs       # complete DPLL solver implementation
├── src/bin/putnam.rs     # CLI entry point
├── tests/                # integration tests
├── benches/              # Criterion benchmark suite
├── benchmarks/           # Test problems (SAT/UNSAT instances)
└── LICENSE               # MIT License
```

---

## Roadmap


  * [x] Implement DIMACS parser + unit tests + integration tests
  * [x] Add queue‑based unit propagation
  * [x] Finish naïve DPLL recursion + tiny SAT/UNSAT test‑suite
  * [x] CLI `putnam <file.cnf>` with `--model` flag
  * [x] Criterion benchmark harness (pigeonhole, chain problems)
  * [ ] Watched‑literal rewrite for O(1) propagation
  * [ ] VSIDS / JW variable heuristics
  * [ ] Conflict‐Driven Clause Learning (CDCL)
  * [ ] Proof logging & DIMACS DRAT export
  * [ ] Python bindings via `pyo3`
  * [ ] WebAssembly demo playground

---

## Getting started

```bash
# clone & build
$ git clone https://github.com/your‑nick/putnam
$ cd putnam
$ cargo test            # run unit + integration tests
```

### Usage

```bash
# Build and test
$ cargo test            # run unit + integration tests
$ cargo bench           # run performance benchmarks

# Run the CLI solver
$ cargo run --bin putnam examples/simple.cnf
$ cargo run --bin putnam examples/simple.cnf -- --model

# Current benchmark results (naive DPLL):
# simple_3var_sat:      ~144ns
# pigeonhole 4→3:       ~723μs  
# pigeonhole 5→4:       ~253ms (optimization target)
```

### Documentation

```bash
# View API documentation
$ cargo doc --open

# Browse guides
$ ls docs/                    # architecture, tutorial, CLAUDE.md
```

---

## Contributing

Bug reports, benchmarks and PRs are welcome! Please open an issue first if you plan a large change.

---

## License

Putnam is released under the **MIT License**. See [`LICENSE`](LICENSE) for the full text.

