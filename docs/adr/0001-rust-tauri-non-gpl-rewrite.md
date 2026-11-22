# ADR-0001: Rust + Tauri v2 による完全非 GPL・ポータブル構成の採用

- **ステータス**: Accepted
- **日付**: 2026-08-02
- **関連文書**: [システムアーキテクチャ](../architecture.md), [概要・技術変遷](../overview.md), [サードパーティライセンス](../../THIRD_PARTY_NOTICES.md)

## 1. 背景と課題 (Context)

MinuoPDF は第 1 世代（Ghostscript バッチ）から第 2 世代（Python + PyMuPDF + Pillow + Tkinter）へ発展したが、以下の課題が顕在化していた。

1. **AGPL ライセンス制約**: PyMuPDF (`fitz`) および Ghostscript は AGPL であり、社内閉域環境や商用製品への組み込み・再配布に強い制約（ソース開示義務等）が生じた。
2. **配布サイズとランタイム依存**: Python ランタイムや依存パッケージのバンドルは配布サイズ肥大化を招く。
3. **二重起動モードの必要性**: 同一バイナリでデスクトップ GUI と CI/スクリプト向けヘッドレス CLI を提供したかった。

## 2. 検討した選択肢 (Alternatives Considered)

### 案 A: Python (PyMuPDF) の継続＋PyInstaller 配布
- **メリット**: 既存実装資産を流用できる。
- **デメリット**: AGPL 制約が解消しない。配布バイナリが数十〜百 MB 規模になりポータビリティに劣る。

### 案 B: Rust (`lopdf` + `pdfium-render`) + Tauri v2 + Svelte 5（採用）
- **メリット**:
  - 全依存コンポーネントを `MIT` / `BSD-3-Clause` / `Apache-2.0` の非 GPL に統一可能。
  - 単体実行バイナリ（約 4.3 MB）＋同階層 `pdfium.dll` / `libpdfium.so` の動的ロードによりインストーラ不要のポータブル配布が可能。
  - `src-tauri/src/main.rs` での引数判定により GUI と CLI を単一実行ファイルへ統合できる。
- **デメリット**: Rust 側での PDF オブジェクト操作・画像エンコード（CTM パース、G4 エンコード、フォントサブセット化等）の自前実装コストが発生する。

## 3. 決定 (Decision)

**案 B（Rust + Tauri v2 + Svelte 5 + `lopdf` / `pdfium-render`）** を採用する。また、PDFium バイナリはビルド/テスト時に自動取得しつつ実行時は実行ファイル同階層を優先探索して動的ロードする設計とする。

## 4. 結果と影響 (Consequences)

- **良い影響**: `THIRD_PARTY_NOTICES.md` に記録する全依存関係で非 GPL 適合を達成し、Windows/Linux 双方でスタンドアロン動作する軽量バイナリを実現した。
- **受け入れるトレードオフ**: ビルドに Rust + Node.js + システム WebView（Linux では WebKit2GTK）が必要となり、PDF 構造処理の保守を `src-tauri/src/` で担う必要がある。
