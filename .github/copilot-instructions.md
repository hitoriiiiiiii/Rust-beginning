<!--
Guidance for AI coding assistants working on this repository.
Focus: be small, actionable, and tied to files in the repo.
-->

# Copilot instructions — Rust (learning/demo) repository

This is a small Rust learning/demo repository (root: `Cargo.toml`, source: `src/`). The code contains many single-file examples illustrating Rust concepts (ownership, borrowing, threads, structs, traits, generics, collections).

Keep suggestions minimal and idiomatic to Rust 2024 edition. Prefer fixes that compile (`cargo check`) and avoid large refactors unless requested.

## Quick facts
- Entry: examples are individual files in `src/` (e.g., `fibonnaci.rs`, `multithread.rs`, `struct.rs`). There is no `lib.rs` or workspace-level integration; treat each file as a standalone example unless the user indicates otherwise.
- Manifest: `Cargo.toml` uses edition `2024` and a single dependency: `chrono = "0.4.42"`.
- Build/test: use `cargo check` or `cargo build` from repository root. There are no tests present.

## What to do first
- Run `cargo check` to get the baseline compiler errors. Suggest edits that reduce errors and make examples runnable.
- When adding code, update `Cargo.toml` only if necessary (e.g., new dependency). Keep dependency versions minimal and consult existing version used for `chrono`.

## Common fixes and patterns in this repo
- Many example files attempt `fn main()` in each file. To compile the project as a package, combine examples under a single `main.rs` or convert examples to modules and call them from `src/main.rs`. Only change this when the user asks.
- Threading examples (e.g., `multithread.rs`, `main.rs`) contain incorrect APIs: replace `thread::handle` with `thread::spawn` and store the JoinHandle (e.g., `let handle = thread::spawn(|| { ... });`) then call `handle.join().unwrap();` after the main loop.
- Incorrect macros: `printIn!` in `struct.rs` is not a standard macro. Replace with `println!` or define the macro explicitly.
- Trait implementations sometimes use invalid syntax (e.g., `struct Fix; impl Fix for User {}`): do not invent traits — either remove or implement correctly (define a trait named `Fix` if intended).

## Repository-specific examples to reference
- Fix threading example: see `src/multithread.rs` and `src/main.rs` — use `std::thread::spawn` and `JoinHandle`.
- Fix print macro: in `src/struct.rs` replace `printIn!` with `println!` and borrow fields by reference if needed.
- Fibonacci example `src/fibonnaci.rs` is self-contained and close to correct; prefer keeping small algorithmic examples untouched.

## Conventions and constraints
- Preserve pedagogical intent: most files are small demonstrations. Avoid over-optimizing for production unless the user asks.
- Keep edits local to the example file being fixed. If you need to change multiple files (e.g., to integrate examples), explain why and get confirmation.
- Do not add tests or CI changes without user consent.

## Helpful file references
- `Cargo.toml` — project metadata and dependency `chrono`.
- `src/multithread.rs`, `src/main.rs` — threading examples (common error patterns).
- `src/struct.rs`, `src/traits.rs` — struct & trait examples and macro misuse.
- `src/fibonnaci.rs` — sample algorithm, used as a canonical working example.

## When to ask the user
- If changes span multiple examples or rework project structure (e.g., convert examples into modules), ask before proceeding.
- If adding a dependency other than `chrono`, check with the user.

If anything in these instructions is unclear or you want me to expand examples or auto-fix a few of the most common errors (threading + print macros), tell me which files to update and I'll apply fixes and run `cargo check`.
