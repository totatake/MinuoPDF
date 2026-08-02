# MinuoPDF Root Makefile
.PHONY: all build dev docs clean distclean test test-gui test-all help

# Default target
all: help

## build: Build the Tauri release binary and package into dist/
build:
	python scripts/ensure_pdfium.py
	@if [ ! -d "node_modules" ]; then echo "node_modules not found. Running npm install..."; npm install; fi
	npx tauri build
	@mkdir -p dist
	@cp -f src-tauri/target/release/minuopdf dist/ 2>/dev/null || true
	@cp -f src-tauri/target/release/minuopdf.exe dist/ 2>/dev/null || true
	@cp -f src-tauri/resources/pdfium.dll dist/ 2>/dev/null || true
	@cp -f src-tauri/resources/libpdfium.so dist/ 2>/dev/null || true
	@cp -f README.ja.md dist/ 2>/dev/null || true
	@cp -f README.md dist/ 2>/dev/null || true
	@cp -f LICENSE.md dist/ 2>/dev/null || true
	@mkdir -p dist/profiles
	@cp -rf profiles/* dist/profiles/ 2>/dev/null || true
	@echo "Build and packaging complete. Distribution ready in dist/"

## dev: Start the Tauri dev environment
dev:
	python scripts/ensure_pdfium.py
	@if [ ! -d "node_modules" ] || ! npx tauri --version >/dev/null 2>&1; then echo "node_modules missing or platform mismatch. Running npm install..."; npm install; fi
	npx tauri dev

## docs: Build Sphinx HTML documentation (ja and en)
docs:
	@echo "Building Sphinx documentation (ja & en)..."
	@if [ -d ".venv" ]; then \
		SPHINXBUILD="$$(pwd)/.venv/bin/sphinx-build" PATH="$$(pwd)/.venv/bin:$$PATH" $(MAKE) -C docs html-all; \
	else \
		$(MAKE) -C docs html-all; \
	fi

## clean: Clean build artifacts, dist, target, and docs
clean:
	rm -rf dist build .svelte-kit src-tauri/target docs/_build vite.config.*.timestamp-* pdf_optimizer_debug.log test_input*.pdf test-results playwright-report
	@echo "Cleanup complete."

## distclean: Clean build artifacts, node_modules, package-lock.json, PDFium binaries, test fixtures, and venv
distclean: clean
	rm -rf node_modules package-lock.json .venv venv
	rm -f src-tauri/resources/pdfium.dll src-tauri/resources/libpdfium.so src-tauri/resources/*.dylib
	rm -f tests/fixtures/*.pdf
	@echo "Deep cleanup complete."

## test: Run Rust unit tests
test:
	python scripts/ensure_pdfium.py
	python scripts/ensure_fixtures.py
	mkdir -p test-results
	cd src-tauri && cargo test 2>&1 | tee ../test-results/cargo-test.log
	python scripts/sync_test_results.py

## test-gui: Run Playwright E2E GUI tests
test-gui:
	python scripts/ensure_pdfium.py
	python scripts/ensure_fixtures.py
	@if [ ! -d "node_modules" ] || ! npx tauri --version >/dev/null 2>&1; then echo "node_modules missing or platform mismatch. Running npm install..."; npm install; fi
	npx playwright install
	npx playwright test
	python scripts/sync_test_results.py

## test-all: Run ALL tests (Rust unit, integration, and GUI E2E)
test-all: test test-gui

## help: Show this help message
help:
	@echo "MinuoPDF Development Commands:"
	@echo "  make build     - Build Tauri release binary and package into dist/"
	@echo "  make dev       - Start Tauri dev environment"
	@echo "  make docs      - Build Sphinx HTML documentation"
	@echo "  make clean     - Clean build caches and outputs"
	@echo "  make distclean - Clean build caches, node_modules, and package-lock.json"
	@echo "  make test      - Run Rust unit and integration tests"
	@echo "  make test-gui  - Run Playwright E2E GUI automated tests"
	@echo "  make test-all  - Run ALL tests (Rust unit, integration, and GUI E2E)"

