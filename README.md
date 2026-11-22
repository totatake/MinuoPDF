# MinuoPDF

> **A Lightweight and Fast Portable PDF Optimization Tool**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![CI](https://github.com/totatake/MinuoPDF/actions/workflows/ci.yml/badge.svg)](https://github.com/totatake/MinuoPDF/actions/workflows/ci.yml)

**Language:** English | [日本語](README.ja.md)

---

## Overview

**MinuoPDF** (*from Latin "minuo": to diminish, reduce*) is an open-source desktop application and CLI tool designed to drastically reduce PDF file sizes while preserving reading quality and text crispness. Built exclusively on permissive non-GPL components (`MIT`, `BSD-3-Clause`, `Apache-2.0`), it runs as a standalone portable executable with no installer required.

---

## Key Features

- **Ultra-lightweight & Portable**: No installer or external runtime required. Standalone executable (~4.3 MB) automatically loads `pdfium.dll` / `libpdfium.so` from its own directory.
- **Smart Fallback & Precision Optimization**:
  - **CTM Effective DPI**: Calculates on-page effective display DPI (pt) from Content Stream transformation matrices (`cm`, `Do`).
  - **Form XObject & SMask**: Recursively traverses `/Subtype /Form` objects and synchronizes soft masks (`SMask`) on resize.
  - **Smart Fallback Guard**: Automatically preserves original streams if re-encoding would enlarge image size.
  - **Rayon Concurrency**: Multi-core parallel image decoding, resizing, and re-encoding.
- **PDF Audit Breakdown**: Inspect size composition (`Images`, `Fonts`, `Content Streams`, `Metadata`, `Struct/Forms`, `Other`) with interactive bar charts and pre/post comparison using zero extra disk I/O.
- **Flexible Color Compression**:
  - **Color**: High-efficiency JPEG re-compression.
  - **Grayscale**: Grayscale conversion with customizable JPEG quality.
  - **Monochrome**: 1-bit B&W conversion using **Otsu's Thresholding** or **Floyd-Steinberg Dithering**, encoded with ultra-compact **CCITT Group 4**.
- **Two Compression Strategies**:
  - **Keep Text**: Preserves original PDF text vectors and layout; compresses image objects only.
  - **Rasterize Page**: Renders full PDF pages into clean images via PDFium (ideal for scanned documents).
- **Font Optimization**:
  - **Subsetting**: Retains only glyph IDs actually used in the document.
  - **Unembedding**: Strips embedded font streams (`FontFile` / `FontFile2` / `FontFile3`), falling back to standard system fonts.
- **Deep Object Cleanup**: Strip annotations, outlines, links, AcroForms, XML metadata, JavaScript, and thumbnail previews.
- **Dual Mode (GUI & CLI)**:
  - Modern GUI (Tauri v2 + Svelte 5, Catppuccin Mocha theme, resizable 740×560px with min 680×520px bounds, automatic OS/locale detection `Auto` + explicit `EN` / `JA` toggle).
  - Headless batch CLI (`-o`, `-p`, `--profile`, `-s`, `-b`) and interactive console menu.

---

## Quick Start

### System Requirements

- **Supported OS**: Windows 10 / 11 (64-bit), Linux (x86_64)
- **Runtime Dependencies**: None (no installer or `.NET` / Python runtime required for end users)

### Distribution Layout

Place the distributed files together in the same folder:

```text
MinuoPDF/
├── minuopdf / minuopdf.exe     # Hybrid GUI & CLI executable
├── pdfium.dll / libpdfium.so   # PDFium rendering shared library
├── README.md                   # User Guide (English)
├── README.ja.md                # User Guide (Japanese)
├── LICENSE                     # MIT License
└── THIRD_PARTY_NOTICES.md      # Third-party component notices
```

A `profiles/` directory is automatically created alongside the executable when custom profiles are saved.

### GUI Mode

1. Launch `minuopdf` without arguments to open the GUI (display language defaults to `Auto` based on your OS locale and can be switched explicitly between `English (EN)` and `日本語 (JA)` in the top bar).
2. Add PDF files via drag-and-drop or the file picker (non-PDF files are filtered out).
3. Choose **Easy Mode** (5 presets or saved profiles), **Detail Mode** (DPI 50–450, JPEG quality, color/mono method, object cleanup, font subsetting/unembedding), or **Profile Management**.
4. Run optimization and click any result row (or **Audit**) to inspect the 6-category before/after size breakdown.

### CLI Mode

```bash
# Optimize with a built-in preset
minuopdf document.pdf -o optimized.pdf -p light

# Optimize with a saved profile and rasterize strategy
minuopdf scan.pdf -o out.pdf --profile my_rule -s rasterize

# Batch optimize multiple PDFs into a target directory
minuopdf a.pdf b.pdf c.pdf -b dist/ -p standard

# Launch interactive console menu (single PDF argument without flags)
minuopdf document.pdf
```

**Main Options**: `-o, --output <PATH>`, `-p, --preset <NAME>`, `--profile <NAME>`, `-s, --strategy <keep-text|rasterize>`, `-b, --batch <DIR>`

For full GUI/CLI instructions, `profiles/` JSON schema v2, and troubleshooting, see [User Guide (`docs/guides/usage.md`)](docs/guides/usage.md).

---

## Documentation

Full technical documentation (in Japanese) lives under [`docs/`](docs/README.md):

- [Overview, History & Comparison (`docs/overview.md`)](docs/overview.md)
- [Requirements Specification (`docs/requirements.md`)](docs/requirements.md)
- [System Architecture (`docs/architecture.md`)](docs/architecture.md)
- [Detailed Design: Backend & GUI (`docs/design/`)](docs/design/README.md)
- [Architecture Decision Records (`docs/adr/`)](docs/adr/README.md)
- [User & CLI Guide (`docs/guides/usage.md`)](docs/guides/usage.md)
- [Testing Strategy & Traceability Matrix (`docs/testing.md`)](docs/testing.md)
- [Latest Test Execution Report (`docs/reports/test-results.md`)](docs/reports/test-results.md)
- [Roadmap (`docs/roadmap.md`)](docs/roadmap.md)

---

## Building & Development

### Prerequisites

- [Rust](https://www.rust-lang.org/) (`stable`, 1.77+)
- [Node.js](https://nodejs.org/) (`v18+` / `v22 LTS`) and `npm`
- [Python 3](https://www.python.org/) (standard library only, for helper scripts in `scripts/`)

For Linux/WSL2 system library (`WebKit2GTK`) and Playwright installation commands, see [CONTRIBUTING.md](CONTRIBUTING.md).

### Quick Commands (`Makefile` / `make.bat`)

```bash
npm install     # Install frontend dependencies (PDFium is auto-fetched on build/test)
make build      # Build Tauri v2 release binary and package into dist/MinuoPDF/
make dev        # Start Tauri GUI development server
make test       # Run Rust unit & integration tests and sync docs/reports/test-results.md
make test-gui   # Run Playwright GUI E2E tests and sync docs/reports/test-results.md
make test-all   # Run all Rust and Playwright E2E tests
make clean      # Clean build caches and test outputs
```

---

## Contributing & Security

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for setup, test guidelines, and strict non-GPL dependency rules, [CHANGELOG.md](CHANGELOG.md) for release history, and [SECURITY.md](SECURITY.md) for private vulnerability reporting.

---

## License

MinuoPDF is licensed under the **MIT License**. See [LICENSE](LICENSE) and [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) for full license and component attribution details.
