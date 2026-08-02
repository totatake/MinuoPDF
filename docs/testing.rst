========================================
テスト仕様および品質検証 (Testing)
========================================

本ドキュメントは、MinuoPDF のユニットテスト、結合テスト、GUI E2E 自動テスト、および適格性テストの検証仕様を定義します。

テスト仕様
===========

単体テスト仕様 (Unit Verification)
=====================================

.. test:: 画像処理 & 二値化アルゴリズム単体検証 (Image & Binarization Algorithm Verification)
   :id: TEST_UNIT_001
   :links: DES_BE_001, REQ_OPT_002

   - **画像処理ロジック**: Otsu の二値化アルゴリズム、CCITT Group 4 エンコード、および画像ダンプ・変換関数の正常性を単体レベルで検証する (``test_otsu_thresholding``, ``test_g4_encoding``)。

.. test:: プロファイルマネージャ単体検証 (Profile Manager Verification)
   :id: TEST_UNIT_002
   :links: DES_BE_PROF, REQ_PROF_002

   - **プロファイルI/O**: 設定オブジェクトのJSON書き出し・読み込み・削除・プロファイルディレクトリの書き込み権限を単体検証する (``test_profile_save_load_delete``, ``test_preset_configs``)。

.. test:: フォント処理エンジン単体検証 (Font Engine Verification)
   :id: TEST_UNIT_003
   :links: DES_BE_FONT, REQ_OPT_006

   - **フォントストリーム解析**: PDF内部のTrueType/OpenType CIDマッピング抽出、W配列パース、およびフォントサブセット化ロジックを単体検証する (``test_extract_hex_gids``, ``test_subset_ikinari_pdf``)。

.. test:: PDF Audit 容量構造解析単体検証 (PDF Audit Analysis Verification)
   :id: TEST_UNIT_004
   :links: DES_BE_001

   - **容量割合計算**: PDF オブジェクト（画像、ストリーム、注釈等）の容量算出およびパーセンテージ計算ロジックを単体検証する (``test_audit_category_percentage``, ``test_audit_user_pdfs``)。

結合テスト仕様 (Integration Verification)
=========================================

.. test:: 最適化コア & カラー置換結合検証 (Optimizer Core & Color Conv Integration)
   :id: TEST_INT_001
   :links: ARCH_BE_OPT, REQ_OPT_001, REQ_OPT_003

   - **カラー置換パイプライン**: Color (DCTDecode JPEG再圧縮)、Gray (グレースケール変換)、Mono (Otsu/Dither) の各モードにおけるPDFオブジェクト走査・最適化処理の結合正常性を検証する。

.. test:: ページラスタライザ (Pdfium) 結合検証 (Page Rasterizer Integration)
   :id: TEST_INT_002
   :links: ARCH_BE_RAST, REQ_OPT_004

   - **完全ラスタライズモード**: ``pdfium.dll`` を介した高精細ページレンダリングおよび画像PDF生成処理の結合正常性を検証する。

.. test:: フォント埋め込み解除 & オブジェクト破棄結合検証 (Font Unembed & Discard Integration)
   :id: TEST_INT_003
   :links: ARCH_BE_FONT, REQ_OPT_005, REQ_OPT_006

   - **安全消去・切替**: 埋め込みフォントストリームの物理削除（Helvetica切替）および不要オブジェクト（注釈、しおり、メタデータ、JS等）の一括破棄の結合正常性を検証する。

.. test:: Tauri IPC コマンド & イベント配信結合検証 (Tauri IPC & Event Integration)
   :id: TEST_INT_004
   :links: ARCH_IPC_001, REQ_GUI_004, REQ_GUI_005

   - **双方向連携**: Tauri ``invoke`` メッセージ受動処理および進捗率（0-100%）・リアルタイムログ配信のIPC統合を検証する。

適格性テスト仕様 (Qualification Testing)
========================================

.. test:: 容量削減率 & 圧縮性能適格性検証 (Compression Performance Qualification)
   :id: TEST_QUAL_001
   :links: REQ_OPT_001, REQ_OPT_002, REQ_OPT_003, REQ_OPT_004, REQ_OPT_005, REQ_OPT_006

   - **容量削減評価**: 実サンプルPDF（カラー、グレー、モノクロ文書）に対する目標圧縮率とPDF閲覧整合性を検証する。

.. test:: 非GPLライセンス適合性検証 (License Compliance Qualification)
   :id: TEST_QUAL_002
   :links: REQ_NFR_001

   - **ライセンス監査**: サードパーティ依存クレート（lopdf, image, fax等）におけるGPL混入ゼロおよびMIT/BSD/Apache-2.0適合を確認する。

.. test:: スタンドアロン・ポータビリティ検証 (Portability Qualification)
   :id: TEST_QUAL_003
   :links: REQ_NFR_002, REQ_NFR_003

   - **ポータブル動作確認**: インストーラ不要の単体起動、同階層 ``pdfium.dll`` / ``profiles/`` 自立読み込み、および黒枠プロンプトのSW_HIDE非表示化を検証する。

E2E GUI 自動テスト仕様 (Playwright GUI Automation)
===================================================

.. note::
   Linux / WSL2 環境で初めて GUI 自動テストを実行する場合は、事前に Playwright 用ブラウザおよびシステム依存ライブラリをインストールしてください：

   .. code-block:: bash

      npx playwright install
      sudo npx playwright install-deps

.. test:: E2E GUI 基本要素・言語・タブ検証 (Playwright Basic GUI Verification)
   :id: TEST_GUI_001
   :links: REQ_GUI_001, REQ_GUI_002, REQ_GUI_003, REQ_PROF_001, DES_GUI_001

   - **Playwright 自動 GUI テスト**: ウィンドウタイトル・レイアウト要素のレンダリング、動的多言語切替 (ja/en)、タブ切り替え (簡単/詳細/プロファイル)、プリセット選択動作を自動検証する。
   - **実行コマンド**: ``make test-gui`` または ``npm run test:gui``

.. test:: E2E GUI 詳細設定コントロール検証 (Playwright Detail Settings Verification)
   :id: TEST_GUI_002
   :links: REQ_OPT_001, REQ_OPT_002, REQ_OPT_003, REQ_OPT_004, REQ_OPT_005, REQ_OPT_006

   - **画像・破棄・フォントGUIコントロール**: 詳細モードにおけるカラーポリシー選択、解像度(DPI)・JPEG品質入力、二値化方式ドロップダウン、不要オブジェクト破棄チェックボックス群、およびフォント一覧ダブルクリックによる埋め込み解除(Unembed)トグル動作を自動検証する。

.. test:: E2E GUI プロファイル管理検証 (Playwright Profile Management Verification)
   :id: TEST_GUI_003
   :links: REQ_PROF_002

   - **プロファイル操作フォーム**: プロファイル保存名の入力と保存ボタン活性化、保存済みプロファイル選択・ロード・削除のGUIトリガーを検証する。

.. test:: E2E GUI ファイルリスト操作検証 (Playwright Target File List Verification)
   :id: TEST_GUI_004
   :links: REQ_FILE_001, REQ_FILE_002, REQ_FILE_003

   - **ファイル追加・クリアUI**: ドロップエリア表示、ファイル追加ダイアログトリガー、個別削除ボタンおよび一括クリアボタンのUI表示・動作を検証する。

.. test:: E2E GUI 処理進捗・ログ表示検証 (Playwright Progress & Log Console Verification)
   :id: TEST_GUI_005
   :links: REQ_GUI_004, REQ_GUI_005

   - **進捗・ステータスバー・ログ**: リアルタイム進捗率表示（0-100%）、プログレスバー、一括最適化開始ボタン、および実行コンソールログエリアのレイアウトと可視性を検証する。



