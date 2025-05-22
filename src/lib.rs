use std::io::{self, BufRead};

#[derive(Debug, Copy, Clone)]
struct Literal(i32);           // 正負込み
type Clause = Vec<Literal>;
type Formula = Vec<Clause>;

fn parse_dimacs<R: BufRead>(mut r: R) -> io::Result<Formula> {
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
                    .map(Literal)
                    .collect::<Vec<_>>();
                formula.push(lits);
            }
        }
    }
    Ok(formula)
}
#[cfg(test)]
mod tests {
    use super::*;                 // parse_dimacs, Literal, Formula が見えるように

    /// 文字列から直接パースするヘルパ
    fn parse_str(src: &str) -> Formula {
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
}

