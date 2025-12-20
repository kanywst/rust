# Rust 03: Basic Concepts — Variables, Mutability, Functions, Control Structures

- [Rust 03: Basic Concepts — Variables, Mutability, Functions, Control Structures](#rust-03-basic-concepts--variables-mutability-functions-control-structures)
  - [Key Terms](#key-terms)
  - [Concise Explanation of Terms](#concise-explanation-of-terms)
  - [Code Example](#code-example)
  - [Technical Points](#technical-points)

## Key Terms

- 変数 (let)
- 可変性 (mut)
- データシャドーイング (shadowing)
- 関数 (fn)
- 制御構造 (if/loop/while/for)

## Concise Explanation of Terms

- **変数 (let)**: 値を束縛する基本構文。デフォルトで不変。
- **可変性 (mut)**: `mut`で変数を可変にする。明示的な可変は安全性向上に寄与する。
- **シャドーイング**: 同じ名前で再度`let`を使って値を置き換える機能。型の変更も可能。
- **関数 (fn)**: 振る舞いをまとめる基本単位。戻り値や引数の型注釈が重要。
- **制御構造**: 条件分岐やループでプログラムの流れを制御する仕組み。

## Code Example

```rust
let x = 5;
let mut y = 6;
let y = y + 1; // shadowing

fn square(n: i32) -> i32 {
    n * n
}
```

## Technical Points

- 不変をデフォルトにすることで、状態変更によるバグを減らす。
- 型の推論は強力だが、公開APIでは型注釈を行うことが一般的。
