---
title: 'Story 1.4: Base Slint Window Shell, Dual-Pane Workbench & Omarchy Theme Engine'
type: 'feature'
created: '2026-10-06'
status: 'done'
baseline_commit: '60b8be2df0a7cc6308264d908a739dde92a10922'
route: 'dispatch'
review_loop_iteration: 0
context:
  - '_bmad-output/implementation-artifacts/epic-1-context.md'
  - '.agent/skills/toolbox-slint-dev/SKILL.md'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Toolbox currently launches a stub window and lacks the foundational visual shell, resulting in no actual desktop graphical experience, no theme system, and no brutalist UI components. Desktop users launching `tb` have no UI workbench to interact with offline utility workflows.

**Approach:** Implement the native Slint desktop window shell in `crates/tb-ui` adhering to the Model A Dual-Pane Workbench (`mockups/minimal-workbench.html`). Create the Omarchy Brutalist design system with strict 0px border-radius, pure monospace typography, and runtime switching across all 10 Omarchy palettes (default: Industrial Graphite). Integrate boxy `slintcn` components (`Button`, `Card`, `Input`, `Dialog`, `Badge`, `Dropzone`) and configure automated software rendering fallback (`femtovg-software`). Follow the mandatory TDD Red-Green-Refactor protocol.

## Boundaries & Constraints

**Always:**
- Strict adherence to Rule 6 (TDD Mandatory Protocol): Automated unit and component tests MUST be written first and observed failing before writing minimal production code.
- Strict 0px border radius across all UI elements (`border-radius: 0px`). Rounded corners (`> 0px`) are strictly forbidden by Rule 3 of AGENTS.md.
- 100% monospace typography across the entire interface (`ui-monospace, "JetBrains Mono", monospace`).
- Zero hardcoded hex colors in Slint markup — all colors MUST bind dynamically to `Theme` tokens.
- Support all 10 curated Omarchy palettes (Industrial Graphite, Vantablack, Tokyo Night, Nord, Gruvbox Dark, Hackerman, Kanagawa, Catppuccin Mocha, Rose Pine, Everforest).
- Quiet 200px left sidebar with 5 primary navigation categories: `Image`, `PDF`, `Archive`, `Developer`, and `Settings`.
- Pressing `Escape` or the window close button cleanly exits with code `0`.
- Software rendering fallback resilience (`femtovg-software`) when GPU hardware acceleration is unavailable.

**Never:**
- Never depend on Slint in `tb-core` (preserves hexagonal crate boundaries).
- Never introduce drop shadows, gradient backgrounds, or non-monospace fonts.
- Never add cluttered sliders, nested tab cockpits, or telemetry calls.

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| GUI Cold Launch | `tb` or `tb gui` with active `$DISPLAY` or `$WAYLAND_DISPLAY` | Window shell appears in <350ms with 200px sidebar, Industrial Graphite theme, and tool canvas | Graceful fallback to software backend if GPU init fails |
| Escape Key Exit | User presses `Escape` within Slint window | Window hides and process exits cleanly with exit code 0 | None (standard clean termination) |
| Runtime Theme Switch | User switches theme to index 1 (Vantablack) or index 5 (Hackerman) | Active `Palette` updates immediately across all surfaces, borders, and text without app restart | Invalid theme index clamps to default (0: Industrial Graphite) |
| Headless Invocation | `tb gui` when display server is absent | Exits code 2 with `DISPLAY_NOT_FOUND` error envelope/message | Displays clear suggestion to run in graphical session |
| Navigation Switching | User clicks sidebar category (`PDF`, `Archive`, `Settings`) | Active view changes in canvas, active sidebar item highlights with 1px border | Default view remains `Image` on startup |

</frozen-after-approval>

## Code Map

- `crates/tb-ui/Cargo.toml` -- UI crate configuration adding `slint-build` in build-dependencies and slint backend features.
- `crates/tb-ui/build.rs` -- Build script compiling `ui/main.slint` into Rust modules.
- `crates/tb-ui/ui/theme.slint` -- Global `Theme` and `Spacing` tokens, `Palette` struct, and 10 Omarchy theme definitions.
- `crates/tb-ui/ui/slintcn/button.slint` -- Boxy brutalist `Button` component with primary, secondary, and outline variants.
- `crates/tb-ui/ui/slintcn/card.slint` -- Boxy brutalist `Card` container with 1px border.
- `crates/tb-ui/ui/slintcn/input.slint` -- Boxy brutalist monospace `Input` field.
- `crates/tb-ui/ui/slintcn/badge.slint` -- Boxy brutalist `Badge` status indicator.
- `crates/tb-ui/ui/slintcn/dropzone.slint` -- Boxy brutalist `Dropzone` with dashed border and hover focus highlight.
- `crates/tb-ui/ui/slintcn/dialog.slint` -- Boxy modal `Dialog` container.
- `crates/tb-ui/ui/slintcn/mod.slint` -- Re-export module for all `slintcn` components.
- `crates/tb-ui/ui/main.slint` -- Model A Dual-Pane Workbench window shell containing 200px sidebar, navigation items, canvas, and live CLI dock.
- `crates/tb-ui/src/lib.rs` -- Slint integration, software rendering fallback configuration, event handlers, and public `run()` entry point.
- `crates/tb-ui/tests/ui_tests.rs` -- Automated tests asserting theme switching, token validity, window creation, and 0px radius constraints.

## Tasks & Acceptance

**Execution:**
- [x] `crates/tb-ui/tests/ui_tests.rs` -- Write automated unit/integration tests verifying `Theme` tokens, all 10 Omarchy palettes, 0px border-radius compliance, and window creation (Red phase).
- [x] `crates/tb-ui/Cargo.toml` & `crates/tb-ui/build.rs` -- Add `slint-build` dependency and configure build script to compile Slint markup.
- [x] `crates/tb-ui/ui/theme.slint` -- Implement `Palette` struct, `Theme` global with all 10 Omarchy palettes, and `Spacing` with `radius_none: 0px`.
- [x] `crates/tb-ui/ui/slintcn/` -- Implement boxy brutalist components: `button.slint`, `card.slint`, `input.slint`, `badge.slint`, `dropzone.slint`, and `dialog.slint`.
- [x] `crates/tb-ui/ui/main.slint` -- Implement `MainWindow` with 200px sidebar, brand header, category navigation (`Image`, `PDF`, `Archive`, `Developer`, `Settings`), and single-task workbench canvas.
- [x] `crates/tb-ui/src/lib.rs` -- Include compiled modules, wire window callbacks (Escape hotkey, theme switching), configure software render fallback, and pass all tests (Green phase).
- [x] `crates/tb-ui/tests/ui_tests.rs` -- Run verification and refactor for zero warnings (Refactor phase).

**Acceptance Criteria:**
- Given `tb` launched in GUI mode, when the window appears, then it displays the Model A Dual-Pane Workbench with 200px sidebar and Industrial Graphite theme.
- Given any UI component rendered in `tb-ui`, all border radii are strictly 0px and font family is monospace.
- Given the theme switcher invoked with index 0 through 9, the theme transitions dynamically across all 10 Omarchy color palettes.
- Given `Escape` key pressed in GUI mode, the window closes and process exits with code 0.
- Given a machine without GPU acceleration, Slint falls back to software rendering without crashing.

## Implementation Notes

## Spec Change Log

## Review Triage Log

| Finding ID | Layer | Location | Verdict | Evidence / Notes | Route |
|------------|-------|----------|---------|------------------|-------|
| BH2 | blind-hunter | `crates/tb-ui/ui/slintcn/dialog.slint:9` | high | Hardcoded `#000000` violates Rule 3 and spec dynamic token constraints. | patch |
| BH16 | blind-hunter | `crates/tb-ui/ui/theme.slint:45` | high | Font stack missing `"Fira Code"` and `"Courier New"` mandated by AGENTS.md Rule 3. | patch |
| VG3 | verification-gap | `crates/tb-ui/tests/ui_tests.rs` | high | Test missing assertion on `Theme.font_family` monospace stack. | patch |
| EC2 | edge-case-hunter | `crates/tb-ui/src/lib.rs:8` | medium | `create_main_window` lacks fallback to software backend when GPU init fails. | patch |
| EC3 | edge-case-hunter | `crates/tb-ui/build.rs:14` | medium | Dangling symlink in `OUT_DIR` causes `symlink` call to fail with AlreadyExists. | patch |
| EC5 | edge-case-hunter | `crates/tb-ui/ui/main.slint:196` | medium | Switching sidebar categories retains incompatible `loaded_file` from prior category. | patch |
| BH1 | blind-hunter | `crates/tb-ui/src/lib.rs:23` | medium | Slint backend selector singleton caching makes `set_var` after failure ineffective. | patch |
| BH3 | blind-hunter | `crates/tb-ui/ui/main.slint:141` | medium | Empty sibling `FocusScope` loses Escape key events when children gain focus. | patch |
| BH6 | blind-hunter | `crates/tb-ui/ui/main.slint:497` | medium | CLI commands include leading `"$ "` prompt character breaking paste in terminal. | patch |
| BH12 | blind-hunter | `crates/tb-ui/tests/ui_tests.rs` | medium | Missing automated tests covering `slintcn` components (`Button`, `Card`, `Badge`, `Dropzone`, `Input`, `Dialog`). | patch |
| BH14 | blind-hunter | `crates/tb-ui/ui/slintcn/input.slint:32` | medium | Placeholder disappears immediately upon focus even when text is empty. | patch |
| BH15 | blind-hunter | `crates/tb-ui/build.rs:7` | medium | Missing aarch64 fontconfig candidate path and `cargo:rerun-if-changed=build.rs`. | patch |
| VG1 | verification-gap | `crates/tb-ui/tests/ui_tests.rs` | medium | Missing test verifying Escape key press triggers `request_close()`. | patch |
| VG2 | verification-gap | `crates/tb-ui/tests/ui_tests.rs` | medium | Missing test verifying `MainWindow` creation under software backend. | patch |
| VG4 | verification-gap | `crates/tb-ui/tests/ui_tests.rs` | medium | Missing assertions verifying dynamic CLI dock updates across categories and presets. | patch |
| EC1 | edge-case-hunter | `crates/tb-ui/src/lib.rs:59` | low | `TB_TEST_AUTO_CLOSE` does not check for empty string or "0"/"false". | patch |
| EC4 | edge-case-hunter | `crates/tb-ui/ui/main.slint:289` | low | Out-of-bounds `active_category` (>4 or <0) causes title/view divergence. | patch |
| EC6 | edge-case-hunter | `crates/tb-ui/ui/main.slint:488` | low | `active_preset` negative or >2 not clamped in CLI generator. | patch |
| BH4 | blind-hunter | `crates/tb-ui/src/lib.rs:36` | low | Window manager close button ('X') should hook into close request. | patch |
| BH5 | blind-hunter | `crates/tb-ui/src/lib.rs:50` | low | Live CLI dock copy action lacks feedback. | patch |
| BH7 | blind-hunter | `crates/tb-ui/ui/main.slint:497` | low | CLI command path concatenation lacks quoting for paths with spaces. | patch |
| BH13 | blind-hunter | `crates/tb-ui/ui/slintcn/dialog.slint:12` | low | Modal backdrop click dismissal missing. | patch |
| BH9 | blind-hunter | `crates/tb-ui/ui/slintcn/dropzone.slint` | false | Drag-and-drop file picker integration belongs to Story 2.3 per epic backlog. | rejected (out of scope) |
| BH11 | blind-hunter | `crates/tb-ui/ui/main.slint` | low | Monolithic layout is cosmetic; compiles cleanly with zero warnings. | rejected (cosmetic) |
| BH17 | blind-hunter | `crates/tb-ui/src/lib.rs` | false | Theme config file persistence belongs to settings/config story, not 1.4. | rejected (out of scope) |

## Verification

**Commands:**
- `cargo test -p tb-ui` -- expected: All UI unit, theme token, and window tests pass.
- `cargo check --workspace` -- expected: Workspace compiles with zero errors.
- `cargo check -p tb --no-default-features` -- expected: Headless CLI build passes without UI dependencies.
- `cargo clippy --workspace --all-targets -- -D warnings` -- expected: 0 linter warnings.
- `cargo test --workspace` -- expected: All workspace tests pass.
