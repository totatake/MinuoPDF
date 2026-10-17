========================================
フロントエンド詳細設計 (GUI Design)
========================================

概要
====

フロントエンドは Svelte 5 + TypeScript + Vite で構築されており、コンポーネント指向のモジュール設計となっています。

コンポーネント詳細設計
=======================

.. design:: +page.svelte & Modular Components - UIコンポーネント設計
   :id: DES_GUI_001
   :links: ARCH_FE_001

   - **コンポーネント構成**:
     - ``+page.svelte``: アプリケーション全体のコンテナ。グローバルステート（files, config, logs, auditComparison, isProcessing）および Tauri イベントリスナーを統括。
     - ``FileDropBar.svelte``: ドラッグ＆ドロップバー、ファイル追加ダイアログ、選択中ファイル件数バッジ、一覧アコーディオン展開、個別削除機能。
     - ``EasyModeTab.svelte``: プリセット（高画質・標準・軽量化・白黒化・強制画像化）選択および保存済みプロファイルのワンクリック適用。
     - ``DetailModeTab.svelte``: 画像解像度/圧縮品質、カラーモード、破棄設定（メタデータ・コメント・注釈等）、埋め込みフォント解除/サブセット化の精密設定。
     - ``ProfileTab.svelte``: 現在の設定のプロファイル名指定保存・削除管理。
     - ``StatusFooter.svelte``: プログレスバー（0-100%）、最適化開始・キャンセルボタン、およびログコンソール。
     - ``AuditModal.svelte``: PDF容量内訳（PDF Audit）の棒グラフ・パーセンテージ・バイト数表示および最適化前後比較モーダル。
   - **型定義とユーティリティ**:
     - ``src/lib/types/index.ts``: ``OptimizeConfig``, ``PdfAuditComparison``, ``LogItem``, ``CATEGORY_COLORS`` 等の厳格な TypeScript 型定義。
     - ``src/lib/utils/format.ts``: ``formatBytes`` 共通フォーマッタ。

.. design:: 国際化 (i18n) モジュール設計
   :id: DES_GUI_002
   :links: ARCH_FE_001

   - ``src/lib/utils/i18n.ts`` にて日本語 (ja) および英語 (en) の対訳キー辞書とプリセット説明文を保持。
   - Svelte 5 の Reactive State (``$state``) を用いて、言語ボタンクリック時に全UIテキストおよびツールチップをシームレスに再描画する。

.. design:: AuditModal.svelte - PDF容量内訳ビジュアライザ
   :id: DES_GUI_003
   :links: ARCH_FE_001, ARCH_BE_AUDIT

   - 最適化ログ行をクリック、またはファイル一覧から「Audit」をクリックすることで起動するモーダルダイアログ。
   - Images (青), Fonts (黄), Content Streams (緑), Metadata/Catalog (マゼンタ), Struct/Forms (シアン), Other (グレー) のカテゴリ比率を CSS Flexbox 複合プログレスバーおよび比較テーブルで視覚化。
