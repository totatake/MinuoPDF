# 詳細設計一覧 (`MinuoPDF`)

バックエンド（Rust / Tauri Host）およびフロントエンド（Svelte 5 / TypeScript）のモジュール詳細設計です。

| ドキュメント | 対象領域 | 主な設計 ID | 関連アーキテクチャ |
| :--- | :--- | :--- | :--- |
| [backend.md](backend.md) | Rust バックエンド・CLI・画像/フォント/監査エンジン | `DES_BE_001`, `DES_BE_IMG`, `DES_BE_RAST`, `DES_BE_AUDIT`, `DES_BE_CLI`, `DES_BE_FONT`, `DES_BE_PROF` | `ARCH_BE_*` |
| [gui.md](gui.md) | Svelte 5 コンポーネント・i18n・PDF Audit モーダル | `DES_GUI_001`, `DES_GUI_002`, `DES_GUI_003` | `ARCH_FE_001`, `ARCH_IPC_001` |
