# MinuoPDF ドキュメント一覧

本ディレクトリは **MinuoPDF**（非 GPL・完全ポータブルな PDF 最適化ツール）の設計・要件・検証・利用ガイドを集約する技術ドキュメントハブです。

## 基本ドキュメント

| ドキュメント | 内容 |
| :--- | :--- |
| [overview.md](overview.md) | 開発背景・技術スタック変遷（Ghostscript → Python → Rust/Tauri v2）・技術的特徴 |
| [requirements.md](requirements.md) | ソフトウェア要件仕様書（機能要件 `REQ_*` / 非機能要件） |
| [architecture.md](architecture.md) | システムアーキテクチャ設計書（構成図・`ARCH_*` コンポーネント・ライセンス/信頼境界） |
| [testing.md](testing.md) | テスト仕様および要件トレーサビリティマトリクス（`TEST_*`・`REQ → ARCH → DES → TEST`） |
| [roadmap.md](roadmap.md) | 達成済みマイルストーン（`v0.1.0`〜`v0.3.1`）と今後の展望 |

## サブディレクトリ

| ディレクトリ | 用途 |
| :--- | :--- |
| [guides/usage.md](guides/usage.md) | GUI / バッチ CLI / 対話 CLI・プロファイル v2 仕様・トラブルシューティング |
| [design/README.md](design/README.md) | 詳細設計（[backend.md](design/backend.md) / [gui.md](design/gui.md)・`DES_*`） |
| [adr/README.md](adr/README.md) | Architecture Decision Records（非 GPL リライト、CTM DPI / Smart Fallback / インメモリ Audit 等の設計判断） |
| [reports/test-results.md](reports/test-results.md) | `scripts/sync_test_results.py` により自動同期されるテスト実行結果レポート（`TR_*`） |
| `assets/` | README や設計書で参照するスクリーンショット・図表配置用ディレクトリ |
