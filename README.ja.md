# MinuoPDF (PDF軽量化・画質最適化ツール) - 取扱説明書

> **完全ポータブルな超軽量 PDF 容量最適化アプリケーション**

**Language:** [English](README.md) | 日本語

---

**MinuoPDF** は、PDF ファイルのレイアウトや視認性を維持しながら、ファイルサイズを強力に軽量化・圧縮するオープンソースソフトウェア（MIT ライセンス・完全非 GPL 構成）です。

---

## 1. 配布ファイル構成

本ソフトを使用・配布する際は、以下のファイル一式を同一フォルダに配置してください。

```text
[配布フォルダ]/
  ├── minuopdf / minuopdf.exe  (アプリ本体 / GUI & CLI ハイブリッド実行ファイル)
  ├── pdfium.dll / libpdfium.so (PDF 描画エンジンライブラリ)
  ├── README.ja.md             (取扱説明書 - 日本語版 / 本文書)
  ├── README.md                (User Guide - 英語版)
  ├── LICENSE                  (MIT ライセンス文書)
  └── THIRD_PARTY_NOTICES.md   (サードパーティコンポーネント帰属表示)
```

※ 本ソフトを実行すると、同階層にカスタムプロファイル保存用の `profiles/` フォルダが自動生成されます。

---

## 2. 動作環境

- **対応 OS**: Windows 10 / Windows 11 (64-bit), Linux
- **依存関係**: 追加のランタイム（.NET 等）やインストーラーは不要です。

---

## 3. 使用方法

### GUI モード (デスクトップ操作)

1. **アプリの起動**: `minuopdf` をダブルクリック（または引数なしで実行）します。初期 740×560px（最小 680×520px 制約付きでリサイズ可能）の Catppuccin Mocha ダークテーマウィンドウが開きます。
2. **PDF ファイルの選択・追加**: ドロップエリアへのドラッグ＆ドロップ、または「PDF追加...」ボタンから複数ファイルを追加します（アコーディオン展開で個別削除・一括クリア可能）。
3. **圧縮モードの選択**:
   - **簡単モード (Easy Mode)**: 高画質・標準・軽量化・白黒化・強制画像化の5プリセット、または保存済みプロファイルをワンクリック適用。
   - **詳細モード (Detail Mode)**: カラーモード（Color / Gray / Mono）、目標 DPI（50〜450）、JPEG 品質、二値化方式（大津 / Floyd-Steinberg）、不要オブジェクト破棄、フォント処理（サブセット化 / 埋め込み解除）を調整。
   - **プロファイル (Profile)**: 詳細設定を名前付き JSON として `profiles/` に保存・読込・削除。
4. **PDF Audit (容量内訳ビジュアライザ)**: 最適化後のログ行または「Audit」ボタンから、6カテゴリ（Images, Fonts, Content Streams, Metadata, Struct/Forms, Other）の内訳と変換前後比較を棒グラフで確認できます。

### CLI モード (コマンドライン・バッチ処理)

```bash
# プリセット指定で最適化
minuopdf document.pdf -o optimized.pdf -p light

# プロファイルおよびラスタライズ戦略を指定
minuopdf scan.pdf -o out.pdf --profile my_rule -s rasterize

# 複数ファイルを指定フォルダへ一括バッチ処理
minuopdf a.pdf b.pdf c.pdf -b dist/ -p standard

# 対話式コンソール起動 (引数1つのみ指定)
minuopdf document.pdf
```

**主なオプション**: `-o, --output <PATH>`, `-p, --preset <NAME>`, `--profile <NAME>`, `-s, --strategy <keep-text|rasterize>`, `-b, --batch <DIR>`

詳細な使い方・プロファイル仕様・トラブルシューティングは [利用ガイド (`docs/guides/usage.md`)](docs/guides/usage.md) を参照してください。

---

## 4. ビルドおよび開発コマンド (`Makefile` / `make.bat`)

### Linux (Ubuntu 24.04 / WSL2) ビルド前提条件

```bash
# 1. システム依存ライブラリのインストール
sudo apt update
sudo apt install -y build-essential curl wget file libssl-dev libgtk-3-dev \
  libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev pkg-config \
  python3 python-is-python3

# 2. npm パッケージのインストール (PDFium ライブラリはビルド/テスト時に自動取得)
npm install

# 3. Playwright E2E GUI テスト用ブラウザと依存ライブラリのインストール
npx playwright install
sudo npx playwright install-deps
```

### 開発コマンド一覧

```bash
make build      # Tauri リリースバイナリのビルド & dist/MinuoPDF/ へのパッケージング
make dev        # 開発サーバー起動
make test       # Rust 単体・結合テスト実行 & docs/reports/test-results.md 更新
make test-gui   # Playwright GUI E2E テスト実行 & docs/reports/test-results.md 更新
make test-all   # 全テスト実行
make clean      # ビルド成果物のクリーンアップ
```

---

## 5. ドキュメント

設計・要件・検証の詳細は [`docs/`](docs/README.md) を参照してください：

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

## 6. コントリビュート・ライセンス

コントリビュート手順は [CONTRIBUTING.md](CONTRIBUTING.md)、更新履歴は [CHANGELOG.md](CHANGELOG.md)、脆弱性報告方針は [SECURITY.md](SECURITY.md) をご覧ください。

MinuoPDF は **MIT License** のもとで公開されています。詳細は [LICENSE](LICENSE) および [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) を参照してください。
