use std::io::{self, BufRead};
use crate::types::{Lit, Formula};

#[derive(Debug, Copy, Clone)]
struct DimacsLiteral(i32);           // 正負込み
type DimacsClause = Vec<DimacsLiteral>;
type DimacsFormula = Vec<DimacsClause>;

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

