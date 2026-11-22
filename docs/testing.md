# テスト仕様およびトレーサビリティ (`MinuoPDF`)

本ドキュメントは、**MinuoPDF** のユニットテスト、結合テスト、適格性テスト、Playwright GUI E2E 自動テストの仕様、実行手順、および要件・設計・テスト間のトレーサビリティマトリクスを定義します。最新のテスト実行結果ログは [reports/test-results.md](reports/test-results.md) を参照してください。

## 1. テスト実行手順

テストフィクスチャ（[tests/fixtures/README.md](../tests/fixtures/README.md)）と PDFium は `make test` / `scripts/ensure_*.py` により自動準備されます。

```bash
# Rust 単体 & 結合テスト実行 + docs/reports/test-results.md 自動同期
make test

# Playwright GUI E2E テスト実行 + docs/reports/test-results.md 自動同期
# (Linux / WSL2 初回のみ: npx playwright install && sudo npx playwright install-deps)
make test-gui

# 全テスト実行 & フロントエンド型チェック
npm run check
make test-all
```

---

## 2. テスト仕様一覧

### 2.1 単体テスト仕様 (Unit Verification)

| テスト ID | 名称 | 関連設計・要件 | 検証内容・主なテスト関数 |
| :--- | :--- | :--- | :--- |
| `TEST_UNIT_001` | 画像処理 & 二値化アルゴリズム単体検証 | `DES_BE_001`, `REQ_OPT_002` | Otsu 二値化、CCITT Group 4 エンコード等を検証（`test_otsu_thresholding`, `test_g4_encoding`） |
| `TEST_UNIT_002` | プロファイルマネージャ単体検証 | `DES_BE_PROF`, `REQ_PROF_002` | JSON 書出・読込・削除・書込権限・プリセット定義を検証（`test_profile_save_load_delete`, `test_preset_configs`, `test_profile_dir_writable`, `test_config_version`） |
| `TEST_UNIT_003` | フォント処理エンジン単体検証 | `DES_BE_FONT`, `REQ_OPT_006` | CID マッピング抽出、W 配列パース、サブセット化を検証（`test_extract_hex_gids`, `test_extract_paren_gids`, `test_extract_hex_gids_multi`, `test_parse_w_array_bracket_form`, `test_parse_w_array_range_form`, `test_subset_ikinari_pdf_verification`） |
| `TEST_UNIT_004` | PDF Audit 容量構造解析単体検証 | `DES_BE_AUDIT`, `REQ_AUDIT_001` | カテゴリ別容量算出と比率計算を検証（`test_audit_category_percentage`, `test_audit_user_pdfs_classification`） |

### 2.2 結合テスト仕様 (Integration Verification)

| テスト ID | 名称 | 関連アーキ・設計・要件 | 検証内容 |
| :--- | :--- | :--- | :--- |
| `TEST_INT_001` | 最適化コア & カラー置換結合検証 | `ARCH_BE_OPT`, `REQ_OPT_001`, `REQ_OPT_003` | Color (JPEG)・Gray・Mono (Otsu/Dither)・カスタム DPI/品質のオブジェクト置換パイプライン検証 |
| `TEST_INT_002` | ページラスタライザ (Pdfium) 結合検証 | `ARCH_BE_RAST`, `REQ_OPT_004` | `pdfium-render` による全ページレンダリングおよび Color/Gray/Mono 画像 PDF 生成検証 |
| `TEST_INT_003` | フォント埋め込み解除 & オブジェクト破棄結合検証 | `ARCH_BE_FONT`, `REQ_OPT_005`, `REQ_OPT_006` | フォントサブセット化・埋め込み解除および不要オブジェクト一括破棄の結合検証 |
| `TEST_INT_004` | Tauri IPC コマンド & イベント配信結合検証 | `ARCH_IPC_001`, `REQ_GUI_004`, `REQ_GUI_005` | Tauri `invoke`・進捗率・ログ配信・インメモリ監査比較の IPC 統合検証 |
| `TEST_INT_005` | 非対話バッチ CLI 実行結合検証 | `ARCH_BE_CLI`, `DES_BE_CLI`, `REQ_CLI_002` | `--output`, `--preset`, `--profile`, `--strategy` 指定でのヘッドレス実行完了検証 |
| `TEST_INT_006` | CTM 実効 DPI・Form XObject・SMask・Smart Fallback 結合検証 | `ARCH_BE_OPT`, `DES_BE_001`, `REQ_OPT_007`, `REQ_OPT_008`, `REQ_PERF_001` | CTM 解析 DPI 推定、Form XObject 再帰検出、SMask 追従リサイズ、逆圧縮防止、Rayon 並列動作検証 |

### 2.3 適格性テスト仕様 (Qualification Testing)

| テスト ID | 名称 | 関連要件 | 検証内容 |
| :--- | :--- | :--- | :--- |
| `TEST_QUAL_001` | 容量削減率 & 圧縮性能適格性検証 | `REQ_OPT_001`..`006` | 実サンプル PDF に対する目標圧縮率と閲覧整合性確認 |
| `TEST_QUAL_002` | 非 GPL ライセンス適合性検証 | `REQ_NFR_001` | 全依存クレート（`lopdf`, `image`, `fax` 等）の GPL フリー（MIT/BSD/Apache-2.0）適合確認 |
| `TEST_QUAL_003` | スタンドアロン・ポータビリティ検証 | `REQ_NFR_002`, `REQ_NFR_003` | インストーラ不要起動、同階層 `pdfium.dll` / `profiles/` 読込、Windows プロンプト非表示化・切り離し確認（`conhost.exe` / Windows Terminal） |

### 2.4 E2E GUI 自動テスト仕様 (`tests/e2e/gui.spec.ts`)

| テスト ID | 名称 | 関連設計・要件 | 検証内容 |
| :--- | :--- | :--- | :--- |
| `TEST_GUI_001` | E2E GUI 基本要素・言語・タブ検証 | `DES_GUI_001`, `REQ_GUI_001`..`003`, `REQ_PROF_001` | タイトル・レイアウト、動的 i18n (`auto`/`ja`/`en`・`ja-JP` 自動判定)、タブ切替、プリセット選択 |
| `TEST_GUI_002` | E2E GUI 詳細設定コントロール検証 | `REQ_OPT_001`..`006` | カラーモード、DPI/品質入力、二値化ドロップダウン、破棄チェックボックス、フォントダブルクリック解除切替 |
| `TEST_GUI_003` | E2E GUI プロファイル管理検証 | `REQ_PROF_002` | プロファイル名入力・保存ボタン活性化・読込・削除操作 |
| `TEST_GUI_004` | E2E GUI ファイルリスト操作検証 | `REQ_FILE_001`..`003` | ドロップエリア、追加ボタン、個別削除、一括クリア |
| `TEST_GUI_005` | E2E GUI 処理進捗・ログ表示検証 | `REQ_GUI_004`, `REQ_GUI_005` | 進捗率（0-100%）、プログレスバー、一括最適化開始ボタン、実行ログ表示 |
| `TEST_GUI_006` | E2E GUI PDF Audit 容量内訳モーダル検証 | `DES_GUI_003`, `REQ_AUDIT_001` | Audit モーダル表示、複合棒グラフ・前後比較テーブルのレンダリング |

---

## 3. 要件トレーサビリティマトリクス

### 3.1 要件 ➔ アーキテクチャ

| アーキテクチャ ID | 名称 | 対応要件 ID |
| :--- | :--- | :--- |
| `ARCH_FE_001` | フロントエンド UI & ステート管理 | `REQ_GUI_001`, `REQ_GUI_002`, `REQ_GUI_003`, `REQ_FILE_001` |
| `ARCH_IPC_001` | Tauri IPC ゲートウェイ | `REQ_GUI_004`, `REQ_GUI_005` |
| `ARCH_BE_CLI` | CUI コマンドラインコントローラ | `REQ_CLI_001`, `REQ_CLI_002`, `REQ_NFR_003` |
| `ARCH_BE_PROF` | プロファイルマネージャ | `REQ_PROF_002`, `REQ_NFR_002` |
| `ARCH_BE_OPT` | 最適化コアエンジン | `REQ_OPT_001`..`005`, `REQ_OPT_007`, `REQ_OPT_008`, `REQ_PERF_001` |
| `ARCH_BE_RAST` | ページラスタライザ | `REQ_OPT_004`, `REQ_NFR_002` |
| `ARCH_BE_FONT` | フォント処理エンジン | `REQ_OPT_006` |
| `ARCH_BE_AUDIT` | PDF 容量内訳解析エンジン | `REQ_AUDIT_001`, `REQ_PERF_001` |

### 3.2 アーキテクチャ ➔ 詳細設計

| 詳細設計 ID | 対象ファイル / モジュール | 対応アーキテクチャ ID | 参照 |
| :--- | :--- | :--- | :--- |
| `DES_GUI_001` | `+page.svelte` & Components | `ARCH_FE_001` | [gui.md](design/gui.md) |
| `DES_GUI_002` | `i18n.ts` | `ARCH_FE_001` | [gui.md](design/gui.md) |
| `DES_GUI_003` | `AuditModal.svelte` | `ARCH_FE_001`, `ARCH_BE_AUDIT` | [gui.md](design/gui.md) |
| `DES_BE_001` | `optimizer.rs` | `ARCH_BE_OPT` | [backend.md](design/backend.md) |
| `DES_BE_IMG` | `image_util.rs` | `ARCH_BE_OPT`, `ARCH_BE_RAST` | [backend.md](design/backend.md) |
| `DES_BE_RAST` | `rasterizer.rs` | `ARCH_BE_RAST` | [backend.md](design/backend.md) |
| `DES_BE_AUDIT` | `audit.rs` | `ARCH_BE_AUDIT` | [backend.md](design/backend.md) |
| `DES_BE_CLI` | `cli.rs` | `ARCH_BE_CLI` | [backend.md](design/backend.md) |
| `DES_BE_FONT` | `font_subsetter.rs` | `ARCH_BE_FONT` | [backend.md](design/backend.md) |
| `DES_BE_PROF` | `profile.rs` | `ARCH_BE_PROF` | [backend.md](design/backend.md) |

### 3.3 要件・設計 ➔ テスト仕様 ➔ 実行結果

各テスト仕様（`TEST_UNIT_*`, `TEST_INT_*`, `TEST_QUAL_*`, `TEST_GUI_*`）に対応する最新実行ログおよび結果サマリー（`TR_*`）は [reports/test-results.md](reports/test-results.md) で管理されています。
