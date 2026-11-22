# ロードマップ (`MinuoPDF`)

本ドキュメントでは **MinuoPDF** の達成済みマイルストーンと今後の機能拡張・改善計画を管理します。詳細な変更履歴は [../CHANGELOG.md](../CHANGELOG.md) を参照してください。

## 1. 完了済みマイルストーン

| バージョン | 完了日 | 主な達成事項 |
| :--- | :--- | :--- |
| `v0.1.0` | 2026-08-02 | Rust + Tauri v2 + Svelte 5 による非 GPL 再構築、Color/Gray/Mono (Otsu/Dither + G4)、Keep Text/Rasterize、対話/バッチ CLI、Playwright E2E・自動テスト基盤 |
| `v0.1.1` | 2026-10-11 | Linux リリースバイナリの実行権限付与修正とパッケージ改善 |
| `v0.2.0` | 2026-10-17 | アーキテクチャ・性能・機能拡充（CTM DPI・Form XObject・SMask・Smart Fallback・Rayon 並列・PDF Audit）、450 DPI オプション追加 |
| `v0.2.1` | 2026-11-03 | CTM オペレータパース堅牢化、CIDFont マッピング安全性向上、Audit 正規化、CLI プリセットエイリアス・stdin EOF ガード |
| `v0.3.0` | 2026-11-22 | OSS リポジトリ構造標準化（`LICENSE` / `THIRD_PARTY_NOTICES.md` / `AGENTS.md` / `.github/`）、`docs/` の GitHub Markdown 移行・ADR 整備、GUI 言語自動判定（`auto` デフォルト） |

## 2. 今後の展望 (Future Horizons)

- **ベンチマーク・圧縮率レポート拡充**: 多様な PDF コーパス（スキャン文書・CAD 図面・DTP 資料）に対する定量削減率と処理速度レポートを `docs/reports/` へ追加。
- **GUI ビジュアル素材整備**: 操作画面スクリーンショット・デモ GIF を `docs/assets/` へ追加。
- **CI における E2E / 静的検査強化**: `cargo clippy` / `cargo fmt --check` および Playwright E2E テスト結果の本番ゲート化。
- **圧縮アルゴリズムの高度化**: JPEG2000 / JBIG2（非 GPL 実装の範囲内）や重複 XObject ストリームの重複排除検討（導入時は `docs/adr/` にて判断記録）。
