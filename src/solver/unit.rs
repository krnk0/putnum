//! Unit propagation implementation
//!
//! Unit propagation is a key optimization in SAT solving that automatically
//! assigns values to variables when they appear in unit clauses (clauses with
//! only one unassigned literal).
//!
//! This process continues recursively until no more unit clauses exist or
//! a contradiction is found.

use crate::types::*;

/// Type alias for contradiction errors, containing the variable that caused the conflict
type Contradict = Var;

/// Performs unit propagation on the given formula and model.
///
/// Unit propagation is a form of constraint propagation that identifies unit clauses
/// (clauses with exactly one unassigned literal) and forces the assignment of those
/// literals to satisfy the clauses.
///
/// The algorithm works by:
/// 1. Finding all initial unit clauses
/// 2. Assigning the forced values to satisfy those clauses
/// 3. Checking if new unit clauses are created by these assignments
/// 4. Repeating until no more propagation is possible or a contradiction occurs
///
/// # Arguments
///
/// * `formula` - The CNF formula to propagate on
/// * `model` - The current variable assignments (will be modified)
///
/// # Returns
///
/// * `Ok(())` - Propagation completed successfully without conflicts
/// * `Err(var)` - A contradiction was found involving the specified variable
///
/// # Examples
///
/// ```no_run
/// use putnam::types::{Model, Lit, Formula};
/// use putnam::solver::unit::unit_propagate;
///
/// let mut model = Model::new(2);
/// let formula = vec![
///     vec![Lit { var: 0, neg: false }],  // Unit clause: x₁
/// ];
///
/// match unit_propagate(&formula, &mut model) {
///     Ok(()) => println!("Propagation successful"),
///     Err(var) => println!("Contradiction at variable {}", var),
/// }
/// ```
///
/// # Algorithm Details
///
/// The implementation uses a queue-based approach:
/// - Initial unit clauses are added to a processing queue
/// - Each literal is processed by assigning it and checking for new unit clauses
/// - The process continues until the queue is empty or a contradiction is found
///
/// # Time Complexity
///
/// O(L × P) where L is the number of literals in the formula and P is the number
/// of propagation steps.
pub fn unit_propagate(formula: &Formula, model: &mut Model) -> Result<(), Contradict> {
    use std::collections::VecDeque;
    let mut queue: VecDeque<Lit> = formula
        .iter()
        .filter(|c| c.len() == 1)
        .map(|c| c[0])
        .collect();
    while let Some(lit) = queue.pop_front() {
        match model.value(lit.var) {
            Val::True | Val::False if model.is_true(lit) => continue,
            Val::True | Val::False => return Err(lit.var),
            Val::Undef => {
                let val = if lit.neg { Val::False } else { Val::True };
                model.assign(lit.var, val);
            }
        }
        // Check for new unit clauses after this assignment
        for clause in formula.iter() {
            if clause.iter().any(|l| model.is_true(*l)) {
                continue; // Clause is satisfied
            }
            let unassigned: Vec<Lit> = clause.iter()
                .filter(|l| model.value(l.var) == Val::Undef)
                .copied()
                .collect();
            
            if unassigned.is_empty() {
                return Err(lit.var); // Empty clause = contradiction
            }
            if unassigned.len() == 1 {
                queue.push_back(unassigned[0]); // New unit clause
            }
        }
    }
    Ok(())
}
