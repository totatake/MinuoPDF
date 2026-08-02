========================================
フロントエンド詳細設計 (GUI Design)
========================================

概要
====

フロントエンドは Svelte 5 + TypeScript + Vite で構築されており、コンポーネント指向のモジュール設計となっています。

コンポーネント詳細設計
=======================

.. design:: App.svelte - メインビューコンポーネント
   :id: DES_GUI_001
   :links: ARCH_FE_001

   - **状態変数 (State)**:
     - ``files: FileItem[]``: 追加されたPDFファイルのリスト
     - ``activeTab: 'easy' | 'detail'``: モード選択状態
     - ``config: OptimizeConfig``: 圧縮設定オブジェクト
     - ``isProcessing: boolean``: 処理実行フラグ
     - ``progress: { percent: number, currentFile: string }``: 進捗状態
   - **イベントハンドラ**:
     - ``handleDrop(e: DragEvent)``: ファイルドロップ受領・PDFフィルタリング
     - ``startOptimization()``: Tauri ``invoke('optimize_files', ...)`` の呼び出し

.. design:: 国際化 (i18n) モジュール設計
   :id: DES_GUI_002
   :links: ARCH_FE_001

   - `i18n.ts` にて日本語 (ja) および英語 (en) の対訳キー辞書を保持。
   - Svelte 5 の Reactive State (``$state``) を用いて、言語ボタンクリック時に全UIテキストをシームレスに再描画する。
