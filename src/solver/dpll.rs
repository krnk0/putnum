//! DPLL (Davis-Putnam-Logemann-Loveland) algorithm implementation
//!
//! The DPLL algorithm is a complete, sound, and terminating algorithm for
//! deciding the satisfiability of propositional logic formulas in CNF.
//!
//! This implementation includes:
//! - Unit propagation for constraint propagation
//! - Systematic variable selection
//! - Backtracking search with branch pruning
//! - Early termination on satisfiability or unsatisfiability

use crate::types::*;
use super::unit::unit_propagate;

/// Result of a SAT solving attempt.
///
/// This enum represents the two possible outcomes when solving a SAT problem:
/// either the formula is satisfiable (with a satisfying assignment) or it is
/// unsatisfiable.
///
/// # Examples
///
/// ```
/// use putnam::solver::dpll::{SolveResult, solve};
/// use putnam::types::{Lit, Val};
///
/// let formula = vec![vec![Lit { var: 0, neg: false }]];
/// match solve(&formula, 1) {
///     SolveResult::Sat(model) => {
///         assert_eq!(model.value(0), Val::True);
///         println!("Satisfiable!");
///     }
///     SolveResult::Unsat => println!("Unsatisfiable"),
/// }
/// ```
#[derive(Debug, PartialEq)]
pub enum SolveResult {
    /// The formula is satisfiable with the given model
    Sat(Model),
    /// The formula is unsatisfiable
    Unsat,
}

/// Solves a SAT problem using the DPLL algorithm.
///
/// This is the main entry point for solving Boolean satisfiability problems.
/// It creates an initial model and invokes the DPLL search procedure.
///
/// # Arguments
///
/// * `formula` - The CNF formula to solve
/// * `num_vars` - The total number of variables in the problem
///
/// # Returns
///
/// * `SolveResult::Sat(model)` - If satisfiable, with a satisfying assignment
/// * `SolveResult::Unsat` - If unsatisfiable
///
/// # Examples
///
/// ```
/// use putnam::solver::dpll::{solve, SolveResult};
/// use putnam::types::{Lit, Val};
///
/// // Formula: (x₁ ∨ x₂) ∧ (¬x₁ ∨ x₃)
/// let formula = vec![
///     vec![Lit { var: 0, neg: false }, Lit { var: 1, neg: false }],
///     vec![Lit { var: 0, neg: true }, Lit { var: 2, neg: false }],
/// ];
///
/// match solve(&formula, 3) {
///     SolveResult::Sat(model) => {
///         // Check that the solution satisfies all clauses
///         println!("Found solution!");
///     }
///     SolveResult::Unsat => println!("No solution exists"),
/// }
/// ```
///
/// # Algorithm
///
/// The DPLL algorithm works by:
/// 1. **Unit Propagation**: Assign forced values from unit clauses
/// 2. **Satisfiability Check**: Test if all clauses are satisfied
/// 3. **Variable Selection**: Choose an unassigned variable for branching
/// 4. **Branching**: Try both True and False assignments recursively
/// 5. **Backtracking**: Undo assignments when contradictions are found
///
/// # Performance
///
/// - **Time Complexity**: O(2^n) in the worst case (exponential)
/// - **Space Complexity**: O(n) for the recursion stack
/// - **Practical Performance**: Often much better due to unit propagation and pruning
pub fn solve(formula: &Formula, num_vars: usize) -> SolveResult {
    let mut model = Model::new(num_vars);
    match dpll_search(formula, &mut model) {
        Ok(()) => SolveResult::Sat(model),
        Err(_) => SolveResult::Unsat,
    }
}

/// Core DPLL search procedure with systematic branching and backtracking.
///
/// This recursive function implements the heart of the DPLL algorithm,
/// performing the search for a satisfying assignment through the space
/// of possible variable assignments.
///
/// # Arguments
///
/// * `formula` - The CNF formula being solved
/// * `model` - Current partial assignment (modified during search)
///
/// # Returns
///
/// * `Ok(())` - A satisfying assignment was found (stored in model)
/// * `Err(())` - No satisfying assignment exists in this search branch
///
/// # Algorithm Steps
///
/// 1. **Unit Propagation**: Apply all forced assignments
/// 2. **Base Cases**: Check for satisfaction or contradiction
/// 3. **Variable Selection**: Choose next variable to branch on
/// 4. **Recursive Branching**: Try True assignment first, then False
/// 5. **Backtracking**: Restore state if both branches fail
fn dpll_search(formula: &Formula, model: &mut Model) -> Result<(), ()> {
    // Step 1: Unit propagation
    if unit_propagate(formula, model).is_err() {
        return Err(());
    }

    // Step 2: Check if all clauses are satisfied
    if is_satisfied(formula, model) {
        return Ok(());
    }

    // Step 3: Choose an unassigned variable
    let var = match choose_variable(formula, model) {
        Some(v) => v,
        None => return Err(()), // No unassigned variables but not satisfied = UNSAT
    };

    // Step 4: Try assigning True first
    let mut model_copy = model.clone();
    model_copy.assign(var, Val::True);
    if dpll_search(formula, &mut model_copy).is_ok() {
        *model = model_copy;
        return Ok(());
    }

    // Step 5: Try assigning False
    model.assign(var, Val::False);
    dpll_search(formula, model)
}

/// Checks if all clauses in the formula are satisfied by the current model.
///
/// A formula is satisfied if every clause contains at least one literal
/// that evaluates to true under the current variable assignment.
///
/// # Arguments
///
/// * `formula` - The CNF formula to check
/// * `model` - The current variable assignment
///
/// # Returns
///
/// `true` if all clauses are satisfied, `false` otherwise
///
/// # Examples
///
/// ```no_run
/// use putnam::types::{Model, Lit, Val};
/// # use putnam::solver::dpll::*;
///
/// let mut model = Model::new(2);
/// model.assign(0, Val::True);
/// 
/// let formula = vec![
///     vec![Lit { var: 0, neg: false }],  // x₁ (satisfied)
/// ];
///
/// // assert!(is_satisfied(&formula, &model));
/// ```
fn is_satisfied(formula: &Formula, model: &Model) -> bool {
    formula.iter().all(|clause| {
        clause.iter().any(|lit| model.is_true(*lit))
    })
}

/// Selects the next variable to branch on during DPLL search.
///
/// This function implements a simple variable selection heuristic:
/// it chooses the first unassigned variable that appears in an
/// unsatisfied clause.
///
/// # Arguments
///
/// * `formula` - The CNF formula being solved
/// * `model` - The current partial assignment
///
/// # Returns
///
/// * `Some(var)` - The variable to branch on next
/// * `None` - All variables are assigned (used to detect UNSAT when not satisfied)
///
/// # Heuristic Details
///
/// The current implementation uses a basic "first unassigned in unsatisfied clause"
/// heuristic. More sophisticated heuristics like VSIDS (Variable State Independent
/// Decaying Sum) or JW (Jeroslow-Wang) could improve performance significantly.
///
/// # Examples
///
/// ```no_run
/// use putnam::types::{Model, Lit};
/// # use putnam::solver::dpll::*;
///
/// let model = Model::new(3);
/// let formula = vec![
///     vec![Lit { var: 0, neg: false }, Lit { var: 1, neg: false }],
/// ];
///
/// // let var = choose_variable(&formula, &model);
/// // assert_eq!(var, Some(0));  // Would choose variable 0
/// ```
fn choose_variable(formula: &Formula, model: &Model) -> Option<Var> {
    // Simple heuristic: choose first unassigned variable that appears in unsatisfied clauses
    for clause in formula.iter() {
        if clause.iter().any(|lit| model.is_true(*lit)) {
            continue; // This clause is already satisfied
        }
        for lit in clause.iter() {
            if model.value(lit.var) == Val::Undef {
                return Some(lit.var);
            }
        }
    }
    None
}


#[cfg(test)]
mod tests {
    use super::*;

    fn lit(var: usize, neg: bool) -> Lit {
        Lit { var, neg }
    }

    #[test]
    fn test_simple_sat() {
        // Formula: (x0) ∧ (¬x1)
        let formula = vec![
            vec![lit(0, false)],
            vec![lit(1, true)],
        ];
        
        match solve(&formula, 2) {
            SolveResult::Sat(model) => {
                assert_eq!(model.value(0), Val::True);
                assert_eq!(model.value(1), Val::False);
            }
            SolveResult::Unsat => panic!("Expected SAT"),
        }
    }

    #[test]
    fn test_simple_unsat() {
        // Formula: (x0) ∧ (¬x0)
        let formula = vec![
            vec![lit(0, false)],
            vec![lit(0, true)],
        ];
        
        assert_eq!(solve(&formula, 1), SolveResult::Unsat);
    }

    #[test]
    fn test_three_variable_sat() {
        // Formula: (x0 ∨ x1) ∧ (¬x0 ∨ x2) ∧ (¬x1 ∨ ¬x2)
        let formula = vec![
            vec![lit(0, false), lit(1, false)],
            vec![lit(0, true), lit(2, false)],
            vec![lit(1, true), lit(2, true)],
        ];
        
        match solve(&formula, 3) {
            SolveResult::Sat(model) => {
                // Verify the solution satisfies all clauses
                for clause in &formula {
                    assert!(clause.iter().any(|lit| model.is_true(*lit)));
                }
            }
            SolveResult::Unsat => panic!("Expected SAT"),
        }
    }

    #[test]
    fn test_empty_formula() {
        // Empty formula is trivially satisfiable
        let formula = vec![];
        assert!(matches!(solve(&formula, 0), SolveResult::Sat(_)));
    }

    #[test]
    fn test_empty_clause() {
        // Formula with empty clause is unsatisfiable
        let formula = vec![vec![]];
        assert_eq!(solve(&formula, 0), SolveResult::Unsat);
    }
}