# Rust: Comprehensive Glossary

- [Rust: Comprehensive Glossary](#rust-comprehensive-glossary)
  - [Basic Concepts](#basic-concepts)
  - [Types \& Containers](#types--containers)
  - [Ownership, Borrowing \& Lifetimes](#ownership-borrowing--lifetimes)
  - [Generics \& Traits](#generics--traits)
  - [Smart Pointers](#smart-pointers)
  - [Error Handling](#error-handling)
  - [Concurrency \& Async](#concurrency--async)
  - [Advanced Topics](#advanced-topics)
  - [Modules \& Toolchain](#modules--toolchain)

## Basic Concepts

- **Ownership (所有権)**
  - 値の寿命を管理する中核概念。1つの値に所有者は1人。スコープを抜けるとリソースを自動解放（RAII）。
  - **他言語との比較**: GC（Java等）は実行時にゴミを探すが、Rustはコンパイル時に解放タイミングを確定させるためオーバーヘッドがない。
- **Mutability (可変性)**
  - デフォルトは不変。`mut` キーワードで明示的に許可する。データ競合をコンパイル時に防ぐための必須条件。
- **Move (ムーブ)**
  - 代入時、スタック上のポインタ情報のみがコピーされ、元の変数は使用不能になる。二重解放（Double Free）を防ぐ。
- **Zero-cost Abstraction (ゼロコスト抽象化)**
  - 「使わないものにコストを払わない」「手書きの低レベルコードと同等以上の性能を出す」という設計思想。

## Types & Containers

- **Sized / DST (Dynamic Sized Types)**
  - コンパイル時にサイズが既知の型（`i32`等）と、未知の型（`str`, `[T]`等）。後者は直接扱えず、参照（`&str`）等で扱う必要がある。
- **Fat Pointer (太いポインタ)**
  - アドレスに加え、長さやメタデータ（vtable等）を保持するポインタ。スライスやトレイトオブジェクトの実体。
- **String / &str**
  - `String`: ヒープ所有。サイズ変更可能。
  - `&str`: 文字列スライス。メモリ上の連続したUTF-8バイト列への参照。

## Ownership, Borrowing & Lifetimes

- **Borrowing (借用)**
  - `&T`（不変）: 読込のみ。複数可。 / `&mut T`（可変）: 読書可能。唯一無二。
- **Interior Mutability (内部可変性)**
  - 外側が不変参照でも内部を書き換えられる仕組み（`RefCell`, `Mutex`）。借用規則を「コンパイル時」から「実行時」に遅延させる。
- **Lifetimes (ライフタイム)**
  - 参照が有効であるべき期間の注釈。ダングリング参照（解放後使用）を数学的に排除する。
- **Deref / Drop**
  - `Deref`: スマートポインタを参照のように扱う。
  - `Drop`: スコープ終了時のクリーンアップ処理（デストラクタ）。

## Generics & Traits

- **Trait (トレイト)**
  - 型の共通の振る舞いを定義するインターフェース。
- **Static Dispatch (静的ディスパッチ)**
  - ジェネリクスに対し、コンパイル時に具体的な型ごとのコードを生成（単態化）。実行時コストゼロ。
- **Dynamic Dispatch (動的ディスパッチ)**
  - `&dyn Trait` を用い、実行時にvtableを介してメソッドを呼び出す。異なる型を同じコレクションに格納できる。

## Smart Pointers

- **`Box<T>`**: ヒープ確保。サイズ不明な型を固定サイズにする。
- **`Rc<T>` / `Arc<T>`**: 参照カウント。`Arc`はAtomic操作によりスレッド安全。
- **`RefCell<T>`**: シングルスレッド用内部可変性コンテナ。

## Error Handling

- **Result<T, E>**: 成功か失敗かを型で表す。`?` 演算子による早期リターン（エラー伝播）が定石。
- **panic!**: 継続不可能な致命的エラー。スタックの巻き戻し（Unwinding）を発生させる。

## Concurrency & Async

- **Send / Sync**: スレッド間共有の安全性を保証するマーカートレイト。
- **Shared State**: `Arc<Mutex<T>>` 形式で、複数スレッドからの安全なアクセスを実現。
- **Async / Await / Future**:
  - `Future`: 遅延評価される非同期計算の単位。
  - `Waker / Poll`: 非同期タスクの再開と状態確認の低レベル機構。
- **Runtime / Executor**: `tokio` 等。`Future` をポーリングして実行を進めるエンジン。

## Advanced Topics

- **Pin**: 自己参照構造体など、メモリ上の位置を固定（移動禁止）する必要がある型を扱うためのラップ。
- **Unsafe**: コンパイラの安全保証を一部解除し、生ポインタ操作やFFIを行う。
- **FFI (Foreign Function Interface)**: C言語等の外部ライブラリとの連携。

## Modules & Toolchain

- **Crate / Cargo**: パッケージとビルド管理。
- **Clippy / Rustfmt**: 静的解析とコード整形。一貫したコード品質を保つ。
