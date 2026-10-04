# AGENTS.md — Toolbox (`tb`) Project Rules & Guidelines

This document defines the architectural boundaries, UX/UI contracts, and development conventions for the **Toolbox (`tb`)** project. All agents working in this repository must adhere to these rules.

---

## 1. Project Philosophy & Core Identity

- **Toolbox (`tb`)** is a privacy-first, 100% offline Linux desktop utility suite paired with a unified multi-call CLI.
- **Air-Gapped & Offline**: All core utility operations (PDF, Image, Archive, Video, Developer tools) run completely locally. Zero network calls, telemetry, or analytics in core execution paths.
- **1-Second Comprehension**: Every UI screen must be immediately understandable within 1 second of looking at it.
  - The primary flow is strictly: **Drop File $\rightarrow$ Select 1 of 3 Presets (`Small`, `Balanced`, `Best`) $\rightarrow$ Action Button $\rightarrow$ Done**.
  - No cockpit clutter, excessive sliders, or technical jargon in default views.
  - Every UI action generates a visible, copyable terminal command in a persistent live CLI dock at the bottom of the screen.

---

## 2. Hexagonal Architecture & Crate Boundaries

The codebase is organized as a Cargo workspace with strict hexagonal dependency boundaries:

```
crates/
├── tb-core      # Domain leaf crate (Zero presentation dependencies)
├── tb-deps      # Managed external tool resolver & health checker
├── tb-ext       # Extension runner & plugin system
├── tb-cli       # Terminal adapter (Clap CLI commands)
├── tb-ui        # Slint GUI adapter (Desktop interface)
└── tb           # Multi-call root binary entry point (Dispatches CLI or GUI)
```

### Dependency Rules:
1. **`tb-core` Isolation**:
   - `tb-core` MUST NEVER depend on `slint` or `clap`.
   - Contains pure domain logic, traits, common error types (`TbError` via `thiserror`), and data definitions.
   - Enforce with: `cargo tree -p tb-core` (must remain clean of UI/CLI crates).
2. **Tool Wrapping Strategy (`tb-deps`)**:
   - Orchestrates standard Linux utilities (`qpdf`, `ImageMagick`, `7-Zip`, `ffmpeg`).
   - Probe order:
     1. Host `$PATH` executable check.
     2. User-space fallback in `~/.local/share/tb/bin/` (static `musl` binaries).
     3. Distro package command helper (`pacman -S ...`, `apt install ...`).
   - Pure Rust implementations (`serde_json`, `fast_image_resize`, `oxipng`) are used where suitable.
3. **Multi-Call Binary (`tb`)**:
   - Dispatches based on executable name (e.g. symlinked `tb-compress`) or subcommand args (`tb image compress ...`).
   - Features `gui` flag gate (`default = ["gui"]`). The CLI must build and run fully headless with `--no-default-features`.

---

## 3. Slint UI Design System: Boxy Monospace Brutalism

All Slint UI development in `crates/tb-ui/` must strictly comply with the **Omarchy Brutalist Design System**:

1. **Border Radius**:
   - Strict `0px` border-radius across all elements (windows, cards, buttons, text inputs, dropzones).
   - Rounded corners (`border-radius > 0px`) are strictly forbidden.
2. **Typography**:
   - 100% monospace typography across the entire interface.
   - Font stack: `ui-monospace, "JetBrains Mono", "Fira Code", "Courier New", monospace`.
3. **Surfaces & Borders**:
   - Zero gradients, zero drop shadows.
   - 1px crisp borders (`border-subtle`, `border-prominent`).
   - Contrastive states: hover uses surface lightness shifts or inverted colors.
4. **Theme Engine**:
   - Built on 10 curated Omarchy color palettes.
   - **Default Theme**: `Industrial Graphite` (`#141618` canvas, `#1E2124` surface, `#32383E` border, `#E2E8F0` text, `#FFFFFF` accent).
   - All color assignments in `.slint` files must bind to the shared theme tokens, never hardcoded raw hex strings.

---

## 4. CLI Output Conventions

- All CLI commands must support both human-friendly monospace output and machine-readable `--json` output.
- Structured JSON output contract:
  - Success: `{"status": "success", "data": { ... }}`
  - Error: `{"status": "error", "error": {"code": "FILE_NOT_FOUND", "message": "..."}}`
- All errors exit with non-zero status codes and clean stderr formatting.

---

## 5. Development & Verification Workflows

- **Rust Toolchain**: Cargo and rustc reside in `$HOME/.cargo/bin`. Prepend `export PATH="$HOME/.cargo/bin:$PATH"` when executing commands.
- **Verification Routine**:
  1. `cargo check --workspace`
  2. `cargo check --workspace --no-default-features` (Headless check)
  3. `cargo test --workspace`
  4. `cargo clippy --workspace --all-targets -- -D warnings`
- **Release Profile Constraints**:
  - Size-optimized binary profile in root `Cargo.toml`: `opt-level = "z"`, `lto = "fat"`, `codegen-units = 1`, `panic = "abort"`, `strip = true`. Target binary size $< 20\text{ MB}$.

---

## 6. Test-Driven Development (TDD) Mandatory Protocol

From now on, all agents and developers working in this repository must strictly adhere to **Test-Driven Development (TDD)**:

1. **Strict Red-Green-Refactor Cycle**:
   - **Step 1 (Red)**: Before writing ANY functional implementation for a story, feature, or bug fix, write the automated unit or integration tests that assert the desired behavior. Run `cargo test` to observe that the tests fail or do not yet compile.
   - **Step 2 (Green)**: Write the minimal amount of production code needed to make the failing tests pass.
   - **Step 3 (Refactor)**: Clean up, optimize, and organize the code while ensuring all tests continue to pass with zero regressions and zero clippy warnings.
2. **End-to-End (E2E) & Integration Tests**:
   - Multi-crate integration and end-to-end binary tests reside in the top-level `tests/` directory.
   - Tests must exercise real CLI invocations, environment variables, exit codes, and `--json` payloads using standard Rust process assertions (`assert_cmd`, `std::process::Command`).
   - Domain unit tests reside in their respective crate's `src/` files under `#[cfg(test)] mod tests`.
3. **No Code Without Tests**:
   - PRs, stories, and task implementations that add code without corresponding automated tests written first are considered invalid.

