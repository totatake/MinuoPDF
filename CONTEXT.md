# MinuoPDF - CONTEXT

本ドキュメントは、PDF最適化ツール（PDF Optimizer）の背景・技術変遷、要件、アーキテクチャ、ファイル構成、および品質保証・テスト計画を定義するプロジェクトコンテキスト文書です。

---

## 1. プロジェクト概要

### 1.1 背景と技術スタックの変遷
本プロジェクトは、PDFの閲覧品質やレイアウトを維持しながらファイルサイズを極限まで削減することを目的として開発されてきました。ニーズや技術的制約の克服に伴い、以下の3つの世代を経て進化しています。

```
[第1世代: Ghostscript] ──(機能限界)──> [第2世代: Python (PyMuPDF)] ──(ライセンス緩和)──> [第3世代: Rust + Tauri v2 (現行)]
```

#### 第1世代: Ghostscript バッチ時代 (`pdf_compress.bat`)
* **構成**: Ghostscript CLI を呼び出す Windows バッチスクリプト。
* **課題**:
  1. **モノクロ（Mono）化への未対応**: Ghostscript の制約上、白黒二値（1-bit）への高品質な変換と CCITT Group 4 圧縮の適用が困難であった。
  2. **圧縮率の限界**: DPI や画質の変更だけでは、スキャン文書等のサイズ削減に限界があった。
  3. **使い勝手**: ドラッグ＆ドロップのみの CUI ベースであり、詳細な設定を画面上で確認・調整できる GUI が存在しなかった。

#### 第2世代: Python 時代 (`PyMuPDF` + `Pillow` + `Tkinter`)
* **構成**: PyMuPDF (fitz) と Pillow による画像処理・PDF再構成エンジン + Tkinter GUI。
* **成果**: Color / Gray / Mono（大津の二値化 / Floyd-Steinberg ディザリング）変換、テキスト保持（Keep Text）/ 完全ラスタライズ（Rasterize）戦略を確立。
* **課題 (ライセンス問題)**:
  - PyMuPDF は **AGPL (GNU Affero General Public License)** であり、クローズド環境や商用利用・再配布において非常に厳しいライセンス制約（ソースコード開示義務等）が存在した。ライセンスの懸念を払拭し、オープンかつ商用・閉域環境でも自由に利用できる代替構成が急務となった。

#### 第3世代（現行）: Rust + Tauri v2 時代
* **構成**: Tauri v2 + Svelte (Vite) フロントエンド + Rust (lopdf / pdfium-render / image) バックエンド。
* **目的と達成事項**:
  1. **厳格なライセンスの完全排除**: PyMuPDF/AGPL を完全に排除し、**MIT / BSD / Apache 2.0 系の非GPLライセンス** のみで全機能をリライト。商用利用・クローズド環境での安全性を確保。
  2. **超軽量・ポータブル性**: UPX等に頼らず、実行ファイル単体で約 **4.3 MB** の超軽量バイナリを実現。同一ディレクトリの `pdfium.dll` を優先自動ロードするスタンドアロン動作。
  3. **モダンな UX/UI**: Catppuccin Mocha風ダークテーマ、多言語対応（日本語 / 英語）、レイアウト固定によるブレ防止、詳細なバルーンヘルプ（ツールチップ）。
  4. **機能拡張**: 画像圧縮に加え、**フォント解除 (Unembed Fonts)** 機能を新たに搭載。
  5. **品質保証体系**: ASPICE (Automotive SPICE) プロセスに準拠したドキュメント体系（SWE.1〜SWE.6）の整備。

---

## 2. 要件定義（Requirements）

#### 2.1 圧縮・解析コアエンジン (Rust / Pdfium / lopdf / Rayon)
- **カラーモードのサポート**:
  - `Color`: カラー画像を JPEG 再圧縮。
  - `Gray`: 画像をグレースケール変換し JPEG 再圧縮。
  - `Mono`: 画像を白黒二値（1-bit）に変換し、高効率な `CCITT Group 4` 形式で埋め込み。
- **Mono変換の手法選択**:
  - `二値化 (Otsu)`: 大津の二値化（Otsu's thresholding）を用いて文字を鮮明に白黒化。
  - `ディザリング (Dither)`: Floyd-Steinberg 誤差拡散法を用いて階調をドット表現。
- **解像度 (DPI) & JPEG品質調整**:
  - CTM（現在変換行列）解析によるページ上での実効表示サイズ（pt）に基づく正確なDPI推定とリサイズ。
  - Color / Gray モード時の JPEG 品質（10〜100）指定。
- **Smart Fallback（逆圧縮防止ガード）**:
  - リサイズやカラー変換のない画像において、再エンコードで元ストリームより肥大化する場合は元ストリームを自動維持。
- **Form XObject再帰走査 & 透過SMask連動**:
  - `/Subtype /Form` 内の埋め込み画像を漏れなく検出し、親画像のリサイズ時は透過マスク（SMask）も同一比率に追従リサイズ。
- **マルチコア並列処理 (Rayon) & インメモリ最適化**:
  - Rayon を用いて画像デコード・リサイズ・エンコードを並列実行。
  - 最適化処理と容量監査処理で同一の Document オブジェクトを再利用し、重複ディスクI/Oを排除。
- **PDF Audit（容量内訳解析）**:
  - Images, Fonts, Content Streams, Metadata/Catalog, Struct/Forms, Other の6分類による容量割合と前後比較。
- **2つの圧縮戦略**:
  - `テキスト保持 (Keep Text)`: PDF内の画像オブジェクト (`/XObject`) のみを差し替え。テキストデータ・レイアウトを維持。
  - `完全ラスタライズ (Rasterize Page)`: ページ全体を画像レンダリングし、最適化された画像PDFとして再結合。
- **フォント処理**:
  - `フォント解除 (Unembed Fonts)`: 不要なフォントストリームを安全に削除。
  - `フォントサブセット化 (Font Subsetting)`: Content Stream および W 配列から使用GIDを抽出し、TrueType/OpenType フォントを使用グリフのみに軽量化。

### 2.2 実行形態とユーザーインターフェース
- **ハイブリッド起動**:
  - **GUIモード (引数なし起動)**:
    - Svelte 5 + Tauri v2 によるモジュラー UI（初期 740×560px、最小 680×520px の制約付きリサイズ対応）。
    - 多言語切り替え（日本語 / 英語）および詳細なバルーンヘルプ（ツールチップ）。
    - 複数ファイル一括処理（ドラッグ＆ドロップ、アコーディオン個別削除、すべてクリア）。
    - プリセット選択（高画質・標準・軽量化・白黒化・強制画像化）およびプロファイル管理（`profiles/` ディレクトリ保存・読込・削除）。
    - 中断（キャンセル）ボタン付きリアルタイム進捗プログレスバーおよびログコンソール。
    - PDF Audit モーダルダイアログ（容量内訳の複合バーグラフ＆前後比較テーブル）。
  - **CUIモード (引数あり起動)**:
    - **非対話バッチCLI**: `-o/--output`, `-p/--preset`, `--profile`, `-s/--strategy`, `-b/--batch` オプションによる完全ヘッドレス自動化。
    - **対話CLI**: 単体PDF引数指定時は矢印キー対話式コンソールメニューを起動。

### 2.3 ライセンスとセキュリティ
- **完全非GPL構成**:
  - バックエンド: Rust (MIT/Apache-2.0), `lopdf` (MIT), `pdfium` (BSD-3-Clause), `rayon` (MIT/Apache-2.0)
  - フロントエンド: Svelte 5 / Vite (MIT), Tauri v2 (MIT/Apache-2.0)
- **クローズド・商用利用**: AGPL等の制約を受けず、自社内・商用製品への組み込みや単体配布が安全に行える。

---

## 3. ファイル構成（File Structure）

```
MinuoPDF/
├── CONTEXT.md               # プロジェクトコンテキスト文書
├── README.md                # ユーザーガイドおよび開発手順 (英語)
├── README.ja.md             # ユーザーガイドおよび開発手順 (日本語)
├── package.json             # フロントエンド依存関係 (Svelte 5, Vite, Tauri CLI, Playwright)
├── svelte.config.js         # Svelte 構成設定
├── vite.config.ts           # Vite 構成設定
│
├── src/                     # フロントエンド (Svelte 5 + TypeScript)
│   ├── routes/
│   │   └── +page.svelte     # メインコンテナ画面
│   ├── lib/
│   │   ├── components/      # モジュラーUIコンポーネント群
│   │   │   ├── FileDropBar.svelte    # ファイルドロップ＆一覧管理
│   │   │   ├── EasyModeTab.svelte    # プリセット＆プロファイル選択
│   │   │   ├── DetailModeTab.svelte  # 画像・破棄・フォント詳細設定
│   │   │   ├── ProfileTab.svelte     # プロファイル保存・削除
│   │   │   ├── StatusFooter.svelte   # プログレスバー・実行制御・ログ
│   │   │   └── AuditModal.svelte     # PDF Audit 容量内訳グラフ＆テーブル
│   │   ├── types/
│   │   │   └── index.ts              # TypeScript 型定義
│   │   └── utils/
│   │       ├── format.ts             # バイト数等の表示フォーマッタ
│   │       └── i18n.ts               # 日英対訳辞書・説明テキスト
│   ├── app.css              # Catppuccin Mocha テーマスタイル
│   └── app.html             # HTML テンプレート
│
├── src-tauri/               # バックエンド (Rust / Tauri v2)
│   ├── Cargo.toml           # クレート依存定義 (lopdf, pdfium-render, image, fax, rayon, allsorts)
│   ├── tauri.conf.json      # Tauri アプリケーション設定 (740x560, min 680x520, resizable)
│   ├── src/
│   │   ├── main.rs          # CLI/GUI分岐エントリーポイント
│   │   ├── lib.rs           # Tauri コマンドハンドラ・非同期ディスパッチ
│   │   ├── optimizer.rs     # 最適化コア (Rayon並列、CTM/Form XObject/SMask/Smart Fallback)
│   │   ├── rasterizer.rs    # Pdfium ページ全体ラスタライズ
│   │   ├── image_util.rs    # 画像エンコード一元化 (JPEG/Flate/CCITT G4)
│   │   ├── audit.rs         # PDF容量内訳解析 (インメモリ/ディスク)
│   │   ├── font_subsetter.rs# フォントサブセット化 (GID抽出・allsorts)
│   │   ├── cli.rs           # 対話式および非対話バッチCLIコントローラ
│   │   ├── profile.rs       # プロファイル保存・読込・削除
│   │   └── pdfium_util.rs   # PDFium ライブラリ動的ロード
│   └── tests/
│       └── integration_tests.rs # 統合テストスイート
│
├── docs/                    # Sphinx ドキュメント体系 (Sphinx-needs + Mermaid)
│   ├── index.rst            # ドキュメント目次
│   ├── requirements.rst     # ソフトウェア要件仕様書
│   ├── architecture.rst     # アーキテクチャ設計書
│   ├── design/              # 詳細設計書 (backend.rst, gui.rst)
│   ├── testing.rst          # テスト仕様書
│   └── traceability.rst     # 要件トレーサビリティマトリクス
│
├── tests/                   # E2E & テストフィクスチャ
│   ├── e2e/
│   │   └── gui.spec.ts      # Playwright GUI 自動テスト
│   └── fixtures/            # テスト用PDFファイル群
└── profiles/                # ユーザープロファイル JSON 保存先
```

---

## 4. テスト計画と品質保証（Testing & Quality Assurance）

多層的なテストを実施し、機能の正確性と安全性を保証します。

### 4.1 自動テスト体系
- **ユニットテスト (`cargo test --lib`)**:
  - 二値化計算（Otsu）、CCITT Group 4 エンコード、フォントサブセットGID抽出、W配列パース、プロファイルI/O、容量内訳計算（Audit）等を検証。
- **統合テスト (`cargo test --test integration_tests`)**:
  - 実サンプルPDFに対する Color, Gray, Mono (Otsu/Dither), Keep Text, Rasterize, フォント解除, オブジェクト破棄, フォントサブセット化の組み合わせ結合を検証。
- **Playwright GUI E2E 自動テスト (`npm run test:gui` / `make test-gui`)**:
  - UIレンダリング、言語切り替え、タブナビゲーション、詳細設定コントロール、ファイルリスト操作、進捗バーとログ表示を自動検証。

---

## 5. 開発・ビルド手順（Build Instructions）

### 動作前提要件
- Rust 1.77+
- Node.js 18+
- PDFium ライブラリ (`libpdfium.so` または `pdfium.dll`)

### 主要コマンド
1. **依存パッケージインストール**:
   ```bash
   npm install
   ```
2. **テスト実行**:
   ```bash
   cargo test --manifest-path src-tauri/Cargo.toml   # Rust テスト (Unit + Integration)
   npm run check                                    # Svelte 型チェック (svelte-check)
   npm run test:gui                                 # Playwright GUI E2E テスト
   ```
3. **CLI 実行例 (非対話バッチ)**:
   ```bash
   cargo run --manifest-path src-tauri/Cargo.toml -- input.pdf -o output.pdf -p light
   ```
4. **GUI アプリケーション起動**:
   ```bash
   npm run tauri dev
   ```
5. **リリースビルド**:
   ```bash
   npm run tauri build
   ```
