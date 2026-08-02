# Test Fixtures

The PDF files in this directory (`tests/fixtures/`) are sample PDF documents downloaded from [cman.jp PDF Create Page](https://note.cman.jp/pdf/create/).

## Dynamic Downloading

If any PDF file in this directory is missing or deleted, it will be automatically downloaded from `https://note.cman.jp/pdf/create/` by running:

```bash
python scripts/ensure_fixtures.py
```

Or when running Rust unit/integration tests (`make test` or `make.bat test`), missing fixtures are automatically downloaded on demand.

## Fixture File List

- `sample_font_yes.pdf`: PDF with embedded fonts.
- `sample_font_no.pdf`: PDF without embedded fonts.
- `sample_font_img.pdf`: PDF converted to image/shapes.
- `sample_font_cube_yes.pdf`: CubePDF generated sample.
- `sample_font_just_yes.pdf`: JUST-PDF generated sample.
- `sample_font_word_yes.pdf`: MS Word generated sample.
- `sample_font_ikinari_yes.pdf`: Ikinari PDF generated sample (font embedded).
- `sample_font_ikinari_no.pdf`: Ikinari PDF generated sample (font unembedded).
