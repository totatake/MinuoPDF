# AGENTS.md — AI Agent Operational Guide for MinuoPDF

This file provides concise repository context, build/test commands, and invariants for AI coding agents. Human-oriented background and generational evolution live in `docs/overview.md`.

## 1. Repository Map (Tauri v2 + Svelte 5 + Rust)

- `src/` — Frontend UI (Svelte 5 + TypeScript):
  - `src/routes/+page.svelte` — Main container, global reactive state, Tauri event listeners
  - `src/lib/components/` — `FileDropBar.svelte`, `EasyModeTab.svelte`, `DetailModeTab.svelte`, `ProfileTab.svelte`, `StatusFooter.svelte`, `AuditModal.svelte`
  - `src/lib/types/index.ts` — Shared config, audit comparison, and log types
  - `src/lib/utils/` — `format.ts`, `i18n.ts` (ja/en dictionary & tooltips)
- `src-tauri/` — Backend (Rust + Tauri v2 host):
  - `src/main.rs` — GUI/CLI dispatch entrypoint (including Windows console hiding)
  - `src/lib.rs` — Tauri IPC commands (`spawn_blocking`) & progress/log event emitters
  - `src/optimizer.rs` — Core optimizer (Rayon parallel image tasks, CTM DPI estimation, recursive Form XObject traversal, SMask sync, Smart Fallback guard, cleanup)
  - `src/rasterizer.rs` — Full-page PDFium rendering & re-encoding
  - `src/image_util.rs` — Unified JPEG / Flate / CCITT Group 4 (Otsu & Floyd-Steinberg) encoder
  - `src/audit.rs` — In-memory and file-based 6-category PDF size breakdown
  - `src/font_subsetter.rs` — Content stream GID extraction, font subsetting, and unembedding
  - `src/cli.rs` — Non-interactive batch CLI (`-o`, `-p`, `--profile`, `-s`, `-b`) and interactive console menu
  - `src/profile.rs` — JSON profile loader/saver (`profiles/`, schema v2)
  - `src/pdfium_util.rs` — Dynamic PDFium loader preferring executable directory
  - `tests/integration_tests.rs` — Rust integration tests
- `tests/e2e/gui.spec.ts` — Playwright E2E GUI tests; `tests/fixtures/` — Sample PDFs (`scripts/ensure_fixtures.py`)
- `profiles/` — Preset/custom JSON profiles
- `scripts/` — `ensure_pdfium.py`, `ensure_fixtures.py`, `create_test_pdf.py`, `sync_test_results.py`
- `docs/` — Engineering documentation (GitHub Markdown, Japanese default, lowercase `kebab-case`)

## 2. Build, Lint & Test Commands

```bash
# Install frontend dependencies (and fetch PDFium via scripts on build/test)
npm install

# Frontend type & Svelte check
npm run check

# Rust unit & integration tests (also synced via `make test`)
python scripts/ensure_pdfium.py
python scripts/ensure_fixtures.py
cargo test --manifest-path src-tauri/Cargo.toml

# Full Makefile / make.bat shortcuts
make test          # Runs Rust tests and updates docs/reports/test-results.md
make test-gui      # Runs Playwright E2E GUI tests and updates docs/reports/test-results.md
make test-all      # Runs Rust tests + Playwright GUI E2E
make build         # Builds release Tauri binary and packages dist/MinuoPDF/
make dev           # Starts Tauri dev app
```

## 3. Architectural Invariants & Guardrails

1. **Non-GPL License Purity**: Never introduce GPL/AGPL crates or dependencies. Keep all dependencies permissive (`MIT`, `BSD-3-Clause`, `Apache-2.0`).
2. **Standalone Portability**: Do not rely on system-wide installers or hard-coded PDFium paths. `pdfium.dll` / `libpdfium.so` and `profiles/` resolve relative to the executable directory.
3. **Optimization Correctness**:
   - Preserve Smart Fallback: never replace an unchanged image stream with a larger re-encoded stream.
   - Keep `SMask` alpha/soft-mask dimensions synchronized when resizing parent image XObjects, and recurse into `/Subtype /Form` XObjects.
   - Reuse the already-loaded `lopdf::Document` in memory for pre/post `audit_document` calls to avoid duplicate disk I/O.
4. **UI & Cross-Platform Rules**:
   - Keep `make.bat` line endings as CRLF (`.gitattributes`).
   - Keep window constraints (`740x560` initial, `680x520` minimum, resizable) and `ja`/`en` i18n parity in `src/lib/utils/i18n.ts`.

## 4. Documentation Rules

- Root OSS files (`README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, `THIRD_PARTY_NOTICES.md`) are in English; `README.ja.md` is the Japanese twin.
- `docs/` uses lowercase `kebab-case` (except `README.md` hubs) and Japanese by default.
- Record major design trade-offs under `docs/adr/NNNN-<title>.md` and update `docs/adr/README.md`.
- Update `docs/requirements.md`, `docs/testing.md`, and `scripts/sync_test_results.py` together when adding new tests or requirements; run `scripts/sync_test_results.py` to refresh `docs/reports/test-results.md`.
