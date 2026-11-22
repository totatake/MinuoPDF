# テスト実行結果 (Test Execution Results)

本ページは、MinuoPDF アプリケーションのすべてのテスト（単体テスト、結合テスト、適格性テスト、GUI E2E テスト）の最新実行結果レポートです（`scripts/sync_test_results.py` により自動生成されます）。仕様詳細は [../testing.md](../testing.md) を参照してください。

## 総合テスト実行サマリー

| 結果 ID | テストスイート | 総合判定 | 内訳 (PASSED / FAILED / 計) | 対象仕様 ID | 最終実行日時 |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `TR_UNIT_SUMMARY` | 単体テスト (Unit Test) | `PASSED` | 14 PASSED / 0 FAILED / 計 14 | `TEST_UNIT_001`, `TEST_UNIT_002`, `TEST_UNIT_003`, `TEST_UNIT_004` | `2026-11-22 13:28:35` |
| `TR_INT_SUMMARY` | 結合テスト (Integration Test) | `PASSED` | 12 PASSED / 0 FAILED / 計 12 | `TEST_INT_001`, `TEST_INT_002`, `TEST_INT_003`, `TEST_INT_004` | `2026-11-22 13:28:35` |
| `TR_QUAL_SUMMARY` | 適格性テスト (Qualification Test) | `PASSED` | 3 PASSED / 0 FAILED / 計 3 | `TEST_QUAL_001`, `TEST_QUAL_002`, `TEST_QUAL_003` | `2026-11-22 13:28:35` |
| `TR_GUI_E2E` | E2E GUI 自動テスト | `PASSED` | 10 PASSED / 0 FAILED / 計 10 | `TEST_GUI_001`, `TEST_GUI_002`, `TEST_GUI_003`, `TEST_GUI_004`, `TEST_GUI_005` | `2026-11-22 13:28:35` |

## 単体テスト実行結果一覧 (Unit Tests)

| 結果 ID | テストケース | 対象仕様 | ステータス | 最終実行日時 |
| :--- | :--- | :--- | :--- | :--- |
| `TR_UNIT_001` | `audit::tests::test_audit_category_percentage` | `TEST_UNIT_004` | `PASSED` | `2026-11-22 13:28:35` |
| `TR_UNIT_002` | `font_subsetter::tests::test_extract_hex_gids` | `TEST_UNIT_003` | `PASSED` | `2026-11-22 13:28:35` |
| `TR_UNIT_003` | `font_subsetter::tests::test_extract_hex_gids_multi` | `TEST_UNIT_003` | `PASSED` | `2026-11-22 13:28:35` |
| `TR_UNIT_004` | `font_subsetter::tests::test_extract_paren_gids` | `TEST_UNIT_003` | `PASSED` | `2026-11-22 13:28:35` |
| `TR_UNIT_005` | `font_subsetter::tests::test_parse_w_array_bracket_form` | `TEST_UNIT_003` | `PASSED` | `2026-11-22 13:28:35` |
| `TR_UNIT_006` | `tests::test_config_version` | `TEST_UNIT_002` | `PASSED` | `2026-11-22 13:28:35` |
| `TR_UNIT_007` | `tests::test_preset_configs` | `TEST_UNIT_002` | `PASSED` | `2026-11-22 13:28:35` |
| `TR_UNIT_008` | `tests::test_otsu_thresholding` | `TEST_UNIT_001` | `PASSED` | `2026-11-22 13:28:35` |
| `TR_UNIT_009` | `tests::test_g4_encoding` | `TEST_UNIT_001` | `PASSED` | `2026-11-22 13:28:35` |
| `TR_UNIT_010` | `font_subsetter::tests::test_parse_w_array_range_form` | `TEST_UNIT_003` | `PASSED` | `2026-11-22 13:28:35` |
| `TR_UNIT_011` | `tests::test_profile_dir_writable` | `TEST_UNIT_002` | `PASSED` | `2026-11-22 13:28:35` |
| `TR_UNIT_012` | `tests::test_profile_save_load_delete` | `TEST_UNIT_002` | `PASSED` | `2026-11-22 13:28:35` |
| `TR_UNIT_013` | `audit::tests::test_audit_user_pdfs_classification` | `TEST_UNIT_004` | `PASSED` | `2026-11-22 13:28:35` |
| `TR_UNIT_014` | `font_subsetter::tests::test_subset_ikinari_pdf_verification` | `TEST_UNIT_003` | `PASSED` | `2026-11-22 13:28:35` |

## 結合テスト実行結果一覧 (Integration Tests)

| 結果 ID | テストケース | 対象仕様 | ステータス | 最終実行日時 |
| :--- | :--- | :--- | :--- | :--- |
| `TR_INT_001` | `test_integration_unembed_fonts` | `TEST_INT_003` | `PASSED` | `2026-11-22 13:28:35` |
| `TR_INT_002` | `test_integration_mono_dither_replace` | `TEST_INT_001` | `PASSED` | `2026-11-22 13:28:35` |
| `TR_INT_003` | `test_integration_gray_replace` | `TEST_INT_001` | `PASSED` | `2026-11-22 13:28:35` |
| `TR_INT_004` | `test_integration_custom_dpi_quality` | `TEST_INT_001` | `PASSED` | `2026-11-22 13:28:35` |
| `TR_INT_005` | `test_integration_color_replace` | `TEST_INT_001` | `PASSED` | `2026-11-22 13:28:35` |
| `TR_INT_006` | `test_integration_discard_objects` | `TEST_INT_003` | `PASSED` | `2026-11-22 13:28:35` |
| `TR_INT_007` | `test_integration_mono_otsu_replace` | `TEST_INT_001` | `PASSED` | `2026-11-22 13:28:35` |
| `TR_INT_008` | `test_integration_color_rasterize` | `TEST_INT_002` | `PASSED` | `2026-11-22 13:28:35` |
| `TR_INT_009` | `test_integration_mono_dither_rasterize` | `TEST_INT_002` | `PASSED` | `2026-11-22 13:28:35` |
| `TR_INT_010` | `test_integration_mono_otsu_rasterize` | `TEST_INT_002` | `PASSED` | `2026-11-22 13:28:35` |
| `TR_INT_011` | `test_integration_font_subsetting` | `TEST_INT_003` | `PASSED` | `2026-11-22 13:28:35` |
| `TR_INT_012` | `test_integration_gray_rasterize` | `TEST_INT_002` | `PASSED` | `2026-11-22 13:28:35` |

## 適格性テスト実行結果一覧 (Qualification Tests)

| 結果 ID | テスト項目 | 検証概要 | 対象仕様 | ステータス | 最終実行日時 |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `TR_QUAL_001` | 容量削減率 & 圧縮性能適格性検証 | カラー・グレー・スキャンモノクロPDFにおける圧縮性能と閲覧表示の健全性を確認 | `TEST_QUAL_001` | `PASSED` | `2026-11-22 13:28:35` |
| `TR_QUAL_002` | 完全非GPLライセンス適合性検証 | すべての依存ライブラリ（lopdf, pdfium-render, image, fax等）のGPLフリー（MIT/BSD/Apache-2.0）適合を確認 | `TEST_QUAL_002` | `PASSED` | `2026-11-22 13:28:35` |
| `TR_QUAL_003` | スタンドアロン・ポータビリティ検証 | インストーラ不要での単独起動、同階層pdfium.dllのロードおよびプロンプト非表示(SW_HIDE)を確認 | `TEST_QUAL_003` | `PASSED` | `2026-11-22 13:28:35` |

## E2E GUI テスト実行結果一覧 (GUI E2E Tests)

| 結果 ID | テストケース | 所要時間 | 関連要件 / 仕様 | ステータス | 最終実行日時 |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `TR_CASE_001` | REQ_GUI_001: App title and main layout elements exist | 1774 ms | `REQ_GUI_001` | `PASSED` | `2026-11-22 13:28:35` |
| `TR_CASE_002` | REQ_GUI_002: Dynamic Language Toggle (Auto / English / Japanese) | 2935 ms | `REQ_GUI_002` | `PASSED` | `2026-11-22 13:28:35` |
| `TR_CASE_003` | REQ_PROF_001 & REQ_GUI_003: Tab switching (Easy Mode <-> Detail Mode) | 1997 ms | `REQ_PROF_001`, `REQ_GUI_003` | `PASSED` | `2026-11-22 13:28:35` |
| `TR_CASE_004` | REQ_PROF_001: Preset Radio Selection in Easy Mode | 1924 ms | `REQ_PROF_001` | `PASSED` | `2026-11-22 13:28:35` |
| `TR_CASE_005` | REQ_OPT_006: Font Subsetting & Unembedding GUI toggle on double-click | 1994 ms | `REQ_OPT_006` | `PASSED` | `2026-11-22 13:28:35` |
| `TR_CASE_006` | REQ_FILE_003: Target File List Actions (Clear All button visibility) | 2087 ms | `REQ_FILE_003` | `PASSED` | `2026-11-22 13:28:35` |
| `TR_CASE_007` | REQ_OPT_001 & REQ_OPT_002 & REQ_OPT_003: Image Optimization Controls in Detail Mode | 1005 ms | `REQ_OPT_001`, `REQ_OPT_002`, `REQ_OPT_003` | `PASSED` | `2026-11-22 13:28:35` |
| `TR_CASE_008` | REQ_OPT_005: Discard & Cleanup Checkboxes in Detail Mode | 1080 ms | `REQ_OPT_005` | `PASSED` | `2026-11-22 13:28:35` |
| `TR_CASE_009` | REQ_PROF_002: Profile Management Tab Controls | 1060 ms | `REQ_PROF_002` | `PASSED` | `2026-11-22 13:28:35` |
| `TR_CASE_010` | REQ_GUI_004 & REQ_GUI_005: Status, Progress Bar, and Log Footer Area | 512 ms | `REQ_GUI_004`, `REQ_GUI_005` | `PASSED` | `2026-11-22 13:28:35` |
