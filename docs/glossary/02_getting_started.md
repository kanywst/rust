# Rust 02: Getting Started / Development Environment and Cargo

- [Rust 02: Getting Started / Development Environment and Cargo](#rust-02-getting-started--development-environment-and-cargo)
  - [Overview](#overview)
  - [Key Terms](#key-terms)
  - [Command Examples](#command-examples)
  - [Comparison and Supplement](#comparison-and-supplement)

## Overview

Rustプロジェクトは`cargo`を中心に構成されます。`Cargo.toml`で依存関係やメタデータを管理し、`cargo build`/`cargo run`/`cargo test`などでビルド・実行・テストを行います。

## Key Terms

- Cargo
- crate / package
- workspace
- target triple

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
