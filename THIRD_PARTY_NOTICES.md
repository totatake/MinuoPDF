# Third-Party Notices

**MinuoPDF** is built exclusively with permissive, non-GPL open-source components (MIT / BSD / Apache-2.0 compatible) to allow safe commercial, internal, and closed-environment distribution.

## Summary of Core Components

| Component | Ecosystem | License | Role |
| :--- | :--- | :--- | :--- |
| [Tauri v2](https://github.com/tauri-apps/tauri) (`tauri`, `@tauri-apps/api`, plugins) | Rust / JS | MIT OR Apache-2.0 | Desktop application shell, IPC gateway, file dialogs |
| [PDFium](https://pdfium.googlesource.com/pdfium/) | Native (`pdfium.dll` / `libpdfium.so`) | BSD-3-Clause | High-performance PDF page rasterization engine |
| [pdfium-render](https://github.com/ajrcarey/pdfium-render) | Rust (`0.8.21`) | MIT OR Apache-2.0 | Safe Rust bindings and dynamic loader for PDFium |
| [lopdf](https://github.com/J-F-Liu/lopdf) | Rust (`0.31.0`) | MIT | PDF object tree parsing, manipulation, and serialization |
| [image](https://github.com/image-rs/image) | Rust (`0.25.10`) | MIT OR Apache-2.0 | Image decoding, resizing, and pixel format conversion |
| [rayon](https://github.com/rayon-rs/rayon) | Rust (`1.10`) | MIT OR Apache-2.0 | Data-parallel image processing across CPU cores |
| [fax](https://github.com/pdf-rs/fax) | Rust (`0.2.1`) | MIT OR Apache-2.0 | CCITT Group 4 1-bit monochrome fax compression |
| [tiff](https://github.com/image-rs/image-tiff) | Rust (`0.9.0`) | MIT OR Apache-2.0 | TIFF image encoding/decoding support |
| [flate2](https://github.com/rust-lang/flate2-rs) | Rust (`1.0`) | MIT OR Apache-2.0 | Deflate stream compression/decompression |
| [allsorts](https://github.com/yeslogic/allsorts) | Rust (`0.14`) | Apache-2.0 | Font parsing and glyph subsetting support |
| [clap](https://github.com/clap-rs/clap) | Rust (`4.4`) | MIT OR Apache-2.0 | Command-line argument parsing |
| [serde](https://github.com/serde-rs/serde) / `serde_json` | Rust (`1`) | MIT OR Apache-2.0 | Profile JSON & IPC payload serialization |
| [Svelte 5](https://github.com/sveltejs/svelte) / [SvelteKit](https://github.com/sveltejs/kit) / [Vite](https://github.com/vitejs/vite) / TypeScript | Frontend | MIT / Apache-2.0 | Reactive desktop UI framework and build toolchain |

## Distribution Notes

- Prebuilt binaries of PDFium (`pdfium.dll` on Windows, `libpdfium.so` on Linux) are fetched at build time from `bblanchon/pdfium-binaries` and loaded dynamically at runtime from the executable directory.
- Refer to upstream repositories and `Cargo.lock` / `package-lock.json` for full copyright statements of all transitive dependencies.
