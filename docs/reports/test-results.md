# テスト実行結果 (Test Execution Results)

本ページは、MinuoPDF アプリケーションのすべてのテスト（単体テスト、結合テスト、適格性テスト、GUI E2E テスト）の最新実行結果レポートです（`scripts/sync_test_results.py` により自動生成されます）。仕様詳細は [../testing.md](../testing.md) を参照してください。

## 総合テスト実行サマリー

| 結果 ID | テストスイート | 総合判定 | 内訳 (PASSED / FAILED / 計) | 対象仕様 ID | 最終実行日時 |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `TR_UNIT_SUMMARY` | 単体テスト (Unit Test) | `PASSED` | 14 PASSED / 0 FAILED / 計 14 | `TEST_UNIT_001`, `TEST_UNIT_002`, `TEST_UNIT_003`, `TEST_UNIT_004` | `2026-08-02 10:24:29` |
| `TR_INT_SUMMARY` | 結合テスト (Integration Test) | `PASSED` | 12 PASSED / 0 FAILED / 計 12 | `TEST_INT_001`, `TEST_INT_002`, `TEST_INT_003`, `TEST_INT_004` | `2026-08-02 10:24:29` |
| `TR_QUAL_SUMMARY` | 適格性テスト (Qualification Test) | `PASSED` | 3 PASSED / 0 FAILED / 計 3 | `TEST_QUAL_001`, `TEST_QUAL_002`, `TEST_QUAL_003` | `2026-08-02 10:24:29` |
| `TR_GUI_E2E` | E2E GUI 自動テスト | `N/A` | 0 PASSED / 0 FAILED / 計 0 | `TEST_GUI_001`, `TEST_GUI_002`, `TEST_GUI_003`, `TEST_GUI_004`, `TEST_GUI_005` | `N/A` |

## 単体テスト実行結果一覧 (Unit Tests)

| 結果 ID | テストケース | 対象仕様 | ステータス | 最終実行日時 |
| :--- | :--- | :--- | :--- | :--- |
| `TR_UNIT_001` | `audit::tests::test_audit_category_percentage` | `TEST_UNIT_004` | `PASSED` | `2026-08-02 10:24:29` |
| `TR_UNIT_002` | `font_subsetter::tests::test_extract_paren_gids` | `TEST_UNIT_003` | `PASSED` | `2026-08-02 10:24:29` |
| `TR_UNIT_003` | `font_subsetter::tests::test_parse_w_array_bracket_form` | `TEST_UNIT_003` | `PASSED` | `2026-08-02 10:24:29` |
| `TR_UNIT_004` | `font_subsetter::tests::test_extract_hex_gids` | `TEST_UNIT_003` | `PASSED` | `2026-08-02 10:24:29` |
| `TR_UNIT_005` | `font_subsetter::tests::test_extract_hex_gids_multi` | `TEST_UNIT_003` | `PASSED` | `2026-08-02 10:24:29` |
| `TR_UNIT_006` | `font_subsetter::tests::test_parse_w_array_range_form` | `TEST_UNIT_003` | `PASSED` | `2026-08-02 10:24:29` |
| `TR_UNIT_007` | `tests::test_preset_configs` | `TEST_UNIT_002` | `PASSED` | `2026-08-02 10:24:29` |
| `TR_UNIT_008` | `tests::test_config_version` | `TEST_UNIT_002` | `PASSED` | `2026-08-02 10:24:29` |
| `TR_UNIT_009` | `tests::test_g4_encoding` | `TEST_UNIT_001` | `PASSED` | `2026-08-02 10:24:29` |
| `TR_UNIT_010` | `tests::test_otsu_thresholding` | `TEST_UNIT_001` | `PASSED` | `2026-08-02 10:24:29` |
| `TR_UNIT_011` | `tests::test_profile_dir_writable` | `TEST_UNIT_002` | `PASSED` | `2026-08-02 10:24:29` |
| `TR_UNIT_012` | `tests::test_profile_save_load_delete` | `TEST_UNIT_002` | `PASSED` | `2026-08-02 10:24:29` |
| `TR_UNIT_013` | `audit::tests::test_audit_user_pdfs_classification` | `TEST_UNIT_004` | `PASSED` | `2026-08-02 10:24:29` |
| `TR_UNIT_014` | `font_subsetter::tests::test_subset_ikinari_pdf_verification` | `TEST_UNIT_003` | `PASSED` | `2026-08-02 10:24:29` |

## 結合テスト実行結果一覧 (Integration Tests)

| 結果 ID | テストケース | 対象仕様 | ステータス | 最終実行日時 |
| :--- | :--- | :--- | :--- | :--- |
| `TR_INT_001` | `test_integration_unembed_fonts` | `TEST_INT_003` | `PASSED` | `2026-08-02 10:24:29` |
| `TR_INT_002` | `test_integration_gray_replace` | `TEST_INT_001` | `PASSED` | `2026-08-02 10:24:29` |
| `TR_INT_003` | `test_integration_mono_dither_replace` | `TEST_INT_001` | `PASSED` | `2026-08-02 10:24:29` |
| `TR_INT_004` | `test_integration_custom_dpi_quality` | `TEST_INT_001` | `PASSED` | `2026-08-02 10:24:29` |
| `TR_INT_005` | `test_integration_mono_otsu_replace` | `TEST_INT_001` | `PASSED` | `2026-08-02 10:24:29` |
| `TR_INT_006` | `test_integration_discard_objects` | `TEST_INT_003` | `PASSED` | `2026-08-02 10:24:29` |
| `TR_INT_007` | `test_integration_color_replace` | `TEST_INT_001` | `PASSED` | `2026-08-02 10:24:29` |
| `TR_INT_008` | `test_integration_mono_otsu_rasterize` | `TEST_INT_002` | `PASSED` | `2026-08-02 10:24:29` |
| `TR_INT_009` | `test_integration_mono_dither_rasterize` | `TEST_INT_002` | `PASSED` | `2026-08-02 10:24:29` |
| `TR_INT_010` | `test_integration_gray_rasterize` | `TEST_INT_002` | `PASSED` | `2026-08-02 10:24:29` |
| `TR_INT_011` | `test_integration_font_subsetting` | `TEST_INT_003` | `PASSED` | `2026-08-02 10:24:29` |
| `TR_INT_012` | `test_integration_color_rasterize` | `TEST_INT_002` | `PASSED` | `2026-08-02 10:24:29` |

## 適格性テスト実行結果一覧 (Qualification Tests)

| 結果 ID | テスト項目 | 検証概要 | 対象仕様 | ステータス | 最終実行日時 |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `TR_QUAL_001` | 容量削減率 & 圧縮性能適格性検証 | カラー・グレー・スキャンモノクロPDFにおける圧縮性能と閲覧表示の健全性を確認 | `TEST_QUAL_001` | `PASSED` | `2026-08-02 10:24:29` |
| `TR_QUAL_002` | 完全非GPLライセンス適合性検証 | すべての依存ライブラリ（lopdf, pdfium-render, image, fax等）のGPLフリー（MIT/BSD/Apache-2.0）適合を確認 | `TEST_QUAL_002` | `PASSED` | `2026-08-02 10:24:29` |
| `TR_QUAL_003` | スタンドアロン・ポータビリティ検証 | インストーラ不要での単独起動、同階層pdfium.dllのロードおよびプロンプト非表示(SW_HIDE)を確認 | `TEST_QUAL_003` | `PASSED` | `2026-08-02 10:24:29` |
