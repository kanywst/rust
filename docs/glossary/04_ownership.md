# Rust 04: Ownership

- [Rust 04: Ownership](#rust-04-ownership)
  - [Overview](#overview)
  - [Key Terms](#key-terms)
  - [Concise Explanation of Terms](#concise-explanation-of-terms)
  - [Example](#example)
  - [Comparison with Other Languages](#comparison-with-other-languages)
  - [Supplementary Notes: Optimization and Ownership](#supplementary-notes-optimization-and-ownership)

## Overview

所有権はRustの中心概念で、値に対する唯一の所有者が存在します。スコープが終了すると所有する値は自動的に破棄されます。

## Key Terms

- Move
- Copy
- Drop

## Concise Explanation of Terms

- **Move**: 所有権が別の変数へ移る操作。ムーブ後の元所有者は使えなくなる。
- **Copy**: 値をビット単位でコピー可能な型の特性。`Copy`型はムーブでも元が使える。
- **Drop**: 値がスコープを抜けるときに呼ばれるクリーンアップ処理（デストラクタ）。

## Example

```rust
let s = String::from("hello");
let t = s; // sの所有権はtにムーブ
// println!("{}", s); // コンパイルエラー
```

## Comparison with Other Languages

C++ではコピーやムーブが明示的、Java/Pythonはガベージコレクションに依存。

## Supplementary Notes: Optimization and Ownership

コンパイラ最適化により、ムーブやコピーは省略されることがある（RVOや最適化）。
