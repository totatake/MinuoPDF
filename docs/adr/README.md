# Architecture Decision Records (ADR)

**MinuoPDF** における主要なアーキテクチャ選定および技術的トレードオフの判断記録です。

## 運用ルール

1. 新しい重要な設計判断を追加する際は、`docs/adr/NNNN-<short-kebab-title>.md`（4 桁連番）を作成します。
2. 過去の ADR は削除せず、置き換えた場合はステータスを `Superseded by ADR-XXXX` に更新します。
3. 追加・更新時は下の一覧テーブルを同期してください。

## ADR 一覧

| ID | タイトル | ステータス | 日付 | 概要 |
| :--- | :--- | :--- | :--- | :--- |
| [ADR-0001](0001-rust-tauri-non-gpl-rewrite.md) | Rust + Tauri v2 による完全非 GPL・ポータブル構成の採用 | Accepted | 2026-08-02 | Ghostscript / PyMuPDF (AGPL) から `lopdf` + `pdfium-render` + Svelte 5 へ刷新 |
| [ADR-0002](0002-ctm-dpi-smart-fallback-and-in-memory-audit.md) | CTM 実効 DPI・Smart Fallback・インメモリ PDF Audit パイプライン | Accepted | 2026-10-17 | CTM/Form/SMask 精度維持、逆圧縮防止、Rayon 並列化、重複 I/O ゼロの容量監査統合 |
