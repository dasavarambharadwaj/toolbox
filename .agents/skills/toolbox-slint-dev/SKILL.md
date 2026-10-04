---
name: toolbox-slint-dev
description: >-
  Use this skill when developing, styling, refactoring, or verifying Slint UI components,
  views, or themes in the Toolbox (`crates/tb-ui`) codebase. Enforces the 0px boxy brutalist
  design system, monospace typography, Omarchy theme bindings, and 1-second comprehension flow.
---

# Toolbox Slint UI Development Skill

This skill guides the implementation and verification of user interface components in `crates/tb-ui/` using **Slint** and the **Omarchy Brutalist Theme System**.

---

## 1. Core Design Constraints

Whenever authoring or modifying `.slint` files, strictly observe these 4 hard rules:

1. **Strict 0px Border Radius**:
   - `border-radius: 0px;` on EVERY rectangle, button, card, input, and popover.
   - Rounded corners are forbidden.
2. **100% Monospace Typography**:
   - Font family must always be the monospace stack: `"ui-monospace, \"JetBrains Mono\", monospace"`.
3. **No Cockpit Clutter (1-Second Comprehension)**:
   - Tool screens must follow: **Drop zone $\rightarrow$ 3 Presets (`Small`, `Balanced`, `Best`) $\rightarrow$ Primary Action $\rightarrow$ Live CLI Dock**.
   - Avoid overwhelming sliders or nested tabs in default view.
4. **No Hardcoded Hex Colors**:
   - Always reference theme properties (e.g. `Theme.surface`, `Theme.text_primary`).
   - Consult [design-tokens.md](./references/design-tokens.md) and [omarchy-themes.md](./references/omarchy-themes.md).

---

## 2. Component Architecture

All UI components live in `crates/tb-ui/ui/`:

```text
crates/tb-ui/
├── Cargo.toml
├── build.rs             # Slint build script compiling main.slint
├── src/
│   ├── lib.rs           # Slint UI adapter exposing run_app()
│   └── bridge.rs        # Rust-to-Slint data models and callbacks
└── ui/
    ├── main.slint       # Root window (Workbench shell: Left Nav + Main Tool Canvas)
    ├── theme.slint      # Global Theme & Omarchy palette definitions
    ├── components/      # Reusable primitives (cards, buttons, dropzone, presets, CLI dock)
    │   ├── button.slint
    │   ├── dropzone.slint
    │   ├── preset-row.slint
    │   └── cli-dock.slint
    └── views/           # Specific tool view panels
        ├── image_view.slint
        ├── pdf_view.slint
        └── archive_view.slint
```

---

## 3. Step-by-Step Component Authoring

### Step 1: Reference Tokens & Primitive Components
- Import theme definitions:
  ```slint
  import { Theme, Spacing } from "../theme.slint";
  ```
- See the reference implementation in [brutalist-card.slint](./examples/brutalist-card.slint).

### Step 2: Implement the 1-Second Flow
For each tool view:
1. **Dropzone**: Dashed 1px border (`Theme.border_prominent`), hover highlight to `Theme.border_focus`.
2. **Preset Row**: A horizontal layout of 3 selectable pill buttons (`Small`, `Balanced`, `Best`).
3. **Primary Action Button**: High-contrast filled button (`background: Theme.accent_primary`, `color: Theme.accent_foreground`).
4. **Live CLI Dock**: Sunken surface (`Theme.surface_sunken`) displaying the generated CLI invocation with a quick `[ Copy CLI ]` button.

### Step 3: Wire Callbacks to Rust in `tb-ui`
In `crates/tb-ui/src/bridge.rs`, connect UI callbacks to `tb-core` domain services:
```rust
ui.on_execute_compression({
    let ui_handle = ui.as_weak();
    move |input_path, preset| {
        // Dispatch to domain logic or background task
    }
});
```

---

## 4. Verification & Testing

Always verify Slint markup changes before finalizing:

1. **Slint Syntax Check via Cargo**:
   ```bash
   export PATH="$HOME/.cargo/bin:$PATH"
   cargo check -p tb-ui
   ```
2. **Headless Verification**:
   Ensure `tb` builds without GUI dependencies when feature-gated:
   ```bash
   cargo check -p tb --no-default-features
   ```
3. **Run the Full Suite**:
   ```bash
   cargo test --workspace
   ```
