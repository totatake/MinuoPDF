========================================
バックエンド詳細設計 (Backend Design)
========================================

概要
====

バックエンドは Rust (Tauri Host) で実装されており、高速かつ安全な PDF オブジェクト操作と画像処理を提供します。

モジュール詳細設計
===================

.. design:: optimizer.rs - 最適化処理エンジン
   :id: DES_BE_001
   :links: ARCH_BE_OPT

   - **主要関数**: ``optimize_pdf(input_path: &Path, output_path: &Path, config: &OptimizeConfig) -> Result<OptimizeResult>``
   - **処理手順**:
     1. ``lopdf::Document::load`` により PDF 構造をインメモリ読み込み。
     2. ページオブジェクトツリーを走査し、画像 XObject (``/Subtype /Image``) を抽出。
     3. カラーモード選択（Color / Gray / Mono）に応じた再圧縮処理：

        - Color/Gray: ``image::codecs::jpeg::JpegEncoder`` による Quality 指定再圧縮。
        - Mono: ``大津の二値化`` または ``Floyd-Steinberg ディザリング`` 変換後、``fax::encode_g4`` による CCITT Group 4 エンコード。

     4. 設定に応じた破棄処理（``/Annots``, ``/Outlines``, ``/AcroForm``, ``/Metadata`` 辞書キーの除去）。
     5. ``doc.save`` による最適化済み PDF の出力保存。

.. design:: rasterizer.rs - ページ全体ラスタライズ
   :id: DES_BE_RAST
   :links: ARCH_BE_RAST

   - **主要関数**: ``rasterize_pdf_pages(doc: &Document, dpi: u32) -> Result<Vec<DynamicImage>>``
   - ``pdfium-render`` の ``Pdfium`` インスタンスを生成し、全ページをターゲット DPI で Bitmap レンダリングして画像配列を取得する。

.. design:: font_subsetter.rs & font_unembedder.rs - フォント処理
   :id: DES_BE_FONT
   :links: ARCH_BE_FONT

   - **フォント抽出**: Content Stream オペレータ（``Tj``, ``TJ``）を走査して使用されている GID / Glyph を特定。
   - **フォント解除**: Target FontDescriptor の ``FontFile`` / ``FontFile2`` / ``FontFile3`` ストリーム参照を物理的に削除し、標準代替フォントに切り替えさせる。

.. design:: profile.rs - JSONプロファイル管理
   :id: DES_BE_PROF
   :links: ARCH_BE_PROF

   - **関数**: ``load_profiles() -> Vec<Profile>``, ``save_profile(name: &str, config: OptimizeConfig)``
   - ``profiles/`` ディレクトリ構造のファイル読み書きおよび JSON schema バージョン（``version: 2``）チェック。
