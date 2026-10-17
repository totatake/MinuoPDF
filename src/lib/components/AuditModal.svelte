<script lang="ts">
  import type { PdfAuditComparison } from "../types";
  import { CATEGORY_COLORS } from "../types";
  import { formatBytes } from "../utils/format";
  import { translations } from "../utils/i18n";

  let {
    isOpen = false,
    auditData = null,
    isLoading = false,
    errorMessage = "",
    currentLang = "en",
    onClose,
  }: {
    isOpen: boolean;
    auditData: PdfAuditComparison | null;
    isLoading: boolean;
    errorMessage: string;
    currentLang: "ja" | "en";
    onClose: () => void;
  } = $props();

  const t = $derived(translations[currentLang]);
</script>

{#if isOpen}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="modal-backdrop" onclick={onClose}>
    <div class="modal-container" onclick={(e) => e.stopPropagation()}>
      <header class="modal-header">
        <div class="modal-title-box">
          <span class="modal-icon">📊</span>
          <h3 class="modal-title">{t.auditTitle}</h3>
        </div>
        <button class="modal-close-btn" onclick={onClose}>×</button>
      </header>

      <div class="modal-body">
        {#if isLoading}
          <div class="audit-loading-box">
            <div class="spinner"></div>
            <span>{t.auditLoading}</span>
          </div>
        {:else if errorMessage}
          <div class="audit-error-box">
            ❌ {errorMessage}
          </div>
        {:else if auditData}
          <!-- File Info Header -->
          <div class="audit-file-header">
            <div class="file-name">{auditData.original.file_name}</div>
            <div class="file-size-summary">
              <span class="size-orig">{t.auditBefore}: <strong>{formatBytes(auditData.original.total_bytes)}</strong></span>
              {#if auditData.optimized}
                <span class="arrow">➔</span>
                <span class="size-opt">{t.auditAfter}: <strong>{formatBytes(auditData.optimized.total_bytes)}</strong></span>
                {@const diff = auditData.original.total_bytes - auditData.optimized.total_bytes}
                {@const pct = (diff / auditData.original.total_bytes) * 100}
                <span class="badge-saving" class:negative={diff < 0}>
                  {diff >= 0 ? `-${formatBytes(diff)} (-${pct.toFixed(1)}%)` : `+${formatBytes(-diff)} (+${(-pct).toFixed(1)}%)`}
                </span>
              {:else}
                <span class="audit-no-opt-hint">{t.auditNoOptYet}</span>
              {/if}
            </div>
          </div>

          <!-- Stacked Bar Charts (Visual Comparison) -->
          <div class="audit-stacked-bar-section">
            <div class="stacked-bar-row">
              <span class="bar-label">{t.auditBefore}</span>
              <div class="stacked-bar">
                {#each auditData.original.categories as cat}
                  {#if cat.percentage > 0}
                    <div
                      class="bar-segment"
                      style="width: {cat.percentage}%; background-color: {CATEGORY_COLORS[cat.key] || '#64748b'};"
                      title="{currentLang === 'ja' ? cat.name_ja : cat.name_en}: {formatBytes(cat.bytes)} ({cat.percentage.toFixed(1)}%)"
                    ></div>
                  {/if}
                {/each}
              </div>
            </div>

            {#if auditData.optimized}
              <div class="stacked-bar-row">
                <span class="bar-label">{t.auditAfter}</span>
                <div class="stacked-bar">
                  {#each auditData.optimized.categories as cat}
                    {#if cat.percentage > 0}
                      <div
                        class="bar-segment"
                        style="width: {cat.percentage}%; background-color: {CATEGORY_COLORS[cat.key] || '#64748b'};"
                        title="{currentLang === 'ja' ? cat.name_ja : cat.name_en}: {formatBytes(cat.bytes)} ({cat.percentage.toFixed(1)}%)"
                      ></div>
                    {/if}
                  {/each}
                </div>
              </div>
            {/if}

            <!-- Bar Legend -->
            <div class="audit-legend">
              {#each auditData.original.categories as cat}
                <div class="legend-item">
                  <span class="legend-color" style="background-color: {CATEGORY_COLORS[cat.key] || '#64748b'};"></span>
                  <span class="legend-text">{currentLang === 'ja' ? cat.name_ja : cat.name_en}</span>
                </div>
              {/each}
            </div>
          </div>

          <!-- Audit Data Table -->
          <div class="audit-table-container">
            <table class="audit-table">
              <thead>
                <tr>
                  <th>{t.auditColCategory}</th>
                  <th colspan="3">{t.auditBefore}</th>
                  {#if auditData.optimized}
                    <th colspan="3">{t.auditAfter}</th>
                    <th>{t.auditColDiff}</th>
                  {/if}
                </tr>
                <tr class="subhead-row">
                  <th></th>
                  <th>{t.auditColCount}</th>
                  <th>{t.auditColBytes}</th>
                  <th>%</th>
                  {#if auditData.optimized}
                    <th>{t.auditColCount}</th>
                    <th>{t.auditColBytes}</th>
                    <th>%</th>
                    <th>{t.auditDiff}</th>
                  {/if}
                </tr>
              </thead>
              <tbody>
                {#each auditData.original.categories as origCat, idx}
                  {@const optCat = auditData.optimized ? auditData.optimized.categories[idx] : null}
                  {@const diffBytes = optCat ? origCat.bytes - optCat.bytes : 0}
                  <tr>
                    <td class="cat-name">
                      <span class="row-color-dot" style="background-color: {CATEGORY_COLORS[origCat.key] || '#64748b'};"></span>
                      {currentLang === 'ja' ? origCat.name_ja : origCat.name_en}
                    </td>
                    <td class="num">{origCat.count || "--"}</td>
                    <td class="num">{formatBytes(origCat.bytes)}</td>
                    <td class="num-pct">{origCat.percentage.toFixed(2)}%</td>
                    {#if optCat}
                      <td class="num">{optCat.count || "--"}</td>
                      <td class="num">{formatBytes(optCat.bytes)}</td>
                      <td class="num-pct">{optCat.percentage.toFixed(2)}%</td>
                      <td class="num-diff" class:positive={diffBytes > 0} class:negative={diffBytes < 0}>
                        {diffBytes > 0 ? `-${formatBytes(diffBytes)}` : diffBytes < 0 ? `+${formatBytes(-diffBytes)}` : "0 B"}
                      </td>
                    {/if}
                  </tr>
                {/each}
              </tbody>
              <tfoot>
                <tr class="total-row">
                  <td>{t.auditTotal}</td>
                  <td class="num">
                    {auditData.original.categories.reduce((a, b) => a + b.count, 0)}
                  </td>
                  <td class="num">{formatBytes(auditData.original.total_bytes)}</td>
                  <td class="num-pct">100%</td>
                  {#if auditData.optimized}
                    <td class="num">
                      {auditData.optimized.categories.reduce((a, b) => a + b.count, 0)}
                    </td>
                    <td class="num">{formatBytes(auditData.optimized.total_bytes)}</td>
                    <td class="num-pct">100%</td>
                    {@const totalDiff = auditData.original.total_bytes - auditData.optimized.total_bytes}
                    <td class="num-diff" class:positive={totalDiff > 0} class:negative={totalDiff < 0}>
                      {totalDiff >= 0 ? `-${formatBytes(totalDiff)}` : `+${formatBytes(-totalDiff)}`}
                    </td>
                  {/if}
                </tr>
              </tfoot>
            </table>
          </div>
        {/if}
      </div>

      <footer class="modal-footer">
        <button class="btn btn-secondary" onclick={onClose}>{t.auditClose}</button>
      </footer>
    </div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background-color: rgba(15, 23, 42, 0.75);
    backdrop-filter: blur(4px);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 99999;
    animation: fadeIn 0.15s ease-out;
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  .modal-container {
    background-color: #1e293b;
    border: 1px solid #475569;
    border-radius: 10px;
    width: 630px;
    max-width: 96vw;
    max-height: 505px;
    display: flex;
    flex-direction: column;
    box-shadow: 0 20px 30px -10px rgba(0, 0, 0, 0.7);
    overflow: hidden;
    color: var(--text-main);
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 6px 14px;
    background-color: #0f172a;
    border-bottom: 1px solid var(--border-color);
  }

  .modal-title-box {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .modal-icon {
    font-size: 1.0rem;
  }

  .modal-title {
    margin: 0;
    font-size: 0.90rem;
    font-weight: 600;
    color: var(--text-main);
  }

  .modal-close-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    font-size: 1.2rem;
    line-height: 1;
    cursor: pointer;
    padding: 0 4px;
  }

  .modal-close-btn:hover {
    color: #ffffff;
  }

  .modal-body {
    padding: 8px 12px;
    overflow-y: auto;
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .modal-body::-webkit-scrollbar {
    width: 6px;
  }
  .modal-body::-webkit-scrollbar-track {
    background: #0f172a;
  }
  .modal-body::-webkit-scrollbar-thumb {
    background: #475569;
    border-radius: 3px;
  }

  .modal-footer {
    padding: 6px 14px;
    background-color: #0f172a;
    border-top: 1px solid var(--border-color);
    display: flex;
    justify-content: flex-end;
  }

  .audit-file-header {
    background-color: #0f172a;
    border: 1px solid var(--border-color);
    border-radius: 5px;
    padding: 6px 10px;
  }

  .audit-file-header .file-name {
    font-weight: 600;
    font-size: 0.82rem;
    color: #ffffff;
    margin-bottom: 2px;
    word-break: break-all;
  }

  .file-size-summary {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 0.72rem;
    color: var(--text-muted);
    flex-wrap: wrap;
  }

  .file-size-summary strong {
    color: var(--text-main);
  }

  .arrow {
    color: var(--accent-blue);
    font-weight: bold;
  }

  .badge-saving {
    background-color: rgba(16, 185, 129, 0.2);
    color: #34d399;
    border: 1px solid rgba(16, 185, 129, 0.4);
    padding: 0 6px;
    border-radius: 10px;
    font-weight: 600;
    font-size: 0.70rem;
  }

  .badge-saving.negative {
    background-color: rgba(239, 68, 68, 0.2);
    color: #f87171;
    border-color: rgba(239, 68, 68, 0.4);
  }

  .audit-no-opt-hint {
    font-size: 0.70rem;
    color: #64748b;
    font-style: italic;
  }

  .audit-stacked-bar-section {
    display: flex;
    flex-direction: column;
    gap: 4px;
    background-color: #0f172a;
    border: 1px solid var(--border-color);
    border-radius: 5px;
    padding: 6px 10px;
  }

  .stacked-bar-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .bar-label {
    width: 95px;
    font-size: 0.70rem;
    color: var(--text-muted);
    font-weight: 500;
  }

  .stacked-bar {
    flex: 1;
    height: 11px;
    background-color: #1e293b;
    border-radius: 3px;
    display: flex;
    overflow: hidden;
  }

  .bar-segment {
    height: 100%;
    transition: width 0.3s;
  }

  .bar-segment:hover {
    filter: brightness(1.25);
  }

  .audit-legend {
    display: flex;
    flex-wrap: wrap;
    gap: 3px 10px;
    margin-top: 2px;
    padding-top: 4px;
    border-top: 1px dashed rgba(51, 65, 85, 0.6);
  }

  .legend-item {
    display: flex;
    align-items: center;
    gap: 4px;
    font-size: 0.67rem;
    color: var(--text-muted);
  }

  .legend-color {
    width: 8px;
    height: 8px;
    border-radius: 2px;
  }

  .audit-table-container {
    border: 1px solid var(--border-color);
    border-radius: 5px;
    overflow: hidden;
  }

  .audit-table {
    width: 100%;
    border-collapse: collapse;
    font-size: 0.70rem;
    text-align: left;
  }

  .audit-table th, .audit-table td {
    padding: 3px 6px;
    border-bottom: 1px solid rgba(51, 65, 85, 0.4);
  }

  .audit-table thead th {
    background-color: #0f172a;
    color: var(--text-main);
    font-weight: 600;
    text-align: center;
    border-bottom: 1px solid var(--border-color);
  }

  .audit-table thead tr.subhead-row th {
    font-size: 0.65rem;
    color: var(--text-muted);
    font-weight: normal;
    padding: 1px 4px;
  }

  .audit-table thead th:first-child, .audit-table tbody td:first-child {
    text-align: left;
  }

  .cat-name {
    display: flex;
    align-items: center;
    gap: 5px;
    font-weight: 500;
  }

  .row-color-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    display: inline-block;
  }

  .num {
    text-align: right;
    font-family: monospace;
  }

  .num-pct {
    text-align: right;
    font-family: monospace;
    color: var(--text-muted);
  }

  .num-diff {
    text-align: right;
    font-family: monospace;
    font-weight: 600;
  }

  .num-diff.positive {
    color: #34d399;
  }

  .num-diff.negative {
    color: #f87171;
  }

  .total-row td {
    background-color: #0f172a;
    font-weight: bold;
    border-bottom: none;
    border-top: 1px solid var(--border-color);
  }

  .audit-loading-box {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 20px;
    gap: 10px;
    color: var(--text-muted);
  }

  .spinner {
    width: 22px;
    height: 22px;
    border: 3px solid rgba(59, 130, 246, 0.2);
    border-top-color: var(--accent-blue);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .audit-error-box {
    background-color: rgba(239, 68, 68, 0.15);
    border: 1px solid rgba(239, 68, 68, 0.3);
    color: #f87171;
    padding: 10px;
    border-radius: 5px;
    font-size: 0.78rem;
  }

  .btn {
    border: none;
    border-radius: 5px;
    padding: 5px 12px;
    font-size: 0.78rem;
    font-weight: 600;
    cursor: pointer;
  }

  .btn-secondary {
    background-color: var(--bg-surface);
    border: 1px solid var(--border-color);
    color: var(--text-main);
  }
</style>

