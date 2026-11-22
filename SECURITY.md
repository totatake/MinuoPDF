# Security Policy

## Supported Versions

| Version | Supported |
| :--- | :--- |
| `0.2.x` (`main`) | Yes |
| `< 0.2.0` | No |

## Reporting a Vulnerability

Please **do not** report security vulnerabilities through public GitHub issues.

Instead, report them privately via:
- GitHub Security Advisories (`Security` tab -> `Report a vulnerability`) for `totatake/MinuoPDF`.

Please include:
1. Affected version / commit SHA and OS (`Windows` / `Linux`).
2. Reproduction steps or a minimal sanitized sample PDF / CLI invocation if applicable.
3. Observed impact (e.g., crash / panic in PDF parsing, arbitrary path write, unintended command execution, or dependency vulnerability).

## Security Considerations for MinuoPDF

- **Untrusted PDF Parsing**: MinuoPDF processes arbitrary input PDFs via `lopdf` and `PDFium`. Malformed content streams, recursive Form XObjects, or corrupt font tables are primary threat surfaces.
- **Portable Asset Loading**: `pdfium.dll` / `libpdfium.so` is loaded from the executable directory; avoid placing untrusted shared libraries alongside the binary.

We aim to acknowledge reports within 72 hours and coordinate a fix prior to public disclosure.
