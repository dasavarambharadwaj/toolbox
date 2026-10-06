---
title: 'Story 1.3: Standardized CLI JSON Envelope & Error Taxonomy'
type: 'feature'
created: '2026-10-06'
status: 'done'
baseline_commit: '70706f3130551bdb270a21b0babc39cc7d4a770f'
route: 'dispatch'
review_loop_iteration: 0
context:
  - '_bmad-output/implementation-artifacts/epic-1-context.md'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** CLI consumers (scripts, local agents, terminal users) lack a unified output contract for structured success and failure responses. Currently, errors and data output vary between raw stderr text and unstandardized payloads, preventing deterministic machine parsing and consistent terminal diagnostics.

**Approach:** Implement a standardized machine-readable JSON envelope (`{"status": "success", "data": ...}` and `{"status": "error", "error": {"code": "...", "message": "...", "suggested_action": "..."}}`) and structured error taxonomy in `tb-cli` and `tb-core`. Support `--json`, honor standard exit codes (`0` = Success, `1` = Operation error, `2` = Invalid args, `3` = Missing engine, `130` = SIGINT), provide human-friendly monospace output on stderr, and strip ANSI escape codes when `NO_COLOR` is set or `TERM=dumb`. Follow the strict TDD Red-Green-Refactor cycle.

## Boundaries & Constraints

**Always:**
- Strict adherence to Rule 6 (TDD Mandatory Protocol): Red -> Green -> Refactor cycle. All automated unit and E2E integration tests MUST be written first and observed failing or failing to compile before functional implementation.
- Standardized JSON contract defined in `AGENTS.md` (Section 4) and Epic 1 Story 1.3:
  - Success envelope: `{"status": "success", "data": { ... }}` printed to stdout with exit code 0.
  - Error envelope: `{"status": "error", "error": {"code": "<ERROR_CODE>", "message": "<HUMAN_MESSAGE>", "suggested_action": "<ACTION_STRING>"}}` printed to stdout with corresponding non-zero exit code.
- Exit code taxonomy:
  - `0` = Success
  - `1` = Operation / Domain error (e.g. `OPERATION_FAILED`, `IO_ERROR`)
  - `2` = Invalid arguments / CLI syntax (e.g. `INVALID_ARGUMENT`, `USAGE_ERROR`)
  - `3` = Missing external engine (e.g. `ENGINE_MISSING`)
  - `130` = Interrupted (SIGINT)
- Terminal coloring & ANSI handling:
  - In non-JSON terminal mode, errors output formatted human-readable diagnostics with suggested commands.
  - If `NO_COLOR` environment variable is set (any non-empty value) OR `TERM=dumb`, terminal output strictly strips ANSI color escape codes and uses clean ASCII status labels (`[OK]`, `[FAIL]`).
- `tb-core` must remain a pure domain crate: zero dependencies on `slint` or `clap`. JSON envelope and formatting serialization live in `tb-cli` or `tb-core` via `serde`/`serde_json`.
- Headless compilation compatibility: `--no-default-features` must continue to build cleanly.

**Never:**
- Never output JSON error envelopes to stderr when `--json` is requested (structured JSON output must go to stdout so automation pipes receive parseable JSON).
- Never emit ANSI color escape sequences when `NO_COLOR` is set or `TERM=dumb`.
- Never use non-standard exit codes outside the defined taxonomy.
- Never write production code before failing tests exist.

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling | Exit Code |
|----------|--------------|---------------------------|----------------|-----------|
| Successful execution with `--json` | `tb --json` (or any valid subcommand) | stdout: `{"status":"success","data":{...}}` | None | 0 |
| Invalid argument with `--json` | `tb --invalid-flag --json` | stdout: `{"status":"error","error":{"code":"INVALID_ARGUMENT",...}}` | Formatted JSON envelope on stdout | 2 |
| Invalid argument without `--json` | `tb --invalid-flag` | stderr: formatted error with root cause & suggestion | Stderr text diagnostic | 2 |
| Missing engine with `--json` | Subcommand triggering missing engine with `--json` | stdout: `{"status":"error","error":{"code":"ENGINE_MISSING",...}}` | Formatted JSON envelope on stdout | 3 |
| Missing engine without `--json` | Subcommand triggering missing engine without `--json` | stderr: `[FAIL] Missing required engine: ...` | Actionable install hint | 3 |
| Operational domain failure with `--json` | Subcommand failing operation with `--json` | stdout: `{"status":"error","error":{"code":"OPERATION_FAILED",...}}` | Formatted JSON envelope on stdout | 1 |
| Operational domain failure without `--json` | Subcommand failing operation without `--json` | stderr: `[FAIL] ...` | Text diagnostic | 1 |
| Color output in interactive terminal | TTY terminal, `NO_COLOR` unset, `TERM!=dumb` | stderr uses colored text diagnostics | Non-zero exit code | >= 1 |
| `NO_COLOR` set | `NO_COLOR=1` | Stderr strictly ASCII labels (`[FAIL]`, `[OK]`), zero ANSI escapes `\x1b[` | Non-zero exit code | >= 1 |
| `TERM=dumb` | `TERM=dumb` | Stderr strictly ASCII labels (`[FAIL]`, `[OK]`), zero ANSI escapes `\x1b[` | Non-zero exit code | >= 1 |
| Root binary `DispatchTarget::Error` with `--json` | `tb gui` in headless environment with `--json` | stdout: `{"status":"error","error":{"code":"DISPLAY_NOT_FOUND",...}}` | Clean JSON envelope on stdout | 2 |

</frozen-after-approval>

## Code Map

- `crates/tb-core/src/lib.rs` -- Domain error taxonomy definitions (`TbError`), mapping to standard error codes and exit codes.
- `crates/tb-cli/Cargo.toml` -- Dependencies on `serde`, `serde_json`, `tb-core`, and `clap`.
- `crates/tb-cli/src/lib.rs` -- Command runner, envelope structs (`JsonEnvelope<T>`, `JsonError`), formatting helpers (`format_error`), NO_COLOR detection, and exit code routing.
- `crates/tb/src/main.rs` -- Multi-call entry point handling dispatch errors and integrating with JSON envelope formatting when `--json` is present.
- `crates/tb/tests/e2e_binary.rs` & `tests/e2e_binary.rs` -- E2E tests asserting `--json` success/error envelopes, NO_COLOR/TERM=dumb behavior, and exit code taxonomy.

## Tasks & Acceptance

**Execution:**

- [x] `tests/e2e_binary.rs`, `crates/tb/tests/e2e_binary.rs` & `crates/tb-cli/src/lib.rs` (TDD Step 1: RED) -- Author comprehensive failing unit and integration tests covering:
  - Unit tests for JSON success envelope serialization: `{"status":"success","data":...}`
  - Unit tests for JSON error envelope serialization: `{"status":"error","error":{"code":...,"message":...,"suggested_action":...}}`
  - Unit tests for `TbError` mapping to error codes (`INVALID_ARGUMENT`, `OPERATION_FAILED`, `ENGINE_MISSING`, `IO_ERROR`, `INTERRUPTED`) and exit codes (`0`, `1`, `2`, `3`, `130`).
  - Unit tests for ANSI stripping when `NO_COLOR` is set or `TERM=dumb`.
  - E2E tests:
    - `tb --json` outputs valid JSON success envelope and exits 0.
    - `tb --invalid-flag --json` outputs valid JSON error envelope to stdout with exit code 2.
    - `tb --invalid-flag` outputs formatted text to stderr with exit code 2.
    - `NO_COLOR=1 tb --invalid-flag` stderr contains no ANSI escape codes (`\x1b[`).
    - `TERM=dumb tb --invalid-flag` stderr contains no ANSI escape codes.
    - `tb gui --json` in headless outputs valid JSON error envelope on stdout and exits 2.
  - Run `cargo test --workspace` to confirm tests fail / fail to compile (RED phase).

- [x] `crates/tb-cli/Cargo.toml` & `crates/tb-cli/src/lib.rs` (TDD Step 2: GREEN - Implementation) --
  - Add `serde` and `serde_json` dependencies to `crates/tb-cli/Cargo.toml`.
  - Implement `JsonEnvelope<T>`, `JsonSuccess<T>`, and `JsonError` data structures.
  - Implement human-friendly formatter with ASCII/ANSI toggling based on `NO_COLOR` and `TERM=dumb`.
  - Implement structured CLI execution wrapper that captures domain/Clap errors and formats JSON or stderr text accordingly.
  - Wire `crates/tb/src/main.rs` to output JSON envelopes on dispatch error if `--json` was passed.

- [x] Verification & Refactoring (TDD Step 3: REFACTOR) --
  - Run `cargo test --workspace` (all unit and E2E tests pass).
  - Run `cargo check --workspace --no-default-features` (headless build compiles).
  - Run `cargo clippy --workspace --all-targets -- -D warnings` (clean with 0 warnings).

**Acceptance Criteria:**
- Given any CLI command executed with `--json`, when it completes successfully, stdout outputs `{"status":"success","data":{...}}` and exits with code 0.
- Given any CLI command executed with `--json`, when an error occurs, stdout outputs `{"status":"error","error":{"code":"...","message":"...","suggested_action":"..."}}` and exits with the corresponding non-zero exit code (`1`, `2`, or `3`).
- Given an invalid CLI argument without `--json`, stderr outputs clean text explaining the root cause and suggested command, exiting with code 2.
- Given `NO_COLOR` set or `TERM=dumb`, stderr output contains zero ANSI color escape sequences and uses `[FAIL]` / `[OK]` status labels.
- Given `tb --no-default-features`, the entire workspace builds and passes checks.

## Implementation Notes

- Added `EXIT_SUCCESS` (0), `EXIT_OPERATION_ERROR` (1), `EXIT_INVALID_ARGUMENT` (2), `EXIT_MISSING_ENGINE` (3), `EXIT_INTERRUPTED` (130) constants and `TbError::exit_code()`, `TbError::error_code()`, `TbError::suggested_action()` in `tb-core`.
- Implemented `JsonEnvelope<T>`, `JsonSuccess<T>`, `JsonError`, `format_error()`, `format_success()`, `should_use_color()`, `should_color_with_env()`, `strip_ansi()`, `run_cli()`, and `handle_result()` in `tb-cli`.
- Wired multi-call binary `tb/src/main.rs` to invoke `run_cli()` and format JSON envelopes on stdout when `--json` is supplied to `DispatchTarget::Error` (e.g. headless `tb gui --json`).
- Verified zero clippy warnings across workspace targets, passing headless check with `--no-default-features`, and all 46 workspace tests passing.

## Spec Change Log

## Review Triage Log
- crates/tb-cli/src/lib.rs:178 -- medium (patch) -- strip_ansi was not invoked when color is disabled. Patched format_error to invoke strip_ansi if !color_enabled and hardened scanner.
- crates/tb-cli/src/lib.rs:233 -- medium (patch) -- Clap error extraction full_err.lines().next() dropped multi-line error context. Patched to split on "\n\n".
- crates/tb/src/main.rs:137 -- medium (patch) -- Fragile string matching for error classification in DispatchTarget::Error. Mapped headless/disabled GUI to FEATURE_UNAVAILABLE with actionable message.
- crates/tb/Cargo.toml:481 -- low (patch) -- Redundant serde_json in [dev-dependencies]. Removed.
- crates/tb-cli/src/lib.rs:83 -- low (patch) -- Unused JsonSuccess struct. Removed.
- crates/tb-core/src/lib.rs:445 -- medium (patch) -- Missing unit test for TbError::suggested_action(). Added unit test.
- crates/tb-cli/src/lib.rs:134 -- medium (patch) -- Missing verification of should_use_color(true) inspecting NO_COLOR/TERM. Added test.
- crates/tb-cli/src/lib.rs:264 -- low (reject) -- handle_result is the public integration API for domain operations, tested via unit tests and ready for Epic 2.
- tests/e2e_binary.rs -- low (reject) -- Root tests/ directory mirrors crates/tb/tests/ and is maintained as workspace convention.

## Design Notes

### JSON Envelope Schema
```rust
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Envelope<T> {
    Success {
        data: T,
    },
    Error {
        error: ErrorPayload,
    },
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ErrorPayload {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_action: Option<String>,
}
```

### Exit Code Mapping
```rust
impl TbError {
    pub fn exit_code(&self) -> i32 {
        match self {
            TbError::InvalidArgument(_) => 2,
            TbError::OperationFailed(_) | TbError::Io(_) => 1,
            TbError::EngineMissing(_) => 3,
            TbError::Interrupted => 130,
        }
    }

    pub fn error_code(&self) -> &'static str {
        match self {
            TbError::InvalidArgument(_) => "INVALID_ARGUMENT",
            TbError::OperationFailed(_) => "OPERATION_FAILED",
            TbError::Io(_) => "IO_ERROR",
            TbError::EngineMissing(_) => "ENGINE_MISSING",
            TbError::Interrupted => "INTERRUPTED",
        }
    }
}
```

## Verification

**Commands:**
- `export PATH="$HOME/.cargo/bin:$PATH" && cargo test --workspace` -- expected: exits 0 with all unit and E2E tests passing.
- `export PATH="$HOME/.cargo/bin:$PATH" && cargo check --workspace --no-default-features` -- expected: exits 0 without GUI dependencies.
- `export PATH="$HOME/.cargo/bin:$PATH" && cargo clippy --workspace --all-targets -- -D warnings` -- expected: exits 0 with zero warnings.
