# バックエンド詳細設計 (`MinuoPDF`)

バックエンドは Rust（Tauri Host）で実装されており、高速かつ安全な PDF オブジェクト操作と画像処理を提供します。システム全体の構成は [../architecture.md](../architecture.md) を参照してください。

## 1. 設計モジュール一覧

| 設計 ID | 対象ファイル | 関連アーキテクチャ |
| :--- | :--- | :--- |
| `DES_BE_001` | `src-tauri/src/optimizer.rs` | `ARCH_BE_OPT` |
| `DES_BE_IMG` | `src-tauri/src/image_util.rs` | `ARCH_BE_OPT`, `ARCH_BE_RAST` |
| `DES_BE_RAST` | `src-tauri/src/rasterizer.rs` | `ARCH_BE_RAST` |
| `DES_BE_AUDIT` | `src-tauri/src/audit.rs` | `ARCH_BE_AUDIT` |
| `DES_BE_CLI` | `src-tauri/src/cli.rs` | `ARCH_BE_CLI` |
| `DES_BE_FONT` | `src-tauri/src/font_subsetter.rs` | `ARCH_BE_FONT` |
| `DES_BE_PROF` | `src-tauri/src/profile.rs` | `ARCH_BE_PROF` |

---

## 2. モジュール詳細仕様

### `DES_BE_001`: `optimizer.rs` — 最適化処理エンジン (`ARCH_BE_OPT`)

- **主要関数**:
  - `optimize_pdf_with_audit(input_path, output_path, config, progress_callback) -> Result<(u64, u64, Option<PdfAuditReport>, Option<PdfAuditReport>), String>`
  - `optimize_pdf(...) -> Result<(u64, u64), String>`
- **処理手順**:
  1. `lopdf::Document::load` により PDF 構造をインメモリ読み込みし、直ちに `audit_document` で変換前容量内訳を収集する。
  2. 各ページのコンテンツストリームをパースし、現在変換行列（CTM）および描画オペレータ（`cm`, `Do`）から画像の実効描画サイズ（pt）を推定する。
  3. ページリソース内の Form XObject（`/Subtype /Form`）を再帰走査して、ネストされた画像オブジェクトを完全抽出する。
  4. 画像タスク群（`ImageTask`）を構築し、`Rayon` の `into_par_iter()` によりマルチコア並列でデコード・目標 DPI リサイズ・再エンコード・SMask 連動リサイズを実行する。
  5. Smart Fallback（逆圧縮ガード）により、無変換画像の肥大化時は元ストリームを自動維持する。
  6. 設定に応じた破棄処理（`/Annots`, `/Outlines`, `/AcroForm`, `/Metadata`, `/Names`, `/Thumb` 辞書キーの物理除去）を適用する。
  7. フォント解除・フォントサブセット化、未使用オブジェクト消去（`delete_zero_length_streams`, `prune_objects`）、および Deflate 圧縮を行う。
  8. `doc.save_to` による最適化済み PDF 出力後、保存済みインメモリ Document から直ちに `audit_document` で変換後容量内訳を生成する（重複ディスク I/O ゼロ）。

### `DES_BE_IMG`: `image_util.rs` — 画像エンコード一元化モジュール (`ARCH_BE_OPT`, `ARCH_BE_RAST`)

- **主要構造体・関数**: `EncodedPdfImage`, `encode_image_stream(img, format, quality, mono_method) -> Result<EncodedPdfImage, String>`
- **仕様**:
  - JPEG（RGB/Gray）、PNG/FlateDecode、および CCITT Group 4（Otsu / Floyd-Steinberg 二値化 + `fax::encode_g4`）のエンコード処理を一元管理する。
  - PDF 辞書に必要な `ColorSpace`, `BitsPerComponent`, `Filter`, `DecodeParms`（Group 4 パラメータ等）を過不足なく生成する。

### `DES_BE_RAST`: `rasterizer.rs` — ページ全体ラスタライズ (`ARCH_BE_RAST`)

- **主要関数**: `rasterize_pdf_with_audit(...)`, `rasterize_pdf(...)`
- **仕様**:
  - `pdfium-render` の `Pdfium` インスタンスを生成し、全ページをターゲット DPI で Bitmap レンダリングする。
  - レンダリング画像を `image_util::encode_image_stream` にて再圧縮し、新しい空の PDF ドキュメントへページ毎にストリーム配置して出力する。

### `DES_BE_AUDIT`: `audit.rs` — PDF 容量内訳解析モジュール (`ARCH_BE_AUDIT`)

- **主要構造体・関数**: `PdfAuditReport`, `PdfAuditComparison`, `audit_pdf(path)`, `audit_document(doc, file_name, total_bytes)`
- **仕様**:
  - PDF 内の全オブジェクトを走査し、Images, Fonts, Content Streams, Metadata/Catalog, Struct/Forms, Other の 6 カテゴリに完全分類する。
  - ディスクからのファイルパス指定実行と、最適化パイプライン内で開かれたインメモリ Document 参照からの直接解析の両方をサポートする。

### `DES_BE_CLI`: `cli.rs` — CLI コマンドライン実行モジュール (`ARCH_BE_CLI`)

- **主要関数**: `run_interactive_cli(input_path)`, `run_batch_cli(input_paths, output, preset, profile, strategy)`
- **仕様**: 引数指定による完全非対話ヘッドレスバッチ圧縮と、単体ファイル指定時の矢印キー対話式メニューを両立する。

### `DES_BE_FONT`: `font_subsetter.rs` — フォント処理 (`ARCH_BE_FONT`)

- **フォント抽出**: Content Stream オペレータ（`Tj`, `TJ`）を走査して使用されている GID / Glyph を特定する。
- **フォント解除**: Target FontDescriptor の `FontFile` / `FontFile2` / `FontFile3` ストリーム参照を物理的に削除し、標準代替フォントに切り替えさせる。

### `DES_BE_PROF`: `profile.rs` — JSON プロファイル管理 (`ARCH_BE_PROF`)

- **関数**: `load_profiles() -> Vec<Profile>`, `save_profile(name: &str, config: OptimizeConfig)`
- **仕様**: `profiles/` ディレクトリ構造のファイル読み書きおよび JSON schema バージョン（`version: 2`）チェックを行う。
