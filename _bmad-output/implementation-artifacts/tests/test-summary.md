# Test Automation Summary — Story 1.1

## Generated Tests

### End-to-End (E2E) & Integration Tests
- [x] [`crates/tb/tests/e2e_binary.rs`](file:///home/bharath/Documents/toolbox/crates/tb/tests/e2e_binary.rs)
  - `test_e2e_tb_help` — Exercises `tb --help` binary execution, ensuring clean 0 exit status and stdout help documentation.
  - `test_e2e_tb_invalid_flag` — Exercises `tb --invalid-flag`, asserting non-zero error exit status and clear stderr message.
  - `test_e2e_tb_headless_mode` — Exercises headless binary invocation (`DISPLAY` and `WAYLAND_DISPLAY` absent), validating CLI mode fallback without display server panics.
  - `test_e2e_tb_json_flag` — Exercises `tb --json` argument handling.
- [x] [`crates/tb/tests/e2e_boundaries.rs`](file:///home/bharath/Documents/toolbox/crates/tb/tests/e2e_boundaries.rs)
  - `test_e2e_tb_core_isolation` — Programmatically audits `cargo tree -p tb-core` asserting `slint` and `clap` are 100% absent from leaf domain crate.
  - `test_e2e_tb_headless_features` — Programmatically audits `cargo tree -p tb --no-default-features` asserting `slint` and `tb-ui` are completely excluded in headless builds.

### Unit Tests
- [x] [`crates/tb/src/main.rs`](file:///home/bharath/Documents/toolbox/crates/tb/src/main.rs)
  - `test_determine_mode_gui`
  - `test_determine_mode_cli_with_args`
  - `test_determine_mode_headless`
  - `test_entrypoint_wiring`
- [x] [`crates/tb-cli/src/lib.rs`](file:///home/bharath/Documents/toolbox/crates/tb-cli/src/lib.rs)
  - `test_cli_run`
  - `test_cli_run_invalid_arg`
  - `test_cli_args_parsing`
- [x] [`crates/tb-core/src/lib.rs`](file:///home/bharath/Documents/toolbox/crates/tb-core/src/lib.rs)
  - `test_tb_error_display`
- [x] [`crates/tb-deps/src/lib.rs`](file:///home/bharath/Documents/toolbox/crates/tb-deps/src/lib.rs)
  - `test_check_dependency_empty`
  - `test_check_dependency_valid`
- [x] [`crates/tb-ext/src/lib.rs`](file:///home/bharath/Documents/toolbox/crates/tb-ext/src/lib.rs)
  - `test_list_extensions`
- [x] [`crates/tb-ui/src/lib.rs`](file:///home/bharath/Documents/toolbox/crates/tb-ui/src/lib.rs)
  - `test_ui_run`

## Coverage Summary
- **Binary End-to-End Execution**: 4/4 scenarios covered (Help, Invalid Args, Headless, JSON).
- **Architectural Boundary Isolation**: 2/2 invariants enforced (Leaf isolation, Headless exclusion).
- **Workspace Test Suites**: 18/18 tests pass across all crates.

## Next Steps
- Enforce strict TDD (Red $\rightarrow$ Green $\rightarrow$ Refactor) for Story 1.2 (Unified Multi-Call Binary Entry Point & Mode Dispatcher), writing failing tests before implementing multi-call symlink detection.
