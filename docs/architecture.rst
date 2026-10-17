========================================
ソフトウェアアーキテクチャ設計書
========================================

本ドキュメントは、MinuoPDF アプリケーションのシステム構成、コンポーネント設計、およびデータフローを定義します。

システム構成概要
==================

MinuoPDF は、フロントエンド層（Svelte 5）、IPC ゲートウェイ（Tauri v2）、およびバックエンド層（Rust）の3層アーキテクチャで構成されます。

.. mermaid::

   flowchart TD
       subgraph FE ["Frontend Layer (Svelte 5 / Vite / TS)"]
           APP["+page.svelte (App Container)"]
           subgraph COMPONENTS ["Modular Components"]
               FDB["FileDropBar.svelte"]
               EMT["EasyModeTab.svelte"]
               DMT["DetailModeTab.svelte"]
               PFT["ProfileTab.svelte"]
               STF["StatusFooter.svelte"]
               AUM["AuditModal.svelte"]
           end
           I18N["i18n.ts / format.ts"]
           APP --> COMPONENTS
           COMPONENTS --> I18N
       end

       subgraph IPC ["IPC Gateway"]
           GATEWAY["Tauri IPC Gateway<br/>(invoke / Commands & Event Emitter)"]
       end

       subgraph BE ["Backend Layer (Rust / Tauri Host)"]
           ENTRY["lib.rs<br/>Entry point & Async Command Router"]
           CLI["cli.rs<br/>Interactive & Batch CLI Controller"]
           PROF["profile.rs<br/>Profile Loader & Saver"]
           OPT["optimizer.rs<br/>Optimization Core (Rayon Parallel)"]
           IMG["image_util.rs<br/>Unified Image Encoding Engine"]
           AUDIT["audit.rs<br/>PDF Structure Audit Engine"]
           RAST["rasterizer.rs<br/>Page Rendering & Rasterization"]
           FONT["font_subsetter.rs<br/>Font GID Extraction & Subsetting"]
       end

       subgraph CORE ["External & Core Libraries"]
           LOPDF["lopdf<br/>PDF Structure Parser & Writer"]
           PDFIUM["pdfium-render & libpdfium<br/>Page Rasterizer Engine"]
           CRATES["image / fax / rayon<br/>Parallel JPEG / CCITT G4 Encoding"]
       end

       APP <-->|"Tauri IPC (invoke / events)"| GATEWAY
       GATEWAY --> ENTRY
       ENTRY --> CLI
       ENTRY --> PROF
       ENTRY --> OPT
       ENTRY --> AUDIT
       OPT --> IMG
       RAST --> IMG
       OPT --> AUDIT
       OPT --> RAST
       OPT --> FONT
       OPT --> LOPDF
       RAST --> PDFIUM
       OPT --> CRATES


アーキテクチャコンポーネント
=============================

.. arch:: フロントエンド UI & ステート管理
   :id: ARCH_FE_001
   :links: REQ_GUI_001, REQ_GUI_002, REQ_GUI_003, REQ_FILE_001

   Svelte 5 を採用したモジュラー SPA。Catppuccin Mocha ダークテーマに基づき、幅 740px・高さ 560px を初期値としつつ最小 680×520px 制約付きで柔軟にリサイズ可能なレイアウトを提供する。責務ごとに分割されたコンポーネント群（``FileDropBar``, ``EasyModeTab``, ``DetailModeTab``, ``ProfileTab``, ``StatusFooter``, ``AuditModal``）と型定義・辞書モジュール（``types/index.ts``, ``i18n.ts``）で構成される。

.. arch:: Tauri IPC ゲートウェイ
   :id: ARCH_IPC_001
   :links: REQ_GUI_004, REQ_GUI_005

   Tauri v2 ``invoke`` 非同期メッセージハンドラ（``spawn_blocking`` によるUIブロッキング防止）およびイベントエミッター。進捗状況（0-100%）、非同期ログ出力、およびインメモリ計算された容量監査比較（``PdfAuditComparison``）をフロントエンドにリアルタイム配信する。

.. arch:: CUI コマンドラインコントローラ
   :id: ARCH_BE_CLI
   :links: REQ_CLI_001, REQ_CLI_002, REQ_NFR_003

   起動引数の有無およびオプションフラグを判定し、単体PDF指定時は対話型コンソールメニュー、オプション指定時（``-o``, ``-p``, ``--profile``, ``-s``, ``-b``）は非対話ヘッドレスバッチ実行を行う。

.. arch:: プロファイルマネージャ
   :id: ARCH_BE_PROF
   :links: REQ_PROF_002, REQ_NFR_002

   実行ファイルと同一階層の ``profiles/`` サブディレクトリから JSON プロファイルを動的に検出・ロードし、バージョン検証（v2以上）を行う。

.. arch:: 最適化コアエンジン (Optimizer Core)
   :id: ARCH_BE_OPT
   :links: REQ_OPT_001, REQ_OPT_002, REQ_OPT_003, REQ_OPT_004, REQ_OPT_005, REQ_OPT_007, REQ_OPT_008, REQ_PERF_001

   ``lopdf`` クレートを用いて PDF オブジェクトツリーを走査・書き換えする。Rayon による画像タスク並列処理、CTM実効DPI算出、Form XObject再帰走査、透過SMask連動リサイズ、逆圧縮防止（Smart Fallback）、不要辞書キー削除、および Deflate 再圧縮を行う。

.. arch:: ページラスタライザ (Pdfium Wrapper)
   :id: ARCH_BE_RAST
   :links: REQ_OPT_004, REQ_NFR_002

   ``pdfium-render`` を介して PDFium を動的ロードし、ページ全域を高精細レンダリングして画像化（ラスタライズ）する。

.. arch:: フォント処理エンジン (Font Engine)
   :id: ARCH_BE_FONT
   :links: REQ_OPT_006

   PDF内の TrueType/OpenType フォントの CID マッピング解析、サブセット化、および埋め込みストリーム（``FontFile2`` 等）の安全な削除処理を行う。

.. arch:: PDF容量内訳解析エンジン (Audit Engine)
   :id: ARCH_BE_AUDIT
   :links: REQ_AUDIT_001, REQ_PERF_001

   PDF内の全オブジェクトおよびストリームを走査し、Images, Fonts, Content Streams, Metadata/Catalog, Struct/Forms, Other のカテゴリ別に容量内訳（バイト数・パーセンテージ）をインメモリで解析・集計する。最適化パイプラインと密結合し、重複ディスクI/Oを発生させずに前後比較を生成する。
