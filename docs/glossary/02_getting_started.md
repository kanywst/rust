# Rust 02: Getting Started / Development Environment and Cargo

- [Rust 02: Getting Started / Development Environment and Cargo](#rust-02-getting-started--development-environment-and-cargo)
  - [Overview](#overview)
  - [Key Terms](#key-terms)
  - [Concise Explanation of Terms](#concise-explanation-of-terms)
  - [Command Examples](#command-examples)
  - [Comparison and Supplement](#comparison-and-supplement)

## Overview

Rustプロジェクトは`cargo`を中心に構成されます。`Cargo.toml`で依存関係やメタデータを管理し、`cargo build`/`cargo run`/`cargo test`などでビルド・実行・テストを行います。

## Key Terms

- Cargo
- crate / package
- workspace
- target triple

## Concise Explanation of Terms

- **Cargo**: Rustのビルド・依存管理ツール。ビルド・テスト・公開を統合する。
- **crate / package**: crateはコンパイル単位、packageは1つ以上のcrateを含む配布単位。
- **workspace**: 複数のcrateをまとめて管理するための仕組み。
- **target triple**: ビルドターゲットを表す文字列（例: x86_64-apple-darwin）。

## Command Examples

```bash
cargo new my_project --bin
cd my_project
cargo build
cargo run
cargo test
```

## Comparison and Supplement

- npmやpipのパッケージ管理と似ていますが、ビルドシステムが統合されている点が特徴です。
