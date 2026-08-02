========================================
要件トレーサビリティマトリクス
========================================

本ページは、Sphinx-needs により自動生成された要件・アーキテクチャ・詳細設計・テスト仕様のトレーサビリティ一覧です。
開発フェーズ（要件 ➔ アーキテクチャ ➔ 詳細設計 ➔ テスト仕様 ➔ テスト実行結果）の階層ごとに対応関係を一覧化しています。

1. 要件 ➔ アーキテクチャ マトリクス
====================================

.. needtable:: アーキテクチャと対応要件一覧
   :types: arch
   :columns: id, title, links
   :style: table

2. アーキテクチャ ➔ 詳細設計 マトリクス
========================================

.. needtable:: 詳細設計と対応アーキテクチャ一覧
   :types: design
   :columns: id, title, links
   :style: table

3. 設計・要件 ➔ テスト仕様 マトリクス
======================================

単体テスト仕様 (Unit Test Specs)
---------------------------------

.. needtable:: 単体テスト仕様一覧
   :types: test
   :filter: "TEST_UNIT_" in id
   :columns: id, title, links
   :style: table

結合テスト仕様 (Integration Test Specs)
---------------------------------------

.. needtable:: 結合テスト仕様一覧
   :types: test
   :filter: "TEST_INT_" in id
   :columns: id, title, links
   :style: table

適格性テスト仕様 (Qualification Test Specs)
-------------------------------------------

.. needtable:: 適格性テスト仕様一覧
   :types: test
   :filter: "TEST_QUAL_" in id
   :columns: id, title, links
   :style: table

E2E GUI テスト仕様 (GUI E2E Test Specs)
---------------------------------------

.. needtable:: E2E GUI テスト仕様一覧
   :types: test
   :filter: "TEST_GUI_" in id
   :columns: id, title, links
   :style: table


4. テスト仕様 ➔ テスト実行結果 マトリクス
==========================================

単体テスト (Unit Tests) 実行結果
---------------------------------

.. needtable:: 単体テスト実行結果一覧
   :types: test_result
   :filter: "TR_UNIT_" in id
   :columns: id, title, links, result, last_run
   :style: table

結合テスト (Integration Tests) 実行結果
---------------------------------------

.. needtable:: 結合テスト実行結果一覧
   :types: test_result
   :filter: "TR_INT_" in id
   :columns: id, title, links, result, last_run
   :style: table

適格性テスト (Qualification Tests) 実行結果
-------------------------------------------

.. needtable:: 適格性テスト実行結果一覧
   :types: test_result
   :filter: "TR_QUAL_" in id
   :columns: id, title, links, result, last_run
   :style: table

E2E GUI テスト (GUI E2E Tests) 実行結果
---------------------------------------

.. needtable:: E2E GUI テスト実行結果一覧
   :types: test_result
   :filter: "TR_CASE_" in id
   :columns: id, title, links, result, last_run
   :style: table

