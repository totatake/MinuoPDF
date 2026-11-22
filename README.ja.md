# MinuoPDF

> **完全ポータブルな超軽量・高速 PDF 最適化ツール**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![CI](https://github.com/totatake/MinuoPDF/actions/workflows/ci.yml/badge.svg)](https://github.com/totatake/MinuoPDF/actions/workflows/ci.yml)

**Language:** [English](README.md) | 日本語

---

## 概要

**MinuoPDF**（ラテン語で「減少・縮小する」を意味する *minuo* に由来）は、PDF の閲覧品質や文字の鮮明さを維持しながらファイルサイズを大幅に削減する、オープンソースのデスクトップアプリ兼 CLI ツールです。完全非 GPL のパーミッシブなコンポーネント（`MIT`, `BSD-3-Clause`, `Apache-2.0`）のみで構成され、インストーラ不要のポータブルバイナリとして動作します。

---

## 主な特徴

- **超軽量・完全ポータブル**: インストーラや外部ランタイム不要。単体バイナリ（約 4.3 MB）が同階層の `pdfium.dll` / `libpdfium.so` を自動ロードして動作します。
- **高精度な最適化と逆圧縮防止**:
  - **CTM 実効 DPI 算出**: コンテンツストリームの変換行列（`cm`, `Do`）からページ上の実効表示 DPI（pt）を算出して最適リサイズ。
  - **Form XObject & SMask 連動**: `/Subtype /Form` 内のネスト画像を再帰走査し、親画像リサイズ時に透過マスク（`SMask`）も同一比率で同期リサイズ。
  - **Smart Fallback ガード**: 再エンコードにより元ストリームより肥大化する場合は、自動的に元データを維持。
  - **Rayon 並列処理**: 画像のデコード・リサイズ・再エンコードをマルチコア並列で高速実行。
- **PDF Audit（容量内訳解析）**: `Images`, `Fonts`, `Content Streams`, `Metadata`, `Struct/Forms`, `Other` の 6 カテゴリ内訳と最適化前後の比較を、重複ディスク I/O ゼロで棒グラフ表示します。
- **柔軟なカラー圧縮**:
  - **Color**: 高効率 JPEG 再圧縮。
  - **Grayscale**: グレースケール変換＋JPEG 品質調整。
  - **Monochrome**: **大津の二値化（Otsu）** または **Floyd-Steinberg ディザリング** による 1-bit 白黒変換＋高圧縮 **CCITT Group 4** 符号化。
- **2 つの圧縮戦略**:
  - **テキスト保持 (Keep Text)**: 元のテキストベクタとレイアウトを保持し、画像オブジェクトのみを最適化。
  - **完全ラスタライズ (Rasterize Page)**: PDFium でページ全体を画像レンダリングして再構成（スキャン文書に最適）。
- **フォント最適化**:
  - **サブセット化**: 文書内で実際に使用されているグリフ ID のみを保持。
  - **埋め込み解除 (Unembed Fonts)**: 埋め込みフォントストリーム（`FontFile` / `FontFile2` / `FontFile3`）を安全に削除し、システム標準フォントへフォールバック。
- **不要オブジェクトの徹底削除**: 注釈・コメント、アウトライン、リンク、AcroForm、XML メタデータ、JavaScript、サムネイルを一括除去。
- **ハイブリッド起動（GUI & CLI）**:
  - モダン GUI（Tauri v2 + Svelte 5、Catppuccin Mocha テーマ、初期 740×560px・最小 680×520px リサイズ対応、OS ロケール自動判定 `Auto`＋`EN` / `JA` 手動切替）。
  - ヘッドレスバッチ CLI（`-o`, `-p`, `--profile`, `-s`, `-b`）および対話式コンソールメニュー。

---

## クイックスタート

### 動作環境

- **対応 OS**: Windows 10 / 11 (64-bit), Linux (x86_64)
- **依存ランタイム**: 不要（エンドユーザー環境にインストーラや `.NET` / Python 等の導入は不要）

### 配布ファイル構成

配布フォルダ内のファイルは同一階層に配置してください：

```text
MinuoPDF/
├── minuopdf / minuopdf.exe     # GUI & CLI ハイブリッド実行ファイル
├── pdfium.dll / libpdfium.so   # PDFium 共有ライブラリ
├── README.md                   # User Guide (英語版)
├── README.ja.md                # 取扱説明書 (日本語版)
├── LICENSE                     # MIT ライセンス文書
└── THIRD_PARTY_NOTICES.md      # サードパーティコンポーネント帰属表示
```

カスタムプロファイルを保存すると、実行ファイルと同階層に `profiles/` フォルダが自動作成されます。

### GUI モード

1. 引数なしで `minuopdf` を起動すると GUI が開きます（表示言語は OS ロケールに基づく `Auto` がデフォルトで、右上の選択欄から `English (EN)` / `日本語 (JA)` へ明示的に切り替えられます）。
2. ドロップエリアへのドラッグ＆ドロップ、またはファイル選択ボタンから PDF を追加します（PDF 以外は自動除外）。
3. **簡単モード**（5 プリセットまたは保存済みプロファイル）、**詳細モード**（DPI 50〜450、JPEG 品質、カラー／二値化方式、不要オブジェクト破棄、フォント処理）、**プロファイル管理** から設定を調整します。
4. 最適化を実行し、結果行または **容量調査 (Audit)** ボタンから 6 カテゴリの前後比較を確認します。

### CLI モード

```bash
# 組み込みプリセットで最適化
minuopdf document.pdf -o optimized.pdf -p light

# 保存済みプロファイルと完全ラスタライズ戦略を指定
minuopdf scan.pdf -o out.pdf --profile my_rule -s rasterize

# 複数ファイルを指定ディレクトリへ一括バッチ最適化
minuopdf a.pdf b.pdf c.pdf -b dist/ -p standard

# 対話式コンソールメニューを起動（オプションなしで PDF を 1 つ指定）
minuopdf document.pdf
```

**主なオプション**: `-o, --output <PATH>`, `-p, --preset <NAME>`, `--profile <NAME>`, `-s, --strategy <keep-text|rasterize>`, `-b, --batch <DIR>`

詳細な操作方法、`profiles/` の JSON スキーマ v2、トラブルシューティングは [利用ガイド (`docs/guides/usage.md`)](docs/guides/usage.md) を参照してください。

---

## ドキュメント

設計・要件・検証の詳細は [`docs/`](docs/README.md) で管理しています：

- [概要・開発背景（第1〜3世代の変遷） (`docs/overview.md`)](docs/overview.md)
- [ソフトウェア要件仕様書 (`docs/requirements.md`)](docs/requirements.md)
- [システムアーキテクチャ設計書 (`docs/architecture.md`)](docs/architecture.md)
- [詳細設計（バックエンド / GUI） (`docs/design/`)](docs/design/README.md)
- [設計判断記録 ADR (`docs/adr/`)](docs/adr/README.md)
- [利用ガイド (`docs/guides/usage.md`)](docs/guides/usage.md)
- [テスト仕様・トレーサビリティマトリクス (`docs/testing.md`)](docs/testing.md)
- [テスト実行結果レポート (`docs/reports/test-results.md`)](docs/reports/test-results.md)
- [ロードマップ (`docs/roadmap.md`)](docs/roadmap.md)

---

## ビルドと開発

### 前提条件

- [Rust](https://www.rust-lang.org/) (`stable`, 1.77+)
- [Node.js](https://nodejs.org/) (`v18+` / `v22 LTS`) および `npm`
- [Python 3](https://www.python.org/)（`scripts/` 内の補助スクリプト用・標準ライブラリのみ）

Linux / WSL2 環境向けのシステム依存パッケージ（`WebKit2GTK`）や Playwright の導入手順は [CONTRIBUTING.md](CONTRIBUTING.md) を参照してください。

### 開発コマンド (`Makefile` / `make.bat`)

```bash
npm install     # フロントエンド依存の導入（PDFium は build / test 時に自動取得）
make build      # Tauri v2 リリースバイナリをビルドし dist/MinuoPDF/ へパッケージング
make dev        # Tauri GUI 開発サーバーを起動
make test       # Rust 単体・結合テストを実行し docs/reports/test-results.md を同期
make test-gui   # Playwright GUI E2E テストを実行し docs/reports/test-results.md を同期
make test-all   # Rust テストおよび Playwright E2E テストをすべて実行
make clean      # ビルドキャッシュとテスト出力を削除
```

---

## コントリビュート・セキュリティ

コントリビューションを歓迎します。開発環境のセットアップ手順・テスト方針・厳格な非 GPL 依存ポリシーは [CONTRIBUTING.md](CONTRIBUTING.md)、更新履歴は [CHANGELOG.md](CHANGELOG.md)、脆弱性の非公開報告手順は [SECURITY.md](SECURITY.md) をご覧ください。

---

## ライセンス

MinuoPDF は **MIT License** のもとで公開されています。ライセンス全文およびサードパーティコンポーネントの帰属表示は [LICENSE](LICENSE) と [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) を参照してください。
