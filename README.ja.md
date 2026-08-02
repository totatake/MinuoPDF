# MinuoPDF (PDF軽量化・画質最適化ツール) - 取扱説明書

> **完全ポータブルな超軽量PDF容量最適化アプリケーション**

[**English README**](README.md)

---

**MinuoPDF** は、PDFファイルのレイアウトや視認性を維持しながら、ファイルサイズを強力に軽量化・圧縮するオープンソースソフトウェア（MITライセンス）です。

---

## 1. 配布ファイル構成

本ソフトを使用・配布する際は、以下のファイル一式を同一フォルダに配置してください。

```text
[配布フォルダ]/
  ├── MinuoPDF.exe        (アプリ本体 / GUI & CLIハイブリッド実行ファイル)
  ├── pdfium.dll          (PDF描画エンジンDLL)
  ├── README.ja.md        (取扱説明書 - 日本語版 / 本文書)
  ├── README.md           (User Guide - 英語版)
  └── LICENSE.md          (ライセンス定義文書)
```

※ 本ソフトを実行すると、同階層にカスタムプロファイル保存用の `profiles/` フォルダが自動生成されます。

---

## 2. 動作環境

- **対応OS**: Windows 10 / Windows 11 (64-bit), Linux
- **依存関係**: 追加のランタイム（.NET等）やインストーラーは不要です。

---

## 3. 使用方法 (GUIモード)

### ① アプリの起動
`MinuoPDF.exe` をダブルクリックして起動します。コンソール非表示の統合ウィンドウ（680×530）が表示されます。

### ② PDFファイルの選択・追加
- **ドラッグ＆ドロップ**: アプリ画面上のドロップエリアにPDFファイルを直接ドロップします。
- **「PDF追加...」ボタン**: ファイル選択ダイアログから複数のPDFを一括選択します。

### ③ 圧縮モードの選択

#### 【簡単モード (Easy Mode)】
あらかじめ定義された5つのプリセットから選択するだけで、最適な圧縮を実行できます。
- **高画質**: 印刷下書き用（DPI: 200, 品質: 85%）
- **標準**: 電子書籍・画面表示用（DPI: 150, 品質: 75%）
- **軽量化**: Web表示・メール送信向け（DPI: 100, 品質: 50%）
- **白黒化**: スキャン文書・FAX用（大津の二値化 + CCITT Group 4 高率圧縮）
- **強制画像化**: ページ全体を画像にレンダリングして最適化

#### 【詳細モード (Detail Mode)】
カラーポリシー（Color/Gray/Mono）、リサイズDPI、JPEG品質、二値化方式（大津/ディザリング）、不要オブジェクト破棄（注釈、しおり、メタデータ、AcroForm等）、およびフォント（サブセット化/埋め込み解除）を自由に細かく設定できます。

---

## 4. ビルドおよび開発コマンド (Makefile / make.bat)

ローカル開発環境およびビルドには、以下のコマンドをご利用いただけます。

### Linux (Ubuntu 24.04 / WSL2) ビルド前提条件

Linux（WSL2上の Ubuntu 24.04 LTS 環境含む）でビルドする場合は、事前に以下のシステム依存パッケージ、Node.js (npx含む)、Rust ツールチェーン、および PDFium ライブラリをセットアップしてください：

```bash
# 1. システム依存ライブラリのインストール (Ubuntu 24.04 LTS)
sudo apt update
sudo apt install -y build-essential curl wget file libssl-dev libgtk-3-dev libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev pkg-config python3 python-is-python3 python3-venv python3-full

# 2. Node.js (v20 LTS) & npx のインストール
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt install -y nodejs

# 3. Rust ツールチェーンのインストール
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 4. Linux環境用 npm パッケージの再インストール (Tauri CLI Linuxネイティブバイナリの有効化)
# ※ Windows環境で生成された node_modules を共有している場合や新規クローン時に必要
npm install
# ※ PDFium ライブラリ (libpdfium.so / pdfium.dll) はビルド/開発コマンド実行時に自動取得されます。

# 5. Playwright E2E GUI テスト用ブラウザとシステム依存ライブラリのインストール
npx playwright install
sudo npx playwright install-deps
```

### 開発コマンド一覧

```bash
# Tauri リリースバイナリのビルド
make build

# 開発サーバー起動
make dev

# Rust ユニットテスト実行
make test

# Sphinx HTML ドキュメントビルド (初回のみ python3 -m venv .venv && source .venv/bin/activate && pip install -r requirements.txt が必要)
make docs

# ビルド成果物のクリーンアップ
make clean
```

---

## 5. ライセンス

MinuoPDF は **MIT License** のもとで公開されています。個人利用・商用利用を問わず自由にご利用・改変・再配布いただけます。
