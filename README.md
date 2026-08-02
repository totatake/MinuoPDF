# MinuoPDF 📄⚡

> **A Lightweight and Fast Portable PDF Optimization Tool**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE.md)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](#)

[**日本語 (Japanese README)**](README.ja.md)

---

## 🌟 Overview

**MinuoPDF** (*from Latin "minuo": to diminish, reduce*) is a modern, high-performance desktop application and CLI tool designed to drastically reduce PDF file sizes while maintaining reading quality and text crispness.

---

## ✨ Key Features

- **🚀 Ultra-lightweight & Portable**: No installer required. Standalone executable (~4.3 MB) that loads `pdfium.dll` automatically from its directory.
- **🎨 Flexible Color Compression**:
  - **Color**: High-efficiency JPEG re-compression.
  - **Grayscale**: Grayscale conversion with customizable JPEG quality.
  - **Monochrome**: 1-bit B&W conversion using **Otsu's Thresholding** or **Floyd-Steinberg Dithering**, encoded with ultra-compact **CCITT Group 4**.
- **⚡ Two Compression Strategies**:
  - **Keep Text Strategy**: Preserves original PDF text vectors and layout; compresses image objects only.
  - **Rasterize Page Strategy**: Renders full PDF pages into clean images (ideal for scanned documents).
- **🔤 Font Optimization**:
  - **Subsetting**: Retains only characters actually used in the document.
  - **Unembedding (Unembed Fonts)**: Strips embedded font streams (`FontFile2`/`FontFile3`), leveraging standard system fonts for extreme size savings.
- **🧹 Deep Object Cleanup**: Strip annotations, outlines, links, AcroForms, XML metadata, JavaScript, and thumbnail previews.
- **🌐 Dual Mode & i18n**: Modern GUI (Tauri v2 + Svelte 5 with Catppuccin Mocha theme) and interactive CLI console mode. Fully supports **English** and **Japanese**.

---

## 🛠️ Building & Development

### Prerequisites

- [Node.js](https://nodejs.org/) (v18+)
- [Rust](https://www.rust-lang.org/) (stable)
- [Python 3](https://www.python.org/) (optional, for Sphinx docs build)

#### Linux / WSL2 Prerequisites (Ubuntu 24.04 LTS)

To build MinuoPDF on Linux or within WSL2 (Ubuntu 24.04 LTS):

```bash
# 1. Install GTK/WebKit system dependencies (Ubuntu 24.04 LTS)
sudo apt update
sudo apt install -y build-essential curl wget file libssl-dev libgtk-3-dev libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev pkg-config python3 python-is-python3 python3-venv python3-full

# 2. Install Node.js (v20 LTS) & npx
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt install -y nodejs

# 3. Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 4. Install npm dependencies for Linux (fetches @tauri-apps/cli-linux-x64-gnu native binary)
npm install
# Note: PDFium shared library (libpdfium.so / pdfium.dll) is automatically fetched on build/dev.

# 5. Install Playwright browser and system OS dependencies for GUI E2E testing
npx playwright install
sudo npx playwright install-deps
```

### Quick Commands (Makefile / make.bat)

MinuoPDF provides convenient build targets via `make` or `make.bat`:

```bash
# Build release executable (Tauri v2)
make build

# Start GUI development server
make dev

# Run unit tests
make test

# Build Sphinx documentation (HTML)
make docs

# Clean build artifacts
make clean
```

---

## 📖 Documentation

Full documentation (Requirements, Architecture, Detailed Design, and Traceability Matrix) is maintained in ReStructuredText (RST) under `docs/`.

To generate the HTML documentation locally:
```bash
# 1. Create a Python virtual environment and install dependencies
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt

# 2. Build HTML documentation
make docs
```
Open `docs/_build/html/index.html` in your browser.

---

## 📜 License

MinuoPDF is licensed under the **MIT License**. Feel free to use, modify, and distribute it for personal or commercial projects.
