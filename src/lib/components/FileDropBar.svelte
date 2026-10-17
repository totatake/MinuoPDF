<script lang="ts">
  import { translations } from "../utils/i18n";

  let {
    files = [],
    selectedFileIdx = -1,
    currentLang = "en",
    onSelectFiles,
    onClearAll,
    onRemoveSelected,
    onSelectIndex,
    onOpenAudit,
  }: {
    files: string[];
    selectedFileIdx: number;
    currentLang: "ja" | "en";
    onSelectFiles: () => void;
    onClearAll: () => void;
    onRemoveSelected?: (index: number) => void;
    onSelectIndex?: (index: number) => void;
    onOpenAudit: (filePath: string) => void;
  } = $props();

  let isListExpanded = $state<boolean>(false);
  const t = $derived(translations[currentLang]);
</script>

<section class="top-drop-bar">
  {#if files.length === 0}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="drop-zone-slim" onclick={onSelectFiles} style="cursor: pointer;">
      <span class="icon">📄</span>
      <span class="drop-text">{t.dropZone}</span>
    </div>
  {:else}
    <div class="file-summary-bar">
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div
        class="file-info-badge"
        onclick={() => { isListExpanded = !isListExpanded; }}
        style="cursor: pointer;"
        title={files.length > 1 ? "Click to toggle file list / クリックで一覧表示切替" : ""}
      >
        <span class="icon">📄</span>
        <span class="file-name-main">{files[0].split(/[\\/]/).pop()}</span>
        {#if files.length > 1}
          <span class="badge">+{files.length - 1}</span>
          <span class="toggle-icon">{isListExpanded ? "▲" : "▼"}</span>
        {/if}
      </div>
      <div class="top-actions">
        <button
          type="button"
          class="btn btn-sm btn-info-outline"
          onclick={(e) => {
            e.stopPropagation();
            const targetPath = files[selectedFileIdx >= 0 && selectedFileIdx < files.length ? selectedFileIdx : 0];
            onOpenAudit(targetPath);
          }}
        >
          📊 {t.auditBtn}
        </button>
        <button class="btn btn-sm btn-secondary" onclick={onSelectFiles}>{t.addFiles}</button>
        <button class="btn btn-sm btn-danger-outline" onclick={onClearAll}>{t.clearAll}</button>
      </div>
    </div>

    {#if isListExpanded && files.length > 1}
      <div class="file-expanded-list">
        {#each files as f, idx}
          <div
            class="file-item-row"
            class:selected={idx === selectedFileIdx}
            onclick={() => onSelectIndex?.(idx)}
            role="button"
            tabindex="0"
            onkeydown={(e) => { if (e.key === "Enter") onSelectIndex?.(idx); }}
          >
            <span class="file-path-text">{f.split(/[\\/]/).pop()}</span>
            <div class="file-item-actions">
              <button
                type="button"
                class="btn-icon-audit"
                title={t.auditBtn}
                onclick={(e) => {
                  e.stopPropagation();
                  onOpenAudit(f);
                }}
              >
                📊
              </button>
              <button
                type="button"
                class="btn-icon-delete"
                title={t.removeSelected}
                onclick={(e) => {
                  e.stopPropagation();
                  onRemoveSelected?.(idx);
                }}
              >
                ×
              </button>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  {/if}
</section>

<style>
  .top-drop-bar {
    padding: 6px 12px;
    background-color: #0b1120;
    border-bottom: 1px solid var(--border-color);
  }

  .drop-zone-slim {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 6px 12px;
    border: 1.5px dashed var(--border-highlight);
    border-radius: 6px;
    background-color: rgba(30, 41, 59, 0.4);
    cursor: pointer;
    transition: background-color 0.2s, border-color 0.2s;
  }

  .drop-zone-slim:hover {
    background-color: rgba(59, 130, 246, 0.08);
    border-color: var(--accent-blue);
  }

  .drop-text {
    color: var(--text-muted);
    font-size: 0.78rem;
  }

  .file-summary-bar {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .file-info-badge {
    display: flex;
    align-items: center;
    gap: 6px;
    font-weight: 500;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 440px;
  }

  .file-name-main {
    color: var(--text-main);
    font-weight: 600;
  }

  .badge {
    background-color: var(--accent-blue);
    color: #ffffff;
    font-size: 0.7rem;
    padding: 1px 6px;
    border-radius: 10px;
    font-weight: bold;
  }

  .toggle-icon {
    font-size: 0.65rem;
    color: var(--text-muted);
    margin-left: 2px;
  }

  .top-actions {
    display: flex;
    gap: 6px;
  }

  .file-expanded-list {
    margin-top: 6px;
    padding: 4px;
    max-height: 80px;
    overflow-y: auto;
    background-color: #0f172a;
    border: 1px solid var(--border-color);
    border-radius: 5px;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .file-item-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 2px 6px;
    border-radius: 4px;
    font-size: 0.73rem;
    cursor: pointer;
  }

  .file-item-row:hover {
    background-color: rgba(59, 130, 246, 0.15);
  }

  .file-item-row.selected {
    background-color: rgba(59, 130, 246, 0.25);
    border-left: 2px solid var(--accent-blue);
  }

  .file-path-text {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    color: var(--text-main);
  }

  .file-item-actions {
    display: flex;
    gap: 4px;
  }

  .btn-icon-audit, .btn-icon-delete {
    background: none;
    border: none;
    cursor: pointer;
    padding: 0 4px;
    font-size: 0.75rem;
    color: var(--text-muted);
  }

  .btn-icon-delete:hover {
    color: var(--accent-danger);
  }

  .btn-icon-audit:hover {
    color: var(--accent-blue);
  }

  .btn {
    border: none;
    border-radius: 5px;
    padding: 5px 12px;
    font-size: 0.78rem;
    font-weight: 600;
    cursor: pointer;
    transition: filter 0.2s, background-color 0.2s;
  }

  .btn-sm {
    padding: 3px 8px;
    font-size: 0.73rem;
  }

  .btn-secondary {
    background-color: var(--bg-surface);
    border: 1px solid var(--border-color);
    color: var(--text-main);
  }

  .btn-danger-outline {
    background-color: transparent;
    border: 1px solid var(--accent-danger);
    color: var(--accent-danger);
  }

  .btn-info-outline {
    background-color: rgba(16, 185, 129, 0.1);
    border: 1px solid var(--accent-emerald);
    color: var(--accent-emerald);
    transition: background-color 0.2s;
  }

  .btn-info-outline:hover {
    background-color: rgba(16, 185, 129, 0.25);
  }
</style>

