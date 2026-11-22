# Contributing to MinuoPDF

Thank you for considering contributing to **MinuoPDF**!

## Prerequisites

- **Rust** (`stable`, 1.77+)
- **Node.js** (`v18+` / `v22 LTS`) and `npm`
- **Python 3** (used by `scripts/ensure_pdfium.py`, `scripts/ensure_fixtures.py`, and `scripts/sync_test_results.py` — standard library only)
- **Linux (Ubuntu / WSL2)** additionally requires WebKit2GTK & GTK3 system libraries:
  ```bash
  sudo apt update
  sudo apt install -y build-essential curl wget file libssl-dev libgtk-3-dev \
    libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev pkg-config \
    python3 python-is-python3
  ```

## Development Setup & Verification

```bash
git clone https://github.com/totatake/MinuoPDF.git
cd MinuoPDF
npm install
```

Run checks and tests locally before opening a Pull Request:

```bash
# 1. Frontend TypeScript & Svelte typecheck
npm run check

# 2. Rust unit & integration tests (auto-downloads PDFium & test PDFs)
make test
# Or directly:
# python scripts/ensure_pdfium.py && python scripts/ensure_fixtures.py
# cargo test --manifest-path src-tauri/Cargo.toml

# 3. Playwright GUI E2E tests
make test-gui

# 4. Release build check (optional)
make build
```

## License Policy (Strict Non-GPL)

MinuoPDF intentionally maintains a **100% permissive license stack** (`MIT`, `BSD-3-Clause`, `Apache-2.0`). Do **not** introduce GPL or AGPL dependencies in `src-tauri/Cargo.toml` or `package.json`. If you add an external dependency with attribution requirements, update `THIRD_PARTY_NOTICES.md`.

## Workflow & Conventions

1. Create a branch from `main` (`feat/<topic>`, `fix/<topic>`, `docs/<topic>`).
2. Prefer [Conventional Commits](https://www.conventionalcommits.org/) (`feat(pdf): ...`, `fix(cli): ...`, `docs: ...`, `test: ...`).
3. Keep `ja` and `en` UI translations in sync (`src/lib/utils/i18n.ts`).
4. Update relevant docs under `docs/` (and add an ADR under `docs/adr/` for significant architectural changes) and add a note under `[Unreleased]` in `CHANGELOG.md`.
5. Keep `make.bat` line endings as CRLF (`.gitattributes`).
6. Fill out `.github/PULL_REQUEST_TEMPLATE.md` when submitting your PR.
