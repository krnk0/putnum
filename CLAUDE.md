# Putnam SAT Solver - Claude向けドキュメント

## プロジェクト概要

Putnamは教育・研究目的で作られた最小限のRust製SATソルバーです。DPLL（Davis-Putnam-Logemann-Loveland）アルゴリズムを中心とした、理解しやすい実装を目指しています。

### 設計哲学
- **最小主義**: 複雑な最適化より理解しやすさを重視
- **モジュラー設計**: 各層が明確に分離された4層アーキテクチャ
- **拡張性**: 将来のCDCL実装を考慮した設計
- **型安全性**: Rustの型システムを活用したバグ防止

## アーキテクチャ（4層構造）

```text
CLI層 (bin/putnam.rs)          ← ユーザーインターフェース
    ↓
アルゴリズム層 (solver/)        ← DPLL + 単位伝播
    ↓  
パース層 (parser.rs)           ← DIMACS ↔ 内部表現変換
    ↓
データ型層 (types.rs)          ← 基本データ構造
```

### 重要なファイル
- `types.rs`: Var, Lit, Clause, Formula, Model（50行程度）
- `solver/dpll.rs`: メインアルゴリズム（solve関数が中心）
- `solver/unit.rs`: 単位伝播（効率化の要、キューベース実装）
- `parser.rs`: DIMACS CNF形式の解析・変換

## 重要なコマンド

### 開発・テスト
```bash
cargo test                    # 全テスト実行（単体+統合）
cargo test test_simple_sat    # 特定テスト実行
cargo bench                   # 性能ベンチマーク実行
cargo doc --open             # API文書生成・表示
```

### CLI使用
```bash
cargo run --bin putnam examples/simple.cnf           # 基本実行
cargo run --bin putnam examples/simple.cnf -- --model # モデル表示
cargo run --bin putnam benchmarks/php-3-2.cnf       # 困難な問題
```

### ベンチマーク問題
- `examples/simple.cnf`: 基本的なSAT問題
- `examples/unsat.cnf`: 基本的なUNSAT問題  
- `benchmarks/php-3-2.cnf`: 鳩の巣原理（UNSAT、指数時間）

## コード理解のキーポイント

### 1. データフロー
```text
DIMACS file → parse_and_convert → (Formula, num_vars) → solve → SolveResult
```

### 2. DPLLアルゴリズムの流れ（dpll.rs）
1. **単位伝播**: `unit_propagate()` で強制割り当て
2. **充足性チェック**: `is_satisfied()` で解判定
3. **変数選択**: `choose_variable()` で分岐変数決定
4. **再帰分岐**: True/False両方試行、バックトラック

### 3. 単位伝播の仕組み（unit.rs）
- キューベース実装で効率化
- 単位節（1リテラル）から強制割り当てを発見
- 新しい割り当てが新たな単位節を生成するまで継続
- 矛盾検出で早期終了

### 4. モデル管理（types.rs）
- `vals: Vec<Val>`: 各変数の現在値（True/False/Undef）
- `trail: Vec<Var>`: 割り当て履歴（バックトラック用）
- `is_true(lit)`: リテラル満足判定

## 性能特性・ベンチマーク

### 現在の結果（naive DPLL）
- `simple_3var_sat`: ~144ns（基本ケース）
- `pigeonhole 4→3`: ~723μs（中程度UNSAT）
- `pigeonhole 5→4`: ~253ms（困難UNSAT、最適化対象）

### ボトルネック
1. **変数選択**: 現在は「最初に見つかった変数」のみ
2. **伝播コスト**: O(節数)の線形スキャン
3. **学習なし**: 同じ矛盾を繰り返し発見

## ドキュメント構造

### 外部ドキュメント（docs/）
- `architecture.md`: システム全体設計、データフロー、設計原則
- `algorithms.md`: DPLL・単位伝播の詳細、計算量、実行トレース例
- `api-reference.md`: 全型・関数の完全リファレンス
- `examples.md`: 使用例、チュートリアル、トラブルシューティング

### Rustdoc（ソースコード内）
- `lib.rs`: クレート概要、クイックスタート例
- 各ファイル: 関数・型の詳細仕様、引数・戻り値・例
- `cargo doc --open`で統合HTML文書を生成

## 現在の実装状況

### 完了済み ✅
- [x] DIMACS CNF パーサー
- [x] 基本データ型（Var, Lit, Model等）
- [x] キューベース単位伝播
- [x] 完全DPLL探索アルゴリズム
- [x] CLI（`putnam <file.cnf> [--model]`）
- [x] Criterionベンチマークスイート
- [x] 包括的ドキュメント（docs/ + Rustdoc）

### 今後の拡張予定 🚧
- [ ] Watched Literals（O(1)伝播）
- [ ] VSIDS/JW変数選択ヒューリスティック
- [ ] CDCL（Conflict-Driven Clause Learning）
- [ ] 証明ログ・DRAT出力
- [ ] Python bindings（pyo3）
- [ ] WebAssembly demo

## トラブルシューティング

### よくある問題
1. **テスト失敗**: `cargo test`でRustdocテストも含むため、例の実行可能性をチェック
2. **スタックオーバーフロー**: 大きな問題では`export RUST_MIN_STACK=8388608`
3. **性能問題**: pigeonhole 6→5以上は指数爆発、小さな問題で検証推奨

### デバッグ手法
1. **単体テスト**: 各関数の動作を個別確認
2. **統合テスト**: parser.rsの`integration_parse_and_solve`テスト
3. **小さな例**: 手計算可能な2-3変数問題から開始
4. **ベンチマーク**: 性能変化の定量的測定

## 開発時の注意点

### コード修正時
- **テスト実行**: 変更後は必ず`cargo test`
- **ドキュメント更新**: 関数変更時はRustdocコメントも更新
- **性能測定**: アルゴリズム変更時は`cargo bench`で影響確認

### 新機能追加時
- **テスト追加**: 新機能には対応する単体テスト
- **ドキュメント**: 外部docs/とRustdoc両方を更新
- **ベンチマーク**: 性能に影響する変更はベンチマーク追加検討

---

このドキュメントは将来のClaude Codeセッションでの効率的な作業のために作成されました。プロジェクトの理解と継続的な開発にご活用ください。