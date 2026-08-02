# Configuration file for the Sphinx documentation builder.
import os
import sys

# -- Project information -----------------------------------------------------
project = 'MinuoPDF'
copyright = '2026, MinuoPDF Contributors'
author = 'MinuoPDF Contributors'
release = '0.1.0'

# -- General configuration ---------------------------------------------------
extensions = [
    'sphinx.ext.autodoc',
    'sphinx.ext.napoleon',
    'sphinx_needs',
    'sphinx_copybutton',
    'sphinxcontrib.mermaid',
]

templates_path = ['_templates']
exclude_patterns = ['_build', 'Thumbs.db', '.DS_Store']
language = 'ja'
locale_dirs = ['locale/']
gettext_compact = False


# -- Options for HTML output -------------------------------------------------
html_theme = 'furo'
html_title = 'MinuoPDF Documentation'
html_static_path = ['_static']

# -- Sphinx-Needs configuration ----------------------------------------------
needs_types = [
    dict(directive="req", title="Requirement", prefix="REQ_", color="#BFD8D2", style="node"),
    dict(directive="arch", title="Architecture", prefix="ARCH_", color="#FEDCD2", style="node"),
    dict(directive="design", title="Design", prefix="DES_", color="#DFE3E6", style="node"),
    dict(directive="test", title="Test Specification", prefix="TEST_", color="#C4E3CB", style="node"),
    dict(directive="test_result", title="Test Result", prefix="TR_", color="#D4EDDA", style="node"),
]

needs_fields = {"result": {"default": ""}, "last_run": {"default": ""}}
needs_include_needs = True

# -- Sphinx-Needs Title i18n Translation ------------------------------------
NEED_TITLE_TRANSLATIONS = {
    "en": {
        # Requirements
        "ウィンドウ表示サイズと固定化": "Window Display Size & Fixing",
        "多言語切り替え (日本語 / 英語)": "Multilingual Toggle (Japanese / English)",
        "バルーンヘルプ (ツールチップ) 表示": "Balloon Help (Tooltip) Display",
        "処理進捗・ステータス表示": "Processing Progress & Status Display",
        "実行コンソールログ": "Execution Console Log",
        "ドラッグ＆ドロップによるファイル追加": "File Addition via Drag & Drop",
        "ファイル選択ダイアログによる追加": "File Addition via Selection Dialog",
        "リスト要素の個別削除および全クリア": "List Item Removal & Clear All",
        "圧縮プリセットの選択": "Compression Preset Selection",
        "カスタム設定プロファイルの保存・復元・削除": "Custom Setting Profile Save/Load/Delete",
        "カラー変換モード": "Color Conversion Mode",
        "モノクロ二値化アルゴリズム": "Monochrome Binarization Algorithm",
        "解像度(DPI)とJPEG品質調整": "Resolution (DPI) & JPEG Quality Adjustment",
        "圧縮戦略の切替 (テキスト保持 / 完全ラスタライズ)": "Compression Strategy Toggle (Text Retention / Full Rasterization)",
        "テキスト保持 vs 完全ラスタライズ": "Text Retention vs Full Rasterization",
        "不要オブジェクトの破棄": "Unneeded Object Discard",
        "不要オブジェクトの削除・破棄": "Unneeded Object Removal & Discard",
        "フォントサブセット化および埋め込み解除 (Unembed Fonts)": "Font Subsetting & Un-embedding (Unembed Fonts)",
        "フォントサブセット化および埋め込み解除": "Font Subsetting & Un-embedding",
        "コマンドライン対話モード": "Command Line Interactive Mode",
        "CUI 対話モードでの設定・実行": "CUI Interactive Mode Configuration & Execution",
        "完全非GPLライセンス": "Fully Non-GPL License",
        "完全非GPLオープンライセンス構成": "Fully Non-GPL Open Source License Structure",
        "ポータビリティ (ポータブル動作)": "Portability (Standalone Operation)",
        "スタンドアロン・ポータビリティ": "Standalone Portability",
        "Windowsコンソール隠蔽制御": "Windows Console Hiding Control",
        "GUI起動時のコンソールプロンプト非表示": "Hide Console Prompt on GUI Launch",

        # Architecture
        "フロントエンド UI & ステート管理": "Frontend UI & State Management",
        "Tauri IPC ゲートウェイ": "Tauri IPC Gateway",
        "CUI コマンドラインコントローラ": "CUI Command Line Controller",
        "プロファイルマネージャ": "Profile Manager",
        "最適化コアエンジン (Optimizer Core)": "Optimization Core Engine (Optimizer Core)",
        "ページラスタライザ (Pdfium Wrapper)": "Page Rasterizer (Pdfium Wrapper)",
        "フォント処理エンジン (Font Engine)": "Font Processing Engine (Font Engine)",

        # Design - GUI & Backend
        "App.svelte - メインビューコンポーネント": "App.svelte - Main View Component",
        "国際化 (i18n) モジュール設計": "Internationalization (i18n) Module Design",
        "optimizer.rs - 最適化処理エンジン": "optimizer.rs - Optimization Processing Engine",
        "rasterizer.rs - ページ全体ラスタライズ": "rasterizer.rs - Page-wide Rasterization",
        "font_subsetter.rs & font_unembedder.rs - フォント処理": "font_subsetter.rs & font_unembedder.rs - Font Processing",
        "profile.rs - JSONプロファイル管理": "profile.rs - JSON Profile Management",

        # Testing
        "画像処理 & 二値化アルゴリズム単体検証 (Image & Binarization Algorithm Verification)": "Image & Binarization Algorithm Verification",
        "プロファイルマネージャ単体検証 (Profile Manager Verification)": "Profile Manager Verification",
        "フォント処理エンジン単体検証 (Font Engine Verification)": "Font Engine Verification",
        "PDF Audit 容量構造解析単体検証 (PDF Audit Analysis Verification)": "PDF Audit Analysis Verification",
        "最適化コア & カラー置換結合検証 (Optimizer Core & Color Conv Integration)": "Optimizer Core & Color Conv Integration",
        "ページラスタライザ (Pdfium) 結合検証 (Page Rasterizer Integration)": "Page Rasterizer Integration",
        "フォント埋め込み解除 & オブジェクト破棄結合検証 (Font Unembed & Discard Integration)": "Font Unembed & Discard Integration",
        "Tauri IPC コマンド & イベント配信結合検証 (Tauri IPC & Event Integration)": "Tauri IPC & Event Integration",
        "容量削減率 & 圧縮性能適格性検証 (Compression Performance Qualification)": "Compression Performance Qualification",
        "非GPLライセンス適合性検証 (License Compliance Qualification)": "License Compliance Qualification",
        "スタンドアロン・ポータビリティ検証 (Portability Qualification)": "Portability Qualification",
        "E2E GUI 基本要素・言語・タブ検証 (Playwright Basic GUI Verification)": "Playwright Basic GUI Verification",
        "E2E GUI 詳細設定コントロール検証 (Playwright Detail Settings Verification)": "Playwright Detail Settings Verification",
        "E2E GUI プロファイル管理検証 (Playwright Profile Management Verification)": "Playwright Profile Management Verification",
        "E2E GUI ファイルリスト操作検証 (Playwright Target File List Verification)": "Playwright Target File List Verification",
        "E2E GUI 処理進捗・ログ表示検証 (Playwright Progress & Log Console Verification)": "Playwright Progress & Log Console Verification",

        # Test Results
        "単体テスト (Unit Test) 実行サマリー": "Unit Test Execution Summary",
        "結合テスト (Integration Test) 実行サマリー": "Integration Test Execution Summary",
        "適格性テスト (Qualification Test) 実行サマリー": "Qualification Test Execution Summary",
        "E2E GUI 自動テスト実行サマリー": "E2E GUI Automated Test Execution Summary",
        "容量削減率 & 圧縮性能適格性検証": "Compression Performance Qualification",
        "完全非GPLライセンス適合性検証": "Non-GPL License Compliance Qualification",
        "スタンドアロン・ポータビリティ検証": "Standalone Portability Qualification",

        # Needtable captions
        "アーキテクチャと対応要件一覧": "Architecture & Corresponding Requirements List",
        "詳細設計と対応アーキテクチャ一覧": "Detailed Design & Corresponding Architecture List",
        "単体テスト仕様一覧": "Unit Test Specifications List",
        "結合テスト仕様一覧": "Integration Test Specifications List",
        "適格性テスト仕様一覧": "Qualification Test Specifications List",
        "E2E GUI テスト仕様一覧": "GUI E2E Test Specifications List",
        "単体テスト実行結果一覧": "Unit Test Execution Results List",
        "結合テスト実行結果一覧": "Integration Test Execution Results List",
        "適格性テスト実行結果一覧": "Qualification Test Execution Results List",
        "E2E GUI テスト実行結果一覧": "GUI E2E Test Execution Results List",
    }
}

def setup(app):
    def translate_need_titles(app, doctree, docname=None):
        lang = app.config.language
        if lang in NEED_TITLE_TRANSLATIONS:
            trans = NEED_TITLE_TRANSLATIONS[lang]
            if hasattr(app.env, '_needs_all_needs'):
                for need in app.env._needs_all_needs.values():
                    if need.get('title') in trans:
                        need['title'] = trans[need['title']]
            from docutils import nodes
            for node in list(doctree.findall()):
                if hasattr(node, 'attributes') and isinstance(node.attributes, dict):
                    if 'title' in node.attributes and node.attributes['title'] in trans:
                        node.attributes['title'] = trans[node.attributes['title']]
                    if 'caption' in node.attributes and node.attributes['caption'] in trans:
                        node.attributes['caption'] = trans[node.attributes['caption']]
                if isinstance(node, nodes.Text):
                    txt = str(node).strip()
                    if txt in trans:
                        node.parent.replace(node, nodes.Text(trans[txt]))

    app.connect('doctree-read', translate_need_titles)
    app.connect('doctree-resolved', translate_need_titles)








