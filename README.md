# Toolbox (`tb`)

> **Privacy-first, 100% offline Linux desktop utility suite paired with a unified multi-call CLI.**

Toolbox (`tb`) is an air-gapped, minimal, boxy brutalist Linux utility application built with **Rust** and **Slint**. It orchestrates standard open-source Linux tools under the hood (`qpdf`, `ImageMagick`, `7-Zip`, `ffmpeg`) with a strict **1-Second Comprehension** flow: *Drop File $\rightarrow$ Select 1 of 3 Presets (`Small`, `Balanced`, `Best`) $\rightarrow$ Action Button $\rightarrow$ Done*.

Every action in the GUI generates an identical, copyable CLI command in a persistent dock at the bottom of the window.

---

## Workspace Architecture

Toolbox is organized as a Cargo workspace with strict hexagonal dependency boundaries:

```
crates/
├── tb-core      # Domain leaf crate (Pure business logic, 0 presentation dependencies)
├── tb-deps      # Managed external tool resolver & health checker ($PATH, static binaries)
├── tb-ext       # Extension runner & IPC host
├── tb-cli       # Terminal adapter (Clap CLI parsing & JSON output)
├── tb-ui        # Desktop GUI adapter (Slint UI & Omarchy theme engine)
└── tb           # Multi-call root binary entry point (CLI/GUI mode dispatcher)
```

### Architectural Invariants
- **Leaf Isolation**: `tb-core` must **NEVER** depend on `slint` or `clap`, directly or transitively.
- **Headless Support**: The root binary `tb` features a `gui` feature gate (`default = ["gui"]`). The CLI must build and run headlessly with `--no-default-features`.
- **UI Design System**: Strict `0px` border-radius, pure monospace typography, zero gradients, 1px crisp borders, and dynamic Omarchy themes (default: `Industrial Graphite`).

---

## Prerequisites

1. **Rust Toolchain**:
   - Rust 1.85+ (Edition 2024 support).
   - Ensure Cargo is on your `$PATH`:
     ```bash
     export PATH="$HOME/.cargo/bin:$PATH"
     ```

2. **System Dependencies (Linux GUI)**:
   - For GUI builds via Slint, standard desktop development headers are required:
     ```bash
     # Arch Linux
     sudo pacman -S fontconfig freetype2 libxkbcommon wayland

     # Ubuntu / Debian
     sudo apt install -y libfontconfig1-dev libfreetype6-dev libxkbcommon-dev libwayland-dev
     ```

3. **External Tool Engines**:
   - Toolbox orchestrates existing system utilities when available:
     ```bash
     # Arch Linux
     sudo pacman -S qpdf imagemagick p7zip ffmpeg

     # Ubuntu / Debian
     sudo apt install -y qpdf imagemagick p7zip-full ffmpeg
     ```

---

## Running Locally

### 1. Run the Multi-Call Root Binary (`tb`)

- **Default Launch (GUI or CLI)**:
  If a graphical display server (`WAYLAND_DISPLAY` or `DISPLAY`) is present and no subcommand arguments are supplied, `tb` automatically opens the desktop GUI. Otherwise, it delegates to the CLI:
  ```bash
  cargo run -p tb
  ```

- **Run with CLI Arguments**:
  Passing subcommands or flags automatically activates the terminal adapter:
  ```bash
  cargo run -p tb -- --help
  cargo run -p tb -- --json
  ```

### 2. Run in Pure Headless Mode (CLI Only)

To compile and run `tb` without any GUI or Slint dependencies (ideal for servers and minimal containers):
```bash
cargo run -p tb --no-default-features -- --help
```

---

## Verification & Quality Gates

Before committing code or submitting pull requests, run the project's standard verification routine:

### 1. Workspace Compilation
Verify that all 6 crates compile cleanly with zero errors:
```bash
cargo check --workspace
```

### 2. Headless Compilation Check
Verify that the CLI builds without any graphics libraries:
```bash
cargo check --workspace --no-default-features
```

### 3. Test Suite
Run unit and integration tests across all workspace crates:
```bash
cargo test --workspace
```

### 4. Hexagonal Boundary Audits
Ensure presentation layers (`slint`, `clap`) have not leaked into `tb-core`, and headless `tb` excludes GUI dependencies:
```bash
# Verify tb-core has zero slint or clap dependencies:
! cargo tree -p tb-core 2>/dev/null | grep -E "slint|clap"

# Verify headless tb excludes slint and tb-ui:
! cargo tree -p tb --no-default-features 2>/dev/null | grep -E "slint|tb-ui"
```

### 5. Strict Clippy Linter
Enforce standard Rust conventions and zero warnings:
```bash
cargo clippy --workspace --all-targets -- -D warnings
```

---

## Production Release Build

To produce a stripped, size-optimized release binary ($< 20\text{ MB}$):
```bash
cargo build --release -p tb
```

The resulting executable will be located at:
```bash
target/release/tb
```

You can inspect the binary size and symbols:
```bash
ls -lh target/release/tb
```

---

## Project Documentation

- [`AGENTS.md`](./AGENTS.md) — Comprehensive architectural boundaries, design system constraints, and development rules.
- [`.agent/skills/toolbox-slint-dev/`](./.agent/skills/toolbox-slint-dev/) — Slint UI styling skill and Omarchy theme specifications.
- `_bmad-output/` — Project requirements, architecture spines, and sprint tracking artifacts.
