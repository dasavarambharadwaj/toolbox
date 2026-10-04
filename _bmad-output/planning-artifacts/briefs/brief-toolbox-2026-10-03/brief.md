---
title: "Product Brief: Toolbox"
status: finalized
created: 2026-10-03
updated: 2026-10-03
author: Bharath
facilitator: Mary (Business Analyst)
---

# Product Brief: Toolbox (`tb`)

> **"The open-source, 100% offline VLC of Digital Utilities for Linux."**

---

## 1. Executive Summary

Every day, millions of users open a browser, search Google for basic tasks (*"compress pdf"*, *"convert heic to png"*, *"format json"*, *"decode jwt"*), and upload personal, financial, and proprietary data to unknown third-party websites. Users do this not because they want to share their data, but because there is no discoverable, unified, and easy-to-use alternative pre-installed on their machines.

**Toolbox (`tb`)** is an open-source, privacy-first desktop application and terminal CLI suite for Linux that replaces shady online file converters and developer utility websites with pure, on-device local execution. Built 100% in **Rust** with **Slint** (developed by former Qt Core engineers), Toolbox boots in under 20 milliseconds, uses less than 20MB of RAM, and guarantees that zero bytes of user data ever leave the machine.

By uniting **consumer file operations** (PDF, images, video, audio) and **developer data tools** (JSON, JWT, crypto, regex) under a single hardware-accelerated **Omni-Bar** interface and a pipe-friendly CLI, Toolbox eliminates both the anxiety of cloud uploads and the friction of arcane terminal command flags.

---

## 2. The Problem

### The Daily Friction
When a user on a Linux desktop needs to perform a quick 10-second file task (e.g. compress a 25MB invoice PDF before emailing it, or convert an iPhone HEIC photo to PNG):
1. **No Unified Built-in Tool:** The user searches their desktop environment and finds no obvious pre-installed utility.
2. **Arcane CLI Barriers:** While power tools like `ffmpeg` or `poppler` may exist on the system, remembering their complex flag syntax (e.g. `ffmpeg -i input.mp4 -vcodec libx265 -crf 28 out.mp4`) causes cognitive overload and frustration.
3. **Surrender to Google:** The user falls back to their default path of least resistance: opening Chrome, searching Google, and clicking the first search result (e.g., `ilovepdf.com`, `cloudconvert.com`).

### The Hidden Cost & Worst-Case Reality
Users are forced to trust vague corporate promises like *"Files are deleted after 1 hour."* In reality:
* **Data Harvesting:** Sensitive PII, bank statements, tax IDs, and confidential legal contracts are exposed to third-party server logs, scrapers, and malicious ads.
* **Unauthorized AI Training:** Uploaded documents and images are increasingly ingested into corporate machine-learning datasets without explicit user consent.
* **Malware Distribution:** The FBI and security agencies frequently issue advisories regarding online file converter websites delivering info-stealers and trojans disguised as download links.

---

## 3. The Solution & Experience

Toolbox provides an instant, elegant desktop interface paired with an equally powerful terminal CLI:

```
[ Global Hotkey: Super+Shift+T ] ───► [ Omni-Bar Pops Up in <20ms ]
                                                 │
                ┌────────────────────────────────┴────────────────────────────────┐
                ▼                                                                 ▼
      [ Drop Any File ]                                                [ Paste Any Text / Token ]
  - Auto-detects MIME type                                         - Auto-detects JWT -> "Decode Token"
  - Shows relevant actions                                         - Auto-detects JSON -> "Format JSON"
  - Executes locally in seconds                                    - Auto-detects draft -> "Word Count"
                                                 │
                                                 ▼
             [ Confirmation Toast: "Processed 100% locally. Zero bytes left your machine." ]
```

### Key Experience Tenets
1. **Sub-2-Second Speed SLA:** Summoned anywhere via a configurable global hotkey (`Super+Shift+T`), launching instantaneously via Slint GPU acceleration.
2. **The Content-Aware Omni-Bar:**
   * **Smart Clipboard:** Pasting raw JSON prompts *Format/Validate*; pasting a JWT prompts *Decode Payload*.
   * **Smart Dropzone:** Dragging a PDF surfaces *Compress, Merge, Split, Protect*.
   * **Never-Hide Search:** Tools are **never hidden** based on installation state. If a tool requires an external heavy engine (e.g. FFmpeg), the search result shows an inline action: `[Enter to Download Engine & Run]`.
3. **Keyboard-First Optionality:** The entire application can be navigated and executed purely via keyboard (tab navigation, enter to run, escape to close), while casual users enjoy an intuitive drag-and-drop mouse experience.
4. **Emotional Reassurance:** Every completed task displays an explicit confirmation banner: *"Processed 100% locally on your device."*
5. **Storage & Engines Dashboard:** A dedicated management screen displaying disk usage per tool with *"Last used: X days ago"* timestamps and explicit manual uninstall buttons. **No automated background deletion** ensures offline reliability when traveling or disconnected.

---

## 4. What Makes This Different (The Honest Moat)

Toolbox does **not** claim to invent new compression algorithms or video encoders. The open-source community has spent 30 years perfecting low-level engines like `ffmpeg`, `qpdf`, `oxipng`, and `mozjpeg`.

**Toolbox's Unfair Advantage is Orchestration, Sane Defaults, and Zero-Friction UX:**
* **Versus Raw CLI (`ffmpeg`, `imagemagick`):** Eliminates the need to memorize dozens of flags. Toolbox provides pre-tuned, high-quality presets (*"Email friendly <25MB"*, *"Discord size"*, *"High quality lossless"*).
* **Versus DevToys:** DevToys is strictly a developer string/token scratchpad built on heavy .NET/webviews. It has **zero** file, PDF, video, audio, or OCR capabilities. Toolbox unifies developer tokens AND real file processing.
* **Versus Stirling-PDF & IT-Tools:** Stirling-PDF and IT-Tools require Docker containers or web servers. Toolbox is a **single, self-contained ~15MB native executable** that runs directly on bare metal with no Docker, no Java runtime, and no webview RAM overhead.
* **Versus Single-Purpose Tools (Curtail, PDF Arranger):** Replaces 5 single-purpose apps with one unified command palette.

---

## 5. Architectural Foundation: The 3-Tier Dependency Engine

To keep the initial download tiny (<15MB) while avoiding distro package manager fragmentation (`apt`, `pacman`, `dnf`), Toolbox implements a **Zero-Sudo Managed Binary Architecture**:

```
User triggers tool requiring heavy engine (e.g. FFmpeg)
                          │
                          ▼
        ┌───────────────────────────────────┐
        │ Tier 1: Probe Host System ($PATH) │
        │ "Is ffmpeg already on machine?"   │
        └─────────────────┬─────────────────┘
                          │
            ┌─────────────┴─────────────┐
            ▼                           ▼
      [ YES: Found ]             [ NO: Not Found ]
      Download size: 0 MB!       Prompt inline:
      Immediate execution        "Download local engine (45MB)?"
                                        │
                                        ▼
                         ┌───────────────────────────────┐
                         │ Tier 2: Static User-Space DL  │
                         │ Downloads to ~/.toolbox/deps/ │
                         │ Zero sudo. Distro-agnostic.   │
                         └──────────────┬────────────────┘
                                        │
                                        ▼
                         ┌───────────────────────────────┐
                         │ Tier 3: Manual Storage Control│
                         │ Dashboard displays last used. │
                         │ User explicitly uninstalls.   │
                         └───────────────────────────────┘
```

* **Zero Sudo / Zero Distro Hell:** Static `musl` binaries run identically on Ubuntu, Fedora, Arch, Alpine, and Debian. Never prompts for root passwords.
* **Future Cross-Platform Ready:** The exact same dependency resolution model applies to future macOS and Windows ports (downloading `.zip`/`.tar.gz` user-space binaries).

---

## 6. Who This Serves

1. **The Privacy-Conscious Linux Desktop User (Primary Persona):**
   * Uses Ubuntu, Fedora, Arch, or Pop!_OS for work or personal life.
   * Regularly handles sensitive files (tax documents, client NDAs, personal photos, financial spreadsheets).
   * Wants an elegant, modern app that respects their system resources and never uploads their data.
2. **The Linux Software Developer & Sysadmin (Power Persona):**
   * Spends their day in the terminal or code editor.
   * Needs instant JSON formatting, Base64 encoding, JWT inspection, and UUID generation.
   * Prefers keyboard shortcuts and pipe-friendly CLI workflows (`cat data.json | tb dev format`).
3. **The Autonomous Local AI Agent (Future Persona):**
   * Local LLMs running via Ollama or local agent harnesses that need structured, machine-readable tool execution (`--json`) to manipulate files locally without writing custom Bash scripts.

---

## 7. Phased Product Roadmap

```
[ v0.1: CLI Dev Core ] ──► [ v0.2: File Core ] ──► [ v0.3: Media Core ] ──► [ v0.4: Slint GUI ]
         │                          │                          │                     │
         ▼                          ▼                          ▼                     ▼
[ v0.5: UX Polish ]    ──► [ v1.0: Launch ]    ──► [ v1.5: Heavy OCR ] ──► [ v2.0: Local AI ]
```

### Phase 1: Terminal Foundation
* **v0.1 (CLI Core & DevTools Parity):** Pure Rust static binary (`tb`). JSON/YAML/TOML formatters, Base64, JWT decoder, Hashes (MD5/SHA256/BLAKE3), UUIDs, case conversion, text diff.
* **v0.2 (File Operations Core):** Image format conversion (JPG/PNG/WebP/AVIF/HEIC), lossless/lossy compression, EXIF stripper. PDF merge, split, compress, encrypt/decrypt.
* **v0.3 (Everyday Utilities & FFmpeg Wrapper):** QR generator/decoder, timestamp converter, color converter. Video conversion (MP4/WebM/MKV), video compression presets, audio extraction (MP3/WAV/AAC) with 3-tier dependency engine.

### Phase 2: Native Desktop Experience
* **v0.4 (Slint GUI & Omni-Bar):** Hardware-accelerated desktop window (<20ms boot), global hotkey, auto-focused fuzzy search, drag-and-drop file dropzone, smart clipboard detection.
* **v0.5 (Keyboard-First UX & Pipes):** 100% keyboard navigation, CLI shell streaming (`stdin`/`stdout`), machine-readable `--json` flag, local privacy reassurance toast, and Storage & Engines dashboard.

### Phase 3: Public Release
* **v1.0 (Official Launch & Extensibility):** Flathub Flatpak, AppImage, Arch AUR package. Subcommand extension manager (`tb ext install <name>`), official curated GitHub registry, and direct community installs from GitHub URLs (`tb ext install github.com/user/repo`).

### Phase 4: Long-Term Horizons
* **v1.5 (Heavy Media & OCR):** `tb-ocr` extension bundling Tesseract engine with downloadable language packs, searchable PDF generation (`ocrmypdf`), high-quality GIF generator (`gifski`).
* **v2.0 (The Local AI Era):** `tb-ai` extension bridging local Ollama/llama.cpp models for offline document summarization, rewriting, and offline Whisper speech-to-text; native Agent Skill / MCP server manifest.

---

## 8. Scope Boundaries: What is IN vs. OUT for v1.0

### ✅ Strictly IN for v1.0
* Statically linked Rust CLI (`tb`) + native Slint Desktop GUI.
* 35+ core built-in tools covering PDF, Image, Video/Audio (FFmpeg wrapper), Developer Data, and Everyday Utilities.
* Single-binary distribution (~15MB-20MB total footprint).
* Linux desktop support across all major distributions (Flatpak, AppImage, AUR).
* Decentralized extension installer (`tb ext install <github-url>`).
* 3-tier on-demand zero-sudo dependency engine.
* Manual Storage & Engines dashboard.

### ❌ Strictly OUT for v1.0 (Deferred to v1.5 / v2.0)
* **No Local AI / LLM Integration:** Zero Ollama/Whisper runtime requirements in v1.
* **No OCR / Scanned PDF Recognition:** Avoids bundling heavy Tesseract dictionaries and language packs during initial release.
* **No AI Background Removal:** Avoids bundling PyTorch/ONNX machine learning models.
* **No Video Timeline Editing:** Simple conversion and compression presets only; no multi-track editing.
* **No Custom Themes:** Ships with one high-contrast dark theme and one clean light theme.
* **No Complex PDF Form Signing:** No coordinate-based digital signature canvas.
* **No Automated Background Deletion:** Storage is strictly managed by user action.

---

## 9. Success Criteria & Metrics

1. **Performance SLA:**
   * Desktop application cold start in **< 50 milliseconds** (target <20ms).
   * Idle memory consumption under **25MB RAM**.
   * Zero crashes on Wayland, X11, older Intel HD GPUs, or VMs (via Slint's automatic software-render fallback).
2. **User Experience SLA:**
   * Any basic conversion or compression task completed in **less than 3 clicks** or **under 5 seconds** from cold launch.
3. **Ecosystem & Adoption (First 6 Months post-v1.0):**
   * Top trending Rust project on GitHub upon launch ("Show HN").
   * **2,500+ GitHub Stars** within 6 months.
   * Inclusion in the official Arch AUR and Flathub.
   * At least **5 community-contributed extensions** published independently on GitHub.
