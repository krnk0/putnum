//! DIMACS CNF format parser
//! 
//! This module provides functionality to parse DIMACS CNF (Conjunctive Normal Form)
//! files and convert them to the internal representation used by the solver.
//!
//! The DIMACS format is the standard format for representing SAT problems.
//! It consists of:
//! - Comment lines starting with 'c'
//! - A problem line starting with 'p cnf' followed by variable count and clause count
//! - Clause lines containing space-separated literals, terminated by 0
//!
//! # Example DIMACS file
//!
//! ```text
//! c This is a comment
//! p cnf 3 2
//! 1 -3 0
//! 2 3 -1 0
//! ```

use std::io::{self, BufRead};
use crate::types::{Lit, Formula};

/// Internal representation of a DIMACS literal (with sign)
#[derive(Debug, Copy, Clone)]
struct DimacsLiteral(i32);

/// Internal representation of a DIMACS clause
type DimacsClause = Vec<DimacsLiteral>;

/// Internal representation of a DIMACS formula
type DimacsFormula = Vec<DimacsClause>;

/// Parses a DIMACS CNF format from a reader.
///
/// This function reads DIMACS format line by line, ignoring comments
/// and problem declarations, and extracting clauses.
///
/// # Arguments
///
/// * `r` - A reader implementing `BufRead` trait
///
/// # Returns
///
/// * `Ok(DimacsFormula)` - The parsed formula in DIMACS representation
/// * `Err(io::Error)` - If reading fails
///
/// # Format Details
///
/// - Lines starting with 'c' or '%' are treated as comments
/// - Lines starting with 'p' are problem declarations (ignored)
/// - Other lines contain clauses: space-separated integers ending with 0
/// - Positive integers represent positive literals
/// - Negative integers represent negative literals
///
/// # Examples
///
/// ```no_run
/// use std::io::Cursor;
/// # use putnam::parser::*;
/// 
/// let input = "c comment\np cnf 2 1\n1 -2 0\n";
/// let reader = Cursor::new(input);
/// // let result = parse_dimacs(reader)?;
/// ```
fn parse_dimacs<R: BufRead>(r: R) -> io::Result<DimacsFormula> {
    let mut formula = Vec::new();

    for line in r.lines() {
        let line = line?;
        let line = line.trim();

        match line.chars().next() {
            Some('c') | Some('%') |  None => continue, // コメント等
            Some('p') => continue,                                // 問題行は今回は無視
            _ => {
                let lits = line
                    .split_whitespace()
                    .map(|tok| tok.parse::<i32>().unwrap())
                    .take_while(|&n| n != 0)                     // 末尾 0 を捨てる
                    .map(DimacsLiteral)
                    .collect::<Vec<_>>();
                formula.push(lits);
            }
        }
    }
    Ok(formula)
}

/// Converts DIMACS representation to internal solver representation.
///
/// This function performs several transformations:
/// - Converts 1-based DIMACS variable numbering to 0-based internal numbering
/// - Converts `DimacsLiteral` to internal `Lit` structures
/// - Determines the maximum variable number for model initialization
///
/// # Arguments
///
/// * `dimacs_formula` - The formula in DIMACS representation
///
/// # Returns
///
/// A tuple containing:
/// * `Formula` - The formula in internal representation
/// * `usize` - The number of variables in the formula
///
/// # Examples
///
/// ```no_run
/// # use putnam::parser::*;
/// # use putnam::types::*;
/// // Assuming we have a DimacsFormula
/// # let dimacs_formula = vec![];
/// let (formula, num_vars) = convert_to_internal(dimacs_formula);
/// ```
fn convert_to_internal(dimacs_formula: DimacsFormula) -> (Formula, usize) {
    let mut max_var = 0;
    let mut formula = Vec::new();
    
    for dimacs_clause in dimacs_formula {
        let mut clause = Vec::new();
        for dimacs_lit in dimacs_clause {
            let var_num = dimacs_lit.0.abs() as usize;
            if var_num > 0 {
                let var = var_num - 1; // Convert to 0-based
                max_var = max_var.max(var);
                clause.push(Lit {
                    var,
                    neg: dimacs_lit.0 < 0,
                });
            }
        }
        formula.push(clause);
    }
    
    (formula, max_var + 1)
}

/// Parses DIMACS CNF format and converts to internal representation.
///
/// This is the main public interface for parsing DIMACS files. It combines
/// the parsing and conversion steps into a single convenient function.
///
/// # Arguments
///
/// * `reader` - A reader implementing `BufRead` trait (e.g., `BufReader<File>`)
///
/// # Returns
///
/// * `Ok((Formula, usize))` - The parsed formula and variable count
/// * `Err(io::Error)` - If reading or parsing fails
///
/// # Examples
///
/// ```no_run
/// use std::fs::File;
/// use std::io::BufReader;
/// use putnam::parser::parse_and_convert;
///
/// let file = File::open("example.cnf")?;
/// let reader = BufReader::new(file);
/// let (formula, num_vars) = parse_and_convert(reader)?;
/// 
/// println!("Parsed {} variables and {} clauses", num_vars, formula.len());
/// # Ok::<(), std::io::Error>(())
/// ```
///
/// # DIMACS Format
///
/// The function expects standard DIMACS CNF format:
/// ```text
/// c Optional comments
/// p cnf <num_vars> <num_clauses>
/// <literal1> <literal2> ... 0
/// <literal1> <literal2> ... 0
/// ...
/// ```
///
/// Where literals are non-zero integers (positive for variables, negative for negations).
pub fn parse_and_convert<R: BufRead>(reader: R) -> io::Result<(Formula, usize)> {
    let dimacs_formula = parse_dimacs(reader)?;
    Ok(convert_to_internal(dimacs_formula))
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::solve;
    use crate::solver::dpll::SolveResult;
    use crate::types::Val;

    /// 文字列から直接パースするヘルパ
    fn parse_str(src: &str) -> DimacsFormula {
        parse_dimacs(src.as_bytes()).expect("parse failed")
    }

    /// (x1) だけの最小 SAT
    #[test]
    fn single_unit_clause() {
        let f = parse_str("p cnf 1 1\n1 0\n");

        assert_eq!(f.len(), 1);            // 節は 1 個
        assert_eq!(f[0].len(), 1);         // リテラルも 1 個
        assert_eq!(f[0][0].0, 1);          // 中身が 1
    }

    /// コメント混在＆否定リテラルを含む複数節
    ///   (x1 ∨ ¬x2 ∨ x3) ∧ (¬x1)
    #[test]
    fn negated_literals_and_comments() {
        let dimacs = "\
c example with comments
p cnf 3 2
1 -2 3 0
-1 0
";
        let f = parse_str(dimacs);

        assert_eq!(f.len(), 2);
        // 1 つ目の節
        assert_eq!(f[0][0].0, 1);
        assert_eq!(f[0][1].0, -2);
        assert_eq!(f[0][2].0, 3);
        // 2 つ目の節
        assert_eq!(f[1][0].0, -1);
    }

    /// 空節 (0) を含む ―― DPLL テスト用の最小 UNSAT 入力
    #[test]
    fn empty_clause_unsat() {
        let f = parse_str("p cnf 0 1\n0\n");

        assert_eq!(f.len(), 1);            // 節は 1 個
        assert!(f[0].is_empty());          // その節が空
    }

    /// 統合テスト: DIMACS → 内部表現 → ソルバー
    #[test]
    fn integration_parse_and_solve() {
        // Simple SAT case: (x1) AND (NOT x2)
        let (formula, num_vars) = parse_and_convert("p cnf 2 2\n1 0\n-2 0\n".as_bytes()).unwrap();
        
        assert_eq!(num_vars, 2);
        assert_eq!(formula.len(), 2);
        
        match solve(&formula, num_vars) {
            SolveResult::Sat(model) => {
                assert_eq!(model.value(0), Val::True);   // x1 = True
                assert_eq!(model.value(1), Val::False);  // x2 = False
            }
            SolveResult::Unsat => panic!("Expected SAT")
        }
        
        // Simple UNSAT case: (x1) AND (NOT x1)
        let (formula, num_vars) = parse_and_convert("p cnf 1 2\n1 0\n-1 0\n".as_bytes()).unwrap();
        
        assert_eq!(num_vars, 1);
        assert_eq!(solve(&formula, num_vars), SolveResult::Unsat);
    }
}

