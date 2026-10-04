---
stepsCompleted:
  - step-01-validate-prerequisites
  - step-02-design-epics
  - step-03-create-stories
  - step-04-final-validation
inputDocuments:
  - _bmad-output/planning-artifacts/PRD.md
  - _bmad-output/planning-artifacts/ARCHITECTURE-SPINE.md
  - _bmad-output/planning-artifacts/ARCHITECTURE-DESIGN.md
  - _bmad-output/planning-artifacts/ux-designs/ux-toolbox-2026-10-04/DESIGN.md
  - _bmad-output/planning-artifacts/ux-designs/ux-toolbox-2026-10-04/EXPERIENCE.md
---

# Toolbox (tb) - Epic Breakdown

## Overview

This document provides the complete epic and story breakdown for Toolbox (`tb`), decomposing the requirements from the PRD, Architecture Spine, and Solution Design into implementable, value-driven epics and stories.

---

## Requirements Inventory

### Functional Requirements

* **FR-1: Global Hotkey Summoning** — System-wide configurable hotkey (default: `Super+Shift+T`) summons or dismisses Toolbox on Wayland/X11 in <50ms with search auto-focused.
* **FR-2: Content-Aware Clipboard Routing & Proactive Focus Inspection** — Omni-Bar inspects clipboard contents upon window focus and suggests contextual micro-tools (e.g. JWT decode chip, JSON formatting chip) with instant `Enter` execution or `Tab` dismissal.
* **FR-3: Drag-and-Drop MIME Dispatch** — Dropping single or batch files onto the window automatically reveals matching tool actions (PDF merge/compress, image convert/resize, or extension search on unsupported MIME).
* **FR-4: Never-Hide Search Discovery & GUI-to-CLI Parity** — Fuzzy search indexes all tools regardless of engine download status; every GUI tool page provides an interactive, live `[Copy CLI Command]` button reflecting the current configuration.
* **FR-5: Non-Destructive File Output Behavior** — File conversion/compression saves to source directory with semantic suffix (`<name>_compressed.<ext>`) by default; GUI sequentially increments (`doc_compressed (1).pdf`) on collision while CLI prompts unless `--overwrite` / `-f` is passed.
* **FR-6: Pre-Flight Password Detection, Corruption Diagnostics & Actionable Healing** — Files are inspected before processing; encrypted PDFs trigger an upfront password modal; corrupted headers trigger inline diagnostics; recoverable damage offers 1-click repair (`[Attempt Non-Destructive Repair via QPDF]`).
* **FR-7: Subcommand Namespace Hierarchy** — Intuitive terminal CLI categorization: `tb <category> <action> [options] [files]` with formatted help on empty subcommands.
* **FR-8: Standard Input / Output Stream Piping & Dual Input Architecture** — Transparent `InputSource` pipeline allowing text/dev/data tools to stream `stdin` to `stdout` without seek errors, alongside zero-copy memory mapping for disk files.
* **FR-9: Machine-Readable JSON Output Flag** — All CLI commands support `--json` emitting structured envelope (`{"success": true, "data": {...}}` or `{"success": false, "error": {...}}`).
* **FR-10: PDF Manipulation Engine** — Pure local merge, split, compress (Low/Medium/High presets), and AES-256 password protection preserving vector elements and bookmarks.
* **FR-11: Image Conversion & Lossless Compression Engine** — Local conversion across PNG, JPG, WebP, AVIF, BMP, HEIC; lossless PNG compression via `oxipng`; hardware SIMD resizing (`fast_image_resize`); EXIF metadata stripping.
* **FR-12: Archive Extraction & Packaging** — Extraction of `.zip`, `.tar.gz`, `.tar.bz2`, `.tar.xz`, and creation of password-protected `.zip` archives.
* **FR-13: Structured Data Formatters & Converters** — Formatting, minification, and validation for JSON, YAML, XML with syntax error pointer indicators.
* **FR-14: Token, Hash & Cryptographic Utilities** — JWT claim decoding/verification, SIMD BLAKE3 / SHA-256 / MD5 hashing, Base64, and URL encoding/decoding.
* **FR-15: Developer Value Generators** — Instant local generation of UUID (v4, v7), ULID, Nanoid, and hash-based tokens with 1-click copy.
* **FR-16: Text Inspection, Diff & Regex** — Side-by-side graphical text diff, live regex pattern matcher with capture group highlighting, line sorters, and word/character counters.
* **FR-17: Host System Detection (Tier 1)** — Dependency resolver inspects `$PATH` for existing verified binaries (e.g. FFmpeg >= 4.4), utilizing host capabilities with 0MB download.
* **FR-18: Zero-Sudo User-Space Static Downloads (Tier 2)** — Missing heavy engines download precompiled static `musl` binaries into `~/.toolbox/deps/` with compile-time pinned SHA-256 verification and `chmod +x` (Flatpak runtimes resolved via extensions).
* **FR-19: Storage & Engines Dashboard (Tier 3)** — Dedicated dashboard displaying disk space utilized by downloaded engines and metadata (*"Last used: X days ago"*); explicit user-initiated uninstallation; strictly no automated background pruning.
* **FR-20: Subcommand Binary Extension Execution & Authenticated UDS Viewport** — Core detects standalone executables `tb-<ext>` in `~/.toolbox/bin/`, hosts an embedded Slint `<ExtensionViewport>`, and exposes an authenticated Unix Domain Socket Host API with session token `TB_IPC_AUTH_TOKEN` and permission scoping.
* **FR-21: Direct GitHub URL Installation & Sandboxing Verification** — Users install extensions via `tb ext install github.com/...` with manifest permission inspection and warning badges for third-party extensions.

---

### NonFunctional Requirements

* **NFR-1: Cold Start Latency** — <200ms cold start for terminal CLI commands; <350ms cold start for Slint GUI launch.
* **NFR-2: Memory Footprint** — <30MB baseline RAM for CLI operations; <60MB idle RAM for Slint GUI.
* **NFR-3: Frame Presentation & UI Responsiveness** — Rock-solid 120 FPS Wayland/X11 rendering; Slint render thread isolation with 16ms rate-limited background worker progress callbacks.
* **NFR-4: Single Binary Portability** — Single multi-call binary `tb` on disk; zero arguments or desktop launcher launches GUI; subcommands or piped stdin executes CLI.
* **NFR-5: 100% Offline Air-Gapped Core Execution** — Zero network socket libraries in `tb-core`; `tb doctor --network` CLI verification; air-gap verification badge in desktop footer.
* **NFR-6: Non-Destructive File Safety** — File reading guarded by advisory shared read locks (`flock(LOCK_SH)`) preventing `SIGBUS` panics; non-destructive suffix naming policy preserving source files.
* **NFR-7: Cross-Distro Compatibility** — Verified operation on Ubuntu, Fedora, Arch, Debian, openSUSE across Wayland and X11; software-render fallback on older Intel HD GPUs and VMs.

---

### Additional Requirements (from Architecture)

* **Starter / Greenfield Template**: Greenfield Cargo workspace containing:
  - `crates/tb` (root multi-call dispatcher binary)
  - `crates/tb-core` (pure domain library, 0 UI/CLI code)
  - `crates/tb-deps` (3-tier dependency engine)
  - `crates/tb-ext` (subprocess runner & UDS IPC server)
  - `crates/tb-cli` (Clap 4.5+ terminal adapter)
  - `crates/tb-ui` (Slint 1.8+ desktop GUI adapter)
* **Invariant AD-1 (Crate Boundaries)**: Strict boundary isolation; `tb-core` strictly forbidden from importing `slint` or `clap`.
* **Invariant AD-2 (Unified Dispatch)**: Single `tb` binary on disk inspecting `std::env::args()` and TTY status.
* **Invariant AD-3 (Zero-Copy I/O & Dual Pipeline)**: `memmap2` with `flock(LOCK_SH)` for files >1MB; `InputSource` dual pipeline for non-seekable piped `stdin` streams.
* **Invariant AD-4 (Work-Stealing Concurrency)**: Rayon `par_iter` work-stealing for batch operations across all CPU cores.
* **Invariant AD-5 (SIMD Acceleration)**: BLAKE3 (3–5 GB/s) and `fast_image_resize` (AVX2/NEON vectorization).
* **Invariant AD-6 (Render Thread Isolation)**: Slint event loop strictly non-blocking; 16ms rate-limited progress updates via `slint::invoke_from_event_loop()`.
* **Invariant AD-7 (Authenticated Extension Viewport)**: Embedded Slint viewport + JSON-RPC 2.0 over Unix Domain Socket (`$XDG_RUNTIME_DIR/toolbox.sock`) protected by `TB_IPC_AUTH_TOKEN`.
* **Invariant AD-8 (3-Tier Engine & Flatpak Compatibility)**: Host probe ➔ static musl download ➔ manual storage dashboard (Flatpak extension runtime fallback).
* **Invariant AD-9 (Non-Destructive Suffix Policy)**: Automatic `<name>_compressed.<ext>` naming with sequential increment on collision.
* **Consistency Conventions**: `tb-` prefix for crates, typed `thiserror` domain errors, `anyhow` presentation errors, standard exit codes (`0`, `1`, `2`, `3`, `130`), `--json` envelope, XDG filesystem base paths.

---

### UX Design Requirements (Embedded UX Specifications)

* **UX-DR1: Slint Omni-Bar Command Palette (`Super+Shift+T`)** — Auto-focused, keyboard-navigable command palette with instant fuzzy search (<16ms latency), category filtering, integrated drag-and-drop dropzone, and proactive focus clipboard detection chips (`[↵ Decode JWT]`, `[↵ Prettify JSON]`).
* **UX-DR2: Omarchy Theme Engine & All-Monospace Brutalism** — `crates/tb-ui/ui/theme.slint` design token singleton enforcing strict `0px` border-radius (`radius-none`), pure monospace typography (`ui-monospace, "JetBrains Mono", monospace`), 1px crisp borders, zero gradients, and dynamic runtime switching across all 10 Omarchy palettes (Default: Industrial Graphite `#141618`).
* **UX-DR3: `slintcn` Boxy Component Foundation** — In-repo `crates/tb-ui/ui/slintcn/` copy-paste components (`Button`, `Card`, `Input`, `Dialog`, `Badge`, `Dropzone`) bound to `Theme` tokens with high-contrast, instantly recognizable visual affordances.
* **UX-DR4: Model A Dual-Pane Workbench Shell** — Quiet `200px` left sidebar navigation (`Image`, `PDF`, `Archive`, `Developer`, `Settings`) paired with a single-task focused main canvas, `1`–`4` category hotkeys, and full keyboard navigation.
* **UX-DR5: 1-Second Comprehension & 3-Preset Action Flow** — Primary tool screens enforce radical visual reduction (zero cockpit badges or marketing clutter): Tool Header $\rightarrow$ Calm Large Dropzone $\rightarrow$ 3-Preset Row (`[ Small ]  [ Balanced ]  [ Best ]`) $\rightarrow$ Primary Inverted Action Button (`[ Compress ]` / `[ Convert ]`).
* **UX-DR6: Discreet Live GUI-to-CLI Dock** — Pinned single-line footer on every tool screen displaying the exact terminal command (`$ tb ...`) with real-time parameter synchronization and a 1-click `[Copy]` button.
* **UX-DR7: Five Canonical State Transitions & Actionable Inline Error Strips** — Standardized tool screen state pipeline: `Empty State` $\rightarrow$ `Configured State` $\rightarrow$ `Processing State` $\rightarrow$ `Result Done State` (with byte savings & open actions) $\rightarrow$ non-modal, inline actionable error strips (e.g. `[Attempt Non-Destructive Repair via QPDF]`).
* **UX-DR8: Embedded Extension Viewport Container** — Reserved Slint `<ExtensionViewport>` panel with standardized loading skeleton, title header, and extension heartbeat monitor.
* **UX-DR9: Storage & Engines Dashboard View** — Visual storage management screen in Settings displaying installed static engines, disk usage bars, *"Last used: X days ago"* metadata, air-gap status indicator, and explicit manual uninstall buttons.

---

### FR Coverage Map

* **FR-1 (Global Hotkey Summoning)**: Epic 13 (Omni-Bar Command Palette & Proactive Clipboard)
* **FR-2 (Content-Aware Clipboard Routing)**: Epic 13 (Omni-Bar Command Palette & Proactive Clipboard)
* **FR-3 (Drag-and-Drop MIME Dispatch)**: Epic 13 (Omni-Bar Command Palette & Proactive Clipboard)
* **FR-4 (Never-Hide Search & GUI-to-CLI Parity)**: Epic 2 (Image Convert - CLI Parity) & Epic 13 (Omni-Bar Search)
* **FR-5 (Non-Destructive Suffix Policy)**: Epic 2 (Image Convert), Epic 3 (Image Compress), Epic 4 (PDF Compress), Epic 5 (PDF Merge/Split), Epic 7 (Archive)
* **FR-6 (Pre-Flight Diagnostics & Actionable Healing)**: Epic 6 (PDF Encryption, Password Inspection & Repair)
* **FR-7 (Subcommand Namespace Hierarchy)**: Epic 1 (Workspace Infrastructure & CLI Dispatcher)
* **FR-8 (Standard Stream Piping & Dual Input)**: Epic 8 (JSON Formatter & Stream Piping)
* **FR-9 (Machine-Readable JSON Output Flag)**: Epic 1 (Workspace Infrastructure & CLI Dispatcher)
* **FR-10 (PDF Manipulation Engine)**: Epic 4 (PDF Compress), Epic 5 (PDF Merge/Split), Epic 6 (PDF Encryption)
* **FR-11 (Image Conversion & Lossless Compression Engine)**: Epic 2 (Image Format Conversion & SIMD Resize), Epic 3 (Lossless Compression & EXIF Stripping)
* **FR-12 (Archive Extraction & Packaging)**: Epic 7 (Archive Manager)
* **FR-13 (Structured Data Formatters & Converters)**: Epic 8 (JSON Formatter) & Epic 12 (YAML/XML & Text Tools)
* **FR-14 (Token, Hash & Cryptographic Utilities)**: Epic 9 (JWT Debugger), Epic 10 (SIMD Cryptographic Hashing), Epic 11 (Encoding Utilities)
* **FR-15 (Developer Value Generators)**: Epic 11 (Developer ID & Token Generators)
* **FR-16 (Text Inspection, Diff & Regex)**: Epic 12 (Visual Text Diff, Line Tools & Regex Tester)
* **FR-17 (Host System Detection - Tier 1)**: Epic 14 (3-Tier Heavy Dependency Engine & Video Transcoder)
* **FR-18 (Zero-Sudo Static Downloads - Tier 2)**: Epic 14 (3-Tier Heavy Dependency Engine & Video Transcoder)
* **FR-19 (Storage & Engines Dashboard - Tier 3)**: Epic 15 (Storage Transparency & Engine Dashboard)
* **FR-20 (Extension Subcommands & Authenticated UDS Viewport)**: Epic 16 (Sandboxed Extension Viewport & Authenticated UDS Host API)
* **FR-21 (Direct GitHub URL Installation & Sandboxing)**: Epic 16 (Sandboxed Extension Viewport & Authenticated UDS Host API)

---

## Epic List

### Epic 1: Workspace Scaffolding, Multi-Call Dispatcher & Slint Window Shell
Establish the foundational 6-crate Hexagonal Cargo workspace (`tb`, `tb-core`, `tb-cli`, `tb-ui`, `tb-deps`, `tb-ext`), the single `tb` multi-call binary entry point (inspecting CLI args vs TTY to launch Slint GUI or Clap CLI), basic window framing with dark theme, and standardized `--json` output envelope.
**User Outcome:** Users can launch `tb` with no args to see a native Slint window shell, or run `tb --help` / `tb --version` in terminal.
**FRs covered:** FR-7, FR-9, NFR-1, NFR-2, NFR-4, NFR-7.

### Epic 2: Image Format Conversion & Hardware SIMD Resizing
Build the standalone image conversion tool in `tb-core` supporting PNG, JPG, WebP, AVIF, and BMP format transcoding with hardware SIMD image resizing (`fast_image_resize` with AVX2/NEON), exposed via `tb image convert` and an interactive Slint GUI dropzone with live `[Copy CLI Command]` parity.
**User Outcome:** Users can drag a PNG and convert to WebP at 1920x1080 via GUI or run `tb image convert --format webp --resize 1920x1080 input.png` with non-destructive `<name>_converted.webp` output.
**FRs covered:** FR-4 (CLI Parity), FR-5, FR-7, FR-9, FR-11 (convert/resize), NFR-6, UX-DR3.

### Epic 3: Lossless Image Compression & EXIF Stripping
Build lossless PNG compression using `oxipng`, MozJPEG quality optimization, and EXIF metadata stripping in `tb-core`, exposed via `tb image compress` and Slint GUI view with a before/after byte savings badge ("Saved 65%").
**User Outcome:** Users can optimize screenshots and strip camera/GPS metadata locally without losing image quality.
**FRs covered:** FR-5, FR-7, FR-11 (compress/strip EXIF), NFR-6.

### Epic 4: PDF Compression & Optimization
Build the standalone PDF compression tool in `tb-core` providing 3 distinct optimization presets (Low: lossless structure cleanup, Medium: 150 DPI downsampling, High: 72 DPI downsampling) with byte savings indicators, non-destructive `_compressed.pdf` output, and CLI `tb pdf compress`.
**User Outcome:** Users can compress a 30MB PDF report down to 4MB for email attachments with one click or command.
**FRs covered:** FR-5, FR-7, FR-10 (compress), NFR-6.

### Epic 5: PDF Merge, Page Splitting & Reordering
Build multi-document PDF merging, page-range extraction, and visual page reordering in `tb-core`, exposed via `tb pdf merge a.pdf b.pdf -o out.pdf` and `tb pdf split input.pdf --pages 1-3,5`, with Slint drag-and-drop page thumbnail tiles.
**User Outcome:** Users can combine multiple scanned documents or extract specific page ranges offline.
**FRs covered:** FR-5, FR-7, FR-10 (merge/split).

### Epic 6: PDF Encryption, Upfront Password Inspection & 1-Click Repair
Build AES-256 PDF encryption/decryption, pre-flight header validation that prompts for password *upfront before submission*, and 1-click healing (`[Attempt Non-Destructive Repair via QPDF]`) for files with broken EOF/Xref tables.
**User Outcome:** Users encrypt sensitive documents or unlock encrypted files without trial-and-error crashes, with automatic healing for damaged files.
**FRs covered:** FR-6, FR-10 (password), UX-DR4.

### Epic 7: Archive Manager (Extraction & Packaging)
Implement local archive decompression for `.zip`, `.tar.gz`, `.tar.bz2`, `.tar.xz`, and creation of password-protected `.zip` archives with non-destructive extraction directories, exposed via `tb archive extract` and `tb archive create`.
**User Outcome:** Users can extract or package multi-file archives directly from terminal or GUI dropzone.
**FRs covered:** FR-5, FR-7, FR-12.

### Epic 8: JSON Formatter, Minifier & Stream Piping
Implement high-performance JSON formatting, minification, and syntax error validation using `serde_json`, with dual-input pipeline (`InputSource`) supporting Unix `stdin`/`stdout` piping (`cat raw.json | tb dev json format`) and Slint GUI code editor with line numbers.
**User Outcome:** Developers can instantly prettify, minify, and inspect malformed JSON with exact syntax line/column pointer indicators.
**FRs covered:** FR-7, FR-8, FR-9, FR-13 (JSON).

### Epic 9: JWT Debugger & Token Inspector
Implement JWT token decoder and claim validator in `tb-core`, parsing header, payload, and signature without network transmission, calculating real-time expiration countdowns, exposed via `tb dev jwt decode` and Slint GUI token inspector with color-coded syntax.
**User Outcome:** Developers can paste and inspect authentication tokens offline with zero risk of token leakage.
**FRs covered:** FR-7, FR-8, FR-9, FR-14 (JWT).

### Epic 10: SIMD Cryptographic Hashing & File Integrity Checker
Implement SIMD-accelerated BLAKE3 (3–5 GB/s), SHA-256, SHA-512, and MD5 hashing over zero-copy memory maps (`memmap2` with `flock(LOCK_SH)`), with file checksum verification against `.sha256` files, exposed via `tb dev hash` and Slint GUI drag-and-drop hash calculator.
**User Outcome:** Developers can verify ISO/file download integrity in seconds and calculate multiple hashes simultaneously.
**FRs covered:** FR-7, FR-8, FR-9, FR-14 (Hashes), NFR-6.

### Epic 11: Developer ID, Token & Data Encoding Utilities
Implement local generators for UUID (v4, v7), ULID, Nanoid, alongside Base64 and URL encoding/decoding with 1-click clipboard copy, exposed via `tb dev uuid` and `tb dev base64`.
**User Outcome:** Developers generate unique identifiers and encode/decode strings without leaving the app or opening a browser.
**FRs covered:** FR-7, FR-8, FR-14 (Base64/URL), FR-15 (UUID/ULID).

### Epic 12: Visual Text Diff, Line Tools & Regex Tester
Implement side-by-side graphical text comparison with word/line diff highlighting, live regex pattern matcher with real-time capture group visualization, and text line deduplication/sorting, exposed via `tb dev diff` and `tb dev regex`.
**User Outcome:** Developers can compare code snippets, test complex regular expressions, and clean up messy lists.
**FRs covered:** FR-7, FR-13 (YAML/XML), FR-16.

### Epic 13: Omni-Bar Command Palette & Proactive Clipboard Engine
Build the unified top-level Omni-bar with global hotkey (`Super+Shift+T`), sub-16ms fuzzy tool search, and proactive focus clipboard detection that suggests 1-click execution chips (e.g. JWT decode chip or JSON prettify chip upon window focus).
**User Outcome:** Users summon Toolbox from any workspace with a keyboard shortcut and execute utilities without manual navigation or pasting.
**FRs covered:** FR-1, FR-2, FR-3, FR-4 (Search), UX-DR1, UX-DR2.

### Epic 14: 3-Tier Heavy Dependency Engine & Video Transcoder
Implement the 3-tier dependency resolution engine in `tb-deps` (Tier 1 host `$PATH` probe, Tier 2 user-space static `musl` binary downloader to `~/.toolbox/deps/` with compile-time pinned SHA-256 verification, zero sudo), and ship Video Transcoding (MKV/MP4/WebM) via FFmpeg.
**User Outcome:** Users can convert video files without needing root permissions or system package managers, with zero download if host FFmpeg already exists.
**FRs covered:** FR-7, FR-17, FR-18.

### Epic 15: Storage Transparency & Engine Management Dashboard
Build the dedicated Storage & Engines Dashboard in Slint and CLI (`tb storage`), displaying disk space consumed by downloaded engines, *"Last used: X days ago"* tracking, explicit 1-click uninstallation (strictly zero auto-pruning), and the Air-Gap Verification badge (`tb doctor --network`).
**User Outcome:** Users have 100% control over disk footprint and verifiable proof of zero network telemetry.
**FRs covered:** FR-19, NFR-5, UX-DR6, UX-DR7.

### Epic 16: Sandboxed Extension Viewport & Authenticated UDS Host API
Implement the external subcommand runner (`tb-<ext>`) in `tb-ext`, the embedded Slint `<ExtensionViewport>` (iframe equivalent), the authenticated Unix Domain Socket IPC server (`$XDG_RUNTIME_DIR/toolbox.sock`) with one-time `TB_IPC_AUTH_TOKEN` handshakes and permission scoping, and GitHub URL extension installation (`tb ext install`).
**User Outcome:** Community developers can write plugins in any language that run in isolated child processes and render within Toolbox's UI frame.
**FRs covered:** FR-20, FR-21, UX-DR5.

---

## Epic 1: Workspace Scaffolding, Multi-Call Dispatcher & Slint Window Shell

**Epic Goal:** Establish the foundational 6-crate Hexagonal Cargo workspace, the unified multi-call binary `tb` entry point, the base hardware-accelerated Slint desktop window shell with software-render fallback, Clap CLI adapter with `--json` envelopes, and Linux desktop launcher integration.

### Story 1.1: Cargo Workspace Initialization & Hexagonal Crate Boundaries

As a developer,
I want a clean 6-crate Cargo workspace with strictly enforced boundary isolation,
So that domain logic, CLI adapters, and GUI code never leak into each other and compile reliably.

**Acceptance Criteria:**

**Given** a clean project repository
**When** `cargo check --workspace` is executed
**Then** all 6 crates (`tb`, `tb-core`, `tb-cli`, `tb-ui`, `tb-deps`, `tb-ext`) compile with zero warnings
**And** `crates/tb-core/Cargo.toml` contains zero dependencies on `slint` or `clap`
**And** the root `Cargo.toml` defines a release profile with `opt-level = "z"`, `lto = "fat"`, `codegen-units = 1`, `panic = "abort"`, and `strip = true`.

---

### Story 1.2: Unified Multi-Call Binary Entry Point & Mode Dispatcher

As a Linux user,
I want a single executable `tb` that automatically opens the desktop GUI when launched from an app launcher and runs CLI commands when given arguments,
So that I have one single tool installed for both terminal and graphical use.

**Acceptance Criteria:**

**Given** `tb` is invoked in a terminal with subcommands or arguments (e.g. `tb --help`, `tb --version`)
**When** the binary executes
**Then** it dispatches immediately to `tb-cli` in <50ms without initializing Slint GUI libraries
**And** when `tb` is invoked with zero arguments in an active graphical session (`$WAYLAND_DISPLAY` or `$DISPLAY`), it launches the `tb-ui` Slint event loop
**And** when `tb` is invoked with zero arguments in a headless terminal without a display server, it outputs formatted CLI help to stderr and exits with exit code 2.

---

### Story 1.3: Standardized CLI JSON Envelope & Error Taxonomy

As a developer or automation script,
I want all CLI subcommands to support `--json` and emit structured success/error envelopes with actionable diagnostics,
So that tools and local agents can parse results deterministically without scraping terminal text.

**Acceptance Criteria:**

**Given** any CLI subcommand executed with the `--json` flag
**When** the command completes successfully
**Then** stdout outputs a valid JSON envelope: `{"success": true, "data": {...}}` and exits with code 0
**And** when an error occurs with `--json`, stdout outputs `{"success": false, "error": {"code": "...", "message": "...", "suggested_action": "..."}}` and exits with code 1
**And** when executed without `--json` on failure, stderr outputs colored human-readable text explaining the root cause and a suggested next command
**And** if `NO_COLOR` is set or `TERM=dumb`, terminal output strictly strips ANSI color escape codes and uses clean ASCII status labels (`[OK]`, `[FAIL]`).

---

### Story 1.4: Base Slint Window Shell, Dual-Pane Workbench & Omarchy Theme Engine

As a desktop user,
I want a native, hardware-accelerated, boxy brutalist window with all-monospace typography, quiet sidebar navigation, and dynamic Omarchy theme support,
So that Toolbox opens in <350ms, provides immediate 1-second task clarity, and matches my Linux terminal aesthetic.

**Acceptance Criteria:**

**Given** `tb` is launched in GUI mode
**When** the main window appears
**Then** cold-start time to first frame presentation is under 350ms with baseline idle memory under 60MB RAM
**And** when running without Vulkan/OpenGL acceleration, it automatically falls back to Slint's software renderer (`femtovg-software`) at 60Hz without crashing
**And** `crates/tb-ui/ui/theme.slint` defines global `Theme` tokens with strict `0px` border-radius (`radius-none`), pure monospace typography (`ui-monospace, "JetBrains Mono", monospace`), and runtime palette switching across all 10 Omarchy themes (Default: Industrial Graphite `#141618`, Vantablack, Tokyo Night, Nord, Gruvbox, Hackerman, Kanagawa, Catppuccin Mocha, Rose Pine, Everforest)
**And** `crates/tb-ui/ui/slintcn/` integrates `slintcn` copy-paste components (`Button`, `Card`, `Input`, `Dialog`, `Badge`, `Dropzone`) bound to `Theme` tokens with 0px boxy borders and 1px crisp lines
**And** the window implements the Model A Dual-Pane Workbench shell matching `mockups/minimal-workbench.html`: quiet 200px left sidebar (`Image`, `PDF`, `Archive`, `Developer`, `Settings`) and centered single-task canvas
**And** pressing `Escape` or the window close button cleanly terminates the process with exit code 0.

---

### Story 1.5: Linux Desktop Integration & Application Registration

As a desktop user,
I want Toolbox to appear in my Linux system application menu with an official icon and desktop file,
So that I can search for and launch Toolbox using the system Super key like any native app.

**Acceptance Criteria:**

**Given** the application is installed on the system
**When** inspecting `~/.local/share/applications/toolbox.desktop`
**Then** the desktop entry contains valid `Name=Toolbox`, `Exec=tb`, `Icon=toolbox`, `Categories=Utility;`, and `StartupWMClass=toolbox`
**And** the scalable SVG icon is installed in `~/.local/share/icons/hicolor/scalable/apps/toolbox.svg`
**And** clicking the application icon in GNOME Shell or KDE Plasma immediately opens the Toolbox window.

---

## Epic 2: Image Format Conversion & Hardware SIMD Resizing

**Epic Goal:** Build the standalone image conversion tool in `tb-core` supporting PNG, JPG, WebP, AVIF, and BMP transcoding with hardware SIMD image resizing (`fast_image_resize` with AVX2/NEON), exposed via `tb image convert` and an interactive Slint GUI dropzone with live `[Copy CLI Command]` parity and Rayon batching.

### Story 2.1: Image Engine Transcoding via ImageMagick & SIMD Resize in `tb-core`

As a developer,
I want an image engine that wraps ImageMagick (`magick`) for universal format transcoding (PNG, JPG, WebP, AVIF, BMP, HEIC) and utilizes `fast_image_resize` for CPU SIMD vector resizing,
So that image transformations leverage proven Linux CLI tools, are blazingly fast, and memory-safe.

**Acceptance Criteria:**

**Given** an input image file (PNG, JPG, WebP, AVIF, BMP, or HEIC)
**When** format conversion is requested with an optional target dimension (e.g. 1920x1080)
**Then** `tb-core` invokes `magick` (or in-memory `fast_image_resize` with AVX2 on x86_64, NEON on ARM64) to downscale and convert a 4K image in under 100ms
**And** the output is saved to the source directory as `<name>_converted.<ext>` by default
**And** if the source directory is read-only, it falls back to saving in `~/Downloads/<name>_converted.<ext>` with an advisory notice
**And** if `<name>_converted.<ext>` already exists, it auto-increments to `<name>_converted (1).<ext>` without overwriting
**And** `ImageConvertOptions` implements a deterministic `to_cli_args()` method for CLI command parity.

---

### Story 2.2: CLI Image Convert Subcommand (`tb image convert`)

As a shell user,
I want a `tb image convert` CLI subcommand supporting format flags, quality levels, resize options, and `--json` output,
So that I can automate image resizing and conversion in bash scripts or CI pipelines.

**Acceptance Criteria:**

**Given** a valid image file on disk
**When** executing `tb image convert photo.png --format webp --quality 85 --resize 1920x1080`
**Then** it converts the image and outputs a success summary with output path and byte savings
**And** when executed with `--json`, stdout prints a structured envelope with dimensions, bytes before/after, and duration
**And** when given a corrupted or truncated image file, it exits with code 1 and outputs an inline diagnostic identifying the damaged header.

---

### Story 2.3: Slint GUI Drag-and-Drop View with 1-Second 3-Preset Flow & Live CLI Dock

As a desktop user,
I want a calm, uncluttered drag-and-drop image converter view with a 3-preset selector and a discreet live CLI command dock,
So that I can convert images in 1 second without cognitive overload and copy the exact terminal command.

**Acceptance Criteria:**

**Given** the Image Converter view in the Slint GUI matching `mockups/minimal-workbench.html`
**When** the screen is opened, it displays a calm 1px dashed dropzone: `"Drop image here or click to browse"` with zero technical cockpit clutter
**And** when an image is dropped, it transitions immediately to the File Configured state: displays filename and size chip (e.g. `photo.png (4.2 MB)`), reveals the 3-preset row (`[ Small ]`, `[ Balanced ]`, `[ Best ]`), and focuses the primary `[ Convert ]` button
**And** selecting a preset live-updates the discreet single-line CLI dock footer: `$ tb image convert photo.png --format webp --preset balanced`
**And** clicking `[Copy]` on the CLI dock copies the command to the system clipboard and flashes `[Copied]`
**And** clicking `[ Convert ]` or hitting `Enter` spawns a background worker thread, displays a 1px progress indicator, and transitions to the Result Done state with byte savings and `[ Open File ]` action.

---

### Story 2.4: Rayon Work-Stealing Multi-File Batch Conversion

As a photographer or web developer,
I want to convert multiple images in parallel across all available CPU cores without freezing the user interface or running out of RAM,
So that I can process hundreds of assets in seconds.

**Acceptance Criteria:**

**Given** multiple image files passed in CLI (`tb image convert *.png --format webp`) or dropped into the GUI
**When** batch conversion begins
**Then** work is distributed across CPU worker threads via `rayon::prelude::par_iter`
**And** batch concurrency is bounded by available system RAM (`min(num_cpus, available_ram / 256MB)`) to prevent OOM termination
**And** GUI updates progress smoothly at 60–120 FPS without dropped frames, showing individual file progress and overall completion percentage.

---

## Epic 3: Lossless Image Compression & EXIF Stripping

**Epic Goal:** Build lossless PNG compression using `oxipng`, MozJPEG quality optimization, and EXIF metadata stripping in `tb-core`, exposed via `tb image compress` and a Slint GUI view with a before/after byte savings badge ("Saved 65%").

### Story 3.1: Lossless PNG (`oxipng`) & JPEG Optimization Engine in `tb-core`

As a privacy-conscious user,
I want to compress images and strip private EXIF/GPS metadata locally without altering image quality,
So that I can safely share photos and screenshots on the web without leaking my location or camera serial number.

**Acceptance Criteria:**

**Given** an input PNG image
**When** lossless compression is executed
**Then** `oxipng` strips redundant metadata chunks and optimizes IDAT streams with zero pixel degradation
**And** when EXIF stripping is enabled, all GPS coordinates, camera serial numbers, and creation timestamps are eliminated
**And** when an image is already maximally compressed, the engine detects zero savings and avoids producing a larger output file.

---

### Story 3.2: CLI Image Compress Subcommand (`tb image compress`)

As a shell user,
I want a `tb image compress` command supporting optimization levels, EXIF stripping, and `--json` statistics,
So that I can automate website asset optimization and privacy scrubbing from my terminal.

**Acceptance Criteria:**

**Given** a valid image file on disk
**When** executing `tb image compress photo.jpg --level 4 --strip-exif`
**Then** the output is saved as `photo_compressed.jpg` and terminal prints byte reduction metrics
**And** when executed with `--json`, stdout prints `{"original_bytes": 1048576, "compressed_bytes": 352100, "savings_pct": 66.4}`
**And** if `photo_compressed.jpg` already exists, CLI prompts for confirmation unless `--overwrite` is specified.

---

### Story 3.3: Slint GUI Compression View with Before/After Savings Badge

As a desktop user,
I want an interactive compression view in the Slint GUI showing visual before/after file sizes and a 1-click reveal button,
So that I get instant feedback on how much storage space I saved.

**Acceptance Criteria:**

**Given** the Image Compressor view in the Slint GUI
**When** an image is dragged and dropped onto the dropzone
**Then** the user can adjust the compression level slider and toggle EXIF stripping
**And** clicking "Compress" triggers background optimization with a progress spinner
**And** on completion, an animated green badge displays *"Saved XX% (X.X MB ➔ Y.Y MB)"* alongside a `[Reveal in Folder]` button.

---

## Epic 4: PDF Compression & Optimization

**Epic Goal:** Build the standalone PDF compression tool in `tb-core` providing 3 distinct optimization presets (Low: lossless structure cleanup, Medium: 150 DPI downsampling, High: 72 DPI downsampling) with byte savings indicators, non-destructive `_compressed.pdf` output, and CLI `tb pdf compress`.

### Story 4.1: PDF Optimization & Compression via `qpdf` in `tb-core`

As a user with large documents,
I want to compress bloated PDF files locally via `qpdf` without sending my confidential records to a remote web server,
So that I can shrink file sizes for email attachments while preserving document security and vector fidelity.

**Acceptance Criteria:**

**Given** a multi-page PDF document
**When** compression is executed using the `qpdf` backend
**Then** `tb-core` invokes `qpdf` with `--linearize` and `--object-streams=generate` to optimize cross-references and compress uncompressed streams
**And** all vector elements, hyperlinks, forms, and text searchability remain completely intact
**And** the output file is saved to the source directory as `<name>_compressed.pdf` by default (auto-incrementing sequentially on collision)
**And** if the source directory is read-only, it falls back to `~/Downloads/<name>_compressed.pdf` with an explanatory notification.

---

### Story 4.2: CLI PDF Compress Subcommand (`tb pdf compress`)

As a terminal power user,
I want a `tb pdf compress` subcommand supporting presets, output overrides, and `--json` statistics,
So that I can batch-compress documents via shell scripts.

**Acceptance Criteria:**

**Given** a valid PDF file on disk
**When** executing `tb pdf compress report.pdf --preset balanced`
**Then** `report_compressed.pdf` is generated via `qpdf` and terminal prints original size, compressed size, and percentage reduction
**And** when executed with `--json`, stdout prints `{"pages": 12, "original_bytes": 15420100, "compressed_bytes": 3120500, "savings_pct": 79.8}`
**And** when given a password-encrypted PDF, it exits with code 1 and explicitly prompts that the document is password-protected.

---

### Story 4.3: Slint GUI PDF Compression View with 1-Second 3-Preset Flow & Live CLI Dock

As a desktop user,
I want a calm, uncluttered PDF compression view with a 3-preset row (`[ Small ]`, `[ Balanced ]`, `[ Best ]`) and live CLI dock,
So that I can compress PDFs in 1 second and copy the exact terminal command.

**Acceptance Criteria:**

**Given** the PDF Compressor view in the Slint GUI matching `mockups/minimal-workbench.html`
**When** a PDF document is dragged and dropped onto the dropzone
**Then** the screen reveals the file card (filename and size), presents the 3 presets (`[ Small ]`, `[ Balanced ]`, `[ Best ]`), and focuses the primary `[ Compress ]` button
**And** changing presets live-updates the single-line footer CLI dock: `$ tb pdf compress document.pdf --preset balanced`
**And** clicking `[Copy]` on the CLI dock copies the command to the system clipboard
**And** clicking `[ Compress ]` or hitting `Enter` spawns background worker execution, updates progress at 60–120 FPS, and transitions to the Result Done state showing `"Saved XX%"` with an `[ Open File ]` button.

## Epic 5: PDF Merge, Page Splitting & Reordering

**Epic Goal:** Build multi-document PDF merging, page-range extraction, and visual page reordering in `tb-core`, exposed via `tb pdf merge a.pdf b.pdf -o out.pdf` and `tb pdf split input.pdf --pages 1-3,5`, with Slint drag-and-drop page thumbnail tiles.

### Story 5.1: Multi-Document PDF Merge & Page Extraction Engine in `tb-core`

As a researcher or administrative user,
I want to concatenate multiple PDF documents into a unified file and extract arbitrary page ranges,
So that I can organize multi-part documents locally without cloud upload limits.

**Acceptance Criteria:**

**Given** two or more valid PDF files on disk
**When** merge is executed
**Then** documents are combined in specified order, preserving all vector paths, text layers, and embedded links
**And** when page extraction is executed with a page range (e.g. `1-3,5,8-10`), only the requested pages are saved to the destination
**And** output files default to non-destructive naming (`<first_file>_merged.pdf` or `<file>_pages_1-3.pdf`).

---

### Story 5.2: CLI PDF Merge & Split Subcommands (`tb pdf merge`, `tb pdf split`)

As a shell user,
I want CLI commands `tb pdf merge` and `tb pdf split` supporting glob patterns, page range syntax, and `--json` stats,
So that I can automate document assembly in scripts.

**Acceptance Criteria:**

**Given** multiple PDF files passed as arguments (`tb pdf merge doc1.pdf doc2.pdf doc3.pdf -o combined.pdf`)
**When** the command executes
**Then** `combined.pdf` is created and terminal reports total pages merged
**And** when executing `tb pdf split document.pdf --pages 1-5`, pages 1 to 5 are extracted to `document_split.pdf`
**And** when an invalid or inverted page range is specified (e.g. `--pages 5-2`, `--pages 0`, or `--pages 9999-10000` exceeding document length), the command exits with code 1 and outputs an inline diagnostic identifying the invalid range boundary
**And** when executed with `--json`, stdout prints a structured envelope with page counts and output paths.

---

### Story 5.3: Slint GUI Drag-and-Drop Page Tile Visualizer & Reorder

As a desktop user,
I want a visual grid of document page thumbnail cards that I can reorder, delete, or merge by dragging,
So that I can assemble PDF presentations visually.

**Acceptance Criteria:**

**Given** the PDF Merge view in the Slint GUI
**When** multiple PDF files are dropped onto the window
**Then** rendered thumbnail tiles appear for each page/document in sequence
**And** the user can drag tiles to reorder them or click an `[X]` badge to remove a page
**And** clicking "Merge All" outputs the assembled document with a success toast and `[Reveal in Folder]` action.

---

## Epic 6: PDF Encryption, Upfront Password Inspection & 1-Click Repair

**Epic Goal:** Build AES-256 PDF encryption/decryption, pre-flight header validation that prompts for password *upfront before submission*, and 1-click healing (`[Attempt Non-Destructive Repair via QPDF]`) for files with broken EOF/Xref tables.

### Story 6.1: AES-256 PDF Encryption & Decryption Engine in `tb-core`

As a privacy-conscious user,
I want to encrypt sensitive documents with AES-256 passwords and decrypt locked files locally,
So that financial statements and contracts are securely protected.

**Acceptance Criteria:**

**Given** a PDF document and a user-provided passphrase
**When** encryption is executed
**Then** the document is encrypted using standard AES-256 cipher preventing unauthorized opening
**And** when decrypting with the correct password, a fully unlocked PDF is produced
**And** when decrypting with an incorrect password, a typed `TbError::InvalidPassword` is returned without data corruption.

---

### Story 6.2: Pre-Flight Password Detection Modal & 1-Click Repair in Slint GUI

As a desktop user,
I want Toolbox to detect password-protected or corrupted PDFs immediately upon dropping them,
So that I am prompted for the password upfront or offered a 1-click repair instead of an ugly crash.

**Acceptance Criteria:**

**Given** an encrypted PDF file dropped onto the Toolbox window
**When** pre-flight inspection runs
**Then** a dedicated password dialog opens *immediately before submission*, allowing the user to enter the passphrase upfront
**And** when a structurally corrupted file is dropped (missing EOF or broken XRef), an inline diagnostic card appears: *"Damaged PDF structure detected"*
**And** the card provides a 1-click healing button: `[Attempt Non-Destructive Repair via QPDF]` which repairs the document structure in place without data loss.

---

### Story 6.3: CLI PDF Protect & Unlock Subcommands (`tb pdf protect`, `tb pdf unlock`)

As a terminal user,
I want `tb pdf protect` and `tb pdf unlock` subcommands supporting password flags and piped input,
So that I can automate batch document encryption in shell scripts.

**Acceptance Criteria:**

**Given** a valid PDF file on disk
**When** executing `tb pdf protect statement.pdf --password "secret123"`
**Then** `statement_protected.pdf` is generated with AES-256 encryption
**And** executing `tb pdf unlock statement_protected.pdf --password "secret123"` restores the unencrypted file
**And** executing `tb pdf unlock` without a password flag interactively prompts for the password via masked terminal stdin.

---

## Epic 7: Archive Manager (Extraction & Packaging)

**Epic Goal:** Implement local archive decompression for `.zip`, `.tar.gz`, `.tar.bz2`, `.tar.xz`, and creation of password-protected `.zip` archives with non-destructive extraction directories, exposed via `tb archive extract` and `tb archive create`.

### Story 7.1: Multi-Format Archive Extraction & Packaging via `7-Zip` in `tb-core`

As a Linux user,
I want a unified archive engine that wraps `7-Zip` (`7z`) to extract tarballs (`.tar.gz`, `.tar.bz2`, `.tar.xz`), `.zip`, and `.7z`, and packages AES-256 password-protected archives,
So that I don't have to memorize different tar command flags or install separate zip/unzip tools.

**Acceptance Criteria:**

**Given** an archive file (`.zip`, `.tar.gz`, `.tar.bz2`, `.tar.xz`, or `.7z`)
**When** extraction is triggered
**Then** `tb-core` invokes `7z x` to extract all contents into a dedicated non-destructive directory `<archive_name>_extracted/`
**And** when creating an archive with a password, `7z a -p<pass>` applies AES-256 standard encryption
**And** malicious path-traversal entries (`../` or absolute root paths) are strictly sanitized against the target folder via canonical prefix matching, rejecting any entry that escapes `<archive_name>_extracted/` (Zip Slip vulnerability protection)
**And** if an archive contains zero files or is 0 bytes, the engine returns a typed `TbError::EmptyArchive` without creating empty destination directories.

---

### Story 7.2: CLI Archive Subcommands (`tb archive extract`, `tb archive create`)

As a terminal user,
I want `tb archive extract` and `tb archive create` subcommands supporting multiple formats and `--json` file lists,
So that I can handle archives consistently from the shell.

**Acceptance Criteria:**

**Given** an archive file on disk
**When** executing `tb archive extract release.tar.xz`
**Then** it detects the compression algorithm automatically and extracts files with a progress bar
**And** when executing `tb archive create folder/ -o backup.zip --password "pass"`, it produces a password-protected zip
**And** when executed with `--json`, stdout prints the list of extracted/archived file paths and uncompressed byte totals.

---

### Story 7.3: Slint GUI Archive Dropzone & Password Modal

As a desktop user,
I want a visual archive extractor and packer in the Slint GUI with an archive file preview,
So that I can extract or pack archives with a single drag-and-drop gesture.

**Acceptance Criteria:**

**Given** the Archive view in the Slint GUI
**When** an archive is dropped onto the dropzone
**Then** the internal file tree is displayed with item counts and total compressed/uncompressed sizes
**And** if the archive is encrypted, a password prompt modal appears automatically
**And** clicking "Extract All" extracts files to the destination directory and shows a `[Reveal in Folder]` button.

---

## Epic 8: JSON Formatter, Minifier & Stream Piping

**Epic Goal:** Implement high-performance JSON formatting, minification, and syntax error validation using `serde_json`, with dual-input pipeline (`InputSource`) supporting Unix `stdin`/`stdout` piping (`cat raw.json | tb dev json format`) and Slint GUI code editor with line numbers.

### Story 8.1: High-Performance JSON Formatter & Minifier in `tb-core`

As a developer,
I want a pure Rust JSON engine that prettifies, minifies, and validates JSON payloads with precise syntax error pointers,
So that I can inspect and reformat payloads without sending proprietary data to web tools.

**Acceptance Criteria:**

**Given** a valid raw JSON string or file
**When** formatting is executed with indentation options (2 spaces, 4 spaces, tabs)
**Then** the JSON is formatted deterministically with standardized indentation
**And** when minification is executed, all redundant whitespace and newlines are stripped
**And** when invalid JSON is provided, the engine returns line number and column offset pointers identifying the exact syntax violation.

---

### Story 8.2: CLI JSON Subcommand with Unix Pipe Support (`tb dev json`)

As a shell power user,
I want `tb dev json format` and `tb dev json minify` to accept piped `stdin` and stream to `stdout`,
So that I can chain JSON formatting with curl, jq, and clipboard tools in my shell.

**Acceptance Criteria:**

**Given** a piped terminal stream (`cat payload.json | tb dev json format`)
**When** the command executes
**Then** formatted JSON streams directly to stdout without terminal escape artifacts or buffer duplication
**And** when downstream pipe terminates early (`tb dev json format | head -n 5`), `SIGPIPE` is handled gracefully without panics
**And** when executed with `--json`, it outputs a structured envelope with validation status and parse duration.

---

### Story 8.3: Slint GUI Code Editor View with Syntax Error Pointers

As a desktop user,
I want a clean two-pane or single-editor JSON view in the Slint GUI with line numbers, indentation controls, and inline error markers,
So that I can format and debug API payloads visually.

**Acceptance Criteria:**

**Given** the JSON tool view in the Slint GUI
**When** unformatted JSON is pasted or dragged in
**Then** clicking "Prettify" instantly formats the text in the editor
**And** if syntax is malformed, a red inline error banner points to the exact line/column with an explanation: *"Expected ':' at line 14, column 8"*
**And** the header includes a 1-click `[Copy to Clipboard]` button and live `[Copy CLI Command]` button.

---

## Epic 9: JWT Debugger & Token Inspector

**Epic Goal:** Implement an offline JWT token decoder and claim validator in `tb-core` that parses header, payload, and signature without network transmission, calculates real-time expiration countdowns, and exposes this functionality via `tb dev jwt` CLI and an interactive Slint GUI token inspector view.

### Story 9.1: Offline JWT Token Decoder, Claim Parser & Signature Verification in `tb-core`

As a developer or security engineer,
I want an offline JWT parsing and validation engine in `tb-core`,
So that I can inspect token contents and verify signatures locally without exposing sensitive authentication tokens or credentials to the public internet.

**Acceptance Criteria:**

**Given** a valid standard 3-part base64url-encoded JWT token string
**When** parsed by `JwtEngine::decode`
**Then** the header and payload are deserialized into formatted JSON structures and standard claims (`iss`, `sub`, `aud`, `exp`, `nbf`, `iat`, `jti`) are parsed
**And** if an `exp` claim is present, the engine calculates whether the token is currently valid or expired, along with remaining duration or time elapsed since expiration
**And** when an optional secret key or public key (HMAC SHA-256 / HS256, HS384, HS512, RS256) is provided, the engine cryptographically verifies signature integrity and returns a boolean verification status
**And** when a malformed token (invalid dot-separated segment count or corrupted base64url encoding) is supplied, it returns a descriptive error indicating the corrupted segment.

---

### Story 9.2: CLI JWT Decode Subcommand (`tb dev jwt`)

As a backend developer or DevOps engineer,
I want a `tb dev jwt decode` CLI subcommand supporting terminal arguments and piped stdin,
So that I can inspect authentication tokens quickly from terminal sessions and bash scripts.

**Acceptance Criteria:**

**Given** an encoded JWT passed as an argument (`tb dev jwt decode <token>`) or piped via stdin (`echo $AUTH_TOKEN | tb dev jwt decode`)
**When** the command executes
**Then** it prints color-coded, human-readable decoded header and payload JSON with an expiration status summary (e.g. `[STATUS: ACTIVE (Expires in 42 minutes)]`) to stdout
**And** when invoked with `--verify-key <secret>`, it displays signature verification confirmation or mismatch warning
**And** when executed with `--json`, stdout emits a structured envelope: `{"success": true, "data": {"header": {...}, "payload": {...}, "signature_valid": true, "expired": false, "expires_at": "..."}}`
**And** when the token is malformed, it exits with code 1 and outputs an actionable diagnostic message.

---

### Story 9.3: Slint GUI JWT Token Inspector View with Live Expiration Countdown

As a frontend or API developer,
I want a visual JWT inspector in the Slint GUI with color-coded token segmentation and a live expiration timer,
So that I can debug authentication tokens with clear visual clarity and zero risk of token leakage.

**Acceptance Criteria:**

**Given** the JWT Inspector view in the Slint desktop GUI
**When** a JWT token is pasted into the input field or received via clipboard routing
**Then** the input string displays color-coded segments (Header = Red/Pink, Payload = Violet/Purple, Signature = Cyan/Blue)
**And** separate decoded panels render pretty-printed Header and Payload JSON with copy buttons
**And** a prominent expiration status badge updates in real time (e.g. Green "Active — Expires in 01:24:15" with countdown, or Red "Expired — Expired 3 hours ago")
**And** adjusting verification key settings live-updates the header `[Copy CLI Command]` button to `tb dev jwt decode --verify-key <key> <token>`
**And** clicking `[Copy Decoded Payload]` copies the unescaped payload JSON to the system clipboard.

---

## Epic 10: SIMD Cryptographic Hashing & File Integrity Checker

**Epic Goal:** Implement SIMD-accelerated BLAKE3 (3–5 GB/s), SHA-256, SHA-512, and MD5 cryptographic hashing over zero-copy memory maps (`memmap2` with `flock(LOCK_SH)`), with file checksum verification against `.sha256` files, exposed via `tb dev hash` CLI and a Slint GUI drag-and-drop hash calculator.

### Story 10.1: SIMD-Accelerated Multi-Hash Engine with Zero-Copy Memory Mapping in `tb-core`

As a developer or system administrator,
I want a high-throughput multi-hash calculation engine in `tb-core` supporting BLAKE3, SHA-256, SHA-512, and MD5,
So that I can calculate multiple cryptographic hashes in a single pass over large multi-gigabyte files at raw hardware memory bandwidth speeds.

**Acceptance Criteria:**

**Given** an input file on disk (including files larger than 16MB)
**When** hashing is requested
**Then** the engine checks file size; if 0 bytes, it immediately returns standard empty-input hash digests without allocating mmap buffers
**And** for files larger than 16MB, it attempts an advisory shared read lock using non-blocking flags (`flock(LOCK_SH | LOCK_NB)`); if the file is exclusively locked by another process, it times out after 200ms and returns `TbError::FileLocked` with descriptive diagnostics rather than blocking indefinitely
**And** once the lock is acquired, it maps the file into virtual memory via `memmap2` without user-space buffer duplication
**And** for BLAKE3, the engine leverages AVX-512 / AVX2 SIMD instructions to achieve 3–5 GB/s throughput on modern x86_64 CPUs
**And** when multiple algorithms are requested (e.g. BLAKE3, SHA-256, MD5), the engine streams through the byte sequence in a single sequential pass, updating all state machines simultaneously
**And** checksum verification parses standard `.sha256` / `checksums.txt` file entries (`<hex_digest>  <filename>`) and returns whether the file matches the expected digest.

---

### Story 10.2: CLI Hashing & Checksum Verification Subcommand (`tb dev hash`)

As a command-line power user,
I want `tb dev hash` to compute hashes of files or stdin streams and verify checksum files,
So that I can verify downloaded Linux ISOs and software packages directly from my terminal.

**Acceptance Criteria:**

**Given** a target file on disk
**When** executing `tb dev hash ubuntu-24.04.iso --algo sha256`
**Then** it prints the computed SHA-256 hash string in terminal-friendly monospace format
**And** when executing `tb dev hash ubuntu-24.04.iso --check ubuntu-24.04.iso.sha256` (or `--verify <expected_hex>`), it outputs `[OK] Checksum matched` and exits with code 0 if equal, or `[FAILED] Checksum mismatch` and exits with code 1 if mismatched
**And** when executed with `--json`, stdout returns a structured envelope: `{"success": true, "data": {"path": "...", "size_bytes": 1073741824, "hashes": {"sha256": "...", "blake3": "..."}, "verified": true}}`
**And** when streaming data via stdin (`cat file.tar.gz | tb dev hash --algo blake3`), it computes the hash from stream buffers without seeking errors.

---

### Story 10.3: Slint GUI Drag-and-Drop Hash Calculator & Checksum Matcher

As a desktop user,
I want a drag-and-drop hash calculator in the Slint GUI with an instant checksum comparison input,
So that I can verify downloaded archives and ISOs visually without typing terminal commands.

**Acceptance Criteria:**

**Given** the Hash & Integrity tool view in the Slint GUI
**When** an ISO or large file is dropped onto the dropzone
**Then** a progress bar displays current calculation progress and hashing speed (e.g. "Hashing at 3.2 GB/s — 65%")
**And** upon completion, cards for BLAKE3, SHA-256, SHA-512, and MD5 display the resulting hex digests with 1-click `[Copy]` buttons
**And** when the user pastes an expected checksum string into the "Verify against expected checksum" field, the UI immediately compares the string (case-insensitively, trimming whitespace) and displays a prominent green "MATCH VERIFIED" banner or red "MISMATCH" warning
**And** the header provides a live `[Copy CLI Command]` button reflecting the file and selected algorithm.

---

## Epic 11: Developer ID, Token & Data Encoding Utilities

**Epic Goal:** Implement local generators for UUID (v4, v7), ULID, Nanoid, alongside Base64 and URL encoding/decoding with 1-click clipboard copy, exposed via `tb dev uuid`, `tb dev base64`, and an interactive Slint GUI multi-card view.

### Story 11.1: Local ID Generation & Data Encoding in `tb-core`

As a software engineer,
I want pure Rust domain generators for random and time-sorted identifiers (UUID v4, UUID v7, ULID, Nanoid) and text encoders (Base64, URL percent-encoding),
So that I have instant access to cryptographically secure IDs and encoding conversions without external web tools.

**Acceptance Criteria:**

**Given** requests for identifier generation
**When** UUID v4 is generated, it produces a standard RFC 4122 random UUID
**When** UUID v7 is generated, it produces a time-ordered RFC 9562 UUID combining millisecond Unix timestamp with monotonic randomness
**When** ULID is generated, it produces a 26-character Crockford Base32 string with millisecond sorting
**When** Nanoid is generated, it allows custom length and alphabet specifications
**And** Base64 encoding/decoding supports both Standard (`+`, `/`) and URL-Safe (`-`, `_`) alphabets with or without padding (`=`)
**And** URL percent-encoding/decoding handles query strings and URI components adhering to RFC 3986.

---

### Story 11.2: CLI Subcommands for IDs & Encodings (`tb dev uuid`, `tb dev base64`, `tb dev url`)

As a terminal user,
I want dedicated CLI subcommands for generating IDs and encoding/decoding strings,
So that I can generate test data and decode query parameters in shell scripts.

**Acceptance Criteria:**

**Given** `tb dev uuid` is executed with options (`tb dev uuid --version 7 --count 5 --uppercase`)
**When** the command runs
**Then** it prints 5 time-ordered uppercase UUID v7 strings, one per line
**And** executing `tb dev base64 encode "hello world"` or `echo "hello world" | tb dev base64 encode` outputs `aGVsbG8gd29ybGQ=` to stdout
**And** executing `tb dev base64 decode "aGVsbG8gd29ybGQ="` outputs `hello world`
**And** when executed with `--json`, stdout outputs a structured envelope with the generated/encoded values
**And** decoding invalid Base64 or percent-encoded data exits with code 1 and outputs an actionable diagnostic.

---

### Story 11.3: Slint GUI Multi-Card Utility View & Live Copy

As a desktop user,
I want a clean multi-card or segmented interface in the Slint GUI for IDs and Encoders,
So that I can generate batches of identifiers or encode/decode text bidirectionally with 1-click clipboard actions.

**Acceptance Criteria:**

**Given** the Developer Utilities view in the Slint GUI
**When** selecting the "ID Generator" card
**Then** the user can select identifier type (UUID v4, UUID v7, ULID, Nanoid), adjust count slider (1–100), and click "Generate"
**And** each generated ID has an individual 1-click copy icon, alongside a "Copy All" button
**And** when selecting the "Base64 & URL Encoder" card, the view provides side-by-side or stacked text areas that encode/decode in real-time as the user types
**And** toggles for "URL Safe" and "No Padding" dynamically update the output
**And** the header `[Copy CLI Command]` button live-updates to reflect the exact terminal command for the current action.

---

## Epic 12: Visual Text Diff, Line Tools & Regex Tester

**Epic Goal:** Implement side-by-side graphical text comparison with word/line diff highlighting, live regex pattern matcher with real-time capture group visualization, and text line deduplication/sorting, exposed via `tb dev diff`, `tb dev regex`, and an interactive Slint GUI view.

### Story 12.1: Text Diffing, Regex Matching & Line Processing in `tb-core`

As a developer,
I want a pure Rust engine for text diff computation, regular expression evaluation, and line manipulation,
So that I can compare text snippets, evaluate regex patterns with capture groups, and clean up unstructured data locally.

**Acceptance Criteria:**

**Given** two text strings or files
**When** `DiffEngine::diff` is invoked
**Then** it computes line-level and word-level additions, deletions, and unchanged hunks using the Myers / LCS diff algorithm
**And** given a regex pattern and target text, `RegexEngine::evaluate` extracts all matches, start/end byte offsets, and named/numbered capture groups
**And** line manipulation functions provide deterministic deduplication (preserving or sorting order), alphabetical sorting (case-sensitive or insensitive), trimming, and stripping empty lines
**And** all functions are pure domain logic with 100% unit test coverage.

---

### Story 12.2: CLI Subcommands for Diff, Regex & Text Processing (`tb dev diff`, `tb dev regex`, `tb dev lines`)

As a command-line user,
I want CLI subcommands for diffing files, testing regex patterns, and sorting text lines,
So that I can perform quick textual comparisons and regex verifications directly from bash.

**Acceptance Criteria:**

**Given** two files on disk
**When** executing `tb dev diff file_a.txt file_b.txt`
**Then** it prints a colored unified terminal diff with green `+` additions and red `-` deletions
**And** executing `tb dev regex test "(\d{4})-(\d{2})-(\d{2})" "Date: 2026-10-04"` outputs matching spans and capture groups (`Group 1: 2026`, `Group 2: 10`, `Group 3: 04`)
**And** executing `cat list.txt | tb dev lines --dedup --sort` outputs sorted unique lines to stdout
**And** when executed with `--json`, each subcommand returns a structured envelope with match/diff metadata
**And** when diff finds zero differences or regex finds a match, exit code is 0; if diff finds differences or regex has no match, exit code is 1 (standard diff/grep convention).

---

### Story 12.3: Slint GUI Visual Diff Viewer & Interactive Regex Tester

As a desktop user,
I want a side-by-side visual diff editor and an interactive regex testing workbench in the Slint GUI,
So that I can inspect visual differences and develop regular expressions with immediate feedback.

**Acceptance Criteria:**

**Given** the Text Diff view in the Slint GUI
**When** two text blocks or files are loaded
**Then** side-by-side synchronized scrolling editors highlight modified lines in soft red/green, with intra-line word changes highlighted in higher contrast
**And** when switching to the "Regex Tester" view, entering a regex pattern and test text highlights all matches in real-time with numbered capture group chips
**And** regex syntax errors display an immediate inline red indicator explaining the regex parse failure
**And** the "Line Tools" toolbar provides 1-click buttons for `[Sort A-Z]`, `[Remove Duplicates]`, `[Trim Whitespace]`, and `[Remove Empty Lines]`
**And** the header `[Copy CLI Command]` button live-updates to reflect the exact equivalent terminal command.

---

## Epic 13: Omni-Bar Command Palette & Proactive Clipboard Engine

**Epic Goal:** Build the unified top-level Omni-bar with global hotkey (`Super+Shift+T`), sub-16ms fuzzy tool search, and proactive focus clipboard detection that suggests 1-click execution chips (e.g. JWT decode chip or JSON prettify chip upon window focus).

### Story 13.1: Global Desktop Hotkey Listener & Window Summon (`Super+Shift+T`)

As a Linux desktop power user,
I want a system-wide global shortcut (`Super+Shift+T`) that brings Toolbox to the foreground instantly from any virtual desktop or application,
So that I can access my daily utilities in under 50ms without switching windows manually or searching application menus.

**Acceptance Criteria:**

**Given** Toolbox is running in the background or minimized
**When** the user presses `Super+Shift+T` on Wayland (via XDG Global Shortcuts portal) or X11 (via X11 grab)
**Then** the Toolbox window immediately unminimizes, raises to the foreground, centers on the currently active monitor, and focuses the Omni-Bar search input in <50ms
**And** pressing `Super+Shift+T` again or pressing `Escape` when the search query is empty cleanly minimizes or hides the window
**And** if `Super+Shift+T` is already bound by the desktop environment (GNOME/KDE) or the user denies the XDG Desktop Portal shortcut authorization on Wayland, the app logs a non-fatal advisory, remains running in the background/tray, and prompts the user to configure an alternative keybinding in `~/.toolbox/config.toml` or via `tb config set hotkey <bind>`.

---

### Story 13.2: Sub-16ms Fuzzy Tool Search & Command Palette in Slint GUI

As a desktop user,
I want an instant fuzzy search command palette inside the Omni-Bar,
So that I can find and launch any utility in under 16ms by typing just a few characters.

**Acceptance Criteria:**

**Given** the Omni-Bar input in the Slint GUI
**When** the user types a query (e.g., "jwt", "cmp", "pdf mrg", "base")
**Then** the fuzzy search algorithm (`nucleo-matcher`) ranks and filters matching tools in <16ms (within a single 60Hz frame)
**And** all 16 built-in tools, CLI subcommands, and installed extensions are indexed regardless of engine installation status (Never-Hide Discovery)
**And** the keyboard navigation allows pressing `Down`/`Up` arrow keys to highlight results and `Enter` to open the highlighted tool
**And** when the search input is empty, the top 4 most recently used tools are pinned for immediate 1-click access.

---

### Story 13.3: Proactive Focus Clipboard Inspection & 1-Click Action Chips

As a developer copying tokens, payloads, and hashes,
I want Toolbox to automatically inspect my clipboard text upon window focus and offer 1-click action chips,
So that I can prettify JSON, decode JWTs, or verify hashes without manually pasting or navigating.

**Acceptance Criteria:**

**Given** the user has copied text to the system clipboard and switches focus to Toolbox (or summons via hotkey)
**When** the window gains focus
**Then** the Omni-Bar runs local zero-network regex heuristics on the clipboard string without storing it to disk
**And** if a JWT pattern (`^ey[A-Za-z0-9_-]+\.ey[A-Za-z0-9_-]+\.[A-Za-z0-9_-]+$`) is detected, an action chip `[Decode JWT Token]` appears under the Omni-Bar
**And** if a JSON object/array is detected, an action chip `[Prettify JSON Payload]` appears
**And** if a 64-character hex string is detected, an action chip `[Verify SHA-256 Checksum]` appears
**And** pressing `Enter` or clicking the chip immediately navigates to that tool, pre-fills the input with the clipboard content, and executes the action
**And** a user preference toggle in Settings allows disabling proactive clipboard inspection for users with strict privacy policies.

---

## Epic 14: 3-Tier Heavy Dependency Engine & Video Transcoder

**Epic Goal:** Implement the 3-tier dependency resolution engine in `tb-deps` (Tier 1 host `$PATH` probe, Tier 2 user-space static `musl` binary downloader to `~/.toolbox/deps/` with compile-time pinned SHA-256 verification, zero sudo), and ship Video Transcoding (MKV/MP4/WebM) via FFmpeg.

### Story 14.1: Hybrid Smart Dependency Resolver & Zero-Sudo Verifying Downloader in `tb-deps`

As a Linux user (with or without root/sudo privileges),
I want Toolbox to automatically detect existing host CLI binaries (`qpdf`, `magick`, `7z`, `ffmpeg`), download standalone static binaries into user-space (`~/.toolbox/deps/`) without sudo, or provide the native distro install command,
So that tools work immediately across all Linux distributions with zero friction.

**Acceptance Criteria:**

**Given** an operation requiring an external engine (`qpdf`, `magick`, `7z`, `ffmpeg`)
**When** `DependencyManager::resolve(tool_name)` is called
**Then** Tier 1 probes host `$PATH` (e.g. `which qpdf`); if present and compatible, it binds to the host binary immediately with 0MB download
**And** if absent on host, Tier 2 presents a hybrid option: a 1-click `[ Download (Zero-Sudo) ]` button to pull a verified static `musl` binary into `~/.toolbox/deps/bin/`, alongside a link copying the native package manager command (e.g. `sudo pacman -S qpdf` or `sudo apt install qpdf`) based on `/etc/os-release` detection
**And** downloaded binaries are cryptographically verified against compile-time pinned SHA-256 hashes before marking executable (`0o755`); any mismatch aborts and removes the file immediately
**And** in air-gapped environments, it provides an offline import path: `tb deps import <tool>.tar.gz`.

---

### Story 14.2: Video Transcoding Engine in `tb-core`

As a media producer or developer,
I want a robust video transcoding orchestration engine in `tb-core` leveraging FFmpeg,
So that I can convert videos between MKV, MP4, and WebM with web optimization and non-destructive output.

**Acceptance Criteria:**

**Given** an input video file (MKV, MP4, WebM, AVI, MOV)
**When** `VideoEngine::transcode` is called with target format and preset (Web-Optimized H.264/AAC, Fast Transcode H.265, Audio-Only MP3/AAC)
**Then** it spawns the resolved FFmpeg process with bounded worker threads and non-destructive output naming (`<name>_converted.<ext>`)
**And** standard error is parsed in real time to extract encoding metrics (current frame, fps, target bitrate, elapsed time, and ETA)
**And** progress updates are emitted at throttled 16ms intervals to drive smooth UI animations
**And** cancelling or terminating the job cleanly sends `SIGINT` to FFmpeg, waits up to 2 seconds for a graceful exit, and cleans up any incomplete temporary output files.

---

### Story 14.3: CLI Video Subcommand (`tb video`) & Slint GUI Transcoder View

As a desktop or terminal user,
I want a `tb video convert` CLI command and a visual video dropzone in the Slint GUI,
So that I can transcode video files from terminal scripts or visually with clear progress and live CLI command parity.

**Acceptance Criteria:**

**Given** a video file on disk
**When** executing `tb video convert input.mkv --format mp4 --preset web`
**Then** the CLI shows an interactive terminal progress bar with FPS, elapsed time, and ETA, exiting 0 upon completion
**And** executing with `--json` emits structured JSON status messages during progress and a final completion envelope
**And** in the Slint GUI, dropping a video file displays thumbnail, duration, resolution, audio tracks, and preset dropdowns
**And** if FFmpeg is not installed, the GUI displays an informative banner: *"Video Engine Required (42 MB). [Download Now (Zero-Sudo)]"*
**And** adjusting format and preset dynamically updates the header `[Copy CLI Command]` button to `tb video convert input.mkv --format mp4 --preset web`.

---

## Epic 15: Storage Transparency & Engine Management Dashboard

**Epic Goal:** Build the dedicated Storage & Engines Dashboard in Slint and CLI (`tb storage`), displaying disk space consumed by downloaded engines, *"Last used: X days ago"* tracking, explicit 1-click uninstallation (strictly zero auto-pruning), and the Air-Gap Verification badge (`tb doctor --network`).

### Story 15.1: Engine Disk Usage Tracking & Registry Management in `tb-core` / `tb-deps`

As a privacy and storage-conscious user,
I want Toolbox to track exact disk space consumed by downloaded engines and record last-used timestamps in a local registry,
So that I have complete visibility into my storage footprint and no background processes ever delete my engines without consent.

**Acceptance Criteria:**

**Given** downloaded engines in `~/.toolbox/deps/` or extensions in `~/.toolbox/extensions/`
**When** `StorageManager::scan()` is invoked
**Then** it calculates precise byte usage on disk for each installed binary and associated libraries
**And** reads/updates `~/.toolbox/registry.json` recording `engine_name`, `version`, `installed_at`, `size_bytes`, and `last_used_timestamp`
**And** the engine strictly enforces the zero-auto-pruning invariant (AD-4): engines are NEVER automatically pruned or deleted in the background, regardless of age or available disk space.

---

### Story 15.2: CLI Storage Management (`tb storage`) & Slint GUI Dashboard View

As a user managing disk space,
I want a dedicated Storage & Engines Dashboard in both CLI and GUI with 1-click uninstallation,
So that I can view and clean up unused engines on my own schedule.

**Acceptance Criteria:**

**Given** installed engines on the system
**When** executing `tb storage list`
**Then** stdout prints a formatted table displaying Engine, Status (Host / User-space), Size, and Last Used relative time (e.g. "Last used: 12 days ago")
**And** executing `tb storage remove <engine>` removes the binary, cleans registry entries, and frees disk space immediately
**And** executing with `--json` outputs a structured envelope: `{"success": true, "data": {"total_bytes": 44040192, "engines": [...]}}`
**And** in the Slint GUI "Storage & Engines" settings screen, a visual storage breakdown bar displays disk usage, with individual `[Uninstall Engine]` buttons protected by confirmation dialogs.

---

### Story 15.3: Offline Air-Gap Verification Engine & Audit Badge (`tb doctor --network`)

As a security auditor or enterprise privacy user,
I want a verifiable air-gap diagnostic tool that audits network sockets and displays an offline security badge,
So that I have cryptographically provable certainty that Toolbox never communicates over the internet during utility execution.

**Acceptance Criteria:**

**Given** Toolbox is running
**When** executing `tb doctor --network`
**Then** it inspects process socket tables (`/proc/net/tcp`, `/proc/net/tcp6`, `/proc/net/udp`), confirms zero outbound sockets are open, and tests local loopback-only Unix domain socket binding
**And** stdout prints a diagnostic report confirming: `[PASS] Zero outbound telemetry sockets detected. 100% Offline Air-Gap Verified.`
**And** executing with `--json` returns `{"success": true, "data": {"air_gapped": true, "open_sockets": 0, "verified_at": "..."}}`
**And** the Slint GUI status bar displays a green shield badge: *"100% Offline & Private"*, which opens the full audit log on click.

---

## Epic 16: Sandboxed Extension Viewport & Authenticated UDS Host API

**Epic Goal:** Implement the external subcommand runner (`tb-<ext>`) in `tb-ext`, the embedded Slint `<ExtensionViewport>` (iframe equivalent), the authenticated Unix Domain Socket IPC server (`$XDG_RUNTIME_DIR/toolbox.sock`) with one-time `TB_IPC_AUTH_TOKEN` handshakes and permission scoping, and GitHub URL extension installation (`tb ext install`).

### Story 16.1: External Subcommand Runner (`tb-<ext>`) in `tb-ext`

As a command-line developer,
I want `tb` to discover and execute standalone third-party extension binaries prefixed with `tb-`,
So that community plugins integrate seamlessly into the CLI subcommand hierarchy just like `git` plugins.

**Acceptance Criteria:**

**Given** an executable binary named `tb-minify` placed in `~/.toolbox/extensions/bin/` or system `$PATH`
**When** the user executes `tb minify --level 2 input.js`
**Then** `tb` transparently dispatches execution to `tb-minify` passing all command-line arguments and preserving standard input/output/error pipes
**And** the process exit code is forwarded directly back to the parent shell
**And** `tb` strictly restricts extension discovery to `~/.toolbox/extensions/bin/` and trusted system `$PATH`, explicitly ignoring any binary in the current working directory (`./`) to prevent binary hijacking
**And** executing `tb ext list` scans `~/.toolbox/extensions/` and lists all discovered extensions with their versions and descriptions from `manifest.json`.

---

### Story 16.2: Authenticated Unix Domain Socket (UDS) Host IPC Server

As an extension developer,
I want a secure, permission-scoped Unix Domain Socket IPC server hosted by Toolbox,
So that my extension process can request host capabilities (file dialogs, notifications, clipboard) safely without direct system access.

**Acceptance Criteria:**

**Given** the Toolbox main process is running
**When** the IPC server initializes
**Then** it binds to a private Unix Domain Socket at `$XDG_RUNTIME_DIR/toolbox/toolbox.sock` with strict `0o600` file permissions (accessible only by the current user)
**And** if `$XDG_RUNTIME_DIR` is unset in the current environment, the server creates and binds to a dedicated user-private directory `~/.toolbox/run/` with strict `0o700` permissions, strictly avoiding multi-user `/tmp` locations to prevent symlink race vulnerabilities
**And** for each launched extension, Toolbox generates a cryptographically secure random token passed via the environment variable `TB_IPC_AUTH_TOKEN`
**And** all incoming JSON-RPC IPC requests require a valid `auth_token` in the handshake header; unauthenticated requests are rejected immediately with connection drop
**And** requests for privileged capabilities (e.g. `dialog.open_file`, `clipboard.read`) are checked against the extension's declared `manifest.json` permissions; unauthorized method calls return `PERMISSION_DENIED` error envelopes.

---

### Story 16.3: Embedded Slint `<ExtensionViewport>` Container & Lifecycle Management

As a desktop user,
I want third-party extensions to render smoothly inside the Toolbox window and terminate cleanly when closed,
So that extension tools feel like built-in features without compromising system stability.

**Acceptance Criteria:**

**Given** an extension with a graphical user interface view
**When** the user navigates to the extension in the Slint GUI
**Then** Toolbox instantiates the `<ExtensionViewport>` container, spawns the child process with assigned `TB_IPC_AUTH_TOKEN`, and connects rendering buffers
**And** if the extension process crashes or exits unexpectedly, the main Toolbox GUI remains fully responsive, displaying a crash placeholder with error code and `[Restart Extension]` button
**And** when switching away from the extension tab or closing the window, Toolbox cleanly sends `SIGTERM` to the child process and frees associated memory buffers.

---

### Story 16.4: CLI Extension Installer & Manifest Security Review (`tb ext install`)

As a security-conscious developer,
I want `tb ext install` to inspect extension manifests and display requested permissions before installation,
So that I can review security implications before any third-party code is installed.

**Acceptance Criteria:**

**Given** a GitHub repository URL or local tarball (`tb ext install https://github.com/user/tb-minify.git`)
**When** the install command is executed
**Then** the package is downloaded to a temporary staging directory and its `manifest.json` is validated
**And** the CLI displays a formatted permission review prompt:
  ```
  Installing Extension: Minify (v1.0.0) by @author
  Requested Permissions:
    - read_clipboard : Allows reading system clipboard
    - file_dialog    : Allows opening native file dialogs
  Do you trust this extension and grant these permissions? [y/N]:
  ```
**And** if the user enters `y` or passes `--yes`, the binary and assets are moved into `~/.toolbox/extensions/`, marked executable, and registered in `registry.json`
**And** executing `tb ext remove minify` completely purges the extension files from disk.

