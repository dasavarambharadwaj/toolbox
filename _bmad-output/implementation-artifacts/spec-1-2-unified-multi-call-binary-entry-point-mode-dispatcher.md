---
title: 'Story 1.2: Unified Multi-Call Binary Entry Point & Mode Dispatcher'
type: 'feature'
created: '2026-10-04'
status: 'done'
baseline_commit: 'a587322e2959d8e05e6044e0233d7e621bb4f10e'
route: 'dispatch'
review_loop_iteration: 0
context:
  - '_bmad-output/implementation-artifacts/epic-1-context.md'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Users need a single `tb` executable that seamlessly runs CLI subcommands in the terminal and launches the Slint graphical interface when opened from a desktop environment launcher, without displaying unnecessary GUI frames or failing silently in headless environments.

**Approach:** Implement full multi-call dispatch in `crates/tb/src/main.rs` and `crates/tb-cli`: detect symlink invocations (e.g. `tb-image`), detect interactive display sessions (`$WAYLAND_DISPLAY` or `$DISPLAY`), handle zero arguments in headless environments by printing CLI help to stderr and exiting with code 2, and ensure CLI invocations bypass GUI initialization in <50ms. Strictly adhere to Test-Driven Development (TDD) by authoring failing unit and integration tests covering all edge cases prior to implementation.

## Boundaries & Constraints

**Always:**
- Strict adherence to Rule 6 (TDD Mandatory Protocol): Red -> Green -> Refactor cycle. All tests (unit + E2E integration) MUST be written first and observed failing or failing to compile before functional implementation.
- Single unified binary `tb` compiled by `crates/tb`.
- When invoked with zero arguments and no display server (`WAYLAND_DISPLAY` and `DISPLAY` both unset or empty): print formatted CLI help to stderr and exit with code 2.
- When invoked with zero arguments (or subcommand `gui`) in an active display server session: dispatch to `tb-ui::run()`.
- When invoked with arguments/subcommands (e.g. `tb --help`, `tb --version`, `tb <subcommand>`): dispatch directly to `tb-cli::run_with_args()` without calling or initializing `tb-ui`.
- Multi-call executable symlink name detection: if the binary executable name starts with `tb-` (e.g. `tb-image` or `/usr/local/bin/tb-compress`), strip the `tb-` prefix and inject it as the primary subcommand argument dispatched to `tb-cli`.
- Retain `--no-default-features` compatibility (headless compilation excluding `tb-ui` and Slint).

**Never:**
- Never write production code before failing automated tests exist.
- Never initialize Slint GUI event loops or load window backends during CLI command invocations.
- Never exit 0 when zero arguments are passed in a headless terminal without a display server.
- Never add UI or Clap presentation dependencies to `tb-core`.

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| GUI Launcher | 0 args, `$WAYLAND_DISPLAY` or `$DISPLAY` set | Dispatches to `tb-ui::run()`, exits 0 | Exits on GUI window close |
| Headless Invocation | 0 args, no display server | Formatted CLI help printed to stderr, exit code 2 | Exits code 2 |
| Whitespace/Empty Display Vars | 0 args, `DISPLAY=""`, `WAYLAND_DISPLAY=""` | Treated as headless: CLI help to stderr, exit code 2 | Exits code 2 |
| Explicit Help / Version | `tb --help` or `tb --version` | Dispatches to `tb-cli`, stdout prints help/version, exit code 0 | Non-zero on invalid flag |
| Explicit Subcommand | `tb <subcommand> [args]` | Dispatches to `tb-cli` without GUI initialization | Delegated to `tb-cli` |
| Explicit GUI Request with Display | `tb gui` with active display | Dispatches to `tb-ui::run()`, exit code 0 | Exits on window close |
| Explicit GUI Request Headless | `tb gui` without display server | Stderr outputs "No graphical display server detected", exit code 2 | Exits code 2 |
| Symlink Multi-call (Relative Path) | Symlink `tb-image` invoked with `args` | Dispatches to `tb-cli` as `tb image [args]` | Error to stderr on unrecognized command |
| Symlink Multi-call (Absolute Path) | Symlink `/usr/bin/tb-compress` invoked with `args` | Dispatches to `tb-cli` as `tb compress [args]` | Error to stderr on unrecognized command |
| Symlink `tb-gui` | Symlink `tb-gui` invoked | Dispatches to GUI if display active; errors with exit 2 if headless | Error to stderr if no display server |
| Symlink Multi-call Headless 0 args | Symlink `tb-image` invoked with 0 args | Dispatches to `tb-cli` as `tb image`, prints category help | Error code 0 or 2 based on clap subcommand requirement |
| Custom Binary Name (No `tb-` prefix) | Binary invoked as `my-tool` | Preserves args as standard CLI dispatch | Standard CLI error handling |

</frozen-after-approval>

## Code Map

- `crates/tb/src/main.rs` -- Multi-call entry point, dispatch logic (`determine_dispatch_target`), symlink resolution, display check, and headless exit code 2 handling.
- `crates/tb/Cargo.toml` -- Root binary crate dependencies (`tb-cli`, optional `tb-ui`, `anyhow`).
- `crates/tb-cli/src/lib.rs` -- CLI parser and help generation, exposing `run_with_args`, `print_help_to_stderr`, and `gui` subcommand handling.
- `tests/e2e_binary.rs` -- End-to-end integration tests verifying comprehensive edge cases: zero-arg headless exit 2, CLI arg dispatch, symlink dispatch, display variable checks, `tb gui` in headless, and GUI feature gating.

## Tasks & Acceptance

**Execution:**

- [x] `tests/e2e_binary.rs` & `crates/tb/src/main.rs` (TDD Step 1: RED) -- Write comprehensive failing unit and integration tests covering:
  - Unit tests in `crates/tb/src/main.rs` for `determine_dispatch_target`:
    - Zero args with display present (`$WAYLAND_DISPLAY` or `$DISPLAY`) -> `DispatchTarget::Gui`
    - Zero args with empty/whitespace display vars -> `DispatchTarget::HeadlessHelp`
    - Zero args with no display vars -> `DispatchTarget::HeadlessHelp`
    - Explicit `tb gui` with display present -> `DispatchTarget::Gui`
    - Explicit `tb gui` without display present -> `DispatchTarget::Error(...)` with exit code 2
    - Symlink detection: path ending in `tb-image` maps to `DispatchTarget::Cli(["image"])`
    - Symlink detection: absolute path `/usr/bin/tb-pdf` with args `["merge", "a.pdf"]` maps to `DispatchTarget::Cli(["pdf", "merge", "a.pdf"])`
    - Symlink detection: `tb-gui` maps to GUI dispatch
    - Standard args: `tb --help`, `tb --version`, `tb dev json` map directly to CLI dispatch
  - E2E process tests in `tests/e2e_binary.rs`:
    - Process execution with 0 args without display exits 2 with help on stderr
    - Process execution with `DISPLAY=""` and `WAYLAND_DISPLAY=""` exits 2 with help on stderr
    - Process execution with `tb gui` without display exits 2 with error on stderr
    - Process execution with `tb --help` exits 0 with help on stdout
    - Process execution via created temporary symlink `tb-testcmd` invokes CLI dispatch properly
    - Verification that CLI execution runs in <50ms without initializing GUI
  - Run `cargo test --workspace` to confirm tests fail / fail to compile (RED phase).

- [x] `crates/tb-cli/src/lib.rs` (TDD Step 2: GREEN - CLI Support) -- Add support for printing formatted help to `stderr` and supporting the `gui` subcommand or helper.
- [x] `crates/tb/src/main.rs` (TDD Step 2: GREEN - Dispatch Implementation) -- Implement `determine_dispatch_target`, symlink detection, display inspection, and process exit handling to satisfy all tests.
- [x] Verification & Refactoring (TDD Step 3: REFACTOR) -- Run full verification suite (`cargo test`, `cargo check --no-default-features`, `cargo clippy`), ensure zero regressions and zero warnings.

### Review Findings

- [x] [Review][Patch] Fix `tb --version` failure by adding `version` to Clap command derive and adding test [crates/tb-cli/src/lib.rs:5]
- [x] [Review][Patch] Add E2E process integration test for active graphical display session dispatch [crates/tb/tests/e2e_binary.rs:337]
- [x] [Review][Patch] Remove unreachable dead code in headless match arm [crates/tb/src/main.rs:340]
- [x] [Review][Defer] Duplicate root integration tests in `tests/e2e_binary.rs` and `tests/e2e_boundaries.rs` [tests/e2e_binary.rs:1] — deferred: pre-existing workspace scaffolding design where root tests exist alongside crate integration tests
- [x] [Review][Defer] CLI subcommands not yet modeled in `CliArgs` for symlink help parsing [crates/tb-cli/src/lib.rs:5] — deferred: category subcommands are scheduled for Epic 2 onwards

#### Rejected
- `has_active_display` non-UTF-8: Linux display sockets are ASCII/UTF-8; handling unusual non-UTF-8 display names adds unnecessary complexity.
- `exec_stem` non-UTF-8: POSIX binary names conform to standard strings; negligible risk in everyday use.
- Multi-call `tb-` binary name: edge case with zero real-world occurrence; rejecting empty subcmd is correct behavior.
- `full_args` argv[0] symlink name: standard UNIX convention to pass the invoked name as argv[0]; Clap handles binary aliases naturally.
- Symlink test non-Unix platforms: Toolbox is explicitly a Linux-first offline utility suite (Rule 1).
- JSON error formatting in root `main()`: Story 1.3 explicitly handles standardized CLI JSON envelopes and error taxonomy.


**Acceptance Criteria:**
- All tests are written and fail prior to implementing functional dispatch code (Strict TDD adherence).
- Given `tb` invoked with zero arguments without `$WAYLAND_DISPLAY` or `$DISPLAY` (or with empty values), when binary executes, then stderr prints CLI help and process exits with exit code 2.
- Given `tb` invoked with arguments in terminal (e.g. `tb --help`, `tb --version`), when binary executes, then it dispatches to `tb-cli` in <50ms without initializing Slint GUI libraries.
- Given `tb` invoked with zero arguments with `$WAYLAND_DISPLAY` or `$DISPLAY` set, when binary executes with `gui` feature enabled, then it launches `tb-ui::run()`.
- Given `tb gui` invoked in headless mode, process prints error to stderr and exits with exit code 2.
- Given a symlink named `tb-<subcmd>`, when invoked with arguments, it dispatches to `tb-cli` injecting `<subcmd>` as the first argument.
- Given `cargo check --workspace --no-default-features`, when executed, then headless build compiles cleanly.

## Implementation Notes
- Adhered strictly to TDD Red-Green-Refactor cycle: authored failing unit tests and E2E binary tests first, observed failure in cargo test, then implemented multi-call symlink detection and headless handling in crates/tb/src/main.rs and crates/tb-cli/src/lib.rs.
- Resolved clippy and edge-case review findings by tightening symlink assertion in test_e2e_tb_symlink_dispatch, adding drop guard for test symlinks, asserting on tb-gui under #[cfg(not(feature = "gui"))], and testing symlink 0-arg dispatch in active display sessions.

## Spec Change Log

## Review Triage Log
- crates/tb/tests/e2e_binary.rs:137 -- medium (patch) -- Weak assertion in test_e2e_tb_symlink_dispatch (combined.contains("Usage") || output.status.code().is_some() evaluated to true unconditionally). Patched with strict assert!(combined.contains("image-test")).
- crates/tb/tests/e2e_binary.rs:120 -- low (patch) -- Temporary symlink leaked in /tmp if test panicked. Patched with TempSymlink drop guard.
- crates/tb/src/main.rs:262 -- medium (patch) -- target_display unused variable error under --no-default-features and missing headless assertion. Patched with #[cfg(not(feature = "gui"))] assertion.
- crates/tb/src/main.rs:50 -- medium (patch) -- Missing unit test for tb-<subcmd> symlink with 0 args in an active display session. Added test_symlink_with_zero_args_active_display.
- crates/tb/tests/e2e_binary.rs:145 -- low (reject) -- Latency threshold (100ms in debug) is generous enough for local test runs and passes consistently.
- crates/tb/src/main.rs:35 -- low (reject) -- UTF-8 conversion for executable name is standard; binary names in POSIX environments conform to ASCII/UTF-8.
- tests/e2e_binary.rs -- low (reject) -- Root tests/ directory mirrors workspace crate integration tests for cargo test at workspace root.

## Design Notes

### Dispatch Target Logic
```rust
#[derive(Debug, PartialEq, Eq)]
pub enum DispatchTarget {
    Gui,
    Cli(Vec<std::ffi::OsString>),
    HeadlessHelp,
    Error(String, i32),
}
```
1. Detect executable stem from invocation path (`std::env::args_os().next()`).
   - If executable name matches `tb-<subcmd>`:
     - If `<subcmd>` is `"gui"`: check display session -> `Gui` or `Error("...", 2)`.
     - Otherwise: prepend `<subcmd>` to argument list for `Cli`.
2. Inspect remaining arguments:
   - If empty (or only binary name):
     - If `$WAYLAND_DISPLAY` or `$DISPLAY` is set and non-empty: `DispatchTarget::Gui` (when `gui` feature enabled).
     - Else: `DispatchTarget::HeadlessHelp`.
   - If first argument is `"gui"`:
     - If display server is active: `DispatchTarget::Gui`.
     - Else: `DispatchTarget::Error("No graphical display server detected.", 2)`.
   - Otherwise: `DispatchTarget::Cli(args)`.

## Verification

**Commands:**
- `export PATH="$HOME/.cargo/bin:$PATH" && cargo test --workspace` -- expected: exits 0 with all unit and E2E tests passing.
- `export PATH="$HOME/.cargo/bin:$PATH" && cargo check --workspace --no-default-features` -- expected: exits 0 without GUI dependencies.
- `export PATH="$HOME/.cargo/bin:$PATH" && cargo clippy --workspace --all-targets -- -D warnings` -- expected: exits 0 with zero warnings.
