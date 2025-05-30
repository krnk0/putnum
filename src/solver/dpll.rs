use crate::types::*;
use super::unit::unit_propagate;

#[derive(Debug, PartialEq)]
pub enum SolveResult {
    Sat(Model),
    Unsat,
}

pub fn solve(formula: &Formula, num_vars: usize) -> SolveResult {
    let mut model = Model::new(num_vars);
    match dpll_search(formula, &mut model) {
        Ok(()) => SolveResult::Sat(model),
        Err(_) => SolveResult::Unsat,
    }
}

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

fn is_satisfied(formula: &Formula, model: &Model) -> bool {
    formula.iter().all(|clause| {
        clause.iter().any(|lit| model.is_true(*lit))
    })
}

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