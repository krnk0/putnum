use crate::types::*;

fn unit_propagate(formula: &Formula, model: &mut Model) -> Result<(), Contradict> {
    use std::collections::VecDeque;
    let mut queue: VecDeque<Lit> = formula
        .iter()
        .filter(|c|c.len==1)
        .map(|c|c[0])
        .collect();
    while let Some(lit) = queue.pop_front() {
        match model.value(lit.var) {
            (Val::True | Val::False) if model.is_true(lit) => continue,
            Val::True | Val::False => return Err(lit.var),
            Val::Undef => {
                let val = if lit.neg { Val::False } else { Val::True };
                model.assign(lit.var, val);
            }
        }
        // 代入の影響を式全体に伝播
        for clause in formula.iter_mut() {
            if clause.iter().any(|l| model.is_true(*l)) {
                continue;
            }
        }
        // 偽になったリテラル (¬x when x=True など) を除去
        clause.retain(|l| model.value(l.var) == Val::Undef);

        // 1) 空節 → 矛盾
        if clause.is_empty() {
            return Err(lit.var);
        }
        // 2) 新しく単位節になったらキューに追加
        if clause.len() == 1 {
            queue.push_back(clause[0]);
        }
    }
    Ok(())
}
