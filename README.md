# MinuoPDF

> **A Lightweight and Fast Portable PDF Optimization Tool**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![CI](https://github.com/totatake/MinuoPDF/actions/workflows/ci.yml/badge.svg)](https://github.com/totatake/MinuoPDF/actions/workflows/ci.yml)

**Language:** English | [日本語](README.ja.md)

---

## Overview

**MinuoPDF** (*from Latin "minuo": to diminish, reduce*) is a modern, high-performance desktop application and CLI tool designed to drastically reduce PDF file sizes while maintaining reading quality and text crispness. Built entirely on permissive non-GPL components (`MIT`, `BSD-3-Clause`, `Apache-2.0`), it runs as a standalone portable binary with no installer required.

---

## Key Features

- **Ultra-lightweight & Portable**: No installer required. Standalone executable (~4.3 MB) automatically loading `pdfium.dll` / `libpdfium.so` from its directory.
- **Smart Fallback & Precision Optimization**:
  - **CTM Effective DPI**: Calculates on-page effective display DPI (pt) using Content Stream transformation matrices.
  - **Form XObject & SMask**: Recursively traverses `/Subtype /Form` objects and synchronizes soft masks (`SMask`) on resize.
  - **Smart Fallback Guard**: Automatically preserves original streams if re-encoding would enlarge image size.
  - **Rayon Concurrency**: Multi-core parallel image decoding, resizing, and re-encoding.
- **PDF Audit Breakdown**: Inspect size composition (Images, Fonts, Content Streams, Metadata, Struct/Forms, Other) with interactive bar charts and pre/post comparison using zero extra disk I/O.
- **Flexible Color Compression**:
  - **Color**: High-efficiency JPEG re-compression.
  - **Grayscale**: Grayscale conversion with customizable JPEG quality.
  - **Monochrome**: 1-bit B&W conversion using **Otsu's Thresholding** or **Floyd-Steinberg Dithering**, encoded with ultra-compact **CCITT Group 4**.
- **Two Compression Strategies**:
  - **Keep Text Strategy**: Preserves original PDF text vectors and layout; compresses image objects only.
  - **Rasterize Page Strategy**: Renders full PDF pages into clean images (ideal for scanned documents).
- **Font Optimization**:
  - **Subsetting**: Retains only glyph IDs actually used in the document.
  - **Unembedding (Unembed Fonts)**: Strips embedded font streams (`FontFile` / `FontFile2` / `FontFile3`), falling back to standard system fonts for extreme size savings.
- **Deep Object Cleanup**: Strip annotations, outlines, links, AcroForms, XML metadata, JavaScript, and thumbnail previews.
- **Dual Mode & Non-interactive CLI**:
  - Modern GUI (Tauri v2 + Svelte 5, Catppuccin Mocha theme, resizable 740×560px with min 680×520px bounds, ja/en i18n).
  - Headless batch CLI (`-o`, `-p`, `--profile`, `-s`, `-b`) and interactive arrow-key console menu.

---

## CLI Usage

MinuoPDF supports headless scripting and batch processing:

```bash
# Optimize with preset
minuopdf document.pdf -o optimized.pdf -p light

# Optimize with custom profile and rasterize strategy
minuopdf scan.pdf -o out.pdf --profile my_rule -s rasterize

# Batch optimize multiple PDFs into target directory
minuopdf a.pdf b.pdf c.pdf -b dist/ -p standard
```

For full GUI and CLI usage instructions, presets, and profile schemas, see [User Guide (`docs/guides/usage.md`)](docs/guides/usage.md).

---

## Building & Development

### Prerequisites

- [Node.js](https://nodejs.org/) (v18+ / v22 LTS)
- [Rust](https://www.rust-lang.org/) (stable, 1.77+)
- [Python 3](https://www.python.org/) (standard library only, used by helper scripts in `scripts/`)

#### Linux / WSL2 Prerequisites (Ubuntu 24.04 LTS)

```bash
# 1. Install GTK/WebKit system dependencies
sudo apt update
sudo apt install -y build-essential curl wget file libssl-dev libgtk-3-dev \
  libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev pkg-config \
  python3 python-is-python3

# 2. Install npm dependencies (fetches @tauri-apps/cli-linux-x64-gnu native binary)
npm install
# Note: PDFium shared library (libpdfium.so / pdfium.dll) is fetched automatically on build/test.

# 3. Install Playwright browser and OS dependencies for GUI E2E testing
npx playwright install
sudo npx playwright install-deps
```

### Quick Commands (`Makefile` / `make.bat`)

```bash
make build      # Build Tauri v2 release binary and package into dist/MinuoPDF/
make dev        # Start Tauri GUI development server
make test       # Run Rust unit & integration tests and sync docs/reports/test-results.md
make test-gui   # Run Playwright GUI E2E tests and sync docs/reports/test-results.md
make test-all   # Run all Rust and Playwright E2E tests
make clean      # Clean build caches and test outputs
```

---

## Documentation

Full technical documentation (in Japanese) is maintained in GitHub Markdown under [`docs/`](docs/README.md):

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

## Contributing & Security

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for setup, test checks, and non-GPL dependency rules, [CHANGELOG.md](CHANGELOG.md) for release history, and [SECURITY.md](SECURITY.md) for private vulnerability reporting.

---

## License

MinuoPDF is licensed under the **MIT License**. See [LICENSE](LICENSE) and [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) for full license and component attribution details.
