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

### 2.1 圧縮コアエンジン (Rust / Pdfium / lopdf)
- **カラーモードのサポート**:
  - `Color`: カラー画像を JPEG 再圧縮。
  - `Gray`: 画像をグレースケール変換し JPEG 再圧縮。
  - `Mono`: 画像を白黒二値（1-bit）に変換し、FAX等で用いられる高効率な `CCITT Group 4` 形式で保存・埋め込み。
- **Mono変換の手法選択**:
  - `二値化 (Otsu)`: 大津の二値化（Otsu's thresholding）を用いて、文字を鮮明に白黒化。
  - `ディザリング (Dither)`: Floyd-Steinberg 誤差拡散法を用いて、写真やグラデーションの階調をドットで表現。
- **解像度 (DPI) & JPEG品質調整**:
  - DPI指定（50, 72, 100, 150, 200, 300）およびリサイズ処理。
  - Color / Gray モード時の JPEG 品質（10〜100）指定。
- **2つの圧縮戦略**:
  - `テキスト保持 (Keep Text)`: PDF内の画像オブジェクト (`/XObject`) のみを差し替える。元のテキストデータやレイアウトはそのまま維持。
  - `完全ラスタライズ (Rasterize Page)`: ページ全体を画像としてレンダリングし、最適化された画像PDFとして再結合。スキャンPDF等で最大圧縮効果を発揮。
- **フォント解除 (Unembed Fonts)**:
  - PDF内に埋め込まれた不要なフォントを取り外し、閲覧側OSの標準代替フォントを利用させることで劇的な軽量化を実現。

### 2.2 実行形態とユーザーインターフェース
- **ハイブリッド起動**:
  - **GUIモード (引数なし起動)**:
    - Svelte + Tauri v2 によるスリムな UI（Slate/Zinc ミニマルテーマ、680x530固定サイズ）。
    - Win32 API を用いた起動時コンソール自動非表示（黒いプロンプト画面の消去）。
    - 多言語切り替え（日本語 / 英語）を動的に実行可能。
    - 各機能に詳細なバルーンヘルプ（ツールチップ）を搭載。
    - **複数ファイル一括処理 (Batch Mode)**: ドロップエリアまたはファイル選択から複数PDFを一括読み込み・一括圧縮。
    - **プロファイル管理**: 設定を名前付き JSON プロファイルとして `profiles/` ディレクトリに保存・復元。
  - **CUI対話モード (引数あり起動 / ドラッグ＆ドロップ)**:
    - 対象PDFファイルを引数にして実行すると、対話式コンソールメニューが起動。番号選択で手軽に圧縮を実行。
    - CLIヘルプ表示 (`--help` / `-h`) 対応。

### 2.3 ライセンスとセキュリティ
- **完全非GPL構成**:
  - バックエンド: Rust (MIT/Apache-2.0), `lopdf` (MIT), `pdfium` (BSD-3-Clause)
  - フロントエンド: Svelte / Vite (MIT), Tauri v2 (MIT/Apache-2.0)
- **クローズド・商用利用**: AGPL等の制約を受けず、自社内・商用製品への組み込みや単体配布が安全に行える。

---

## 3. ファイル構成（File Structure）

本プロジェクトのワークスペース構成は以下の通りです。

```
pdf_optimizer2/
├── CONTEXT.md               # 本ドキュメント（プロジェクトコンテキスト）
├── README.md                # ユーザーガイドおよび開発・ビルド手順
├── LICENSE.md               # ライセンス定義ファイル
├── package.json             # フロントエンド依存関係定義 (Svelte, Vite, Tauri CLI)
├── svelte.config.js         # Svelte 構成設定
├── vite.config.js           # Vite 構成設定
├── build_exe.bat            # リリースビルド用スクリプト
├── clean.bat                # ビルドキャッシュ・成果物クリーンアップ用スクリプト
│
├── src/                     # Tauri フロントエンド (Svelte + TypeScript)
│   ├── App.svelte           # メイン画面コンポーネント (GUI Layout, i18n, Profiles)
│   ├── main.ts              # エントリーポイント
│   └── app.css              # Catppuccin Mocha テーマスタイル定義
│
├── src-tauri/               # Tauri バックエンド (Rust)
│   ├── Cargo.toml           # Rust 依存関係・パッケージ定義
│   ├── tauri.conf.json      # Tauri アプリケーション設定
│   ├── build.rs             # Rust ビルドスクリプト
│   ├── src/                 # Rust ソースコード
│   │   ├── main.rs          # CLI/GUI分岐エントリーポイント
│   │   ├── lib.rs           # Tauri コマンドハンドラ・ロジック結合部
│   │   ├── compress.rs      # PDF圧縮・画像変換コアロジック (lopdf/pdfium)
│   │   ├── cli.rs           # CUI対話モード・引数解析ロジック
│   │   └── profiles.rs     # プロファイル管理 (JSON I/O)
│   └── tests/               # 結合テストスイート
│       └── integration_tests.rs
│
├── aspice/                  # ASPICE 開発プロセス準拠ドキュメント群
│   ├── SWE1_Software_Requirements_Specification.md   # ソフトウェア要件仕様書
│   ├── SWE2_Software_Architectural_Design.md         # ソフトウェアアーキテクチャ設計書
│   ├── SWE3_Software_Detailed_Design.md             # ソフトウェア詳細設計書
│   ├── SWE4_Software_Unit_Verification.md            # ユニット検証仕様書
│   ├── SWE5_Software_Integration_Verification.md     # 結合検証仕様書
│   ├── SWE6_Software_Qualification_Testing.md        # 適格性確認テスト仕様書
│   └── Traceability_Matrix.md                         # 追跡可能性マトリクス
│
└── profiles/                # 保存されたユーザープロファイル (自動生成 / JSON)
```

---

## 4. テスト計画と品質保証（Testing & Quality Assurance）

ASPICE プロセスモデル（SWE.4 〜 SWE.6）に準拠した多層的なテストを実施し、機能の正確性と安全性を保証します。

### 4.1 自動テスト体系
- **ユニットテスト (SWE.4 / `Cargo.toml`)**:
  - `cargo test --manifest-path src-tauri/Cargo.toml` を用いて、画質変換計算、プロファイル I/O、CLI 引数処理等の最小ユニットを検証。
- **結合テスト (SWE.5 / `tests/integration_tests.rs`)**:
  - テスト用 PDF ファイルに対し、以下の各パラメータ組み合わせで実際に圧縮を実行し、生成された PDF の構造健全性（lopdf で破綻なくロードできるか）およびファイルサイズ削減効果を検証。
    1. Color + Keep Text
    2. Color + Rasterize Page
    3. Gray + Keep Text
    4. Gray + Rasterize Page
    5. Mono (Otsu) + Keep Text
    6. Mono (Otsu) + Rasterize Page
    7. Mono (Dither) + Keep Text
    8. Mono (Dither) + Rasterize Page
    9. フォント解除 (Unembed) の有効化テスト

### 4.2 手動テストおよび互換性確認 (SWE.6)
- **GUI アクションテスト**:
  - ドロップエリアへのドラッグ＆ドロップ動作、プロファイルの保存・読込、日本語/英語言語切り替え、コンソールウィンドウ非表示制御の動作確認。
- **PDF ビューア描画・互換性テスト**:
  - 圧縮出力された PDF を 主要ビューア（Adobe Acrobat Reader, Google Chrome, Microsoft Edge）で表示し、レイアウト崩れ・文字化けが生じないか確認。
  - `Keep Text` モード時において、テキスト選択・検索・コピペ機能が保持されていることを確認。

---

## 5. 開発・ビルド手順（Build Instructions）

### 動作前提要件
- Rust (Cargo) 工具鎖 (Windows MSVC ツールキット必須)
- Node.js (v18 以上推奨)

### ビルドコマンド
1. **フロントエンド依存パッケージインストール**:
   ```bash
   npm install
   ```
2. **スタンドアロン型リリースビルド**:
   ```bash
   npm run tauri build
   ```
   ビルド成果物は `src-tauri/target/release/pdf_optimizer.exe` に出力されます。
3. **テストの実行**:
   ```bash
   cargo test --manifest-path src-tauri/Cargo.toml
   ```
