# 利用ガイド (`MinuoPDF`)

ユーザー向けの導入・操作手順、CLI オプション、カスタムプロファイル（v2）仕様、およびトラブルシューティングをまとめます。背景や設計詳細は [overview.md](../overview.md) / [architecture.md](../architecture.md) を参照してください。

## 1. 配布物構成と動作要件

- **対応 OS**: Windows 10 / 11 (x64), Linux (x86_64)
- **必要な配置構成**（インストーラ不要・ポータブル動作）:
  ```text
  MinuoPDF/
  ├── minuopdf (.exe)           # GUI & CLI ハイブリッド実行ファイル
  ├── pdfium.dll / libpdfium.so # PDFium 共有ライブラリ
  ├── profiles/                 # 自動作成・参照される設定プロファイル保存先
  ├── README.md / README.ja.md
  ├── LICENSE
  └── THIRD_PARTY_NOTICES.md
  ```

---

## 2. GUI モードの使い方

引数なしで `minuopdf` を起動すると GUI（初期 740×560px・最小 680×520px）が表示されます。

1. **対象ファイルの追加**: ドロップエリアへのドラッグ＆ドロップ、または「PDF追加...」から複数 PDF を選択します（PDF 以外は自動除外）。
2. **タブ選択と設定**:
   - **簡単モード (Easy Mode)**: 5 プリセット（高画質 / 標準 / 軽量化 / 白黒化 / 強制画像化）または `profiles/` の保存済みプロファイルを選択。
   - **詳細モード (Detail Mode)**:
     - 戦略: テキスト保持（`keep-text`） / 完全ラスタライズ（`rasterize`）
     - カラーモード: `Color`（JPEG）/ `Gray`（グレースケール JPEG）/ `Mono`（1-bit Otsu 二値化または Floyd-Steinberg ディザリング + CCITT Group 4）
     - 解像度 (DPI): `50`, `72`, `100`, `150`, `200`, `300`, `450`
     - 破棄設定: 注釈・コメント、リンク、しおり、AcroForm、XML メタデータ、JS、サムネイル
     - フォント処理: サブセット化、埋め込み解除（フォント行をダブルクリックで解除対象を切替）
   - **プロファイル (Profile)**: 現在の詳細設定を名前付き JSON として `profiles/` に保存・削除。
3. **実行と PDF Audit**: 下部の実行ボタンで処理を開始し、ログ行または「Audit」ボタンから 6 カテゴリの容量内訳と変換前後比較を確認します。右上のボタンで日英言語を即時切替できます。

---

## 3. CLI モードの使い方

### 非対話バッチ CLI

```bash
# プリセット指定
minuopdf document.pdf -o optimized.pdf -p light

# 保存済みプロファイルと完全ラスタライズ戦略を指定
minuopdf scan.pdf -o out.pdf --profile force_rasterize_50dpi_jpegq35 -s rasterize

# 複数ファイルを出力ディレクトリへ一括処理
minuopdf a.pdf b.pdf c.pdf -b dist/ -p standard
```

| オプション | 短縮 | 説明 | 指定例 |
| :--- | :--- | :--- | :--- |
| `--output <PATH>` | `-o` | 出力 PDF パス | `-o optimized.pdf` |
| `--preset <NAME>` | `-p` | 組み込みプリセット (`high_quality`, `standard`, `light`, `mono`, `rasterize`) | `-p light` |
| `--profile <NAME>` | — | `profiles/<NAME>.json` の適用 | `--profile my_rule` |
| `--strategy <STRATEGY>` | `-s` | 圧縮戦略 (`keep-text`, `rasterize`) | `-s rasterize` |
| `--batch <DIR>` | `-b` | 複数入力のバッチ出力先ディレクトリ | `-b out_dir/` |

### 対話式コンソール CLI

PDF パスを 1 つだけ指定してオプションなしで起動すると、対話式コンソールメニューが起動します（標準入力が EOF の場合はデフォルト選択肢で安全に進行します）。

```bash
minuopdf document.pdf
```

---

## 4. カスタムプロファイル JSON 仕様 (`version: 2`)

プロファイルは `profiles/<name>.json` として保存されます（例: `profiles/force_rasterize_50dpi_jpegq35.json`）：

```json
{
  "version": 2,
  "color_conv": "gray",
  "color_img": { "downsample": "bicubic", "target_dpi": 50, "threshold_dpi": 225, "format": "jpeg", "quality": 35, "mono_method": null },
  "gray_img": { "downsample": "bicubic", "target_dpi": 50, "threshold_dpi": 225, "format": "jpeg", "quality": 35, "mono_method": null },
  "mono_img": { "downsample": "bicubic", "target_dpi": 300, "threshold_dpi": 450, "format": "ccitt", "quality": 0, "mono_method": "threshold" },
  "discard_comments": true,
  "discard_links": true,
  "discard_bookmarks": true,
  "discard_forms": true,
  "discard_metadata": true,
  "discard_js": true,
  "discard_thumbs": true,
  "clean_linearize": true,
  "clean_deflate": true,
  "clean_sanitize": true,
  "font_subset": true,
  "unembed_fonts": [],
  "force_rasterize": true
}
```

---

## 5. トラブルシューティング

- **PDFium が見つからないエラー（ラスタライズ時）**: 実行ファイルと同階層に `pdfium.dll`（Windows）または `libpdfium.so`（Linux）が存在するか確認してください。開発時は `python scripts/ensure_pdfium.py` で自動取得できます。
- **Linux で GUI が起動しない**: `libwebkit2gtk-4.1-dev` 等のシステムパッケージが導入されているか確認してください（[README.ja.md](../../README.ja.md) 参照）。
- **テキスト保持モードでファイルサイズが下がらない**: スキャン画像中心の PDF や埋め込みフォントが大きい PDF では、詳細モードでフォントサブセット化・埋め込み解除を有効にするか、`rasterize` 戦略を試してください。
