//! SAT solving algorithms
//!
//! This module contains the core algorithms for solving Boolean satisfiability problems.
//! The implementation is based on the DPLL (Davis-Putnam-Logemann-Loveland) algorithm
//! with unit propagation.
//!
//! # Modules
//!
//! - [`unit`]: Unit propagation implementation for constraint propagation
//! - [`dpll`]: Main DPLL algorithm with systematic search and backtracking

pub(crate) mod unit;
pub mod dpll;
