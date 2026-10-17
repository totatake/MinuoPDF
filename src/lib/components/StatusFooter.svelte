<script lang="ts">
  import type { LogItem } from "../types";
  import { translations } from "../utils/i18n";

  let {
    progressText = "Ready",
    progressPercent = 0,
    isProcessing = false,
    filesCount = 0,
    logs = [],
    logItems = [],
    currentLang = "en",
    onReset,
    onStartOptimize,
    onCancel,
    onOpenAudit,
  }: {
    progressText: string;
    progressPercent: number;
    isProcessing: boolean;
    filesCount: number;
    logs: string[];
    logItems: LogItem[];
    currentLang: "ja" | "en";
    onReset: () => void;
    onStartOptimize: () => void;
    onCancel?: () => void;
    onOpenAudit: (filePath: string, audit?: any) => void;
  } = $props();

  const t = $derived(translations[currentLang]);
</script>

<div class="status-footer">
  <div class="progress-info">
    <span class="status-msg">{progressText}</span>
    <span class="progress-val">{progressPercent}%</span>
  </div>
  <div class="progress-track">
    <div class="progress-fill" style="width: {progressPercent}%;"></div>
  </div>

  <div class="compact-log-console">
    {#if logItems.length === 0 && logs.length === 0}
      <span class="log-placeholder">{t.logPlaceholder}</span>
    {:else if logItems.length > 0}
      {#each logItems as log}
        <div class="log-entry-row">
          <span class={log.success ? "log-success" : "log-fail"}>{log.message}</span>
          {#if log.success && log.file_path}
            <button
              type="button"
              class="btn-log-audit"
              onclick={() => onOpenAudit(log.file_path, log.audit)}
            >
              📊 {t.auditBtn}
            </button>
          {/if}
        </div>
      {/each}
    {:else}
      {#each logs as log}
        <div class="log-entry">{log}</div>
      {/each}
    {/if}
  </div>
</div>

<!-- Action Bar -->
<footer class="bottom-actions">
  <div class="left-actions">
    <button class="btn btn-sm btn-muted" onclick={onReset} disabled={isProcessing}>{t.resetBtn}</button>
  </div>
  <div class="right-actions">
    {#if isProcessing && onCancel}
      <button class="btn btn-sm btn-danger-outline" onclick={onCancel} style="margin-right: 6px;">
        {t.cancelBtn}
      </button>
    {/if}
    <button class="btn btn-primary" onclick={onStartOptimize} disabled={isProcessing || filesCount === 0}>
      {isProcessing ? t.compressingBtn : t.compressAllBtn}
    </button>
  </div>
</footer>

<style>
  .status-footer {
    display: flex;
    flex-direction: column;
    gap: 3px;
    margin-bottom: 6px;
  }

  .progress-info {
    display: flex;
    justify-content: space-between;
    font-size: 0.73rem;
  }

  .status-msg {
    color: var(--text-muted);
  }

  .progress-val {
    color: var(--accent-blue);
    font-weight: bold;
  }

  .progress-track {
    height: 4px;
    background-color: var(--bg-surface);
    border-radius: 2px;
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background-color: var(--accent-blue);
    transition: width 0.3s;
  }

  .compact-log-console {
    height: 34px;
    background-color: #040711;
    border: 1px solid var(--border-color);
    border-radius: 4px;
    overflow-y: auto;
    font-family: monospace;
    font-size: 0.71rem;
    padding: 3px 6px;
  }

  .log-placeholder {
    color: #475569;
  }

  .log-entry {
    color: var(--accent-emerald);
    line-height: 1.35;
  }

  .log-entry-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    line-height: 1.35;
    padding: 1px 0;
  }

  .log-success {
    color: var(--accent-emerald);
  }

  .log-fail {
    color: var(--accent-danger);
  }

  .btn-log-audit {
    background: rgba(59, 130, 246, 0.15);
    border: 1px solid var(--accent-blue);
    color: var(--accent-blue-hover);
    font-size: 0.65rem;
    padding: 0 5px;
    border-radius: 3px;
    cursor: pointer;
    margin-left: 6px;
    white-space: nowrap;
  }

  .btn-log-audit:hover {
    background: rgba(59, 130, 246, 0.3);
  }

  .bottom-actions {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .right-actions {
    display: flex;
    align-items: center;
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

  .btn-primary {
    background-color: var(--accent-blue);
    color: #ffffff;
  }

  .btn-primary:hover {
    background-color: var(--accent-blue-hover);
  }

  .btn-muted {
    background-color: var(--bg-surface);
    border: 1px solid var(--border-color);
    color: var(--text-muted);
  }

  .btn-danger-outline {
    background-color: transparent;
    border: 1px solid var(--accent-danger);
    color: var(--accent-danger);
  }

  .btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }
</style>

