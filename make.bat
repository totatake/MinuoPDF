@echo off
rem MinuoPDF Root make.bat for Windows

if "%1"=="" goto help
if "%1"=="build" goto build
if "%1"=="dev" goto dev
if "%1"=="docs" goto docs
if "%1"=="clean" goto clean
if "%1"=="distclean" goto distclean
if "%1"=="test" goto test
if "%1"=="test-gui" goto test_gui
if "%1"=="test-all" goto test_all
if "%1"=="help" goto help

echo Unknown target: %1
goto help

:build
echo [MinuoPDF] Ensuring PDFium shared library...
python "%~dp0scripts\ensure_pdfium.py"
echo [MinuoPDF] Building release binary...
if not exist "%~dp0node_modules" (
    echo [MinuoPDF] node_modules not found. Running npm install...
    call npm install
)
set RUSTFLAGS=-D warnings
call npx tauri build
if errorlevel 1 (
    echo [ERROR] Tauri build failed due to build errors or warnings.
    exit /b 1
)

echo.
echo [MinuoPDF] Packaging distribution into dist/ folder...
if not exist "%~dp0dist" mkdir "%~dp0dist"

if exist "%~dp0src-tauri\target\release\minuopdf.exe" (
    copy /y "%~dp0src-tauri\target\release\minuopdf.exe" "%~dp0dist\" >nul
    echo  [OK] Copied minuopdf.exe
)

if exist "%~dp0src-tauri\resources\pdfium.dll" (
    copy /y "%~dp0src-tauri\resources\pdfium.dll" "%~dp0dist\" >nul
    echo  [OK] Copied pdfium.dll
)

if exist "%~dp0README.ja.md" (
    copy /y "%~dp0README.ja.md" "%~dp0dist\" >nul
    echo  [OK] Copied README.ja.md
)

if exist "%~dp0README.md" (
    copy /y "%~dp0README.md" "%~dp0dist\" >nul
    echo  [OK] Copied README.md
)

if exist "%~dp0LICENSE.md" (
    copy /y "%~dp0LICENSE.md" "%~dp0dist\" >nul
    echo  [OK] Copied LICENSE.md
)

if exist "%~dp0profiles" (
    if not exist "%~dp0dist\profiles" mkdir "%~dp0dist\profiles"
    copy /y "%~dp0profiles\*" "%~dp0dist\profiles\" >nul
    echo  [OK] Copied profiles
)

echo.
echo ===================================================
echo  Build ^& Package Completed Successfully!
echo  Distribution files ready in dist/
echo ===================================================
goto end

:dev
echo [MinuoPDF] Ensuring PDFium shared library...
python "%~dp0scripts\ensure_pdfium.py"
echo [MinuoPDF] Starting development server...
if not exist "%~dp0node_modules" (
    echo [MinuoPDF] node_modules not found. Running npm install...
    call npm install
)
call npx tauri dev
goto end

:docs
echo [MinuoPDF] Syncing test results for documentation...
if exist "%~dp0test-results\results.json" (
    python "%~dp0scripts\sync_test_results.py"
)
echo [MinuoPDF] Building Sphinx documentation (ja and en)...
cd docs
call make.bat html-all
if errorlevel 1 (
    cd ..
    echo [ERROR] Documentation build failed due to warnings or errors.
    exit /b 1
)
cd ..
goto end



:clean
echo [MinuoPDF] Cleaning temporary files and build artifacts...
if exist "%~dp0dist" rmdir /s /q "%~dp0dist"
if exist "%~dp0build" rmdir /s /q "%~dp0build"
if exist "%~dp0.svelte-kit" rmdir /s /q "%~dp0.svelte-kit"
if exist "%~dp0src-tauri\target" rmdir /s /q "%~dp0src-tauri\target"
if exist "%~dp0docs\_build" rmdir /s /q "%~dp0docs\_build"
if exist "%~dp0vite.config.js.timestamp-*" del /q "%~dp0vite.config.js.timestamp-*"
if exist "%~dp0vite.config.ts.timestamp-*" del /q "%~dp0vite.config.ts.timestamp-*"
if exist "%~dp0pdf_optimizer_debug.log" del /q "%~dp0pdf_optimizer_debug.log"
if exist "%~dp0test_input*.pdf" del /q "%~dp0test_input*.pdf"
if exist "%~dp0test-results" rmdir /s /q "%~dp0test-results"
if exist "%~dp0playwright-report" rmdir /s /q "%~dp0playwright-report"
echo [OK] Cleanup complete.
goto end

:distclean
call :clean
echo [MinuoPDF] Cleaning node_modules, package-lock.json, PDFium binaries, test fixtures, and venv...
if exist "%~dp0node_modules" rmdir /s /q "%~dp0node_modules"
if exist "%~dp0package-lock.json" del /q "%~dp0package-lock.json"
if exist "%~dp0src-tauri\resources\pdfium.dll" del /q "%~dp0src-tauri\resources\pdfium.dll"
if exist "%~dp0src-tauri\resources\libpdfium.so" del /q "%~dp0src-tauri\resources\libpdfium.so"
if exist "%~dp0src-tauri\resources\*.dylib" del /q "%~dp0src-tauri\resources\*.dylib"
if exist "%~dp0tests\fixtures\*.pdf" del /q "%~dp0tests\fixtures\*.pdf"
if exist "%~dp0.venv" rmdir /s /q "%~dp0.venv"
if exist "%~dp0venv" rmdir /s /q "%~dp0venv"
echo [OK] Deep cleanup complete.
goto end

:test
echo [MinuoPDF] Ensuring PDFium shared library...
python "%~dp0scripts\ensure_pdfium.py"
echo [MinuoPDF] Ensuring test PDF fixtures...
python "%~dp0scripts\ensure_fixtures.py"
echo [MinuoPDF] Running Rust unit and integration tests...
if not exist "%~dp0test-results" mkdir "%~dp0test-results"
pushd "%~dp0src-tauri"
set RUSTFLAGS=-D warnings
cargo test > "%~dp0test-results\cargo-test.log" 2>&1
set CARGO_TEST_EXIT=%errorlevel%
popd
type "%~dp0test-results\cargo-test.log"
if %CARGO_TEST_EXIT% neq 0 (
    echo [ERROR] Rust tests failed or compiler warnings were treated as errors.
    exit /b 1
)
python "%~dp0scripts\sync_test_results.py"
exit /b 0

:test_gui
echo [MinuoPDF] Ensuring PDFium shared library...
python "%~dp0scripts\ensure_pdfium.py"
python "%~dp0scripts\ensure_fixtures.py"
echo [MinuoPDF] Running Playwright E2E GUI automated tests...
if not exist "%~dp0node_modules" (
    echo [MinuoPDF] node_modules not found. Running npm install...
    call npm install
)
call npx playwright install
call npx playwright test
if errorlevel 1 (
    echo [ERROR] Playwright E2E GUI tests failed.
    exit /b 1
)
python "%~dp0scripts\sync_test_results.py"
exit /b 0

:test_all
echo [MinuoPDF] Running ALL tests (Rust unit, integration, and Playwright GUI E2E)...
call :test
if errorlevel 1 (
    echo [ERROR] Rust tests failed during test-all.
    exit /b 1
)
call :test_gui
if errorlevel 1 (
    echo [ERROR] GUI tests failed during test-all.
    exit /b 1
)
echo.
echo ===================================================
echo  All Tests Completed Successfully!
echo ===================================================
goto end

:help
echo MinuoPDF Development Commands (make.bat):
echo   make build     - Build Tauri release binary and package into dist/
echo   make dev       - Start Tauri dev environment
echo   make docs      - Build Sphinx HTML documentation
echo   make clean     - Clean build caches, target, dist, and docs/_build
echo   make distclean - Clean build caches, node_modules, and package-lock.json
echo   make test      - Run Rust unit and integration tests
echo   make test-gui  - Run Playwright E2E GUI automated tests
echo   make test-all  - Run ALL tests (Rust unit, integration, and GUI E2E)
goto end

:end
