# MinuoPDF Root Makefile
.PHONY: all build dev clean distclean test test-gui test-all help

# Default target
all: help

## build: Build the Tauri release binary and package into dist/MinuoPDF/
build:
	python scripts/ensure_pdfium.py
	@if [ ! -d "node_modules" ]; then echo "node_modules not found. Running npm install..."; npm install; fi
	npx tauri build
	@mkdir -p dist/MinuoPDF
	@cp -f src-tauri/target/release/minuopdf dist/MinuoPDF/ 2>/dev/null || true
	@chmod +x dist/MinuoPDF/minuopdf 2>/dev/null || true
	@cp -f src-tauri/target/release/minuopdf.exe dist/MinuoPDF/ 2>/dev/null || true
	@cp -f src-tauri/resources/pdfium.dll dist/MinuoPDF/ 2>/dev/null || true
	@cp -f src-tauri/resources/libpdfium.so dist/MinuoPDF/ 2>/dev/null || true
	@cp -f README.ja.md dist/MinuoPDF/ 2>/dev/null || true
	@cp -f README.md dist/MinuoPDF/ 2>/dev/null || true
	@cp -f LICENSE dist/MinuoPDF/ 2>/dev/null || true
	@cp -f THIRD_PARTY_NOTICES.md dist/MinuoPDF/ 2>/dev/null || true
	@mkdir -p dist/MinuoPDF/profiles
	@cp -rf profiles/* dist/MinuoPDF/profiles/ 2>/dev/null || true
	@echo "Build and packaging complete. Distribution ready in dist/MinuoPDF"

## dev: Start the Tauri dev environment
dev:
	python scripts/ensure_pdfium.py
	@if [ ! -d "node_modules" ] || ! npx tauri --version >/dev/null 2>&1; then echo "node_modules missing or platform mismatch. Running npm install..."; npm install; fi
	npx tauri dev

## clean: Clean build artifacts, dist, target, and test reports
clean:
	rm -rf dist build .svelte-kit src-tauri/target vite.config.*.timestamp-* pdf_optimizer_debug.log test_input*.pdf test-results playwright-report
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
	@echo "  make clean     - Clean build caches and outputs"
	@echo "  make distclean - Clean build caches, node_modules, and package-lock.json"
	@echo "  make test      - Run Rust unit and integration tests"
	@echo "  make test-gui  - Run Playwright E2E GUI automated tests"
	@echo "  make test-all  - Run ALL tests (Rust unit, integration, and GUI E2E)"

