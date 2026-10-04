---
name: Toolbox
status: final
sources:
  - _bmad-output/planning-artifacts/PRD.md
  - _bmad-output/planning-artifacts/ARCHITECTURE-SPINE.md
  - _bmad-output/planning-artifacts/epics.md
updated: 2026-10-04
---

# Toolbox — Experience Spine

> Behavioral specification for Toolbox (`tb`). Single-window dual-pane Linux desktop application built on Slint 1.8+ with Wayland and X11 native rendering. Paired with `DESIGN.md` (Toolbox Design System) and interactive visual reference `mockups/minimal-workbench.html`.

---

## 1. Foundation

- **Platform & Form-Factor:** Dedicated Linux desktop window (Wayland native, X11 fallback), with software fallback (`femtovg-software`) on older Intel HD GPUs and VMs.
- **UI Architecture:** Slint 1.8+ declarative markup utilizing `slintcn` copy-paste components stored directly in `crates/tb-ui/ui/slintcn/`.
- **Theming Engine:** Dynamic theme switching supporting 10 Omarchy palettes (Default: Industrial Graphite) via Slint `global Theme` tokens.
- **Performance Budget:** GUI cold start to first rendered frame <350ms; idle memory footprint <60MB RAM; zero blocking on the UI thread (Rayon background worker pool with 16ms rate-limited event-loop callbacks).
- **Security & Privacy Invariant:** 100% offline and air-gapped core execution; zero outbound network sockets.

Composition reference: [`mockups/minimal-workbench.html`](file:///home/bharath/Documents/toolbox/_bmad-output/planning-artifacts/ux-designs/ux-toolbox-2026-10-04/mockups/minimal-workbench.html). Spines win on conflict.

---

## 2. Information Architecture

Toolbox utilizes a **Model A Dual-Pane Workbench** layout:

| Surface | Reached via | Layout & Purpose |
|---|---|---|
| **Omni-Bar Palette** | `Super+Shift+T` or `Ctrl+K` | Floating centered command modal (<50ms popup). Sub-16ms fuzzy tool search (`nucleo-matcher`), clipboard inspection chips (`[↵ Decode JWT]`), and instant drag-and-drop dispatch. |
| **Quiet Left Sidebar** | Fixed left panel (`200px`) | Understated category navigation (`Image`, `PDF`, `Archive`, `Developer`) and system utilities (`Settings`, `Storage & Engines`). Single click or `1`–`4` hotkeys switch active tools. |
| **Active Tool Canvas** | Center main viewport | Dedicated, uncluttered workspace for the selected utility. Strictly maintains the **1-Second Comprehension** pattern: Tool Title $\rightarrow$ Large Dropzone $\rightarrow$ 3-Preset Selector $\rightarrow$ Primary Action Button. |
| **Discreet CLI Dock** | Pinned canvas footer | Single quiet one-liner displaying the exact equivalent terminal command (`$ tb ...`) that updates dynamically in real-time as GUI parameters change, accompanied by a 1-click `[Copy]` button. |
| **Storage & Engines** | Sidebar $\rightarrow$ Settings | Visual disk usage inspector for downloaded optional static engines (`~/.toolbox/deps/`) with explicit manual uninstall buttons (strictly zero background auto-pruning). |

---

## 3. Voice and Tone

Microcopy is quiet, functional, and devoid of marketing chatter or engineering self-congratulation:

| Context | Do | Don't |
|---|---|---|
| **Empty Dropzone** | `"Drop image here or click to browse"` | `"Drag & drop your files to experience blistering SIMD optimization!"` |
| **File Loaded** | `"hero.png (4.2 MB)"` | `"Loaded 1 file successfully. Ready to compress."` |
| **Compression Preset** | `"[ Small ]  [ Balanced ]  [ Best ]"` | `"[ Ultra Lossy MozJPEG WebP ]  [ Standard 85% ]  [ Extreme AVIF ]"` |
| **Action Execution** | `"[ Compress ]"` | `"[ Execute Fast SIMD Optimization ]"` |
| **Completion Result** | `"Saved 80% (4.2 MB → 820 KB)"` | `"SUCCESS! You have saved disk space and bandwidth."` |
| **Error Feedback** | `"[ ERR: Corrupt PDF header. Attempt repair? ]"` | `"System Exception in tb-core::pdf::xref_table. File is invalid."` |

---

## 4. Component Patterns

### 4.1 Calm Dropzone
- **Empty State:** Clean 1px dashed rectangle with generous padding. Text reads `Drop [file type] here or click to browse`.
- **Drag-Over State:** Dashed border snaps to 1px solid white `{colors.border-focus}` with zero animation delay.
- **File Dropped State:** The dropzone collapses into a sleek 1px file summary card displaying filename, formatted size, and a subtle `[×]` remove button.

### 4.2 Three-Preset Selector
- Every conversion or compression tool presents exactly three simple presets:
  - `[ Small ]`: Maximum reduction, acceptable quality tradeoff.
  - `[ Balanced ]` (Default): Optimal balance of quality and size savings.
  - `[ Best ]`: Highest fidelity, lossless or visually indistinguishable.
- Selecting a preset is instantaneous (one click or hotkey `1`, `2`, `3`).

### 4.3 Action Button
- Primary actions use solid inverted high-contrast styling (`{colors.accent-primary}` background, `{colors.accent-foreground}` text).
- Placed immediately beneath the preset selector. Pressing `Enter` executes the primary action.

### 4.4 Live CLI Command Dock
- Positioned as a calm, single-line footer at the bottom of the canvas.
- Format: `$ tb <category> <action> [options] [file]` followed by `[Copy]`.
- Modifying presets or checkboxes dynamically updates the string without latency. Clicking `[Copy]` or pressing `Ctrl+Shift+C` copies the command and displays a brief `[Copied]` feedback tag.

---

## 5. State Patterns

Every tool screen follows five canonical state phases:

```
[ 1. Empty State ]  ──(File Dropped)──>  [ 2. File Configured ]
                                                 │
                                           (Click [Action] / Enter)
                                                 │
                                                 ▼
[ 5. Error State ]  <──(Failure)──  [ 3. Processing (Progress) ]
                                                 │
                                             (Success)
                                                 ▼
                                        [ 4. Result Done ]
```

1. **Empty State:** Zero clutter. Header, empty dropzone, quiet sidebar.
2. **File Configured:** Dropzone shows file chip, reveals 3-preset row, and focuses primary action button.
3. **Processing State:** Primary button changes to `[ Processing... ]` with a quiet 1px horizontal progress indicator beneath it. UI remains responsive at 120 FPS.
4. **Result Done:** Displays concrete metrics (`"Saved 80% (4.2 MB → 820 KB)"`), non-destructive output path (`"hero_compressed.png"`), and two actions: `[ Open File ]` and `[ Reveal in Folder ]`.
5. **Error State:** Non-modal, actionable notification strip with red accent border (`{colors.state-error}`), a human-readable diagnosis, and an inline recovery action (e.g. `[ Attempt Repair ]` or `[ Enter Password ]`).

---

## 6. Interaction Primitives & Keyboard Navigation

Toolbox is fully operable without touching the mouse:

- `Super+Shift+T`: Summon/dismiss Omni-Bar from anywhere in the OS (<50ms).
- `Ctrl+K`: Focus Omni-Bar from within Toolbox window.
- `Tab` / `Shift+Tab`: Cycle through interactive elements (Sidebar $\rightarrow$ Dropzone $\rightarrow$ Presets $\rightarrow$ Action Button $\rightarrow$ CLI Copy).
- `1`, `2`, `3`, `4`: Quick-switch primary tool suites (Image, PDF, Archive, Developer).
- `Enter`: Execute primary action button.
- `Escape`: Cancel current operation, clear active input, or close window.
- `Ctrl+C` (when CLI dock is focused): Copy CLI command.

---

## 7. Accessibility Floor

- **Contrast Floor:** All text tokens meet WCAG AAA contrast ratios (>7:1 against `{colors.canvas}` and `{colors.surface}`). Focus outlines use pure `#FFFFFF` against dark backgrounds (>15:1 ratio).
- **Keyboard Trapping Protection:** Focus cycles predictably without trapping; `Escape` always returns focus to the tool root.
- **Screen Reader Support:** Slint accessibility roles (`accessible-role: button`, `accessible-name: "..."`) attached to all interactive controls and dropzones.

---

## 8. Key Flow: Devon, Systems Engineer on Linux

1. **Arrival:** Devon receives a confidential 28MB PDF financial audit in an email on Arch Linux. Company policy strictly forbids uploading client data to cloud converters like Smallpdf.
2. **Summon:** Devon presses `Super+Shift+T`. The Omni-Bar appears instantly (<50ms).
3. **Drop & Dispatch:** Devon drags `audit_2026.pdf` from the email client onto the Omni-Bar. Toolbox automatically routes to the **PDF Compress** workbench.
4. **1-Second Comprehension:** Devon sees `PDF Compress`, the dropped file card `audit_2026.pdf (28.4 MB)`, and the preset row. `[ Balanced ]` is pre-selected.
5. **Execution:** Devon hits `Enter`. Rayon background workers optimize the vector streams and images completely on-device.
6. **Climax & Relief:** In 180ms, the screen updates to `"Saved 74% (28.4 MB → 7.4 MB) → audit_2026_compressed.pdf"`.
7. **CLI Continuity:** Devon notices the bottom dock: `$ tb pdf compress audit_2026.pdf --preset balanced`. Devon clicks `[Copy]` to add it to his daily deployment script. Zero data leaked, zero friction.
