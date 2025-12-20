# Rust 02: Getting Started (Development Environment & Cargo)

- [Rust 02: Getting Started (Development Environment \& Cargo)](#rust-02-getting-started-development-environment--cargo)
  - [Overview](#overview)
  - [Key Terms](#key-terms)
  - [Deep Dive: Cargo's Architecture](#deep-dive-cargos-architecture)
  - [Project Anatomy: The File Structure](#project-anatomy-the-file-structure)
  - [Command Execution Flow](#command-execution-flow)
  - [Comparison: Package Management Ecosystems](#comparison-package-management-ecosystems)

## Overview

Rustのビルドシステムとパッケージマネージャである `cargo` は、コンパイル、依存関係の解決、テスト、ドキュメント生成を一手に担います。これにより、開発者は複雑な `makefile` を書くことなく、再現性の高いビルド環境を手に入れることができます。

## Key Terms

- **Cargo**: Rustの万能ツール。プロジェクトのビルド、依存ライブラリの管理、配布を行う。
- **Crate**: コンパイルの最小単位。ライブラリ（`lib.rs`）またはバイナリ（`main.rs`）。
- **Package**: 1つ以上のCrateと、そのビルド方法を記した `Cargo.toml` を含む単位。
- **Workspace**: 複数のPackageを同一の `Cargo.lock` で管理し、ビルドキャッシュを共有する仕組み。
- **Target Triple**: `cpu-vendor-os-abi` 形式で表されるビルド標的（例: `aarch64-apple-darwin`）。

## Deep Dive: Cargo's Architecture

Cargoは、宣言的な設定ファイル（TOML）に基づいて動作します。

- **`Cargo.toml` (Manifest)**: 開発者が編集する。使用したいライブラリ（依存関係）やプロジェクトのメタデータを記述。
- **`Cargo.lock` (Lockfile)**: Cargoが自動生成・管理する。実際に使用されたライブラリの厳密なバージョンを記録し、チーム全員が「全く同じコード」でビルドすることを保証（再現性）。

## Project Anatomy: The File Structure

標準的なプロジェクト構成は、Rustコンパイラがファイルを探索するための規約（Convention）に基づいています。

```text
my_project/
├── Cargo.toml      # プロジェクトの設定・依存関係
├── Cargo.lock      # 固定された依存関係のバージョン（自動生成）
├── src/            # ソースコード
│   └── main.rs     # エントリポイント（バイナリの場合）
└── target/         # ビルド成果物（実行ファイルや中間生成物）

```

## Command Execution Flow

効率的な開発サイクル（Edit-Compile-Run）を支えるコマンド群です。

| コマンド | 内容 | 詳細 |
| --- | --- | --- |
| `cargo new` | プロジェクト作成 | `--bin`（実行ファイル）または `--lib`（ライブラリ） |
| `cargo check` | 高速構文チェック | **バイナリを生成せず**に型チェックのみ。開発中頻用する。 |
| `cargo build` | コンパイル | `target/debug/` にバイナリを出力。 |
| `cargo build --release` | 最適化ビルド | 実行速度を最大化するが、コンパイル時間は伸びる。 |
| `cargo run` | ビルド & 実行 | ソースの変更を検知し、必要なら再ビルドして実行。 |
| `cargo test` | テスト実行 | 単体テスト、統合テスト、ドキュメントテストを実行。 |

## Comparison: Package Management Ecosystems

Rustのツールチェーンは、他言語の「良いとこ取り」をしています。

| 機能 | Rust (Cargo) | C++ (CMake/Conan) | Node.js (npm) |
| --- | --- | --- | --- |
| **標準ビルドシステム** | 統合済み | 非標準 (CMake等) | npm scripts等 |
| **依存関係解決** | 強力 (Semantic Versioning) | 複雑 (OS依存強) | 強力 (node_modules) |
| **中央リポジトリ** | [crates.io](https://crates.io) | 不在 / 分散 | npm registry |
