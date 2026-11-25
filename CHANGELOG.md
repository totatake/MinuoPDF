# Changelog

All notable changes to **MinuoPDF** will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.2] - 2026-11-25

### Changed
- **CI / Release Artifact Versioning**: Updated GitHub Actions release and CI workflows (`.github/workflows/release.yml`, `.github/workflows/ci.yml`) to embed the application version tag into workflow artifact names, distribution directories, and release asset filenames (e.g., `MinuoPDF-v0.3.2-windows-x64.zip`, `MinuoPDF-v0.3.2-linux-x64.tar.gz`).

## [0.3.1] - 2026-11-22

### Fixed
- **Windows GUI Console Detachment**: Updated `hide_console()` in `src-tauri/src/main.rs` to check `GetConsoleProcessList` before calling `ShowWindow(SW_HIDE)` and to call `FreeConsole()`, closing automatically spawned Windows Terminal (ConPTY) console windows in addition to legacy `conhost.exe` without hiding pre-existing interactive shells.

## [0.3.0] - 2026-11-22

### Changed
- Standardized OSS repository layout: moved license text to extensionless `LICENSE` and separated `THIRD_PARTY_NOTICES.md`.
- Split legacy `CONTEXT.md` into `AGENTS.md` (agent operational guide) and `docs/overview.md` (human-facing project background).
- Migrated `docs/` from Sphinx `.rst` to standardized GitHub Markdown (`docs/README.md`, `docs/overview.md`, `docs/requirements.md`, `docs/architecture.md`, `docs/design/`, `docs/adr/`, `docs/guides/`, `docs/testing.md`, `docs/reports/test-results.md`, `docs/roadmap.md`), removing root `requirements.txt`.
- Updated `scripts/sync_test_results.py`, `Makefile`, and `make.bat` for Markdown report output (`docs/reports/test-results.md`) and `LICENSE` / `THIRD_PARTY_NOTICES.md` packaging.
- Split `.github/workflows/build.yml` into `ci.yml` and `release.yml`.

### Added
- Added automatic OS/locale-based GUI language detection (`auto` default) alongside explicit `English (EN)` and `日本語 (JA)` selection.
- Added `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, `.github/ISSUE_TEMPLATE/`, and `.github/PULL_REQUEST_TEMPLATE.md`.
- Added Architecture Decision Records under `docs/adr/`.

## [0.2.1] - 2026-11-03

### Fixed
- Improved robustness of Content Stream CTM operator parsing, CIDFont mapping, and PDF Audit normalization.
- Added CLI preset aliases, enhanced batch argument handling, and guarded stdin EOF in interactive mode.

## [0.2.0] - 2026-10-17

### Added
- Added 450 DPI resolution option in `DetailModeTab` dropdowns.
- Expanded architecture, optimization features, performance, and documentation coverage.

## [0.1.1] - 2026-10-11

### Fixed
- Set executable permission bit on packaged Linux `minuopdf` binary in release archives.

## [0.1.0] - 2026-08-02

### Added
- Initial public release of MinuoPDF (Tauri v2 + Svelte 5 GUI and Rust CLI).
- Keep Text and Rasterize compression strategies with Color, Grayscale, and Monochrome (Otsu / Floyd-Steinberg + CCITT Group 4) encoding.
- CTM effective DPI estimation, recursive Form XObject & SMask handling, Smart Fallback guard, Rayon parallelism, font subsetting/unembedding, object cleanup, and PDF Audit breakdown.
- Automated Rust unit/integration tests, Playwright E2E GUI tests, and cross-platform CI/release builds for Windows and Linux.
