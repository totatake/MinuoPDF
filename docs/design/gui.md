# フロントエンド詳細設計 (`MinuoPDF`)

フロントエンドは Svelte 5 + TypeScript + Vite で構築されており、コンポーネント指向のモジュール設計となっています。システム全体の構成は [../architecture.md](../architecture.md) を参照してください。

## 1. 設計モジュール一覧

| 設計 ID | 対象領域 / ファイル | 関連アーキテクチャ |
| :--- | :--- | :--- |
| `DES_GUI_001` | `src/routes/+page.svelte` & `src/lib/components/*` | `ARCH_FE_001` |
| `DES_GUI_002` | `src/lib/utils/i18n.ts` | `ARCH_FE_001` |
| `DES_GUI_003` | `src/lib/components/AuditModal.svelte` | `ARCH_FE_001`, `ARCH_BE_AUDIT` |

---

## 2. コンポーネント詳細設計

### `DES_GUI_001`: `+page.svelte` & Modular Components (`ARCH_FE_001`)

- **コンポーネント構成**:
  - `+page.svelte`: アプリケーション全体のコンテナ。グローバルステート（`files`, `config`, `logs`, `auditComparison`, `isProcessing`）および Tauri イベントリスナーを統括する。
  - `FileDropBar.svelte`: ドラッグ＆ドロップバー、ファイル追加ダイアログ、選択中ファイル件数バッジ、一覧アコーディオン展開、個別削除機能を提供する。
  - `EasyModeTab.svelte`: プリセット（高画質・標準・軽量化・白黒化・強制画像化）選択および保存済みプロファイルのワンクリック適用を行う。
  - `DetailModeTab.svelte`: 画像解像度/圧縮品質、カラーモード、破棄設定（メタデータ・コメント・注釈等）、埋め込みフォント解除/サブセット化の精密設定を行う。
  - `ProfileTab.svelte`: 現在の設定のプロファイル名指定保存・削除管理を行う。
  - `StatusFooter.svelte`: プログレスバー（0-100%）、最適化開始・キャンセルボタン、およびログコンソールを表示する。
  - `AuditModal.svelte`: PDF 容量内訳（PDF Audit）の棒グラフ・パーセンテージ・バイト数表示および最適化前後比較モーダルを提供する。
- **型定義とユーティリティ**:
  - `src/lib/types/index.ts`: `OptimizeConfig`, `PdfAuditComparison`, `LogItem`, `CATEGORY_COLORS` 等の厳格な TypeScript 型定義。
  - `src/lib/utils/format.ts`: `formatBytes` 共通フォーマッタ。

### `DES_GUI_002`: 国際化 (i18n) モジュール設計 (`ARCH_FE_001`)

- `src/lib/utils/i18n.ts` にて日本語 (`ja`) および英語 (`en`) の対訳キー辞書とプリセット説明文を保持する。
- Svelte 5 の Reactive State (`$state`) を用いて、言語ボタンクリック時に全 UI テキストおよびツールチップをシームレスに再描画する。

### `DES_GUI_003`: `AuditModal.svelte` — PDF 容量内訳ビジュアライザ (`ARCH_FE_001`, `ARCH_BE_AUDIT`)

- 最適化ログ行をクリック、またはファイル一覧から「Audit」をクリックすることで起動するモーダルダイアログ。
- `Images`（青）, `Fonts`（黄）, `Content Streams`（緑）, `Metadata/Catalog`（マゼンタ）, `Struct/Forms`（シアン）, `Other`（グレー）のカテゴリ比率を CSS Flexbox 複合プログレスバーおよび比較テーブルで視覚化する。
