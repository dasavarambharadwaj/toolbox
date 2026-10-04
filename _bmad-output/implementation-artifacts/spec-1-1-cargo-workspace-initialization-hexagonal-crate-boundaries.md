---
title: 'Story 1.1: Cargo Workspace Initialization & Hexagonal Crate Boundaries'
type: 'feature'
created: '2026-10-04'
status: 'done'
baseline_commit: 'ecebfe717c55409e70d8e26f907ca5aa5a5b0092'
route: 'dispatch'
review_loop_iteration: 0
context:
  - '_bmad-output/implementation-artifacts/epic-1-context.md'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** The Toolbox codebase currently lacks a Cargo workspace manifest and crate structure. Development of domain logic, CLI adapters, and GUI code cannot begin without establishing the foundational 6-crate hexagonal boundaries.

**Approach:** Initialize the root `Cargo.toml` workspace and scaffold the 6 crates (`tb`, `tb-core`, `tb-deps`, `tb-ext`, `tb-cli`, `tb-ui`) with strict dependency boundary isolation, release compilation profiles, and boundary verification.

## Boundaries & Constraints

**Always:**
- Root `Cargo.toml` must declare workspace members: `crates/tb`, `crates/tb-core`, `crates/tb-deps`, `crates/tb-ext`, `crates/tb-cli`, `crates/tb-ui`.
- Root `Cargo.toml` must declare `[workspace.package]` with `edition = "2024"`, `version = "0.1.0"` inherited by all crates via `workspace = true`.
- Shared dependencies (`thiserror`, `anyhow`, `clap`, `slint`, `serde`, `serde_json`) must be centrally pinned in `[workspace.dependencies]` and consumed via `{ workspace = true }`.
- Error framework partitioning strictly enforced:
  - Pure domain and library crates (`tb-core`, `tb-deps`, `tb-ext`) must use `thiserror` for typed errors.
  - Presentation adapters and root binary (`tb-cli`, `tb-ui`, `tb`) must use `anyhow` for application context.
- Release profile in root `Cargo.toml` must specify `opt-level = "z"`, `lto = "fat"`, `codegen-units = 1`, `panic = "abort"`, and `strip = true`.
- Acyclic Dependency DAG strictly enforced:
  - `tb-core`: Pure leaf crate (no workspace dependencies).
  - `tb-deps`, `tb-ext`: Depend on `tb-core`.
  - `tb-cli`, `tb-ui`: Depend on `tb-core` and domain crates.
  - `tb`: Root binary crate depending on `tb-cli` and `tb-ui`.
- `crates/tb` must feature-gate GUI support (`default = ["gui"]`, `gui = ["tb-ui"]`) to guarantee headless compilation resilience.

**Never:**
- `crates/tb-core` must NEVER depend on `slint` or `clap`, directly or transitively.
- No circular dependencies between workspace crates (`tb-core` must never depend on `tb-deps` or `tb-ext`).

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|---|---|---|---|
| Workspace compilation | `cargo check --workspace` | Exits 0 with 0 errors or warnings across all 6 crates | Fail build on warning |
| Workspace test harness | `cargo test --workspace` | Exits 0 with 0 errors or warnings across all 6 crates | Fail build on test error |
| Headless compilation | `cargo check --workspace --no-default-features` | Exits 0 with 0 errors or warnings | Fail build |
| Domain isolation audit | `cargo tree -p tb-core` | Zero references to `slint` or `clap` across entire transitive dependency graph | Static boundary check fails |
| Release profile build | `cargo build --profile release -p tb` | Compiles with opt-level = "z", LTO fat, stripped symbols | Fail build |

</frozen-after-approval>

## Code Map

- `Cargo.toml` -- Root workspace manifest with member crates, `[workspace.package]`, `[workspace.dependencies]`, and release profile.
- `crates/tb/Cargo.toml` -- Root multi-call binary crate depending on `tb-cli`, optional `tb-ui` (`gui` feature), and `anyhow`.
- `crates/tb/src/main.rs` -- Entry point stub for multi-call binary wiring into `tb_cli::run()` and `tb_ui::run()`.
- `crates/tb-core/Cargo.toml` -- Pure domain engine crate depending on `thiserror` with zero presentation dependencies.
- `crates/tb-core/src/lib.rs` -- Pure domain engine library root exporting `pub enum TbError`.
- `crates/tb-deps/Cargo.toml` -- Managed dependency resolver crate depending on `tb-core` and `thiserror`.
- `crates/tb-deps/src/lib.rs` -- Dependency manager stub.
- `crates/tb-ext/Cargo.toml` -- Extension host and IPC crate depending on `tb-core` and `thiserror`.
- `crates/tb-ext/src/lib.rs` -- Extension host stub.
- `crates/tb-cli/Cargo.toml` -- Terminal interface crate depending on `clap`, `tb-core`, and `anyhow`.
- `crates/tb-cli/src/lib.rs` -- CLI adapter library root exporting `pub fn run() -> anyhow::Result<()>`.
- `crates/tb-ui/Cargo.toml` -- Desktop GUI crate depending on `slint`, `tb-core`, and `anyhow`.
- `crates/tb-ui/src/lib.rs` -- Slint GUI adapter library root exporting `pub fn run() -> anyhow::Result<()>`.

## Tasks & Acceptance

**Execution:**
- [x] `Cargo.toml` -- Create root workspace manifest with 6 crate members, shared `[workspace.package]` (`edition = "2024"`), pinned `[workspace.dependencies]`, and optimized release profile.
- [x] `crates/tb-core/Cargo.toml` & `crates/tb-core/src/lib.rs` -- Scaffold pure domain library with `thiserror` (exporting `TbError`), verifying zero dependencies on `slint` or `clap`.
- [x] `crates/tb-deps/Cargo.toml` & `crates/tb-deps/src/lib.rs` -- Scaffold dependency management crate with `thiserror` depending on `tb-core`.
- [x] `crates/tb-ext/Cargo.toml` & `crates/tb-ext/src/lib.rs` -- Scaffold extension runner crate with `thiserror` depending on `tb-core`.
- [x] `crates/tb-cli/Cargo.toml` & `crates/tb-cli/src/lib.rs` -- Scaffold CLI adapter crate with `anyhow` and `clap`, exporting `pub fn run() -> anyhow::Result<()>`.
- [x] `crates/tb-ui/Cargo.toml` & `crates/tb-ui/src/lib.rs` -- Scaffold Slint GUI crate with `anyhow` and `slint`, exporting `pub fn run() -> anyhow::Result<()>`.
- [x] `crates/tb/Cargo.toml` & `crates/tb/src/main.rs` -- Scaffold multi-call root binary with `anyhow`, linking `tb-cli` and `tb-ui` (gated by `gui` feature).

**Acceptance Criteria:**
- Given a clean workspace, when `cargo check --workspace` is executed, then all 6 crates compile with zero errors and zero warnings.
- Given a clean workspace, when `cargo test --workspace` is executed, then all test suites across all 6 crates compile and pass with zero errors.
- Given a headless environment, when `cargo check --workspace --no-default-features` is executed, then workspace compiles without GUI libraries.
- Given `crates/tb-core`, when running `cargo tree -p tb-core`, then `slint` and `clap` are completely absent from the entire dependency graph.
- Given the root `Cargo.toml`, when inspecting `[profile.release]`, then `opt-level = "z"`, `lto = "fat"`, `codegen-units = 1`, `panic = "abort"`, and `strip = true` are defined.

## Implementation Notes

## Spec Change Log
- 2026-10-04: Hardened workspace against cyclic dependencies, headless build failure, and version drift via Pre-mortem Analysis.
- 2026-10-04: Aligned with Rust 2024 edition, partitioned thiserror/anyhow contracts, and added test harness verification via Self-Consistency Validation.

## Review Triage Log
- crates/tb-deps/src/lib.rs:24 -- medium (patch) -- clippy::bool_assert_comparison triggered by assert_eq!(result.unwrap(), false). Fix with assert!(!result.unwrap()).
- crates/tb-cli/src/lib.rs:13 -- medium (patch) -- unwrap_or_default() suppresses clap parse errors, --help, and --version. Fix with CliArgs::try_parse()?.
- crates/tb/src/main.rs:1-15 -- false (reject) -- Multi-call symlink detection is explicitly scheduled for Story 1.2; Story 1.1 establishes crate boundaries.
- crates/tb-ui/Cargo.toml -- false (reject) -- slint-build and build.rs are deferred to Epic 2 / Story 1.4 when .slint files are created.
- crates/tb-cli/Cargo.toml -- false (reject) -- JSON envelopes and serde dependencies are scheduled for Story 1.3 / Story 4.1.
- crates/tb-core/src/lib.rs:15 -- low (reject) -- From<std::io::Error> not needed in Story 1.1; TbError is a baseline stub.
- crates/tb-cli/Cargo.toml -- false (reject) -- Unused dependencies on tb-deps/tb-ext should not be added prematurely.
- crates/tb-core/src/lib.rs -- false (reject) -- Preset and domain operation abstractions are scheduled for Story 2.1.
- crates/tb-deps/src/lib.rs -- false (reject) -- Tool probe and fallback directories are scheduled for Story 1.4.
- Cargo.toml:14 -- low (reject) -- Workspace package metadata is sufficient for Story 1.1 crate initialization.
- .gitignore:1 -- low (reject) -- .gitignore contains required target ignore.
- crates/tb/src/main.rs:18 -- medium (patch) -- Root binary entrypoint mode dispatch logic was untested. Extract determine_mode() and unit test it.
- crates/tb/Cargo.toml -- medium (patch) -- Verification routine lacked negative dependency tree assertion for tb --no-default-features.
- crates/tb/src/main.rs:4 -- medium (patch) -- std::env::args() panics on invalid UTF-8 args. Use std::env::args_os().
- crates/tb/src/main.rs:6 -- low (patch) -- Empty string for DISPLAY or WAYLAND_DISPLAY should not trigger GUI mode. Guard with !v.is_empty().

## Verification
**Commands:**
- `export PATH="$HOME/.cargo/bin:$PATH" && cargo check --workspace` -- expected: exits 0 with no warnings or errors.
- `export PATH="$HOME/.cargo/bin:$PATH" && cargo test --workspace` -- expected: exits 0 with no test failures.
- `export PATH="$HOME/.cargo/bin:$PATH" && cargo check --workspace --no-default-features` -- expected: exits 0.
- `! cargo tree -p tb-core 2>/dev/null | grep -E "slint|clap"` -- expected: exits 0 (no occurrences).
- `! cargo tree -p tb --no-default-features 2>/dev/null | grep -E "slint|tb-ui"` -- expected: exits 0 (no occurrences).
- `export PATH="$HOME/.cargo/bin:$PATH" && cargo clippy --workspace --all-targets -- -D warnings` -- expected: exits 0.

