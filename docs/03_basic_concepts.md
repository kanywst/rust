# Rust 03: Basic Concepts (Variables, Functions, Control Flow)

- [Rust 03: Basic Concepts (Variables, Functions, Control Flow)](#rust-03-basic-concepts-variables-functions-control-flow)
  - [Key Terms](#key-terms)
  - [Concise Explanation: Immutability and State](#concise-explanation-immutability-and-state)
  - [Deep Dive: Shadowing vs Mutability](#deep-dive-shadowing-vs-mutability)
  - [Functions and Expressions](#functions-and-expressions)
  - [Control Flow: Functional Patterns](#control-flow-functional-patterns)
  - [Code Example](#code-example)
  - [Technical Points: Why Default Immutable?](#technical-points-why-default-immutable)

## Key Terms

- **Immutability (不変性)**: `let` で宣言された変数はデフォルトで変更不可。
- **Mutability (`mut`)**: 変数名の前に `mut` を付与し、メモリ上の値を変更可能にする。
- **Shadowing (シャドーイング)**: 同じ名前で新しい変数を宣言し、古い変数を隠す。
- **Expression (式) vs Statement (文)**: 値を返すか、操作を行うかの区別。
- **Control Flow**: `if`、`loop`、`while`、`for` による実行パスの制御。

## Concise Explanation: Immutability and State

Rustは「状態の変化（State Mutation）」を慎重に扱います。

- **不変変数の束縛**: 値を名前に関連付ける（Binding）。一度束縛すると、そのメモリアドレスの内容が書き換わらないことをコンパイラが保証します。
- **可変変数の宣言**: `mut` を使うことで、同一のメモリアドレスに対して新しい値を上書き（Overwrite）することを許可します。

## Deep Dive: Shadowing vs Mutability

この2つは似ていますが、メモリと型システムにおける役割が異なります。

| 特徴 | Mutability (`mut`) | Shadowing (`let`) |
| --- | --- | --- |
| **メモリ挙動** | 同一アドレスの値を書き換える | 新しい変数を確保し、以前を隠す |
| **型変更** | **不可能** (型は固定) | **可能** (別の型で再宣言OK) |
| **安全性** | 意図しない上書きに注意 | 古い状態を誤って使うのを防ぐ |

## Functions and Expressions

Rustは「式ベース（Expression-based）」の言語です。

- **文 (Statement)**: セミコロン（`;`）で終わり、値を返さない。
- **式 (Expression)**: 値を計算し、評価結果を返す。関数の末尾にセミコロンを置かないことで、それが戻り値（Implicit Return）になります。

## Control Flow: Functional Patterns

Rustの制御構造の多くは **「式」** です。

- **if 式**: 条件分岐の結果をそのまま変数に代入可能。
- **loop / for / while**: ループ。特に `for` はイテレータと組み合わせて、範囲外アクセスのない安全な反復処理を実現します。

## Code Example

```rust
fn main() {
    // 不変と可変
    let x = 5;
    let mut y = 10;
    y += x; 

    // シャドーイング (型を string から usize へ変更)
    let spaces = "   ";
    let spaces = spaces.len(); // shadowing

    // if式の結果を変数に束縛
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("Result: {}", square(number));
}

// 戻り値は矢印 -> で指定
fn square(n: i32) -> i32 {
    n * n // セミコロンなし = 式として評価され、戻り値になる
}

```

## Technical Points: Why Default Immutable?

1. **スレッド安全性の担保**: 共有データが不変であれば、ロック（Mutex等）なしで安全に複数スレッドから読み込めます。
2. **コンパイラ最適化**: 値が変わらないことが確実であれば、コンパイラはよりアグレッシブな定数畳み込みや最適化を行えます。
3. **予測可能性**: 関数に不変参照を渡した場合、その関数が中身を勝手に書き換える心配がない（副作用の排除）。
