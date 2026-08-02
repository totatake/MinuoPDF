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
           UI["App.svelte<br/>• UI Layout & Catppuccin Theme<br/>• i18n State (ja/en)<br/>• Drag & Drop & Tooltips<br/>• Preset & Profile UI"]
       end

       subgraph IPC ["IPC Gateway"]
           GATEWAY["Tauri IPC Gateway<br/>(invoke / Commands & Event Emitter)"]
       end

       subgraph BE ["Backend Layer (Rust / Tauri Host)"]
           ENTRY["lib.rs<br/>Entry point & Command Router"]
           CLI["cli.rs<br/>CUI Interactive Mode Controller"]
           PROF["profile.rs<br/>Profile Loader & Saver"]
           OPT["optimizer.rs<br/>Optimization Core Engine"]
           RAST["rasterizer.rs<br/>Page Rendering & Image Extraction"]
           FONT["font_subsetter.rs / font_unembedder.rs<br/>Font Stream Processing"]
       end

       subgraph CORE ["External & Core Libraries"]
           LOPDF["lopdf<br/>PDF Structure Parser & Writer"]
           PDFIUM["pdfium-render & pdfium.dll<br/>Page Rasterizer Engine"]
           CRATES["image / fax crates<br/>JPEG / CCITT Group 4 Encodings"]
       end

       UI <-->|"Tauri IPC (invoke / events)"| GATEWAY
       GATEWAY --> ENTRY
       ENTRY --> CLI
       ENTRY --> PROF
       ENTRY --> OPT
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

   Svelte 5 を採用した単一ページアプリケーション（SPA）。Catppuccin Mocha ダークテーマに基づき、固定サイズ（680x530）レイアウトと動的多言語（ja/en）ステートを提供する。

.. arch:: Tauri IPC ゲートウェイ
   :id: ARCH_IPC_001
   :links: REQ_GUI_004, REQ_GUI_005

   Tauri v2 ``invoke`` メッセージハンドラおよびイベントエミッター。進捗状況（0-100%）や非同期ログ出力をフロントエンドにリアルタイム配信する。

.. arch:: CUI コマンドラインコントローラ
   :id: ARCH_BE_CLI
   :links: REQ_CLI_001, REQ_NFR_003

   起動引数の有無を判定し、引数が存在する場合は Win32 SW_HIDE をスキップして対話型コンソールインターフェース（``cli.rs``）を起動する。

.. arch:: プロファイルマネージャ
   :id: ARCH_BE_PROF
   :links: REQ_PROF_002, REQ_NFR_002

   実行ファイルと同一階層の ``profiles/`` サブディレクトリから JSON プロファイルを動的に検出・ロードし、バージョン検証（v2以上）を行う。

.. arch:: 最適化コアエンジン (Optimizer Core)
   :id: ARCH_BE_OPT
   :links: REQ_OPT_001, REQ_OPT_002, REQ_OPT_003, REQ_OPT_004, REQ_OPT_005

   ``lopdf`` クレートを用いて PDF オブジェクトツリーを走査・書き換えし、画像 XObject の抽出・置換、不要辞書キーの削除、およびストリーム再圧縮を行う。

.. arch:: ページラスタライザ (Pdfium Wrapper)
   :id: ARCH_BE_RAST
   :links: REQ_OPT_004, REQ_NFR_002

   ``pdfium-render`` を介して ``pdfium.dll`` を動的ロードし、ページ全域を高精細レンダリングして画像化（ラスタライズ）する。

.. arch:: フォント処理エンジン (Font Engine)
   :id: ARCH_BE_FONT
   :links: REQ_OPT_006

   PDF内の TrueType/OpenType フォントの CID マッピング解析、サブセット化、および埋め込みストリーム（``FontFile2`` 等）の安全な削除処理を行う。
