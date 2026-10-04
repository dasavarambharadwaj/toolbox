---
name: Toolbox
description: 100% offline, privacy-first digital utility suite for Linux. Minimal, modern, boxy brutalism with all-monospace typography, 0px borders, and Omarchy-inspired theme engine in Slint.
status: final
sources:
  - _bmad-output/planning-artifacts/PRD.md
  - _bmad-output/planning-artifacts/ARCHITECTURE-SPINE.md
  - _bmad-output/planning-artifacts/epics.md
updated: 2026-10-04

colors:
  # Default Theme: Industrial Graphite (Workstation Cold Slate)
  canvas: '#141618'
  surface: '#1E2124'
  surface-hover: '#262A2E'
  surface-sunken: '#0E1012'
  border-subtle: '#282C32'
  border-prominent: '#32383E'
  border-focus: '#FFFFFF'
  text-primary: '#E2E8F0'
  text-secondary: '#94A3B8'
  text-muted: '#64748B'
  accent-primary: '#FFFFFF'
  accent-foreground: '#141618'
  state-success: '#4ADE80'
  state-error: '#F87171'

  # Supported Omarchy Theme Registry (Mapped in theme.slint)
  theme-graphite-canvas: '#141618'
  theme-graphite-surface: '#1E2124'
  theme-graphite-border: '#32383E'
  theme-graphite-text: '#E2E8F0'

  theme-vantablack-canvas: '#000000'
  theme-vantablack-surface: '#0D0D0D'
  theme-vantablack-border: '#262626'
  theme-vantablack-text: '#FAFAFA'

  theme-tokyonight-canvas: '#1A1B26'
  theme-tokyonight-surface: '#24283B'
  theme-tokyonight-border: '#414868'
  theme-tokyonight-text: '#C0CAF5'

  theme-nord-canvas: '#2E3440'
  theme-nord-surface: '#3B4252'
  theme-nord-border: '#4C566A'
  theme-nord-text: '#ECEFF4'

  theme-gruvbox-canvas: '#1D2021'
  theme-gruvbox-surface: '#282828'
  theme-gruvbox-border: '#3C3836'
  theme-gruvbox-text: '#EBDBB2'

  theme-hackerman-canvas: '#080C08'
  theme-hackerman-surface: '#0E160E'
  theme-hackerman-border: '#1A2B1A'
  theme-hackerman-text: '#00FF66'

  theme-kanagawa-canvas: '#1F1F28'
  theme-kanagawa-surface: '#2A2A37'
  theme-kanagawa-border: '#363646'
  theme-kanagawa-text: '#DCD7BA'

  theme-catppuccin-canvas: '#1E1E2E'
  theme-catppuccin-surface: '#181825'
  theme-catppuccin-border: '#313244'
  theme-catppuccin-text: '#CDD6F4'

  theme-rosepine-canvas: '#191724'
  theme-rosepine-surface: '#1F1D2E'
  theme-rosepine-border: '#26233A'
  theme-rosepine-text: '#E0DEF4'

  theme-everforest-canvas: '#272E33'
  theme-everforest-surface: '#2D353B'
  theme-everforest-border: '#414B50'
  theme-everforest-text: '#D3C6AA'

typography:
  font-family: 'ui-monospace, "JetBrains Mono", "Fira Code", "Courier New", monospace'
  h1:
    fontSize: '18px'
    fontWeight: '700'
    lineHeight: '1.2'
    letterSpacing: '-0.02em'
  h2:
    fontSize: '14px'
    fontWeight: '600'
    lineHeight: '1.3'
    letterSpacing: '0.04em'
  body:
    fontSize: '13px'
    fontWeight: '400'
    lineHeight: '1.4'
  mono-code:
    fontSize: '12px'
    fontWeight: '400'
    lineHeight: '1.4'
  caption:
    fontSize: '11px'
    fontWeight: '400'
    lineHeight: '1.3'

rounded:
  none: '0px'
  sm: '0px'
  md: '0px'
  lg: '0px'
  DEFAULT: '0px'

spacing:
  1: '4px'
  2: '8px'
  3: '12px'
  4: '16px'
  6: '24px'
  8: '32px'
  12: '48px'

components:
  card:
    background: '{colors.surface}'
    border: '1px solid {colors.border-subtle}'
    radius: '{rounded.none}'
  button-primary:
    background: '{colors.accent-primary}'
    foreground: '{colors.accent-foreground}'
    border: '1px solid {colors.accent-primary}'
    radius: '{rounded.none}'
    fontWeight: '700'
  button-secondary:
    background: 'transparent'
    foreground: '{colors.text-primary}'
    border: '1px solid {colors.border-prominent}'
    radius: '{rounded.none}'
  input:
    background: '{colors.surface-sunken}'
    foreground: '{colors.text-primary}'
    border: '1px solid {colors.border-subtle}'
    focusBorder: '1px solid {colors.border-focus}'
    radius: '{rounded.none}'
  dropzone:
    background: '{colors.surface}'
    border: '1px dashed {colors.border-prominent}'
    hoverBorder: '1px solid {colors.border-focus}'
    radius: '{rounded.none}'
  cli-dock:
    background: '{colors.surface-sunken}'
    border: '1px solid {colors.border-subtle}'
    foreground: '{colors.text-secondary}'
    radius: '{rounded.none}'
---

# Toolbox — Design System Specification

## 1. Brand & Style
Toolbox (`tb`) is the **"VLC of Digital Utilities for Linux"** — an offline, air-gapped, high-performance desktop suite designed to replace bloated web tools with hardware-accelerated local execution. 

Its aesthetic philosophy is **Radical Reduction and All-Monospace Brutalism**:
- **Zero Cockpit Noise:** No gratuitous status tags, no dense engineering dashboards, and no marketing buzzwords. 
- **1-Second Comprehension:** Every screen communicates its sole purpose immediately. The user knows where to drop a file, which preset to pick, and what button executes the task within one second of viewing.
- **Pure Boxy Geometry:** 100% sharp 0px corners, crisp 1px borders, zero gradients, and zero drop shadows.
- **Terminal Cyberdeck Heritage:** Grounded in pure monospace typography (`JetBrains Mono` / system monospace) paired with Omarchy's beloved Linux color palettes.

Visual Reference: [`mockups/minimal-workbench.html`](file:///home/bharath/Documents/toolbox/_bmad-output/planning-artifacts/ux-designs/ux-toolbox-2026-10-04/mockups/minimal-workbench.html). Spines win on conflict.

---

## 2. Colors
The default color palette is **Industrial Graphite** — a cool, muted slate that minimizes eye fatigue during long terminal sessions:
- `{colors.canvas}` (`#141618`): Deep workstation backdrop providing high visual contrast without harsh glare.
- `{colors.surface}` (`#1E2124`): Crisp card and sidebar container layer.
- `{colors.surface-sunken}` (`#0E1012`): Sunken input field background and CLI dock tray.
- `{colors.border-subtle}` (`#282C32`): 1px structural framing line.
- `{colors.border-prominent}` (`#32383E`): Boundary delineator for interactive dropzones and secondary buttons.
- `{colors.border-focus}` (`#FFFFFF`): 1px high-contrast focus outline for keyboard navigation.
- `{colors.text-primary}` (`#E2E8F0`): Clean, crisp primary copy and active tool indicators.
- `{colors.text-secondary}` (`#94A3B8`): Preset labels, secondary actions, and CLI string text.
- `{colors.text-muted}` (`#64748B`): Inactive tools, helper text, and subtle dividers.

### Multi-Theme Omarchy Support
Toolbox natively supports 10 theme palettes imported from Omarchy's `colors.toml` standard:
1. **Industrial Graphite** (Default: `#141618`)
2. **Vantablack / Matte Black** (OLED pitch black: `#000000`)
3. **Tokyo Night** (Cyberpunk navy: `#1A1B26`)
4. **Nord** (Arctic slate: `#2E3440`)
5. **Gruvbox Dark** (Retro warm amber: `#1D2021`)
6. **Hackerman** (Matrix phosphor green: `#080C08`)
7. **Kanagawa** (Samurai ink: `#1F1F28`)
8. **Catppuccin Mocha** (Soothing slate: `#1E1E2E`)
9. **Rose Pine** (Moonlit minimal: `#191724`)
10. **Everforest** (Forest pine: `#272E33`)

Theme switching dynamically updates the `global Theme` properties in Slint without application restart.

---

## 3. Typography
- **Typeface:** Pure monospace across all UI elements (`ui-monospace, "JetBrains Mono", "Fira Code", monospace`).
- **Hierarchy:**
  - `h1`: 18px / 700 / uppercase for Tool Titles (e.g. `IMAGE COMPRESS`).
  - `h2`: 14px / 600 / uppercase for Section Headers and Group Labels.
  - `body`: 13px / 400 for dropzone instructions and standard readouts.
  - `mono-code`: 12px / 400 for file paths, CLI command strings, and raw tokens.
  - `caption`: 11px / 400 for secondary file metadata and keyboard shortcuts.

---

## 4. Layout & Spacing
- **Spacing Scale:** Strict 4px grid (`4px`, `8px`, `12px`, `16px`, `24px`, `32px`, `48px`).
- **Shell Dimensions:**
  - Window Default Size: `1000px` width × `640px` height (Wayland/X11 centered).
  - Left Sidebar: Fixed `200px` width.
  - Main Workspace: Flex-grow with minimum `40px` horizontal and `32px` vertical padding.
  - Dropzone: Generous `140px` to `180px` height with centered content.

---

## 5. Elevation & Depth
- **Zero Drop Shadows:** Elevation is communicated purely through tonal layering and 1px border contrast.
- **Layering Order:**
  - Base: `{colors.canvas}`
  - Sunken (Inputs / CLI Dock): `{colors.surface-sunken}` with 1px `{colors.border-subtle}`.
  - Raised (Cards / Sidebar / Dropzone): `{colors.surface}` with 1px `{colors.border-subtle}`.
  - Focus Ring: 1px solid `{colors.border-focus}` with zero blur.

---

## 6. Shapes
- **All Corner Radii:** Locked to `0px` (`rounded.none`). No rounded corners anywhere in buttons, cards, dropzones, inputs, or window frames.

---

## 7. Components (`slintcn` Integration)
Built using `slintcn` copy-paste components styled to the design tokens:
- **Button (Primary):** Solid `{colors.accent-primary}` background, `{colors.accent-foreground}` text, 0px border. Inverts slightly on press.
- **Button (Secondary / Presets):** Transparent background, 1px `{colors.border-prominent}`, `{colors.text-secondary}` text. Becomes `{colors.accent-primary}` outline and `{colors.text-primary}` on active/selected.
- **Dropzone:** Large box with 1px dashed `{colors.border-prominent}`. Snaps to solid 1px `{colors.border-focus}` on file drag-over.
- **Live CLI Dock:** Sunken 1px container displaying the real-time terminal invocation with a discreet `[Copy]` button.

---

## 8. Do's and Don'ts
- **DO** maintain generous negative space around every primary action.
- **DO** use the live CLI command one-liner on every tool screen.
- **DO** keep button labels short and active (`[ Compress ]`, `[ Merge ]`, `[ Format ]`).
- **DON'T** use gradients, blur, shadows, or rounded pill shapes.
- **DON'T** clutter the UI with engineering buzzwords (`SIMD`, `Air-Gapped`, `v1.0.4`) on primary tool screens.
- **DON'T** require more than 2 clicks to execute any default operation after dropping a file.
