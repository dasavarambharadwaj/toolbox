# 📊 Toolbox — Comprehensive Feature Research & Analysis

> **Prepared by:** Mary — Business Analyst
> **Date:** 2026-10-03
> **Project:** Toolbox — Privacy-First Offline CLI Utility Suite
> **Scope:** Complete feature inventory with build/skip verdicts

---

## Market Research Thesis

**Core Thesis:** Millions of users daily upload sensitive files to random online tools found via Google — tax returns, contracts, medical records, personal photos, proprietary code. The FBI has publicly warned about malicious file converters. Yet no single, open-source, cross-platform, consumer-friendly tool exists that covers all these use cases locally. The market is fragmented across developer-only tools (DevToys), single-purpose tools (Stirling-PDF), and intimidating power tools (CyberChef). The opportunity is a **unified CLI-first toolbox** that processes everything locally, is open-source for trust, modular for size, and AI-agent-friendly for the future.

**5-Year Vision:** As AI agents become the primary interface for computing, a CLI-first architecture becomes a **strategic moat**. AI agents (local LLMs, coding assistants) can invoke `tb pdf compress report.pdf` as naturally as calling any API. Users who care about privacy will run local AI agents that need local tools — not cloud APIs. The toolbox becomes the **hands** that local AI uses to manipulate files.

**Target Users:**
1. Privacy-conscious individuals (primary)
2. Developers & sysadmins (power users)
3. Local AI/LLM agents (programmatic consumers)
4. Small businesses avoiding cloud dependency
5. Users in bandwidth-constrained or restricted network environments

---

## Reading This Document

Each feature is analyzed with:

| Field | Meaning |
|-------|---------|
| **What it does** | Plain-English description |
| **Competitors charge?** | Free / Freemium / Paid on major platforms |
| **OSS Library** | Open-source library that enables local implementation |
| **Difficulty** | Easy / Medium / Hard / Very Hard |
| **Criticism** | Honest devil's advocate analysis — why this might fail or be wasted effort |
| **Verdict** | ✅ BUILD or ❌ SKIP (with reasoning) |

---

## Category 1: 📄 Document / PDF Processing

**Market Thesis:** PDF is the #1 file type people upload to online tools. iLovePDF alone gets 100M+ monthly visits. Every upload potentially exposes tax docs, legal contracts, medical records. This is the single highest-impact category.

**Criticism of Category:** Stirling-PDF (39k GitHub stars) already dominates the self-hosted PDF space. However, it requires Docker — a barrier for 95% of normal users. A CLI tool has zero setup friction. Stirling-PDF validates demand but doesn't serve the CLI/desktop audience.

| # | Feature | What It Does | Competitors Charge? | OSS Library | Difficulty | Criticism | Verdict |
|---|---------|-------------|---------------------|-------------|------------|-----------|---------|
| 1.1 | **Merge PDFs** | Combine multiple PDFs into one file | Free (with limits) | `pdfcpu`, `PyMuPDF`, `pdftk` | Easy | Extremely common need, trivial to implement. No reason not to build. Low-hanging fruit that validates the tool's existence. | ✅ BUILD |
| 1.2 | **Split / Extract Pages** | Pull specific pages out into a new PDF | Free (limited tasks/day) | `pdfcpu`, `PyMuPDF` | Easy | Complementary to merge. Users expect both. Trivial effort. | ✅ BUILD |
| 1.3 | **Compress PDF** | Reduce file size via image/font optimization | Free (basic), **Paid** (high compression) | `Ghostscript`, `pdfcpu`, `qpdf` | Medium | Quality-vs-size tradeoffs are nuanced. "Compress" can mean many things (downsampling images, stripping metadata, subsetting fonts). Need multiple compression levels. Worth it — this is the #1 reason people go to iLovePDF. | ✅ BUILD |
| 1.4 | **PDF → Image (JPG/PNG)** | Rasterize each PDF page to an image | Free | `poppler` (pdftoppm), `PyMuPDF` | Easy | Simple, high demand. Useful for previews, sharing pages as images. | ✅ BUILD |
| 1.5 | **Image → PDF** | Convert one or more images into a PDF | Free | `Pillow`, `ImageMagick` | Easy | Common need (scanning to PDF workflow). Trivial. | ✅ BUILD |
| 1.6 | **PDF → Word/Excel** | Convert PDF to editable .docx/.xlsx | **Often Paid / Premium** | `pdf2docx`, `tabula-py` | Hard | Layout preservation is notoriously difficult. Open-source results are often garbage compared to Adobe. **However**, this is a highly searched paid feature — even 70% quality would satisfy most users. Tables-to-Excel via `tabula` is actually quite good. | ✅ BUILD (with quality disclaimer) |
| 1.7 | **Add Watermark** | Stamp text or image overlay on pages | Free | `PyMuPDF`, `pdfrw` | Easy | Low effort, useful for branding/draft marking. | ✅ BUILD |
| 1.8 | **Rotate Pages** | Change page orientation | Free | `pdfcpu`, `PyMuPDF` | Easy | Trivial utility. Expected in any PDF tool. | ✅ BUILD |
| 1.9 | **Encrypt / Password-Protect** | Add password and permission restrictions | Free | `qpdf`, `pdfcpu`, `PyMuPDF` | Easy | Essential privacy feature. Directly aligned with the product's mission. | ✅ BUILD |
| 1.10 | **Unlock / Decrypt** | Remove password (user must know password) | Free | `qpdf`, `pdfcpu` | Easy | Important complement to encrypt. Not password cracking — requires the password. | ✅ BUILD |
| 1.11 | **OCR PDF** | Make scanned PDFs searchable/selectable | **Paid** (Smallpdf, iLovePDF, Adobe) | `ocrmypdf` (wraps Tesseract) | Medium | **This is money on the table.** Competitors charge for this. `ocrmypdf` is excellent and battle-tested. Requires Tesseract + language data (~30-100MB per language). Worth it — privacy-critical (scanned IDs, medical docs). | ✅ BUILD |
| 1.12 | **Flatten PDF** | Bake form fields/annotations into static pages | Free | `Ghostscript`, `PyMuPDF` | Easy | Niche but important for legal/compliance workflows. Low effort. | ✅ BUILD |
| 1.13 | **PDF → HTML** | Convert PDF content to web-readable HTML | Mixed | `pdf2htmlEX`, `PyMuPDF` | Hard | Quality varies wildly. Complex layouts rarely convert well. Users who need this usually need very specific fidelity. | ❌ SKIP (for now — revisit in Phase 3) |
| 1.14 | **E-Signature / Stamp** | Overlay a signature image at specific coordinates on a PDF | Free tools exist, **DocuSign/Adobe Sign are paid** | `PyPDF2`, `reportlab`, `pdftk` | Medium | Huge demand ("sign PDF online" is massive). Users hate uploading tax forms and legal docs to random signing sites. Coordinate-based placement is fiddly but doable. CLI: `tb pdf sign doc.pdf --signature sig.png --page 3 --position bottom-right` | ✅ BUILD |
| 1.15 | **PDF Form Filling** | Programmatically fill form fields | Mixed | `pdftk`, `PyMuPDF` | Medium | Useful for batch operations and AI agents. Medium effort but high programmatic value. | ✅ BUILD |

**Category Verdict: 14 of 15 features → BUILD**

---

## Category 2: 🖼️ Image Processing

**Market Thesis:** Image manipulation is the second most common online tool use case. TinyPNG, Remove.bg, and iLoveIMG collectively serve hundreds of millions of users. Background removal alone (Remove.bg) charges $1.99/image for high-res.

**Criticism of Category:** ImageMagick already does most of this via CLI. But ImageMagick's syntax is arcane (`magick convert input.jpg -resize 50% -quality 85 output.jpg`). The value-add is making it **human-readable** (`tb image resize photo.jpg --width 800`) and bundling AI features (background removal) that ImageMagick can't do.

| # | Feature | What It Does | Competitors Charge? | OSS Library | Difficulty | Criticism | Verdict |
|---|---------|-------------|---------------------|-------------|------------|-----------|---------|
| 2.1 | **Format Conversion** | Convert between JPG, PNG, WebP, AVIF, BMP, TIFF, HEIC, SVG→PNG | Free | `ImageMagick`, `Sharp`, `Pillow` | Easy | Table-stakes feature. Must support HEIC (iPhone photos) and WebP (web). | ✅ BUILD |
| 2.2 | **Lossy/Lossless Compression** | Reduce file size (like TinyPNG) | Free (file limits) | `pngquant`, `mozjpeg`, `oxipng` | Easy | High demand. TinyPNG's entire business is this. `pngquant` + `mozjpeg` produce results equal to or better than online tools. | ✅ BUILD |
| 2.3 | **Resize / Scale** | Change dimensions (by pixels, percentage, or max dimension) | Free | `ImageMagick`, `Pillow`, `Sharp` | Easy | Fundamental operation. Every image tool has this. | ✅ BUILD |
| 2.4 | **Crop** | Extract a region of the image | Free | `ImageMagick`, `Pillow` | Easy | Basic, expected. CLI: `tb image crop photo.jpg --rect 100,100,500,400` | ✅ BUILD |
| 2.5 | **Background Removal** | AI-powered foreground isolation | Free (low-res), **$1.99/image** (high-res on Remove.bg) | `rembg` (U²-Net model) | Easy-Medium | **This is a premium feature competitors charge for.** `rembg` works shockingly well locally. Model is ~170MB. No GPU required (CPU works, just slower). This is a marquee feature that demonstrates the tool's value. | ✅ BUILD |
| 2.6 | **Image Upscaling (AI Super-Resolution)** | Enlarge images without quality loss using AI | **Paid** (Canva Pro, dedicated services) | `Real-ESRGAN`, `waifu2x` | Hard | Requires large models (~60-100MB) and benefits enormously from GPU. CPU inference is slow (30s+ per image). Still valuable for privacy-conscious users processing sensitive images. Worth building but marking as "optional heavy feature." | ✅ BUILD (optional module) |
| 2.7 | **Add Watermark** | Overlay text or image | Free | `Pillow`, `ImageMagick` | Easy | Simple, useful for photographers and content creators. | ✅ BUILD |
| 2.8 | **Blur Faces / Redact** | Auto-detect and blur faces for anonymization | Usually Free | `OpenCV` (Haar Cascades, DNN) | Medium | Privacy-aligned feature. Useful for sharing photos publicly while protecting identity. Requires bundling a face detection model (~10MB). | ✅ BUILD |
| 2.9 | **Strip EXIF / Metadata** | Remove GPS, camera info, timestamps from images | Free | `Pillow`, `exiftool` | Easy | Core privacy feature. EXIF data reveals your location, device, and timestamp. Must-have for the privacy mission. | ✅ BUILD |
| 2.10 | **Batch Processing** | Apply any operation to multiple files at once | Often **Paid** (premium tier) | All above libraries | Medium | Essential for CLI. `tb image compress *.jpg --quality 80`. Competitors often gate batch behind premium. Free in our tool = competitive advantage. | ✅ BUILD |
| 2.11 | **SVG Optimization** | Minify and clean SVG files | Free | `svgo` (Node.js) | Easy | Useful for web developers. `svgo` is excellent. | ✅ BUILD |
| 2.12 | **Image to ASCII Art** | Convert image to text-art representation | Free (novelty) | Various libraries | Easy | Fun and demonstrates CLI capabilities, but extremely niche. Low priority. | ❌ SKIP (novelty, not core) |

**Category Verdict: 11 of 12 features → BUILD**

---

## Category 3: 🎬 Video & Audio Processing

**Market Thesis:** Video processing is heavily monetized online. Kapwing charges $24/mo, Clideo $9/mo. Free tiers add watermarks and cap at 720p. Yet FFmpeg (free, open-source) can do everything — it's just impossibly hard to use for normal humans.

**Criticism of Category:** FFmpeg is a dependency behemoth (~80-150MB binary). Bundling it inflates the tool massively. Solution: download-on-demand to `~/.toolbox/deps/`. Also, video processing is CPU/GPU intensive — large files will be slow regardless.

| # | Feature | What It Does | Competitors Charge? | OSS Library | Difficulty | Criticism | Verdict |
|---|---------|-------------|---------------------|-------------|------------|-----------|---------|
| 3.1 | **Format Conversion** | Convert between MP4, MKV, WebM, AVI, MOV, MP3, WAV, FLAC, OGG, AAC | Free (size/time limits) | `FFmpeg` | Easy | The #1 reason people visit online video tools. FFmpeg wrapper with sane defaults. | ✅ BUILD |
| 3.2 | **Compress Video** | Reduce bitrate/resolution for smaller files | Free (limited) | `FFmpeg` (libx264/libx265 CRF) | Medium | Second most common need. CRF tuning needs sensible presets (low/medium/high compression). | ✅ BUILD |
| 3.3 | **Trim / Cut** | Extract a specific time range | Free (watermarked) | `FFmpeg` (`-ss`, `-t`, `-to`) | Easy | Very common. `tb video trim input.mp4 --from 00:01:30 --to 00:03:00` | ✅ BUILD |
| 3.4 | **Extract Audio** | Pull audio track from video as MP3/WAV | Free | `FFmpeg` (`-vn`) | Easy | Extremely common Google search: "extract audio from video." | ✅ BUILD |
| 3.5 | **Merge / Concatenate** | Join multiple clips into one | Free (watermarked) | `FFmpeg` (concat demuxer) | Medium | Format mismatches between clips cause errors. Need to handle re-encoding gracefully. | ✅ BUILD |
| 3.6 | **GIF Creation** | Video clip → optimized animated GIF | Free | `FFmpeg`, `gifski` | Medium | Very popular. `gifski` produces much better quality than FFmpeg's native GIF output. | ✅ BUILD |
| 3.7 | **Change Speed** | Speed up or slow down playback | Free | `FFmpeg` (setpts, atempo) | Medium | Useful but less commonly searched. Low effort since it's just FFmpeg flags. | ✅ BUILD |
| 3.8 | **Audio Normalization** | Level out volume across tracks | Free | `FFmpeg` (loudnorm), `SoX` | Easy | Important for podcast/video creators. Simple FFmpeg filter. | ✅ BUILD |
| 3.9 | **Auto-Subtitles / Transcription** | Speech-to-text → SRT subtitle file | **Paid** (Kapwing Pro, Otter.ai) | `whisper.cpp`, OpenAI Whisper | Hard | **Premium feature competitors charge for.** `whisper.cpp` runs locally but requires downloading models (39MB-1.5GB depending on quality). CPU inference is viable but slow for long videos. GPU dramatically speeds it up. Incredibly valuable for privacy (meeting recordings, legal depositions). | ✅ BUILD (optional module) |
| 3.10 | **Add Subtitles / Burn-in** | Overlay SRT subtitles onto video | Free | `FFmpeg` (subtitles filter) | Easy | Natural companion to 3.9. | ✅ BUILD |
| 3.11 | **Video Thumbnail / Screenshot** | Extract a frame at a timestamp | Free | `FFmpeg` (`-ss -frames:v 1`) | Easy | Trivial but surprisingly commonly googled. | ✅ BUILD |
| 3.12 | **Audio Conversion** | Convert between MP3, WAV, FLAC, OGG, AAC, M4A | Free | `FFmpeg` | Easy | Overlaps with 3.1 but should be a dedicated subcommand for discoverability. | ✅ BUILD |
| 3.13 | **Video Resolution Change** | Downscale 4K → 1080p → 720p etc. | Free (limited) | `FFmpeg` (`-vf scale=`) | Easy | Common need for reducing file size for sharing. | ✅ BUILD |
| 3.14 | **Remove Audio** | Strip audio track from video (mute) | Free | `FFmpeg` (`-an`) | Easy | Simple, useful for creating silent loops/backgrounds. | ✅ BUILD |

**Category Verdict: 14 of 14 features → BUILD** (2 as optional heavy modules)

---

## Category 4: 👁️ OCR & Text Extraction

**Market Thesis:** OCR is a premium feature across the board. Adobe charges for it. Google Cloud Vision and Amazon Textract are usage-based APIs. Meanwhile, Tesseract OCR (open source) is mature and excellent. This is free money — offering locally what others charge for.

**Criticism of Category:** Tesseract is great for printed text but struggles with handwriting. Advanced features (layout-preserving OCR, form parsing) require heavy ML models that bloat the tool. Also, OCR quality depends heavily on image quality — users may blame the tool for bad input.

| # | Feature | What It Does | Competitors Charge? | OSS Library | Difficulty | Criticism | Verdict |
|---|---------|-------------|---------------------|-------------|------------|-----------|---------|
| 4.1 | **Basic Text Extraction** | Extract plain text from images/scanned PDFs | Free (few pages), **Paid** (bulk) | `Tesseract OCR`, `EasyOCR` | Easy | Core feature. Tesseract supports 100+ languages. Model data is ~15-30MB per language. | ✅ BUILD |
| 4.2 | **Searchable PDF** | Run OCR and embed invisible text layer into PDF | **Paid** (Adobe, Smallpdf) | `ocrmypdf` | Medium | Already covered in 1.11 but should also be accessible via `tb ocr` subcommand. | ✅ BUILD |
| 4.3 | **Layout-Preserving OCR** | Maintain table/column/paragraph structure | **Paid** (Textract, Cloud Vision) | `PaddleOCR`, `LayoutParser` | Hard | This is where open-source falls behind cloud services. `PaddleOCR` is improving rapidly but results are inconsistent. Worth attempting but managing user expectations. Models are 100-200MB. | ✅ BUILD (best-effort, optional module) |
| 4.4 | **Handwriting Recognition** | Read handwritten notes | **Paid** (Cloud APIs only) | `TrOCR` (HuggingFace) | Hard | Accuracy is highly variable. Models are large (300MB+). Impressive when it works, frustrating when it doesn't. Useful for digitizing notes but must be clearly labeled as experimental. | ✅ BUILD (experimental, optional module) |
| 4.5 | **Form / Invoice Parsing** | Extract key-value pairs (Total, Date, Address) from structured docs | **Paid** (Enterprise APIs) | `Donut`, `LayoutLM` | Very Hard | Requires specialized ML models trained on invoice layouts. Extremely complex to generalize. Cloud services charge significantly for this because it's genuinely hard. | ❌ SKIP (too complex, too niche for v1. Revisit when local LLMs can handle this via prompting) |
| 4.6 | **Screenshot Text Extraction** | Quick OCR on a screenshot or clipboard image | Free (basic) | `Tesseract`, OS clipboard APIs | Easy | Extremely useful for developers. `tb ocr screenshot.png` or pipe from clipboard. | ✅ BUILD |
| 4.7 | **Batch OCR** | Process entire folders of scanned documents | **Paid** (premium tier) | `ocrmypdf`, `Tesseract` | Medium | Essential for digitization projects. Competitors gate this behind payment. Free = competitive advantage. | ✅ BUILD |

**Category Verdict: 6 of 7 features → BUILD** (2 as optional/experimental modules)

---

## Category 5: ✍️ Grammar & Writing Tools

**Market Thesis:** Grammarly has 30M+ daily active users. ProWritingAid and QuillBot are growing fast. Users paste confidential emails, legal drafts, and internal memos into these tools. LanguageTool is the only viable offline alternative but requires a Java runtime.

**Criticism of Category:** This is the hardest category to compete in. Grammarly's quality comes from proprietary ML models. LanguageTool's open-source rules are decent for basic grammar but nowhere near Grammarly's Premium. Paraphrasing/rewriting requires LLM-level capability. **However**, with local LLMs (Ollama) becoming viable, this becomes achievable as an optional AI-powered module.

| # | Feature | What It Does | Competitors Charge? | OSS Library | Difficulty | Criticism | Verdict |
|---|---------|-------------|---------------------|-------------|------------|-----------|---------|
| 5.1 | **Spell Check** | Fix typos and misspellings | Free | `LanguageTool`, `hunspell` | Easy | Basic and expected. `hunspell` is lightweight and well-established. | ✅ BUILD |
| 5.2 | **Basic Grammar Check** | Catch common grammar mistakes (subject-verb agreement, articles, etc.) | Free (basic) | `LanguageTool` (offline Java server) | Medium | LanguageTool requires a Java runtime (~50MB). The free rule set is limited compared to Grammarly. Good enough for 80% of users. Worth building. | ✅ BUILD |
| 5.3 | **Style & Tone Suggestions** | Passive voice detection, readability scores, tone adjustment | **Paid** (Grammarly Premium, ProWritingAid) | `LanguageTool` (limited free rules) | Hard | Free LanguageTool rules cover some style issues but not tone. Full parity requires LLM. Build basic (readability scores, passive voice) now; enhance with local AI later. | ✅ BUILD (basic now, AI-enhanced later) |
| 5.4 | **Paraphrasing / Rewriting** | Rephrase sentences in different styles | **Paid** (QuillBot Premium) | HuggingFace models (T5, BART), local LLMs | Hard | Requires local LLM inference. Quality depends on model size and hardware. With Ollama integration, this becomes `tb text paraphrase --model llama3`. | ✅ BUILD (requires local AI module) |
| 5.5 | **Plagiarism Check** | Cross-reference text against web sources | **Paid** (all services) | None (requires web crawling) | Very Hard | **This fundamentally cannot work offline.** Plagiarism detection requires comparing against a web-scale corpus. Cannot be local by definition. Antithetical to the offline-first mission. | ❌ SKIP (impossible offline) |
| 5.6 | **Text Summarization** | Generate TL;DR of long text | Paid/Freemium | Local LLMs (Ollama), HuggingFace | Medium | Natural use case for local LLMs. `tb text summarize document.txt --model llama3` | ✅ BUILD (requires local AI module) |
| 5.7 | **Word / Character Count** | Count words, characters, sentences, paragraphs, reading time | Free | Standard libraries | Easy | Trivial but surprisingly commonly searched. Must-have basic utility. | ✅ BUILD |
| 5.8 | **Text Diff / Compare** | Show differences between two text files/strings | Free | `diff-match-patch`, `difflib` | Easy | Very useful. `tb text diff file1.txt file2.txt`. CLI-native with colored output. | ✅ BUILD |

**Category Verdict: 7 of 8 features → BUILD** (3 depend on local AI module)

---

## Category 6: 💻 Developer Utilities

**Market Thesis:** DevToys (24k stars) and CyberChef (27k stars) prove massive developer demand. These tools are already free and local — but DevToys is Windows-only and CyberChef's UI is intimidating. A CLI-first approach serves developers better anyway (piping, scripting, CI/CD integration).

**Criticism of Category:** Developers already have CLI tools for most of this (`jq`, `base64`, `openssl`). The value-add is **discoverability** — a single namespace where you don't have to remember if it's `echo -n 'text' | base64` or `base64 -w 0 <<< 'text'`. Also, cross-platform consistency (same command on macOS, Linux, Windows).

| # | Feature | What It Does | Competitors Charge? | OSS Library | Difficulty | Criticism | Verdict |
|---|---------|-------------|---------------------|-------------|------------|-----------|---------|
| 6.1 | **JSON Format / Validate** | Pretty-print, minify, validate JSON | Free | Standard `json` libs | Easy | Most-used dev utility. Must-have. | ✅ BUILD |
| 6.2 | **XML/HTML Format** | Pretty-print and validate XML/HTML | Free | `xml.dom`, `lxml` | Easy | Common need for API developers. | ✅ BUILD |
| 6.3 | **SQL Formatter** | Pretty-print SQL queries | Free | `sqlparse` | Easy | Frequently searched. Trivial to build. | ✅ BUILD |
| 6.4 | **YAML ↔ JSON ↔ TOML** | Convert between configuration formats | Free | Standard libs, `pyyaml`, `toml` | Easy | Developers constantly need this. Very low effort. | ✅ BUILD |
| 6.5 | **Base64 Encode/Decode** | Encode/decode strings and files to/from Base64 | Free | Standard libs (`base64`) | Easy | Staple utility. `tb dev base64 encode "hello"` or `tb dev base64 decode file.b64` | ✅ BUILD |
| 6.6 | **URL Encode/Decode** | Percent-encoding for URLs | Free | Standard libs (`urllib`) | Easy | Common need. Trivial. | ✅ BUILD |
| 6.7 | **HTML Entity Encode/Decode** | Convert `&amp;` ↔ `&` etc. | Free | Standard libs | Easy | Niche but easy. | ✅ BUILD |
| 6.8 | **JWT Decode** | Decode JWT tokens (header + payload) without verification | Free | `base64` decoding | Easy | Every web developer needs this. Currently most use jwt.io, which means pasting auth tokens into a website. Privacy nightmare. | ✅ BUILD |
| 6.9 | **Hash Generation** | MD5, SHA1, SHA256, SHA512, BLAKE2 of strings and files | Free | Standard libs (`hashlib`, `crypto`) | Easy | File integrity verification. Core security feature. | ✅ BUILD |
| 6.10 | **UUID Generator** | Generate V4, V5, V7 UUIDs | Free | Standard libs (`uuid`) | Easy | Quick utility. | ✅ BUILD |
| 6.11 | **Regex Tester** | Test regex patterns against input strings with match highlighting | Free | Standard libs (`re`, `regexp`) | Easy | Developers use Regex101.com — pasting potentially sensitive patterns/data. | ✅ BUILD |
| 6.12 | **Lorem Ipsum Generator** | Generate placeholder text | Free | Simple algorithm | Easy | Common but trivial. | ✅ BUILD |
| 6.13 | **Timestamp Converter** | Unix epoch ↔ Human-readable date, timezone conversion | Free | Standard datetime libs | Easy | "Unix timestamp converter" is a daily search for devs. | ✅ BUILD |
| 6.14 | **Number Base Converter** | Hex ↔ Dec ↔ Oct ↔ Bin | Free | Standard libs | Easy | Common quick-lookup tool. | ✅ BUILD |
| 6.15 | **Cron Expression Parser** | Explain what a cron expression means in plain English, and next N run times | Free | `croniter` or custom parser | Easy | Developers constantly google "cron expression explain." | ✅ BUILD |
| 6.16 | **Code Screenshot (Carbon.sh alternative)** | Generate beautiful code snippet images | Free (Carbon.sh) | `silicon` (Rust) | Medium | Carbon.sh requires uploading code to a web service. `silicon` generates identical output locally. Privacy win for proprietary code. | ✅ BUILD |
| 6.17 | **Minify JS/CSS/HTML** | Remove whitespace and compress code | Free | `terser` (JS), `csso` (CSS) | Medium | Requires bundling language-specific parsers. Useful but adds complexity. Worth it for web developers. | ✅ BUILD |
| 6.18 | **Slug Generator** | Convert text to URL-friendly slugs | Free | Simple string manipulation | Easy | Trivial utility. | ✅ BUILD |
| 6.19 | **Color Converter** | HEX ↔ RGB ↔ HSL ↔ CMYK | Free | Pure math | Easy | Designers and frontend devs google this constantly. | ✅ BUILD |

**Category Verdict: 19 of 19 features → BUILD**

---

## Category 7: 📦 File Compression & Archives

**Market Thesis:** Users on locked-down corporate machines (no admin rights to install 7-Zip) google "extract RAR online" and upload entire company backups to random sites. The privacy exposure is extreme since archives contain entire directory trees.

**Criticism of Category:** 7-Zip and native OS archivers already exist. But this category is about being part of the unified toolbox — not a standalone archive app. And on macOS, native archive support is limited (no RAR, no 7z).

| # | Feature | What It Does | Competitors Charge? | OSS Library | Difficulty | Criticism | Verdict |
|---|---------|-------------|---------------------|-------------|------------|-----------|---------|
| 7.1 | **Create Archives** | Compress files/folders to ZIP, TAR.GZ, 7Z | Free | `libarchive`, `p7zip` | Easy | Basic utility. Must support common formats. | ✅ BUILD |
| 7.2 | **Extract Archives** | Decompress ZIP, RAR, 7Z, TAR, GZ, BZ2, XZ, ZSTD | Free | `libarchive`, `p7zip`, `unrar` | Easy | Must handle all common formats. RAR requires `unrar` (free for extraction). | ✅ BUILD |
| 7.3 | **List Contents** | Preview archive contents without extracting | Free | `libarchive` | Easy | Useful for verifying before extraction. `tb archive list backup.7z` | ✅ BUILD |
| 7.4 | **Password-Protected Archives** | Create/extract encrypted archives | Free | `7z`, `zip` (AES-256) | Easy | Security feature aligned with privacy mission. | ✅ BUILD |
| 7.5 | **Split Archives** | Split large archives into multi-volume parts | Free | `7z`, `split` | Easy | Useful for sharing large files via email or limited storage. | ✅ BUILD |
| 7.6 | **Compression Level Control** | Adjust speed vs. size tradeoff | Free | All archive libs | Easy | Power-user feature. `tb archive create --level max folder/` | ✅ BUILD |

**Category Verdict: 6 of 6 features → BUILD**

---

## Category 8: 📊 Data / Spreadsheet Tools

**Market Thesis:** Data transformation is surprisingly common — users convert CSV↔JSON, clean up exported data, merge spreadsheets. Online tools like ConvertCSV and Zamzar handle these, exposing customer lists, financial data, and employee records.

**Criticism of Category:** Power users already have `jq`, `csvkit`, `xsv`, `pandas`. But these each have different syntaxes and installation procedures. Unifying under one tool adds value for discoverability and cross-platform consistency.

| # | Feature | What It Does | Competitors Charge? | OSS Library | Difficulty | Criticism | Verdict |
|---|---------|-------------|---------------------|-------------|------------|-----------|---------|
| 8.1 | **CSV ↔ JSON** | Convert between tabular and structured formats | Free | `csvkit`, standard libs | Easy | Extremely common developer need. | ✅ BUILD |
| 8.2 | **CSV ↔ Excel (.xlsx)** | Convert between CSV and Excel format | Free | `openpyxl`, `calamine` (Rust) | Medium | Business users need this. Excel is proprietary format but well-documented. | ✅ BUILD |
| 8.3 | **JSON → CSV** | Flatten nested JSON to tabular format | Free | Custom logic, `jq` | Medium | JSON flattening strategies vary. Need sensible defaults for nested structures. | ✅ BUILD |
| 8.4 | **Query CSV with SQL** | Run SQL queries on CSV files | Free | `csvq`, `sqlite` (import) | Medium | Power feature. `tb data query "SELECT name, email FROM users.csv WHERE age > 25"` | ✅ BUILD |
| 8.5 | **Sort / Filter / Deduplicate** | Clean data operations on CSV | Free | `csvkit`, `xsv`, `awk` | Easy | Common data prep tasks. | ✅ BUILD |
| 8.6 | **TSV ↔ CSV** | Tab-separated ↔ comma-separated | Free | Standard libs | Easy | Trivial but frequently needed. | ✅ BUILD |
| 8.7 | **Excel → PDF** | Render spreadsheet as PDF | Mixed | `libreoffice` (headless) | Hard | Requires LibreOffice binary (~300MB) or complex rendering. Dependencies too heavy. | ❌ SKIP (dependency too large for a CLI tool) |

**Category Verdict: 6 of 7 features → BUILD**

---

## Category 9: 🔐 Security & Encryption

**Market Thesis:** Users google "generate strong password", "encrypt file online", "check file hash" — and paste credentials or upload files to unknown sites. Security tools are the most ironic category for online use.

**Criticism of Category:** Most of these are thin wrappers around standard crypto libraries. Low effort, high value. The criticism is that sophisticated users already know `openssl` and `gpg`. But the value is in simplicity and discoverability — `tb sec password` vs `openssl rand -base64 32 | tr -dc 'a-zA-Z0-9!@#$%' | head -c 20`.

| # | Feature | What It Does | Competitors Charge? | OSS Library | Difficulty | Criticism | Verdict |
|---|---------|-------------|---------------------|-------------|------------|-----------|---------|
| 9.1 | **Password Generator** | Generate strong random passwords with customizable rules | Free | Standard crypto libs | Easy | Massive search volume. Trivial to build. | ✅ BUILD |
| 9.2 | **Passphrase Generator** | Generate memorable multi-word passphrases (Diceware) | Free | Wordlist + RNG | Easy | Growing in popularity. More secure and memorable than passwords. | ✅ BUILD |
| 9.3 | **File Encryption (symmetric)** | Encrypt/decrypt files with password using AES-256 | Free | `age` (Go), `openssl` | Medium | Directly aligned with privacy mission. `age` is modern and simple. | ✅ BUILD |
| 9.4 | **File Checksum / Hash** | Generate and verify MD5, SHA256, SHA512, BLAKE3 checksums | Free | Standard libs (`hashlib`, `crypto`) | Easy | Essential for verifying downloads. Already partially in 6.9 but file-oriented here. | ✅ BUILD |
| 9.5 | **SSH Key Generation** | Generate ed25519/RSA key pairs | Free | `ssh-keygen`, `crypto` libs | Easy | Common developer need. Currently requires knowing `ssh-keygen` syntax. | ✅ BUILD |
| 9.6 | **Certificate Inspector** | View SSL certificate details, chain, expiry | Free (online tools) | Go/Rust `crypto/tls` + `x509` | Medium | Developers and sysadmins check this online. `tb sec cert example.com` or `tb sec cert cert.pem` | ✅ BUILD |
| 9.7 | **TOTP Generator** | Generate time-based one-time passwords (2FA codes) | Free | Standard HMAC libs | Easy | Useful as a backup 2FA. CLI: `tb sec totp --secret JBSWY3DPEHPK3PXP` | ✅ BUILD |
| 9.8 | **File Shredder** | Securely delete files (overwrite with random data) | Free | OS-level file operations | Easy | Privacy feature. Note: limited effectiveness on SSDs due to wear leveling. Document this honestly. | ✅ BUILD (with SSD caveat documentation) |

**Category Verdict: 8 of 8 features → BUILD**

---

## Category 10: 📱 QR Code & Barcode

**Market Thesis:** "Create QR code" is a massive search term. Many online QR generators inject tracking redirects or add paywalls for customization. Local generation is cleaner and more trustworthy.

**Criticism of Category:** Narrower scope than other categories. But extremely low effort to build and frequently requested. Good ROI.

| # | Feature | What It Does | Competitors Charge? | OSS Library | Difficulty | Criticism | Verdict |
|---|---------|-------------|---------------------|-------------|------------|-----------|---------|
| 10.1 | **Generate QR Code** | Create QR from URL, text, WiFi config, vCard, email | Free (basic), **Paid** (customized) | `qrencode` (C), Go/Rust QR libs | Easy | High demand, trivial to build. | ✅ BUILD |
| 10.2 | **Decode QR Code** | Read QR from image file | Free | `zbar`, `quirc` | Easy | Complement to generation. `tb qr decode photo.png` | ✅ BUILD |
| 10.3 | **Customize QR** | Colors, embedded logo, error correction level | **Paid** (premium QR services) | Custom rendering on top of QR libs | Medium | Competitors charge for this. Differentiator. | ✅ BUILD |
| 10.4 | **Generate Barcode** | Create Code128, EAN-13, UPC-A, Code39 barcodes | Free | `python-barcode`, Go/Rust barcode libs | Easy | Less common than QR but useful for inventory/retail. | ✅ BUILD |
| 10.5 | **Batch QR Generation** | Generate multiple QR codes from a CSV/list | **Paid** | QR libs + loop | Easy | Competitors charge for batch. Free = advantage. `tb qr batch urls.csv -o qr_codes/` | ✅ BUILD |

**Category Verdict: 5 of 5 features → BUILD**

---

## Category 11: 📝 Markdown & Document Authoring

**Market Thesis:** Developers and students constantly google "markdown to PDF" and use online converters, pasting documentation and notes into unknown services.

**Criticism of Category:** `pandoc` already does all of this and is the gold standard. But pandoc is 100MB+ and has complex flags. The value is wrapping it with sane defaults. However, bundling pandoc inflates the tool. Solution: download-on-demand.

| # | Feature | What It Does | Competitors Charge? | OSS Library | Difficulty | Criticism | Verdict |
|---|---------|-------------|---------------------|-------------|------------|-----------|---------|
| 11.1 | **Markdown → HTML** | Render markdown to HTML | Free | `goldmark` (Go), `pulldown-cmark` (Rust), `marked` | Easy | Very common. Lightweight, no heavy deps. | ✅ BUILD |
| 11.2 | **Markdown → PDF** | Render markdown to printable PDF | Free (online) | `pandoc` + `weasyprint` or `wkhtmltopdf` | Medium-Hard | Requires heavy dependencies (pandoc or a headless renderer). Download-on-demand. Worth it — very high demand. | ✅ BUILD (on-demand dep) |
| 11.3 | **HTML → Markdown** | Convert web pages / HTML to markdown | Free | `html2text`, `turndown` | Easy | Useful for content migration. Lightweight. | ✅ BUILD |
| 11.4 | **Document Format Conversion** | DOCX → Markdown, DOCX → PDF, etc. | Free/Mixed | `pandoc` | Medium | Pandoc handles this well. Wrap with sane defaults. | ✅ BUILD (on-demand dep) |
| 11.5 | **Markdown Table of Contents** | Auto-generate TOC from headers | Free | Custom parser | Easy | Small but useful utility. | ✅ BUILD |
| 11.6 | **Resume Builder** | Generate PDF resume from YAML/JSON/Markdown template | Niche tools exist | `pandoc` + templates | Medium | Niche but extremely privacy-relevant (resume = SSN, address, employment history). People upload resumes to random builder sites constantly. | ✅ BUILD (Phase 3) |

**Category Verdict: 6 of 6 features → BUILD**

---

## Category 12: 🔍 Metadata Tools

**Market Thesis:** EXIF data in photos reveals your exact GPS location, device model, and timestamps. Users search "remove EXIF data" and upload personal photos to online tools — ironically exposing the very data they're trying to remove.

**Criticism of Category:** `exiftool` already does this perfectly. The value is including it in the unified namespace and simplifying the interface. But `exiftool` requires Perl runtime. Alternative: Go/Rust native EXIF libraries.

| # | Feature | What It Does | Competitors Charge? | OSS Library | Difficulty | Criticism | Verdict |
|---|---------|-------------|---------------------|-------------|------------|-----------|---------|
| 12.1 | **View Metadata** | Display all metadata (EXIF, IPTC, XMP, ID3) for any file | Free | `exiftool`, Go/Rust EXIF libs, `ffprobe` | Easy | Essential privacy awareness tool. | ✅ BUILD |
| 12.2 | **Strip All Metadata** | Remove all metadata from files (images, audio, video, PDFs) | Free | `exiftool -all=`, `mat2` | Easy | Core privacy feature. `tb meta strip photo.jpg` — removes GPS, camera info, everything. | ✅ BUILD |
| 12.3 | **Edit Specific Tags** | Modify individual metadata fields | Free | `exiftool`, EXIF write libs | Medium | Power-user feature. Photographers need to edit copyright/title fields. | ✅ BUILD |
| 12.4 | **Batch Metadata Strip** | Remove metadata from entire folders | Free (limited) | Same libs + glob | Easy | `tb meta strip *.jpg --recursive` | ✅ BUILD |

**Category Verdict: 4 of 4 features → BUILD**

---

## Category 13: 🌐 Network Utilities

**Market Thesis:** Sysadmins and developers use online tools for DNS lookups, SSL checks, and API testing. Some of these expose internal hostnames, API endpoints, and authentication details.

**Criticism of Category:** These inherently require network access — they're not "offline" tools. But they run locally, meaning the data doesn't go through a third-party web service. The distinction is: your machine talks to the target directly, not through someone else's proxy.

| # | Feature | What It Does | Competitors Charge? | OSS Library | Difficulty | Criticism | Verdict |
|---|---------|-------------|---------------------|-------------|------------|-----------|---------|
| 13.1 | **DNS Lookup** | Resolve A, AAAA, MX, TXT, CNAME records | Free | `net` (Go), `dnspython` | Easy | Common search. `tb net dns example.com --type MX` | ✅ BUILD |
| 13.2 | **WHOIS** | Domain registration info | Free | `whois` command, Go libs | Easy | Frequently googled. | ✅ BUILD |
| 13.3 | **SSL Certificate Check** | Inspect remote SSL cert, chain, expiry | Free (online) | `crypto/tls` stdlib | Medium | Overlaps with 9.6 but network-oriented. `tb net ssl example.com` | ✅ BUILD |
| 13.4 | **HTTP Request / API Test** | cURL-like HTTP client with response formatting | Free | `net/http`, `curl` | Medium | Simplified cURL. `tb net http GET https://api.example.com --header "Auth: Bearer xxx"`. Keeps auth tokens local. | ✅ BUILD |
| 13.5 | **Port Scanner** | Check open ports on a host | Free | Native socket libs | Medium | Useful for sysadmins. Could raise security concerns if misused. Document responsibly. | ✅ BUILD (with responsible-use docs) |
| 13.6 | **IP / Geolocation Info** | Look up info about an IP address | Free | MaxMind GeoLite2 (offline DB) | Medium | Can use offline GeoIP database. Privacy-friendly alternative to online lookups. | ✅ BUILD |
| 13.7 | **Ping / Traceroute** | Network diagnostic tools | Free | OS-level commands | Easy | Thin wrappers but useful for cross-platform consistency. | ❌ SKIP (too thin a wrapper over OS commands — just use ping/traceroute) |

**Category Verdict: 6 of 7 features → BUILD**

---

## Category 14: 🔤 Text Manipulation

**Market Thesis:** CyberChef and TextMechanic prove that simple text transformations are heavily used. Users paste sensitive data into online tools for trivial operations like case conversion or line sorting.

**Criticism of Category:** These are almost embarrassingly simple to implement. But that's the point — they're quick wins that bulk up the tool's utility for near-zero effort.

| # | Feature | What It Does | Competitors Charge? | OSS Library | Difficulty | Criticism | Verdict |
|---|---------|-------------|---------------------|-------------|------------|-----------|---------|
| 14.1 | **Case Conversion** | UPPER, lower, Title, camelCase, snake_case, kebab-case, PascalCase | Free | Standard string ops | Easy | Very commonly googled. `tb text case --to snake "MyVariable"` | ✅ BUILD |
| 14.2 | **Sort Lines** | Alphabetical, numerical, reverse, random | Free | Standard sort | Easy | Trivial. | ✅ BUILD |
| 14.3 | **Deduplicate Lines** | Remove duplicate lines | Free | Set/hash operations | Easy | Useful for cleaning lists. | ✅ BUILD |
| 14.4 | **Find & Replace (with Regex)** | Search and replace in text with regex support | Free | Standard regex | Easy | Power feature. `tb text replace input.txt --find "foo" --replace "bar" --regex` | ✅ BUILD |
| 14.5 | **Line Numbers** | Add/remove line numbers | Free | String manipulation | Easy | Trivial utility. | ✅ BUILD |
| 14.6 | **Trim / Strip Whitespace** | Remove leading/trailing whitespace from lines | Free | Standard string ops | Easy | Common text cleaning operation. | ✅ BUILD |
| 14.7 | **Reverse Text/Lines** | Reverse string or line order | Free | Standard ops | Easy | Quick utility. | ✅ BUILD |
| 14.8 | **Extract Emails/URLs/IPs** | Regex extraction of common patterns from text | Free | Standard regex | Easy | Very useful for data processing. | ✅ BUILD |
| 14.9 | **Count Frequency** | Word/character frequency analysis | Free | Hash map | Easy | Useful for text analysis. | ✅ BUILD |

**Category Verdict: 9 of 9 features → BUILD**

---

## Category 15: 🤖 Local AI Processing

**Market Thesis:** Users increasingly upload sensitive documents to ChatGPT, Claude, and Gemini for summarization, translation, and rewriting. Corporate IP, legal documents, and personal data flow to cloud AI providers. Local LLMs (Ollama, llama.cpp) are now viable on consumer hardware.

**Criticism of Category:** Local AI quality is 60-80% of cloud AI (GPT-4, Claude). Models are large (4-8GB for good quality). Inference is slow on CPU-only machines. This is the most hardware-dependent category. **However**, this is the future — and the privacy argument is strongest here. Position as optional, power-user features.

> [!IMPORTANT]
> This category should **integrate with Ollama** rather than bundling its own LLM runtime. Ollama handles model management, GPU detection, and inference. The toolbox calls Ollama's local API.

| # | Feature | What It Does | Competitors Charge? | OSS Library | Difficulty | Criticism | Verdict |
|---|---------|-------------|---------------------|-------------|------------|-----------|---------|
| 15.1 | **Text Summarization** | Condense long documents to key points | **Paid** (ChatGPT, Claude) | Ollama + local LLMs | Medium | High demand. Privacy-critical. Depends on Ollama being installed. | ✅ BUILD |
| 15.2 | **Translation** | Translate between languages locally | **Paid** (DeepL, Google Translate) | `argos-translate`, Ollama, `CTranslate2` | Medium | `argos-translate` is excellent and lightweight. Purpose-built for offline translation. | ✅ BUILD |
| 15.3 | **Text Rewriting / Paraphrase** | Rephrase text in different tones/styles | **Paid** (QuillBot, Grammarly) | Ollama + local LLMs | Medium | Quality depends on model. Good with 7B+ parameter models. | ✅ BUILD |
| 15.4 | **Speech-to-Text (Transcription)** | Transcribe audio/video to text/SRT | **Paid** (Otter.ai, Kapwing) | `whisper.cpp`, OpenAI Whisper | Hard | Premium feature. `whisper.cpp` works on CPU but is slow for long audio. GPU dramatically helps. Models: 39MB (tiny) to 1.5GB (large). | ✅ BUILD |
| 15.5 | **Text-to-Speech** | Generate spoken audio from text | **Paid** (ElevenLabs, Google TTS) | `piper-tts`, `espeak` | Medium | `piper-tts` has good quality local voices. Useful for accessibility. | ✅ BUILD (Phase 3) |
| 15.6 | **Document Q&A (RAG)** | Ask questions about uploaded documents | **Paid** (ChatGPT, NotebookLM) | Ollama + embedding model + vector search | Very Hard | Requires embedding model + vector DB + LLM. Complex pipeline. Extremely valuable but high implementation effort. | ✅ BUILD (Phase 3, experimental) |
| 15.7 | **Code Explanation** | Explain code snippets in plain English | **Paid** (GitHub Copilot) | Ollama + code-focused LLM | Medium | Natural extension of LLM integration. | ✅ BUILD |
| 15.8 | **Image Description (Alt Text)** | Generate descriptions of images for accessibility | **Paid** (Cloud vision APIs) | `llava`, multimodal local models | Hard | Requires multimodal model. Emerging capability in local AI. | ❌ SKIP (multimodal local AI not mature enough for reliable results) |

**Category Verdict: 7 of 8 features → BUILD** (most depend on Ollama)

---

## Category 16: 🎨 Color & Design (Bonus)

| # | Feature | What It Does | Competitors Charge? | OSS Library | Difficulty | Criticism | Verdict |
|---|---------|-------------|---------------------|-------------|------------|-----------|---------|
| 16.1 | **Color Format Converter** | HEX ↔ RGB ↔ HSL ↔ CMYK | Free | Pure math | Easy | Already in 6.19. Expose under both `tb dev color` and `tb color`. | ✅ BUILD |
| 16.2 | **Palette Generator** | Generate harmonious color palettes from a base color | Free | Color theory math | Easy | Useful for designers. | ✅ BUILD |
| 16.3 | **Extract Colors from Image** | Identify dominant colors in an image | Free | K-means clustering on pixels | Medium | Useful but niche. | ✅ BUILD |
| 16.4 | **Contrast Checker** | WCAG accessibility contrast ratio check | Free | Simple math formula | Easy | Important for web accessibility. | ✅ BUILD |

**Category Verdict: 4 of 4 features → BUILD**

---

## Category 17: 🔧 Font Tools (Bonus)

| # | Feature | What It Does | Competitors Charge? | OSS Library | Difficulty | Criticism | Verdict |
|---|---------|-------------|---------------------|-------------|------------|-----------|---------|
| 17.1 | **Font Format Conversion** | TTF → WOFF2, OTF → TTF, etc. | Free (online tools) | `fonttools`, `woff2` | Medium | Niche but web devs need it. | ✅ BUILD (Phase 3) |
| 17.2 | **Font Subsetting** | Strip unused glyphs to reduce font size | Free/Paid | `fonttools`, `glyphhanger` | Medium | Important for web performance. Saves bandwidth. | ✅ BUILD (Phase 3) |
| 17.3 | **Font Info Viewer** | Display font metadata, glyph count, supported scripts | Free | `fonttools` | Easy | Quick utility. | ✅ BUILD (Phase 3) |

**Category Verdict: 3 of 3 features → BUILD (Phase 3)**

---

## Category 18: ✉️ Email Tools (Bonus)

| # | Feature | What It Does | Competitors Charge? | OSS Library | Difficulty | Criticism | Verdict |
|---|---------|-------------|---------------------|-------------|------------|-----------|---------|
| 18.1 | **Parse .eml Files** | Extract body, headers, attachments from email files | Free | Standard `email`/MIME parsers | Medium | Niche but useful for forensics/debugging. | ✅ BUILD (Phase 3) |
| 18.2 | **Analyze Email Headers** | Check SPF, DKIM, DMARC, trace routing | Free (online) | Custom parsing logic | Hard | Niche — mostly security analysts. | ❌ SKIP (too niche) |
| 18.3 | **Extract Attachments** | Pull all attachments from .eml file | Free | MIME parsing | Easy | Useful companion to 18.1. | ✅ BUILD (Phase 3) |

**Category Verdict: 2 of 3 features → BUILD (Phase 3)**

---

## Grand Summary — Build vs. Skip

### Final Scorecard

| Decision | Count |
|----------|-------|
| ✅ **BUILD** | **120** |
| ❌ **SKIP** | **8** |
| **Total Analyzed** | **128** |

### What We're Skipping and Why

| # | Feature | Reason for Skip |
|---|---------|----------------|
| 1.13 | PDF → HTML | Quality too inconsistent; revisit later |
| 2.12 | Image to ASCII Art | Novelty, not core utility |
| 5.5 | Plagiarism Check | Fundamentally impossible offline |
| 8.7 | Excel → PDF | Dependency (LibreOffice) too large |
| 13.7 | Ping / Traceroute wrapper | Too thin — just use OS commands |
| 15.8 | Image Description (Alt Text) | Multimodal local AI not mature enough |
| 18.2 | Email Header Analysis | Too niche for general toolbox |
| 4.5 | Form / Invoice Parsing | Too complex and specialized for v1 |

### Phased Delivery Roadmap

```mermaid
flowchart LR
    subgraph P1["Phase 1: Foundation\n(MVP — 3 months)"]
        A["PDF Processing (14 tools)"]
        B["Image Processing (11 tools)"]
        C["File Compression (6 tools)"]
        D["Developer Utilities (19 tools)"]
        E["Text Manipulation (9 tools)"]
        F["Security & Encryption (8 tools)"]
        G["QR / Barcode (5 tools)"]
        H["CLI Framework + Plugin System"]
    end

    subgraph P2["Phase 2: Expansion\n(+3 months)"]
        I["Video & Audio (14 tools)"]
        J["OCR & Text Extraction (6 tools)"]
        K["Data / Spreadsheet (6 tools)"]
        L["Grammar & Writing (7 tools)"]
        M["Metadata Tools (4 tools)"]
        N["Network Utilities (6 tools)"]
    end

    subgraph P3["Phase 3: AI & Niche\n(+6 months)"]
        O["Local AI Processing (7 tools)"]
        P["Markdown & Authoring (6 tools)"]
        Q["Color & Design (4 tools)"]
        R["Font Tools (3 tools)"]
        S["Email Tools (2 tools)"]
    end

    P1 --> P2 --> P3
```

### Phase 1 Feature Count: **72 tools**
### Phase 2 Feature Count: **43 tools**
### Phase 3 Feature Count: **22 tools** (mostly AI-dependent)
### **Grand Total: ~137 tools across 18 categories** (including some cross-listed)

---

## CLI Architecture Recommendation

Based on research into ImageMagick, FFmpeg, and pandoc CLI patterns:

```
tb <category> <action> [options] [files...]

Examples:
  tb pdf merge *.pdf -o combined.pdf
  tb pdf compress report.pdf --level high
  tb image convert photo.heic --to webp
  tb image bg-remove portrait.jpg
  tb video compress movie.mp4 --quality medium
  tb video transcribe meeting.mp4 --model whisper-medium
  tb ocr extract scan.png --lang eng+fra
  tb dev json format data.json
  tb dev base64 encode "secret"
  tb sec password --length 20 --symbols
  tb qr encode "https://example.com" -o qr.png
  tb text case --to snake "MyVariableName"
  tb archive extract backup.7z
  tb data convert sales.csv --to json
  tb meta strip photo.jpg
  tb ai summarize document.pdf --model llama3
```

**Implementation Language:** Go or Rust (single static binary, no runtime deps)
**Heavy Deps (FFmpeg, pandoc, Ollama):** Download-on-demand to `~/.toolbox/deps/`
**Plugin System:** Allow community to add new tools without forking

---

> [!TIP]
> **Next Steps:** This document is your feature backlog. When you're ready, we can:
> - **Create a Product Brief** — formalize vision, target users, success metrics
> - **Write the PRD** — turn features into implementable requirements with acceptance criteria
> - **Plan the Architecture** — define the plugin system, CLI framework, and dependency management
> - **Create Epics & Stories** — break Phase 1 into sprintable work items
