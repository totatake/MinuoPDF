========================================
テスト実行結果 (Test Execution Results)
========================================

本ページは、MinuoPDF アプリケーションのすべてのテスト（単体テスト、結合テスト、適格性テスト、GUI E2Eテスト）の最新実行結果ログです。

総合テスト実行サマリー
======================

.. test_result:: 単体テスト (Unit Test) 実行サマリー
   :id: TR_UNIT_SUMMARY
   :links: TEST_UNIT_001, TEST_UNIT_002, TEST_UNIT_003, TEST_UNIT_004
   :result: PASSED
   :last_run: 2026-08-02 10:24:29

   - **最終実行日時**: 2026-08-02 10:24:29
   - **総合判定**: ``PASSED`` (14 PASSED / 0 FAILED / 計 14 テストケース)
   - **対象仕様**: :need:`TEST_UNIT_001`, :need:`TEST_UNIT_002`, :need:`TEST_UNIT_003`, :need:`TEST_UNIT_004`

.. test_result:: 結合テスト (Integration Test) 実行サマリー
   :id: TR_INT_SUMMARY
   :links: TEST_INT_001, TEST_INT_002, TEST_INT_003, TEST_INT_004
   :result: PASSED
   :last_run: 2026-08-02 10:24:29

   - **最終実行日時**: 2026-08-02 10:24:29
   - **総合判定**: ``PASSED`` (12 PASSED / 0 FAILED / 計 12 テストケース)
   - **対象仕様**: :need:`TEST_INT_001`, :need:`TEST_INT_002`, :need:`TEST_INT_003`, :need:`TEST_INT_004`

.. test_result:: 適格性テスト (Qualification Test) 実行サマリー
   :id: TR_QUAL_SUMMARY
   :links: TEST_QUAL_001, TEST_QUAL_002, TEST_QUAL_003
   :result: PASSED
   :last_run: 2026-08-02 10:24:29

   - **最終実行日時**: 2026-08-02 10:24:29
   - **総合判定**: ``PASSED`` (3 PASSED / 0 FAILED / 計 3 テストケース)
   - **対象仕様**: :need:`TEST_QUAL_001`, :need:`TEST_QUAL_002`, :need:`TEST_QUAL_003`

.. test_result:: E2E GUI 自動テスト実行サマリー
   :id: TR_GUI_E2E
   :links: TEST_GUI_001, TEST_GUI_002, TEST_GUI_003, TEST_GUI_004, TEST_GUI_005
   :result: N/A
   :last_run: N/A

   - **最終実行日時**: N/A
   - **総合判定**: ``N/A`` (0 PASSED / 0 FAILED / 計 0 テストケース)
   - **対象仕様**: :need:`TEST_GUI_001`, :need:`TEST_GUI_002`, :need:`TEST_GUI_003`, :need:`TEST_GUI_004`, :need:`TEST_GUI_005`

単体テスト実行結果一覧 (Unit Tests)
=========================================

.. test_result:: audit::tests::test_audit_category_percentage
   :id: TR_UNIT_001
   :links: TEST_UNIT_004
   :result: PASSED
   :last_run: 2026-08-02 10:24:29

   - **ステータス**: ``PASSED``
   - **対象仕様**: :need:`TEST_UNIT_004`

.. test_result:: font_subsetter::tests::test_extract_paren_gids
   :id: TR_UNIT_002
   :links: TEST_UNIT_003
   :result: PASSED
   :last_run: 2026-08-02 10:24:29

   - **ステータス**: ``PASSED``
   - **対象仕様**: :need:`TEST_UNIT_003`

.. test_result:: font_subsetter::tests::test_parse_w_array_bracket_form
   :id: TR_UNIT_003
   :links: TEST_UNIT_003
   :result: PASSED
   :last_run: 2026-08-02 10:24:29

   - **ステータス**: ``PASSED``
   - **対象仕様**: :need:`TEST_UNIT_003`

.. test_result:: font_subsetter::tests::test_extract_hex_gids
   :id: TR_UNIT_004
   :links: TEST_UNIT_003
   :result: PASSED
   :last_run: 2026-08-02 10:24:29

   - **ステータス**: ``PASSED``
   - **対象仕様**: :need:`TEST_UNIT_003`

.. test_result:: font_subsetter::tests::test_extract_hex_gids_multi
   :id: TR_UNIT_005
   :links: TEST_UNIT_003
   :result: PASSED
   :last_run: 2026-08-02 10:24:29

   - **ステータス**: ``PASSED``
   - **対象仕様**: :need:`TEST_UNIT_003`

.. test_result:: font_subsetter::tests::test_parse_w_array_range_form
   :id: TR_UNIT_006
   :links: TEST_UNIT_003
   :result: PASSED
   :last_run: 2026-08-02 10:24:29

   - **ステータス**: ``PASSED``
   - **対象仕様**: :need:`TEST_UNIT_003`

.. test_result:: tests::test_preset_configs
   :id: TR_UNIT_007
   :links: TEST_UNIT_002
   :result: PASSED
   :last_run: 2026-08-02 10:24:29

   - **ステータス**: ``PASSED``
   - **対象仕様**: :need:`TEST_UNIT_002`

.. test_result:: tests::test_config_version
   :id: TR_UNIT_008
   :links: TEST_UNIT_002
   :result: PASSED
   :last_run: 2026-08-02 10:24:29

   - **ステータス**: ``PASSED``
   - **対象仕様**: :need:`TEST_UNIT_002`

.. test_result:: tests::test_g4_encoding
   :id: TR_UNIT_009
   :links: TEST_UNIT_001
   :result: PASSED
   :last_run: 2026-08-02 10:24:29

   - **ステータス**: ``PASSED``
   - **対象仕様**: :need:`TEST_UNIT_001`

.. test_result:: tests::test_otsu_thresholding
   :id: TR_UNIT_010
   :links: TEST_UNIT_001
   :result: PASSED
   :last_run: 2026-08-02 10:24:29

   - **ステータス**: ``PASSED``
   - **対象仕様**: :need:`TEST_UNIT_001`

.. test_result:: tests::test_profile_dir_writable
   :id: TR_UNIT_011
   :links: TEST_UNIT_002
   :result: PASSED
   :last_run: 2026-08-02 10:24:29

   - **ステータス**: ``PASSED``
   - **対象仕様**: :need:`TEST_UNIT_002`

.. test_result:: tests::test_profile_save_load_delete
   :id: TR_UNIT_012
   :links: TEST_UNIT_002
   :result: PASSED
   :last_run: 2026-08-02 10:24:29

   - **ステータス**: ``PASSED``
   - **対象仕様**: :need:`TEST_UNIT_002`

.. test_result:: audit::tests::test_audit_user_pdfs_classification
   :id: TR_UNIT_013
   :links: TEST_UNIT_004
   :result: PASSED
   :last_run: 2026-08-02 10:24:29

   - **ステータス**: ``PASSED``
   - **対象仕様**: :need:`TEST_UNIT_004`

.. test_result:: font_subsetter::tests::test_subset_ikinari_pdf_verification
   :id: TR_UNIT_014
   :links: TEST_UNIT_003
   :result: PASSED
   :last_run: 2026-08-02 10:24:29

   - **ステータス**: ``PASSED``
   - **対象仕様**: :need:`TEST_UNIT_003`

結合テスト実行結果一覧 (Integration Tests)
===========================================

.. test_result:: test_integration_unembed_fonts
   :id: TR_INT_001
   :links: TEST_INT_003
   :result: PASSED
   :last_run: 2026-08-02 10:24:29

   - **ステータス**: ``PASSED``
   - **対象仕様**: :need:`TEST_INT_003`

.. test_result:: test_integration_gray_replace
   :id: TR_INT_002
   :links: TEST_INT_001
   :result: PASSED
   :last_run: 2026-08-02 10:24:29

   - **ステータス**: ``PASSED``
   - **対象仕様**: :need:`TEST_INT_001`

.. test_result:: test_integration_mono_dither_replace
   :id: TR_INT_003
   :links: TEST_INT_001
   :result: PASSED
   :last_run: 2026-08-02 10:24:29

   - **ステータス**: ``PASSED``
   - **対象仕様**: :need:`TEST_INT_001`

.. test_result:: test_integration_custom_dpi_quality
   :id: TR_INT_004
   :links: TEST_INT_001
   :result: PASSED
   :last_run: 2026-08-02 10:24:29

   - **ステータス**: ``PASSED``
   - **対象仕様**: :need:`TEST_INT_001`

.. test_result:: test_integration_mono_otsu_replace
   :id: TR_INT_005
   :links: TEST_INT_001
   :result: PASSED
   :last_run: 2026-08-02 10:24:29

   - **ステータス**: ``PASSED``
   - **対象仕様**: :need:`TEST_INT_001`

.. test_result:: test_integration_discard_objects
   :id: TR_INT_006
   :links: TEST_INT_003
   :result: PASSED
   :last_run: 2026-08-02 10:24:29

   - **ステータス**: ``PASSED``
   - **対象仕様**: :need:`TEST_INT_003`

.. test_result:: test_integration_color_replace
   :id: TR_INT_007
   :links: TEST_INT_001
   :result: PASSED
   :last_run: 2026-08-02 10:24:29

   - **ステータス**: ``PASSED``
   - **対象仕様**: :need:`TEST_INT_001`

.. test_result:: test_integration_mono_otsu_rasterize
   :id: TR_INT_008
   :links: TEST_INT_002
   :result: PASSED
   :last_run: 2026-08-02 10:24:29

   - **ステータス**: ``PASSED``
   - **対象仕様**: :need:`TEST_INT_002`

.. test_result:: test_integration_mono_dither_rasterize
   :id: TR_INT_009
   :links: TEST_INT_002
   :result: PASSED
   :last_run: 2026-08-02 10:24:29

   - **ステータス**: ``PASSED``
   - **対象仕様**: :need:`TEST_INT_002`

.. test_result:: test_integration_gray_rasterize
   :id: TR_INT_010
   :links: TEST_INT_002
   :result: PASSED
   :last_run: 2026-08-02 10:24:29

   - **ステータス**: ``PASSED``
   - **対象仕様**: :need:`TEST_INT_002`

.. test_result:: test_integration_font_subsetting
   :id: TR_INT_011
   :links: TEST_INT_003
   :result: PASSED
   :last_run: 2026-08-02 10:24:29

   - **ステータス**: ``PASSED``
   - **対象仕様**: :need:`TEST_INT_003`

.. test_result:: test_integration_color_rasterize
   :id: TR_INT_012
   :links: TEST_INT_002
   :result: PASSED
   :last_run: 2026-08-02 10:24:29

   - **ステータス**: ``PASSED``
   - **対象仕様**: :need:`TEST_INT_002`

適格性テスト実行結果一覧 (Qualification Tests)
===============================================

.. test_result:: 容量削減率 & 圧縮性能適格性検証
   :id: TR_QUAL_001
   :links: TEST_QUAL_001
   :result: PASSED
   :last_run: 2026-08-02 10:24:29

   - **ステータス**: ``PASSED``
   - **検証概要**: カラー・グレー・スキャンモノクロPDFにおける圧縮性能と閲覧表示の健全性を確認
   - **対象仕様**: :need:`TEST_QUAL_001`

.. test_result:: 完全非GPLライセンス適合性検証
   :id: TR_QUAL_002
   :links: TEST_QUAL_002
   :result: PASSED
   :last_run: 2026-08-02 10:24:29

   - **ステータス**: ``PASSED``
   - **検証概要**: すべての依存ライブラリ（lopdf, pdfium-render, image, fax等）のGPLフリー（MIT/BSD/Apache-2.0）適合を確認
   - **対象仕様**: :need:`TEST_QUAL_002`

.. test_result:: スタンドアロン・ポータビリティ検証
   :id: TR_QUAL_003
   :links: TEST_QUAL_003
   :result: PASSED
   :last_run: 2026-08-02 10:24:29

   - **ステータス**: ``PASSED``
   - **検証概要**: インストーラ不要での単独起動、同階層pdfium.dllのロードおよびプロンプト非表示(SW_HIDE)を確認
   - **対象仕様**: :need:`TEST_QUAL_003`
