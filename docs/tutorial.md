# Putnam SAT Solver - 初心者チュートリアル

## 基本的な使用方法

### 1. CNFファイルの解決

既存のサンプルファイルを使った基本的な使用例：

```bash
# シンプルなSAT問題
$ cargo run --bin putnam examples/simple.cnf
SAT

# モデルも表示
$ cargo run --bin putnam examples/simple.cnf -- --model
SAT
v 1 2 3 0
```

### 2. プログラムからの利用

```rust
use putnam::{solve, parser::parse_and_convert, solver::dpll::SolveResult};
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    // CNF文字列から解析
    let cnf_input = "p cnf 3 3\n1 2 0\n-1 3 0\n-2 -3 0\n";
    let (formula, num_vars) = parse_and_convert(cnf_input.as_bytes())?;
    
    // ソルバー実行
    match solve(&formula, num_vars) {
        SolveResult::Sat(model) => {
            println!("充足可能！");
            for var in 0..num_vars {
                println!("x{} = {:?}", var + 1, model.value(var));
            }
        }
        SolveResult::Unsat => {
            println!("充足不可能");
        }
    }
    Ok(())
}
```

## チュートリアル: 段階的な問題解決

### Step 1: 最小のSAT問題

**問題**: 単一変数の肯定 `(x₁)`

```text
p cnf 1 1
1 0
```

**解決過程**:
1. 単位節 `(x₁)` を発見
2. 単位伝播: `x₁ = True`
3. 全節が満足 → **SAT**

```bash
$ echo "p cnf 1 1\n1 0" | cargo run --bin putnam /dev/stdin -- --model
SAT  
v 1 0
```

### Step 2: 単純なUNSAT問題

**問題**: 矛盾する節 `(x₁) ∧ (¬x₁)`

```text
p cnf 1 2
1 0
-1 0
```

**解決過程**:
1. 単位節 `(x₁)` → `x₁ = True`
2. 単位節 `(¬x₁)` → `x₁ = False` (矛盾!)
3. **UNSAT**

### Step 3: 3変数のSAT問題

**問題**: `(x₁ ∨ x₂) ∧ (¬x₁ ∨ x₃) ∧ (¬x₂ ∨ ¬x₃)`

```text
p cnf 3 3
1 2 0
-1 3 0  
-2 -3 0
```

**手動解決**:
1. 変数選択: `x₁`
2. 分岐: `x₁ = True`
3. 単位伝播: `(¬x₁ ∨ x₃)` → `(x₃)` → `x₃ = True`
4. 単位伝播: `(¬x₂ ∨ ¬x₃)` → `(¬x₂)` → `x₂ = False`
5. 全節をチェック:
   - `(x₁ ∨ x₂)`: `True ∨ False = True` ✓
   - `(¬x₁ ∨ x₃)`: `False ∨ True = True` ✓  
   - `(¬x₂ ∨ ¬x₃)`: `True ∨ False = True` ✓
6. **SAT**: `{x₁=T, x₂=F, x₃=T}`

## 実際のベンチマーク問題

### 鳩の巣原理 (Pigeonhole Principle)

4羽の鳩を3つの巣に入れる問題（必ずUNSAT）：

```rust
// 4羽の鳩、3つの巣  
// 変数: p_ij = 鳩iが巣jにいる (i=0..3, j=0..2)

// 各鳩は少なくとも1つの巣にいる
(p₀₀ ∨ p₀₁ ∨ p₀₂)  // 鳩0
(p₁₀ ∨ p₁₁ ∨ p₁₂)  // 鳩1  
(p₂₀ ∨ p₂₁ ∨ p₂₂)  // 鳩2
(p₃₀ ∨ p₃₁ ∨ p₃₂)  // 鳩3

// 各巣に入るのは最大1羽まで
(¬p₀₀ ∨ ¬p₁₀) ∧ (¬p₀₀ ∨ ¬p₂₀) ∧ (¬p₀₀ ∨ ¬p₃₀) ∧ ... // 巣0
(¬p₀₁ ∨ ¬p₁₁) ∧ (¬p₀₁ ∨ ¬p₂₁) ∧ (¬p₀₁ ∨ ¬p₃₁) ∧ ... // 巣1  
(¬p₀₂ ∨ ¬p₁₂) ∧ (¬p₀₂ ∨ ¬p₂₂) ∧ (¬p₀₂ ∨ ¬p₃₂) ∧ ... // 巣2
```

**実行時間**: ベンチマークで約253ms（指数的困難性を示す）

## デバッグとトレース

### ログ出力の追加

デバッグ情報を得るため、一時的にprintlnを追加：

```rust
// dpll.rs の dpll_search 関数内
fn dpll_search(formula: &Formula, model: &mut Model) -> Result<(), ()> {
    println!("探索開始: {:?}", model.vals);
    
    if unit_propagate(formula, model).is_err() {
        println!("単位伝播で矛盾発生");
        return Err(());
    }
    
    if is_satisfied(formula, model) {
        println!("解発見: {:?}", model.vals);
        return Ok(());
    }
    
    let var = match choose_variable(formula, model) {
        Some(v) => {
            println!("変数{}を選択", v);
            v
        },
        None => return Err(()),
    };
    
    // ... 分岐処理
}
```

### テストの実行

```bash
# 単体テスト
$ cargo test

# 特定のテスト
$ cargo test test_simple_sat

# 統合テスト  
$ cargo test integration_parse_and_solve
```

## 性能測定

### ベンチマーク実行

```bash
# 全ベンチマーク
$ cargo bench

# 特定のベンチマーク
$ cargo bench pigeonhole

# 結果保存
$ cargo bench -- --save-baseline before
# (コード変更後)
$ cargo bench -- --baseline before
```

### カスタムベンチマーク

独自の問題でベンチマークを作成：

```rust
use criterion::{black_box, Criterion};
use putnam::solve;

fn bench_custom_problem(c: &mut Criterion) {
    let (formula, num_vars) = create_my_problem();
    
    c.bench_function("my_problem", |b| {
        b.iter(|| solve(black_box(&formula), black_box(num_vars)))
    });
}
```

## トラブルシューティング

### よくある問題

#### 1. パースエラー
```text
Error parsing DIMACS file: パース失敗
```
**原因**: DIMACS形式の誤り  
**解決**: ファイル形式をチェック（節の0終端、変数番号など）

#### 2. スタックオーバーフロー
```text
thread 'main' has overflowed its stack
```
**原因**: 深すぎる再帰（大きな問題）  
**解決**: スタックサイズ増加 `export RUST_MIN_STACK=8388608`

#### 3. 長時間実行
**原因**: 指数的に困難な問題  
**対策**: より小さな問題で試すか、タイムアウト設定

### デバッグ手法

1. **小さな例から始める**: 手計算可能な問題でテスト
2. **単位テストを活用**: 各関数の動作を個別確認  
3. **ステップ実行**: 探索過程をトレース
4. **既知の問題**: benchmarks/内のファイルで検証

---

詳細なAPI仕様は `cargo doc --open` を参照してください。  
システム設計については [`architecture.md`](architecture.md) を参照してください。