use crate::types::*;

type Contradict = Var;

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
