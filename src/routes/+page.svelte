<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { open as openFileDialog } from "@tauri-apps/plugin-dialog";
  import { onMount } from "svelte";

  import type { OptimizeConfig, PdfAuditComparison, LogItem } from "$lib/types";
  import { translations } from "$lib/utils/i18n";

  import FileDropBar from "$lib/components/FileDropBar.svelte";
  import EasyModeTab from "$lib/components/EasyModeTab.svelte";
  import DetailModeTab from "$lib/components/DetailModeTab.svelte";
  import ProfileTab from "$lib/components/ProfileTab.svelte";
  import StatusFooter from "$lib/components/StatusFooter.svelte";
  import AuditModal from "$lib/components/AuditModal.svelte";

  // State Management (Svelte 5 Runes)
  let files = $state<string[]>([]);
  let selectedFileIdx = $state<number>(-1);
  let activeTab = $state<"easy" | "detail" | "profile">("easy");
  let isDragging = $state<boolean>(false);

  // Language state
  let currentLang = $state<"ja" | "en">("en");
  const t = $derived(translations[currentLang]);

  // Presets and profile state
  let selectedPreset = $state<string>("standard");
  let loadedProfile = $state<string>("");
  let savedProfiles = $state<string[]>([]);
  let newProfileName = $state<string>("");

  // Optimization Config
  let config = $state<OptimizeConfig>({
    version: 2,
    color_conv: "original",
    color_img: { downsample: "bicubic", target_dpi: 150, threshold_dpi: 225, format: "jpeg", quality: 70, mono_method: null },
    gray_img: { downsample: "bicubic", target_dpi: 150, threshold_dpi: 225, format: "jpeg", quality: 70, mono_method: null },
    mono_img: { downsample: "bicubic", target_dpi: 300, threshold_dpi: 450, format: "ccitt", quality: 0, mono_method: "threshold" },
    discard_comments: false,
    discard_links: false,
    discard_bookmarks: false,
    discard_forms: false,
    discard_metadata: false,
    discard_js: false,
    discard_thumbs: false,
    clean_linearize: true,
    clean_deflate: true,
    clean_sanitize: false,
    font_subset: true,
    unembed_fonts: [],
    force_rasterize: false
  });

  // Fonts & Processing
  let detectedFonts = $state<string[]>([]);
  let isProcessing = $state<boolean>(false);
  let progressPercent = $state<number>(0);
  let progressText = $state<string>("Ready");
  let logs = $state<string[]>([]);
  let logItems = $state<LogItem[]>([]);

  // Audit Modal State
  let isAuditModalOpen = $state<boolean>(false);
  let currentAuditData = $state<PdfAuditComparison | null>(null);
  let isAuditLoading = $state<boolean>(false);
  let auditErrorMessage = $state<string>("");

  onMount(() => {
    fetchProfiles();

    const unlistenDragDrop = listen<any>("tauri://drag-drop", (event) => {
      handleTauriDropPayload(event.payload);
      isDragging = false;
    });

    const unlistenDrop = listen<any>("tauri://drop", (event) => {
      handleTauriDropPayload(event.payload);
      isDragging = false;
    });

    const unlistenDragOver = listen<any>("tauri://drag-over", () => {
      isDragging = true;
    });

    const unlistenDragLeave = listen<any>("tauri://drag-leave", () => {
      isDragging = false;
    });

    const unlistenProgress = listen<{ file_path: string; current: number; total: number; message: string }>(
      "optimization-progress",
      (event) => {
        const payload = event.payload;
        if (payload.total > 0) {
          progressPercent = Math.round((payload.current / payload.total) * 100);
        }
        progressText = payload.message;
      }
    );

    const unlistenLog = listen<{ file_path: string; success: boolean; orig_size: number; opt_size: number; message: string; audit?: PdfAuditComparison }>(
      "optimization-log",
      (event) => {
        const payload = event.payload;
        logs = [payload.message, ...logs];
        logItems = [payload, ...logItems];
      }
    );

    return () => {
      unlistenDragDrop.then((fn) => fn());
      unlistenDrop.then((fn) => fn());
      unlistenDragOver.then((fn) => fn());
      unlistenDragLeave.then((fn) => fn());
      unlistenProgress.then((fn) => fn());
      unlistenLog.then((fn) => fn());
    };
  });

  function handleTauriDropPayload(payload: any) {
    if (!payload) return;
    let paths: string[] = [];
    if (Array.isArray(payload)) {
      paths = payload;
    } else if (payload.paths && Array.isArray(payload.paths)) {
      paths = payload.paths;
    } else if (typeof payload === 'string') {
      paths = [payload];
    }

    const pdfPaths = paths.filter(p => typeof p === 'string' && p.toLowerCase().endsWith('.pdf'));
    if (pdfPaths.length > 0) {
      addPdfFiles(pdfPaths);
    }
  }

  function addPdfFiles(newPaths: string[]) {
    const unique = newPaths.filter(p => !files.includes(p));
    if (unique.length > 0) {
      files = [...files, ...unique];
      if (selectedFileIdx === -1) {
        selectedFileIdx = 0;
      }
      scanFontsForFiles();
    }
  }

  function handleHtmlDrop(e: DragEvent) {
    e.preventDefault();
    isDragging = false;
    if (e.dataTransfer && e.dataTransfer.files) {
      const droppedFiles: string[] = [];
      for (let i = 0; i < e.dataTransfer.files.length; i++) {
        const file = e.dataTransfer.files[i];
        // @ts-ignore
        const path = file.path || file.name;
        if (path.toLowerCase().endsWith('.pdf')) {
          droppedFiles.push(path);
        }
      }
      if (droppedFiles.length > 0) {
        addPdfFiles(droppedFiles);
      }
    }
  }

  async function fetchProfiles() {
    try {
      const res = await invoke<Record<string, OptimizeConfig>>("load_profiles_cmd");
      savedProfiles = Object.keys(res);
    } catch (e) {
      console.error("Failed to load profiles:", e);
    }
  }

  async function selectPreset(preset: string) {
    selectedPreset = preset;
    loadedProfile = "";
    try {
      const pConfig = await invoke<OptimizeConfig>("get_preset_config_cmd", { preset });
      config = pConfig;
    } catch (e) {
      console.error("Failed to fetch preset config:", e);
    }
  }

  async function loadProfile(name: string) {
    if (!name) return;
    try {
      const res = await invoke<Record<string, OptimizeConfig>>("load_profiles_cmd");
      if (res[name]) {
        const profileData = res[name];
        if (profileData.version !== 2) {
          alert(`Error: Profile "${name}" is from an older version (v1) and cannot be loaded.`);
          loadedProfile = "";
          selectPreset("standard");
          return;
        }
        config = profileData;
        loadedProfile = name;
        selectedPreset = "user_profile";
      }
    } catch (e) {
      console.error("Failed to load profile:", e);
    }
  }

  async function handleSaveProfile() {
    if (!newProfileName) return;
    try {
      await invoke("save_profile_cmd", {
        name: newProfileName,
        config: $state.snapshot(config)
      });
      logs = [`${t.profileSavedLog}: ${newProfileName}`, ...logs];
      const name = newProfileName;
      newProfileName = "";
      await fetchProfiles();
      loadedProfile = name;
      selectedPreset = "user_profile";
    } catch (e) {
      alert(`Save failed: ${e}`);
    }
  }

  async function handleDeleteProfile(name: string) {
    if (!confirm(`Delete profile "${name}"?`)) return;
    try {
      await invoke("delete_profile_cmd", { name });
      logs = [`${t.profileDeletedLog}: ${name}`, ...logs];
      if (loadedProfile === name) {
        loadedProfile = "";
        selectPreset("standard");
      }
      await fetchProfiles();
    } catch (e) {
      alert(`Delete failed: ${e}`);
    }
  }

  async function handleReset() {
    selectPreset("standard");
    logs = [t.settingsResetLog, ...logs];
  }

  async function selectPdfFiles() {
    try {
      const selected = await openFileDialog({
        multiple: true,
        filters: [{ name: 'PDF Document', extensions: ['pdf'] }],
        title: 'Select PDF Files'
      });
      if (selected) {
        const paths = Array.isArray(selected) ? selected : [selected];
        addPdfFiles(paths);
      }
    } catch (err) {
      console.error('File dialog error:', err);
    }
  }

  function clearFiles() {
    files = [];
    selectedFileIdx = -1;
    detectedFonts = [];
  }

  function removeFileByIndex(index: number) {
    files = files.filter((_, i) => i !== index);
    if (selectedFileIdx >= files.length) {
      selectedFileIdx = files.length - 1;
    }
    scanFontsForFiles();
  }

  async function scanFontsForFiles() {
    if (files.length === 0) {
      detectedFonts = [];
      return;
    }
    try {
      const fontList = await invoke<string[]>("scan_fonts_cmd", {
        inputPaths: $state.snapshot(files)
      });
      detectedFonts = fontList;
    } catch (e) {
      console.error("Failed to scan fonts:", e);
    }
  }

  function toggleFontUnembed(font: string) {
    if (config.unembed_fonts.includes(font)) {
      config.unembed_fonts = config.unembed_fonts.filter(f => f !== font);
    } else {
      config.unembed_fonts = [...config.unembed_fonts, font];
    }
  }

  async function openAuditModal(filePath: string, existingAudit?: PdfAuditComparison) {
    let targetPath = filePath;
    if (!targetPath) {
      if (files.length > 0) {
        targetPath = files[0];
      } else {
        alert(t.selectPdfAlert);
        return;
      }
    }
    isAuditModalOpen = true;
    auditErrorMessage = "";
    if (existingAudit) {
      currentAuditData = existingAudit;
      isAuditLoading = false;
      return;
    }
    isAuditLoading = true;
    currentAuditData = null;
    try {
      const res = await invoke<PdfAuditComparison>("audit_pdf_cmd", { path: targetPath });
      currentAuditData = res;
    } catch (e: any) {
      console.error("Failed to audit PDF:", e);
      auditErrorMessage = typeof e === "string" ? e : (e.message || "Failed to audit PDF");
    } finally {
      isAuditLoading = false;
    }
  }

  function closeAuditModal() {
    isAuditModalOpen = false;
  }

  async function startOptimize() {
    if (files.length === 0) {
      alert(t.selectPdfAlert);
      return;
    }

    isProcessing = true;
    progressPercent = 0;
    progressText = t.startingOptimization;

    const strategy = config.force_rasterize ? "rasterize" : "text";

    try {
      await invoke("optimize_pdf_cmd", {
        inputPaths: $state.snapshot(files),
        config: $state.snapshot(config),
        strategy
      });
      progressPercent = 100;
      progressText = t.done;
    } catch (e) {
      logs = [`Error: ${e}`, ...logs];
      progressText = t.failed;
    } finally {
      isProcessing = false;
    }
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<main 
  class="app-layout" 
  class:dragging={isDragging}
  ondragover={(e) => { e.preventDefault(); isDragging = true; }}
  ondragleave={(e) => { e.preventDefault(); isDragging = false; }}
  ondrop={handleHtmlDrop}
>
  <!-- Top Section: Compact PDF Selection & Drop Area -->
  <FileDropBar
    {files}
    {selectedFileIdx}
    {currentLang}
    onSelectFiles={selectPdfFiles}
    onClearAll={clearFiles}
    onRemoveSelected={removeFileByIndex}
    onSelectIndex={(idx) => { selectedFileIdx = idx; }}
    onOpenAudit={(p) => openAuditModal(p)}
  />

  <!-- Middle Section: Mode Navigation & Active Tab Settings -->
  <section class="main-panel">
    <header class="nav-bar">
      <nav class="tab-group">
        <button class="tab-btn" class:active={activeTab === 'easy'} onclick={() => activeTab = 'easy'}>{t.easyMode}</button>
        <button class="tab-btn" class:active={activeTab === 'detail'} onclick={() => activeTab = 'detail'}>{t.detailMode}</button>
        <button class="tab-btn" class:active={activeTab === 'profile'} onclick={() => activeTab = 'profile'}>{t.profileMode}</button>
      </nav>
      <select class="lang-select-box" bind:value={currentLang}>
        <option value="en">English (EN)</option>
        <option value="ja">日本語 (JA)</option>
      </select>
    </header>

    <div class="content-body">
      {#if activeTab === 'easy'}
        <EasyModeTab
          bind:selectedPreset
          bind:loadedProfile
          {savedProfiles}
          {currentLang}
          onSelectPreset={selectPreset}
          onLoadProfile={loadProfile}
        />
      {:else if activeTab === 'detail'}
        <DetailModeTab
          bind:config
          {detectedFonts}
          {currentLang}
          onToggleFontUnembed={toggleFontUnembed}
        />
      {:else if activeTab === 'profile'}
        <ProfileTab
          {savedProfiles}
          bind:newProfileName
          {currentLang}
          onSaveProfile={handleSaveProfile}
          onDeleteProfile={handleDeleteProfile}
        />
      {/if}
    </div>

    <!-- Status Footer & Action Controls -->
    <StatusFooter
      {progressText}
      {progressPercent}
      {isProcessing}
      filesCount={files.length}
      {logs}
      {logItems}
      {currentLang}
      onReset={handleReset}
      onStartOptimize={startOptimize}
      onOpenAudit={(path, audit) => openAuditModal(path, audit)}
    />
  </section>
</main>

<!-- Audit Modal -->
<AuditModal
  isOpen={isAuditModalOpen}
  auditData={currentAuditData}
  isLoading={isAuditLoading}
  errorMessage={auditErrorMessage}
  {currentLang}
  onClose={closeAuditModal}
/>

<style>
  :root {
    --bg-base: #0f172a;
    --bg-surface: #1e293b;
    --bg-input: #334155;
    --border-color: #334155;
    --border-highlight: #475569;
    --text-main: #f8fafc;
    --text-muted: #94a3b8;
    --accent-blue: #3b82f6;
    --accent-blue-hover: #60a5fa;
    --accent-emerald: #10b981;
    --accent-danger: #ef4444;
    
    color: var(--text-main);
    background-color: var(--bg-base);
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
  }

  :global(body) {
    margin: 0;
    padding: 0;
    overflow: hidden;
    background-color: var(--bg-base);
    user-select: none;
  }

  :global(input::-webkit-outer-spin-button),
  :global(input::-webkit-inner-spin-button) {
    -webkit-appearance: none !important;
    margin: 0 !important;
  }
  :global(input[type=number]) {
    -moz-appearance: textfield !important;
    appearance: textfield !important;
  }

  :global(select), select {
    appearance: none !important;
    -webkit-appearance: none !important;
    -moz-appearance: none !important;
    background-color: #1e293b !important;
    color: #f8fafc !important;
    border: 1px solid var(--border-color);
  }

  :global(select option), select option, option, :global(option) {
    background-color: #1e293b !important;
    color: #f8fafc !important;
  }

  .app-layout {
    display: flex;
    flex-direction: column;
    width: 100vw;
    height: 100vh;
    min-width: 680px;
    min-height: 520px;
    box-sizing: border-box;
    font-size: 0.82rem;
    background-color: var(--bg-base);
    overflow: hidden;
    transition: outline 0.2s;
  }

  .app-layout.dragging {
    outline: 2px dashed var(--accent-blue);
    outline-offset: -4px;
  }

  .main-panel {
    flex: 1;
    display: flex;
    flex-direction: column;
    padding: 8px 12px;
    overflow: hidden;
  }

  .nav-bar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid var(--border-color);
    padding-bottom: 4px;
    margin-bottom: 8px;
  }

  .tab-group {
    display: flex;
    gap: 4px;
  }

  .tab-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    padding: 4px 10px;
    font-size: 0.83rem;
    font-weight: 600;
    cursor: pointer;
    border-bottom: 2px solid transparent;
    transition: color 0.2s, border-color 0.2s;
  }

  .tab-btn:hover {
    color: var(--text-main);
  }

  .tab-btn.active {
    color: var(--accent-blue);
    border-bottom: 2px solid var(--accent-blue);
  }

  .lang-select-box {
    appearance: none;
    -webkit-appearance: none;
    -moz-appearance: none;
    background-color: #1e293b !important;
    border: 1px solid var(--border-color);
    color: #f8fafc !important;
    font-size: 0.73rem;
    padding: 2px 20px 2px 8px;
    border-radius: 4px;
    cursor: pointer;
    font-weight: 600;
    outline: none;
    transition: border-color 0.2s;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='10' height='10' viewBox='0 0 24 24' fill='none' stroke='%2394a3b8' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpolyline points='6 9 12 15 18 9'%3E%3C/polyline%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: right 6px center;
  }

  .lang-select-box:hover {
    border-color: var(--accent-blue);
  }

  .content-body {
    flex: 1;
    overflow-y: auto;
    margin-bottom: 4px;
  }
</style>
