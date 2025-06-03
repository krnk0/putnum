//! # Putnam SAT Solver
//!
//! A minimal DPLL-based SAT solver built around a clean, modular architecture.
//! 
//! Putnam provides a complete implementation of the Davis-Putnam-Logemann-Loveland (DPLL)
//! algorithm with unit propagation for solving Boolean satisfiability problems.
//!
//! ## Architecture Overview
//!
//! The solver is organized into four main layers:
//!
//! - **Data Types** ([`types`]): Core data structures for variables, literals, clauses, and models
//! - **Parser** ([`parser`]): DIMACS CNF format parsing and conversion
//! - **Solver** ([`solver`]): DPLL algorithm implementation with unit propagation
//! - **CLI** (bin/putnam): Command-line interface for file-based solving
//!
//! ## Quick Start
//!
//! ```rust
//! use putnam::{solve, types::*, solver::dpll::SolveResult};
//!
//! // Create a simple formula: (x0) ∧ (¬x1)
//! let formula = vec![
//!     vec![Lit { var: 0, neg: false }],  // (x0)
//!     vec![Lit { var: 1, neg: true }],   // (¬x1)
//! ];
//!
//! match solve(&formula, 2) {
//!     SolveResult::Sat(model) => {
//!         println!("Satisfiable!");
//!         // model.value(0) == Val::True
//!         // model.value(1) == Val::False
//!     }
//!     SolveResult::Unsat => println!("Unsatisfiable"),
//! }
//! ```
//!
//! ## Features
//!
//! - **Complete DPLL implementation**: Systematic search with backtracking
//! - **Unit propagation**: Automatic constraint propagation for efficiency
//! - **DIMACS parsing**: Standard CNF format support
//! - **Clean API**: Simple, type-safe interface
//! - **Performance benchmarks**: Criterion-based measurement suite

pub mod types;
pub mod parser;
pub mod solver;

pub use solver::dpll::solve;

