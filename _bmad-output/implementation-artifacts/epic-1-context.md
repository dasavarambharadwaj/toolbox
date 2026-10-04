# Epic 1 Context: Workspace Scaffolding, Multi-Call Dispatcher & Slint Window Shell

<!-- Compiled from planning artifacts. Edit freely. Regenerate with compile-epic-context if planning docs change. -->

## Goal

Establish the foundational 6-crate Hexagonal Cargo workspace, the unified multi-call binary `tb` entry point, the base hardware-accelerated Slint desktop window shell with software-render fallback, Clap CLI adapter with `--json` envelopes, and Linux desktop launcher integration. This provides the execution substrate for all subsequent utility suites.

## Stories

- Story 1.1: Cargo Workspace Initialization & Hexagonal Crate Boundaries
- Story 1.2: Unified Multi-Call Binary Entry Point & Mode Dispatcher
- Story 1.3: Standardized CLI JSON Envelope & Error Taxonomy
- Story 1.4: Base Slint Window Shell, Dual-Pane Workbench & Omarchy Theme Engine
- Story 1.5: Linux Desktop Integration & Application Registration

## Requirements & Constraints

- **Workspace Boundaries:** The repository must be structured as a 6-crate Hexagonal Cargo workspace (`tb`, `tb-core`, `tb-deps`, `tb-ext`, `tb-cli`, `tb-ui`). `tb-core` must remain a pure domain engine strictly forbidden from depending on `slint` or `clap`.
- **Unified Binary:** Single `tb` executable on disk. Dispatches to CLI in <50ms when invoked with subcommands or piped stdin; launches Slint GUI when invoked with 0 args in a graphical session (`$WAYLAND_DISPLAY` or `$DISPLAY`).
- **Performance Budget:** GUI cold start to first rendered frame <350ms; CLI cold start <50ms; baseline idle memory <60MB RAM.
- **Rendering Resilience:** Hardware-accelerated Wayland and X11 rendering via GPU with automatic fallback to Slint's software renderer (`femtovg-software`) at 60Hz on older GPUs and VMs.
- **Machine-Readable CLI Output:** All CLI commands must support `--json` emitting structured envelopes (`{"success": true, "data": ...}` or `{"success": false, "error": ...}`).
- **Binary Footprint:** Root release profile configured with `opt-level = "z"`, `lto = "fat"`, `codegen-units = 1`, `panic = "abort"`, and `strip = true` targeting <20MB stripped binary size.

## Technical Decisions

- **AD-1 (Cargo Workspace Crate Boundaries):** Strict boundary isolation between pure domain engine (`tb-core`), managed dependencies (`tb-deps`), extensions (`tb-ext`), terminal adapter (`tb-cli`), and Slint GUI (`tb-ui`).
- **AD-2 (Unified Multi-Call Binary Dispatch):** Single multi-call binary `tb` inspecting arguments and display session.
- **Error Taxonomy:** `tb-core` and domain crates use `thiserror` for typed errors; presentation adapters (`tb-cli`, `tb-ui`) use `anyhow` with standard exit codes (`0` = Success, `1` = Operation error, `2` = Invalid args, `3` = Missing engine, `130` = SIGINT).
- **Filesystem Base Paths:** Adhere to XDG base directory specification (`~/.config/toolbox/`, `~/.local/share/toolbox/deps/`, `~/.local/state/toolbox/logs/`).

## UX & Interaction Patterns

- **Design System & Theming:** `crates/tb-ui/ui/theme.slint` implements global `Theme` tokens with strict `0px` border-radius (`radius-none`), pure monospace typography (`ui-monospace, "JetBrains Mono", monospace`), 1px crisp borders, and dynamic runtime switching across 10 Omarchy themes (Default: Industrial Graphite `#141618`).
- **Component Registry:** `crates/tb-ui/ui/slintcn/` copy-paste components (`Button`, `Card`, `Input`, `Dialog`, `Badge`, `Dropzone`) bound to `Theme` tokens with 0px boxy borders and 1px crisp lines.
- **Shell Architecture:** Model A Dual-Pane Workbench shell matching `mockups/minimal-workbench.html`: quiet 200px left sidebar (`Image`, `PDF`, `Archive`, `Developer`, `Settings`) and centered single-task tool canvas.

## Cross-Story Dependencies

- **Story 1.1** is the root prerequisite that scaffolds `Cargo.toml` and all 6 crate skeletons.
- **Story 1.2** builds on 1.1, implementing the multi-call dispatcher in `crates/tb/src/main.rs`.
- **Story 1.3** and **Story 1.4** build on 1.2, implementing the CLI formatter and Slint window shell in parallel.
- **Story 1.5** registers the desktop launcher file pointing to the compiled `tb` binary.
