---
title: "Product Requirements Document: Toolbox"
status: final
created: 2026-10-03
updated: 2026-10-03
author: Bharath
facilitator: Mary (Business Analyst)
---

# Product Requirements Document: Toolbox (`tb`)

> **"The open-source, 100% offline VLC of Digital Utilities for Linux."**

---

## 0. Document Purpose

This Product Requirements Document (PRD) defines the functional requirements, non-functional constraints, user experience tenets, and architecture boundaries for **Toolbox (`tb`)**. It is written for engineering, architecture, and quality assurance teams preparing for implementation. 

This document builds directly upon:
* **[PRODUCT-BRIEF.md](file:///home/bharath/Documents/toolbox/_bmad-output/planning-artifacts/PRODUCT-BRIEF.md)** *(Vision, 8-stage roadmap, zero-sudo 3-tier dependency engine)*
* **[FEATURE-RESEARCH.md](file:///home/bharath/Documents/toolbox/_bmad-output/planning-artifacts/FEATURE-RESEARCH.md)** *(128 features analyzed across 18 categories)*
* **[COMPETITIVE-ANALYSIS.md](file:///home/bharath/Documents/toolbox/_bmad-output/planning-artifacts/COMPETITIVE-ANALYSIS.md)** *(DevToys, Stirling-PDF, IT-Tools teardowns)*

---

## 1. Vision

Every day, millions of users search Google for basic micro-utilities (*"compress pdf"*, *"convert heic to png"*, *"format json"*, *"decode jwt"*), uploading personal, financial, and proprietary data to unknown third-party websites. 

**Toolbox (`tb`)** is an open-source, privacy-first desktop application and terminal CLI suite for Linux that replaces cloud file converters with pure, on-device execution. Built 100% in **Rust** with the **Slint** hardware-accelerated UI framework, Toolbox boots in under 20 milliseconds, uses less than 20MB of RAM, and guarantees that zero bytes of user data ever leave the machine.

By uniting **consumer file operations** (PDF, images, video, audio) and **developer data tools** (JSON, JWT, crypto, regex) under a single hardware-accelerated **Omni-Bar** interface and a pipe-friendly CLI, Toolbox eliminates both the anxiety of cloud uploads and the friction of arcane terminal command flags.

---

## 2. Target User & Journeys

### 2.1 Jobs To Be Done (JTBD)
* **The Emotional Job:** *"Relieve the anxiety and guilt of uploading my tax returns, legal contracts, or company IP to random online converters with unverifiable privacy policies."*
* **The Functional Job:** *"Perform common micro-tasks (compressing a 25MB PDF, converting an iPhone HEIC to WebP, formatting an ugly JSON payload, decoding a JWT) in under 5 seconds on my local machine without remembering esoteric command-line flags."*
* **The Contextual Job:** *"Have an offline-capable tool that works on an airplane or disconnected workstation without requiring Docker containers, Java runtimes, or root `sudo` access."*

### 2.2 Non-Users (v1.0)
* Users seeking full-blown video editing timelines or multi-track audio production (use Kdenlive or Audacity).
* Enterprise organizations seeking cloud-synced document collaboration or team file management.
* Users wanting cloud-hosted AI document search / web-scale plagiarism checking.

### 2.3 Key User Journeys

#### UJ-1. Marcus compresses an invoice PDF before a client deadline
* **Persona & Context:** Marcus, a freelance software engineer on Fedora Workstation, needs to email an invoice PDF to a client's billing department. The file is 28MB (due to scanned receipts) and email bounces it.
* **Entry State:** Toolbox is installed. Marcus presses `Super+Shift+T`.
* **Path:**
  1. The Slint Omni-Bar pops up centered on screen in 18 milliseconds with search auto-focused.
  2. Marcus drags `invoice_october.pdf` from his desktop into the Omni-Bar.
  3. The Omni-Bar detects `MIME: application/pdf (28 MB)` and displays primary actions: `[Compress PDF (High)]`, `[Compress PDF (Medium)]`, `[Split Pages]`.
  4. Marcus hits `Enter` on `[Compress PDF (High)]`.
  5. The tool processes the document locally via the internal PDF compression engine.
* **Climax:** In 1.8 seconds, a toast appears: *"Compressed from 28 MB → 3.2 MB (88% reduction). Saved as invoice_october_compressed.pdf. Processed 100% locally."*
* **Resolution:** The output file sits highlighted in his file manager. Zero bytes were transmitted.

#### UJ-2. Sarah inspects an encrypted production JWT token
* **Persona & Context:** Sarah, a backend developer on Arch Linux, is debugging an API auth failure. She has an active JWT token on her clipboard containing customer identifiers that cannot leave her air-gapped terminal.
* **Entry State:** Sarah copies the token `eyJhbGci...` to her clipboard and presses `Super+Shift+T`.
* **Path:**
  1. Toolbox detects JWT syntax on the clipboard and immediately presents: *"JWT Token detected on clipboard → Press Enter to Decode"*.
  2. Sarah hits `Enter`.
  3. The JWT Inspector view opens with color-coded syntax: Header (algorithm), Payload (claims), and Signature status.
  4. She toggles between formatted JSON and expiration timestamp human-readable translation using `Tab` and `Arrow` keys.
* **Climax:** She spots an expired `exp` claim without pasting company authentication tokens into `jwt.io`.
* **Resolution:** Sarah presses `Esc`. Toolbox closes cleanly, using 0MB background memory.

#### UJ-3. Alex converts a video on a clean installation without pre-installed FFmpeg
* **Persona & Context:** Alex, a student on a fresh Ubuntu install, downloads an `.mkv` lecture recording and needs it in `.mp4` for a presentation.
* **Entry State:** Alex opens Toolbox and types `"convert mkv to mp4"`.
* **Path:**
  1. The Omni-Bar displays the tool: `Convert Video (MKV to MP4)` with an inline sub-badge: `[Press Enter to Download Video Engine (45MB) & Run]`.
  2. Alex presses `Enter` and selects the video.
  3. Toolbox probes `$PATH` (Tier 1) and detects no host `ffmpeg`.
  4. Toolbox initiates a direct user-space static binary download into `~/.toolbox/deps/ffmpeg` (Tier 2).
  5. A 3-second progress indicator shows the download; zero `sudo` or root prompts appear.
  6. The conversion runs automatically upon extraction.
* **Climax:** Alex gets `lecture.mp4` in his folder. The video engine is now cached for all future conversions.
* **Edge Case:** If Alex’s internet drops midway through download, Toolbox catches the network timeout, removes partial archive fragments, and displays: *"Download interrupted. Check your connection or drop an existing ffmpeg binary into ~/.toolbox/deps/."*

---

## 3. Glossary

* **Omni-Bar:** The unified top-level input component of the desktop application that accepts typed search queries, dragged files, or pasted clipboard content and routes them to the appropriate tool.
* **Core Built-in Tool:** A micro-utility whose logic and dependencies are statically compiled directly into the root `tb` binary (e.g. JSON formatters, image converters, PDF split/merge).
* **Managed Dependency / Heavy Engine:** A third-party, pre-compiled standalone binary (e.g. `ffmpeg`) downloaded on-demand into `~/.toolbox/deps/` without utilizing system package managers or requiring root permissions.
* **3-Tier Dependency Resolution:** The multi-stage process where Toolbox (1) checks host `$PATH`, (2) downloads standalone static binaries to user-space, and (3) tracks references for manual storage dashboard management.
* **Subcommand Binary (`tb-<name>`):** The architectural model where external extensions compile into standalone executables adhering to the Git/Cargo plugin pattern.
* **Non-Destructive Suffix:** The file output policy that appends a semantic tag (e.g. `_compressed`, `_converted`) to the output filename within the source directory, preserving the original file intact.

---

## 4. Features & Functional Requirements

```
                       +---------------------------------------------+
                       |              TOOLBOX ARCHITECTURE           |
                       +---------------------------------------------+
                                      |               |
                                      v               v
                             [ Slint GUI Engine ]  [ CLI Dispatcher ]
                                      |               |
                                      +-------+-------+
                                              |
                                              v
                                   [ Core Built-in Tools ]
                 ┌────────────────────────────┼───────────────────────────┐
                 ▼                            ▼                           ▼
          [ PDF Suite ]                [ Image Suite ]             [ DevToys Parity ]
          Merge, Split,                Convert, Compress,          JSON, JWT, UUID,
          Compress, Password           Resize, Strip EXIF          Hashes, Diff, Regex
                                              │
                                              v
                              [ 3-Tier Dependency Engine ]
                                      |               |
                                      v               v
                              [ Host $PATH Probe ] [ ~/.toolbox/deps/ ]
```

### 4.1 Omni-Bar & Slint Desktop Interface

**Description:** The primary graphical entry point for Toolbox on Linux (Wayland & X11). Built using the Slint framework, providing an auto-focused, content-aware command palette that supports keyboard navigation, drag-and-drop file routing, and pre-flight error inspection. Realizes UJ-1, UJ-2, UJ-3.

#### FR-1: Global Hotkey Summoning
* **Actor:** Desktop user.
* **Capability:** The user can summon or dismiss Toolbox from any desktop workspace via a system-wide configurable hotkey (default: `Super+Shift+T`).
* **Consequences (Testable):**
  * Window appears with search input focused in **< 50 milliseconds** from keypress.
  * Window hides immediately upon pressing `Escape` or losing focus (configurable in settings).

#### FR-2: Content-Aware Clipboard Routing & Proactive Focus Inspection
* **Actor:** Desktop user.
* **Capability:** The Omni-Bar inspects clipboard contents upon window focus and suggests contextual micro-tools.
* **Consequences (Testable):**
  * When clipboard contains valid JWT format (`^[A-Za-z0-9-_]+\.[A-Za-z0-9-_]+\.[A-Za-z0-9-_]*$`), Omni-Bar displays *"Decode JWT Token"* as top suggestion chip.
  * When clipboard contains unformatted JSON (`{...}` or `[...]`), Omni-Bar displays *"Format & Prettify JSON"*.
  * User can press `Enter` to execute suggested action immediately without typing, or `Tab` to dismiss.

#### FR-3: Drag-and-Drop MIME Dispatch
* **Actor:** Desktop user.
* **Capability:** The user can drag and drop one or multiple files onto the application window to reveal supported operations.
* **Consequences (Testable):**
  * Dropping `.pdf` files displays PDF merge, split, compress, and password actions.
  * Dropping image files (`.png`, `.jpg`, `.webp`, `.heic`) displays convert, compress, resize, and strip EXIF actions.
  * Dropping an unsupported file format displays: *"Toolbox does not currently have a tool for this file type. Search community extensions?"*.

#### FR-4: Never-Hide Search Discovery & GUI-to-CLI Parity
* **Actor:** Desktop user / Developer.
* **Capability:** The Omni-Bar fuzzy search indexes all tools regardless of engine download status; every GUI tool page provides an interactive, live `[Copy CLI Command]` button reflecting the current configuration.
* **Consequences (Testable):**
  * Searching for a video tool when FFmpeg is not downloaded displays the tool with badge `[Press Enter to Download Engine & Run]`.
  * Hitting `Enter` initiates on-demand engine setup directly without navigating to a settings page.
  * Adjusting parameters in any GUI tool (e.g. Image Resizer to 1920x1080 WebP) updates the header `[Copy CLI Command]` button to emit the exact terminal invocation (`tb image convert --format webp --resize 1920x1080 input.jpg`).

#### FR-5: Non-Destructive File Output Behavior
* **Actor:** User (GUI or CLI).
* **Capability:** All file conversion and compression operations save the output to the source file's directory with a semantic suffix by default.
* **Consequences (Testable):**
  * Processing `/home/user/doc.pdf` via compression produces `/home/user/doc_compressed.pdf`.
  * If the target filename already exists, GUI appends numerical increment (`doc_compressed (1).pdf`) while CLI prompts for confirmation unless `--overwrite` / `-f` is explicitly passed.
  * CLI allows explicit destination override via `-o` / `--output <path>`.

#### FR-6: Pre-Flight Password Detection, Corruption Diagnostics & Actionable Healing
* **Actor:** User.
* **Capability:** The system inspects files before processing and presents proactive prompts, human-readable diagnostics, and 1-click repair actions.
* **Consequences (Testable):**
  * Opening an encrypted PDF immediately renders an upfront password prompt input *before* initiating processing.
  * Opening a truncated or corrupted file evaluates headers; if headers are invalid, displays inline diagnostic: *"File appears corrupted: unexpected end of stream or damaged header."*
  * When structural corruption is recoverable (e.g. damaged PDF XRef table), UI offers a 1-click action: `[Attempt Non-Destructive Repair via QPDF]`.

---

### 4.2 Terminal CLI Engine (`tb`)

**Description:** A standalone, statically linked binary that executes all built-in and external tools from the shell, supporting Unix pipes, standard streams, and machine-readable output. Realizes UJ-2.

#### FR-7: Subcommand Namespace Hierarchy
* **Actor:** CLI user / Shell script.
* **Capability:** The CLI organizes all functionality into intuitive category subcommands: `tb <category> <action> [options] [files]`.
* **Consequences (Testable):**
  * `tb pdf merge a.pdf b.pdf -o combined.pdf`
  * `tb image compress photo.png --level max`
  * `tb dev jwt decode $TOKEN`
  * Running `tb` or `tb <category>` with no arguments prints formatted help documentation.

#### FR-8: Standard Input / Output Stream Piping & Dual Input Architecture
* **Actor:** Developer / CLI pipeline.
* **Capability:** Text, developer, and data utilities accept input from `stdin` and stream output to `stdout` via a unified `InputSource` abstraction (transparently switching between zero-copy memory maps for seekable files and buffered streaming for pipes).
* **Consequences (Testable):**
  * `cat raw.json | tb dev json format` outputs formatted JSON directly to terminal without seek errors.
  * Streaming chained commands (`cat raw.json | tb dev json | tb dev base64 | xclip -selection clipboard`) works without terminal escape artifacts or buffer duplication.

#### FR-9: Machine-Readable JSON Output Flag
* **Actor:** Local AI Agent / Shell automation.
* **Capability:** All commands accept an optional `--json` flag that formats the response as structured JSON.
* **Consequences (Testable):**
  * `tb dev jwt decode $TOKEN --json` returns `{"header": {...}, "payload": {...}, "valid": true}`.
  * On failure with `--json`, returns `{"success": false, "error": "code", "message": "human explanation"}` with non-zero exit code.

---

### 4.3 Built-in Core File Utilities (PDF, Image, Archive)

**Description:** High-frequency document and media manipulation tools statically compiled into the base binary. Realizes UJ-1.

#### FR-10: PDF Manipulation Engine
* **Actor:** User.
* **Capability:** Merge, split, compress, and password-protect PDF documents locally.
* **Consequences (Testable):**
  * Merging multiple PDFs preserves vector elements, bookmarks, and links.
  * PDF compression provides 3 presets: Low (lossless font/structure cleanup), Medium (downsamples images to 150 DPI), High (downsamples to 72 DPI).
  * Password encryption applies standard AES-256 security.

#### FR-11: Image Conversion & Lossless Compression Engine
* **Actor:** User.
* **Capability:** Convert, compress, resize, and strip metadata from images locally.
* **Consequences (Testable):**
  * Supports input/output across JPG, PNG, WebP, AVIF, BMP, and HEIC.
  * Lossless PNG compression via `oxipng` produces smaller byte sizes without altering pixel values.
  * EXIF stripping completely removes GPS coordinates, camera serial numbers, and thumbnail previews.

#### FR-12: Archive Extraction & Packaging
* **Actor:** User.
* **Capability:** Create and extract standard archives without external tools.
* **Consequences (Testable):**
  * Extracts `.zip`, `.tar.gz`, `.tar.bz2`, `.tar.xz`.
  * Creates password-protected `.zip` archives.

---

### 4.4 Built-in Developer & Cryptographic Tools (DevToys Parity)

**Description:** Pure Rust implementations of daily developer formatting, encoding, and hashing utilities. Realizes UJ-2.

#### FR-13: Structured Data Formatters & Converters
* **Actor:** Developer.
* **Capability:** Format, validate, and convert between JSON, YAML, and TOML.
* **Consequences (Testable):**
  * Prettifies minified JSON with customizable indentation (2 spaces, 4 spaces, tabs).
  * Converts YAML to valid JSON and vice versa.
  * Validates JSON syntax and highlights exact error line/column on invalid payloads.

#### FR-14: Encoders and Decoders
* **Actor:** Developer.
* **Capability:** Encode and decode strings and binary files across standard web formats.
* **Consequences (Testable):**
  * Base64 text and file encode/decode.
  * URL percent-encoding and decoding.
  * HTML character entity escaping/unescaping.

#### FR-15: Cryptographic Hash & Checksum Generation
* **Actor:** Developer / Security user.
* **Capability:** Compute and verify cryptographic digests for strings and files.
* **Consequences (Testable):**
  * Generates MD5, SHA-1, SHA-256, SHA-512, and BLAKE3 digests.
  * Compares an input checksum hash against a target file and displays a green/red match indicator.

#### FR-16: Text Diff & Regex Evaluator
* **Actor:** Developer.
* **Capability:** Compare two blocks of text or evaluate regular expressions against sample inputs.
* **Consequences (Testable):**
  * Side-by-side colorized diff highlighting additions, deletions, and inline word changes.
  * Regex tester with real-time match highlighting, group extraction, and capture breakdown.

---

### 4.5 3-Tier Dependency Engine & Storage Dashboard

**Description:** The system that resolves, downloads, executes, and tracks heavy external binary engines (e.g. FFmpeg) with zero root privileges. Realizes UJ-3.

#### FR-17: Host System Detection (Tier 1)
* **Actor:** Internal Dependency Resolver.
* **Capability:** Before attempting any download, the system inspects `$PATH` for existing verified binaries.
* **Consequences (Testable):**
  * If `which ffmpeg` returns a valid executable, Toolbox marks the video tool as ready with **0 MB** downloaded.
  * Validates version compatibility (e.g. requires FFmpeg >= 4.4).

#### FR-18: Zero-Sudo User-Space Static Downloads (Tier 2)
* **Actor:** Internal Dependency Resolver.
* **Capability:** When a required engine is missing, downloads pre-compiled static `musl` binaries to `~/.toolbox/deps/`.
* **Consequences (Testable):**
  * Downloads are verified against SHA-256 checksums before extraction.
  * Never prompts for `sudo` or administrator privileges.
  * Binaries are marked executable (`chmod +x`) automatically.

#### FR-19: Storage & Engines Dashboard (Tier 3)
* **Actor:** User.
* **Capability:** View disk space utilized by downloaded engines and manually remove inactive dependencies.
* **Consequences (Testable):**
  * Dashboard displays exact disk size per engine and metadata: *"Last used: 12 days ago"*.
  * User can click `[Uninstall Engine]` to delete `~/.toolbox/deps/<engine>` and free disk space.
  * **No automated background deletion** occurs under any circumstance.

---

### 4.6 Extension Lifecycle & Decentralized GitHub Sourcing

**Description:** Management interface for third-party community extensions hosted on GitHub.

#### FR-20: Subcommand Binary Extension Execution & Authenticated UDS Viewport
* **Actor:** Core CLI / GUI Dispatcher & External Extensions.
* **Capability:** The core engine detects and executes standalone executables named `tb-<ext>` located in `~/.toolbox/bin/`, hosts an embedded Slint `<ExtensionViewport>`, and exposes an authenticated Unix Domain Socket Host API.
* **Consequences (Testable):**
  * Running `tb custom action` executes `~/.toolbox/bin/tb-custom action`.
  * Launching a graphical extension in GUI embeds its UI directly into the dedicated viewport container.
  * UDS IPC requires a cryptographically random one-time session authentication token (`TB_IPC_AUTH_TOKEN`) passed on child spawn; unauthenticated connections are rejected immediately.
  * Sensitive operations (e.g. `tb.clipboard.read`) mandate explicit user consent dialogs before returning data to the extension.
  * An extension crash or panic does not crash the parent application.

#### FR-21: Direct GitHub URL Installation & Sandboxing Verification
* **Actor:** User.
* **Capability:** Users can install community extensions by providing a GitHub repository URL, with manifest permission inspection.
* **Consequences (Testable):**
  * `tb ext install github.com/user/tb-custom` downloads the release asset for current OS/Arch into `~/.toolbox/bin/`.
  * Displays manifest permissions requested by the extension before installation.
  * The GUI displays a warning badge: *"Community Extension: Not officially verified by Toolbox team."*

---

## 5. Non-Goals (Explicit)

* **No Cloud Accounts or Telemetry:** Toolbox will never require user registration, cloud sync, or telemetry beacons.
* **No Closed-Source Add-ons:** All official tools and extensions must be 100% open-source.
* **No Distro Package Manager Hijacking:** Toolbox will never execute `sudo apt`, `sudo pacman`, or `sudo dnf`.
* **No Non-Linux Desktop Targets in v1.0:** Windows and macOS support are deferred to future milestones.
* **No Heavy Local AI Model Bundling in v1.0:** LLMs (Ollama) and OCR language packs are strictly deferred to v1.5 / v2.0.

---

## 6. MVP Scope (Version 1.0)

### 6.1 In Scope for v1.0
* Complete base `tb` CLI binary + Slint native desktop GUI.
* 35+ core built-in tools across PDF, Image, Developer Data, and Everyday categories.
* 3-tier on-demand zero-sudo dependency engine for FFmpeg.
* Dedicated Storage & Engines dashboard.
* Omni-Bar with smart clipboard detection and drag-and-drop file routing.
* Packaging for Linux: Flatpak (Flathub), AppImage, and Arch AUR.
* Decentralized extension installer (`tb ext install <url>`).

### 6.2 Out of Scope for v1.0 (Deferred)
* **OCR & Scanned PDF Recognition (Deferred to v1.5):** Requires bundling Tesseract engine and multi-language dictionary packs.
* **Local AI / LLM Integration (Deferred to v2.0):** Offline summarization, rewriting, and Whisper speech-to-text.
* **AI Background Removal (Deferred to v1.5):** Requires bundling ONNX/PyTorch machine learning runtimes.
* **Multi-track Video Timeline Editing:** Simple conversion and compression presets only.
* **Custom User Themes:** One dark theme and one light theme only.

---

## 7. Cross-Cutting Non-Functional Requirements (NFRs)

### 7.1 Performance & Resource Budgets
* **NFR-1 (Startup Latency):** Desktop application cold start must complete in **< 50 milliseconds** (target < 20ms) on standard SSD hardware.
* **NFR-2 (Memory Footprint):** Idle RAM consumption must not exceed **25 MB** when resting in memory.
* **NFR-3 (Binary Footprint):** The base multi-call executable (`tb`) must not exceed **20 MB** stripped.

### 7.2 Reliability & Hardware Compatibility
* **NFR-4 (Graphics Driver Resilience):** The Slint UI must use hardware GPU acceleration when available, with automatic, transparent fallback to software rendering. The app must never crash on virtual machines (VMware, VirtualBox), older Intel HD graphics, or software-only environments.
* **NFR-5 (Wayland & X11 Fidelity):** Full fractional scaling support and native window decorations under both Wayland compositors (GNOME, Sway, Hyprland, KDE) and traditional X11.

### 7.3 Security & Privacy Assurance
* **NFR-6 (Zero-Egress Guarantee):** Core file operations must open zero outbound network sockets. Network calls are strictly restricted to the extension downloader (`~/.toolbox/deps/`) only when the user explicitly triggers an engine install.
* **NFR-7 (Local Log Isolation):** Crash reports and diagnostic logs are stored exclusively on the local filesystem (`~/.toolbox/logs/`) and are never automatically transmitted over the network.

---

## 8. Success Metrics & Counter-Metrics

### Primary Metrics
* **SM-1 (Cold Start SLA):** 99% of desktop launches complete in < 50ms. Validates FR-1.
* **SM-2 (Task Completion Speed):** Users complete basic tasks (e.g. compress PDF, decode JWT) in under 3 clicks or under 5 seconds from summon. Validates FR-2, FR-3.
* **SM-3 (Community Adoption):** Reach 2,500+ GitHub stars and 10,000+ Flatpak downloads within 6 months of public release. Validates FR-7, FR-21.

### Counter-Metrics (Do Not Optimize)
* **SM-C1 (Do Not Optimize Download Footprint by Sacrificing Stability):** Do not strip essential error-handling routines or safety checks to force binary size below 10MB.
* **SM-C2 (Do Not Optimize Ease-of-Install by Violating Sudo Policy):** Never prompt for root `sudo` to speed up dependency installs, even if users request it.

---

## 9. Assumptions Index

* `[ASSUMPTION: Slint-Wayland-Stability]` We assume the current Slint 1.x Linux backend provides sufficient stability for window decorations and global shortcut hooks across GNOME and KDE Wayland sessions.
* `[ASSUMPTION: Static-FFmpeg-Availability]` We assume John Van Sickle's static Linux builds and official static release builds provide stable, long-term URLs with compatible GPL/LGPL licensing for distribution.
* `[ASSUMPTION: Single-User-Local-Cache]` We assume all dependency and extension caching can safely reside in the user's home directory (`~/.toolbox/`) without requiring multi-user system-wide daemon architecture.
