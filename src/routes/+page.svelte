<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { open as openFileDialog } from "@tauri-apps/plugin-dialog";
  import { onMount } from "svelte";

  // Audit Interfaces
  interface AuditCategory {
    key: string;
    name_ja: string;
    name_en: string;
    count: number;
    bytes: number;
    percentage: number;
  }

  interface PdfAuditReport {
    file_name: string;
    total_bytes: number;
    categories: AuditCategory[];
  }

  interface PdfAuditComparison {
    original: PdfAuditReport;
    optimized: PdfAuditReport | null;
  }

  interface LogItem {
    file_path: string;
    success: boolean;
    orig_size: number;
    opt_size: number;
    message: string;
    audit?: PdfAuditComparison;
  }

  // Category Theme Colors for Bar Chart
  const categoryColors: Record<string, string> = {
    images: "#10b981",          // Emerald green
    content_streams: "#3b82f6", // Blue
    fonts: "#8b5cf6",           // Purple
    annotations: "#f59e0b",     // Amber
    forms: "#ec4899",           // Pink
    metadata: "#06b6d4",        // Cyan
    overhead: "#64748b",        // Slate gray
  };

  // Svelte 5 states
  let files = $state<string[]>([]);
  let selectedFileIdx = $state<number>(-1);
  let activeTab = $state<"easy" | "detail" | "profile">("easy");
  let detailSubTab = $state<"image" | "discard" | "font">("image");
  let isDragging = $state<boolean>(false);

  // Language state
  let currentLang = $state<"ja" | "en">("en");

  // Presets and configs
  let selectedPreset = $state<string>("standard");
  let loadedProfile = $state<string>("");
  let savedProfiles = $state<string[]>([]);
  let newProfileName = $state<string>("");

  // Target config (matching Rust struct OptimizeConfig)
  let config = $state({
    version: 2,
    color_conv: "original",
    color_img: { downsample: "bicubic", target_dpi: 150, threshold_dpi: 225, format: "jpeg", quality: 70, mono_method: null as string | null },
    gray_img: { downsample: "bicubic", target_dpi: 150, threshold_dpi: 225, format: "jpeg", quality: 70, mono_method: null as string | null },
    mono_img: { downsample: "bicubic", target_dpi: 300, threshold_dpi: 450, format: "ccitt", quality: 0, mono_method: "threshold" as string | null },
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
    unembed_fonts: [] as string[],
    force_rasterize: false
  });

  // Font lists
  let detectedFonts = $state<string[]>([]);

  // Processing state
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

  // Multilingual translations
  const t = {
    ja: {
      targetFiles: "対象ファイル",
      dropZone: "PDFファイルをドラッグ＆ドロップ、またはクリックして追加",
      addFiles: "+ PDF追加",
      removeSelected: "選択削除",
      clearAll: "すべてクリア",
      easyMode: "簡単モード",
      detailMode: "詳細モード",
      profileMode: "プロファイル管理",
      presets: "圧縮プリセット",
      presetHigh: "高画質 (印刷用下書き)",
      presetStandard: "標準 (電子書籍・閲覧向け)",
      presetLow: "軽量化 (Web表示・メール送信用)",
      presetMono: "白黒化 (スキャン文書・FAX向け)",
      presetRaster: "強制画像化 (完全ラスタライズ)",
      forceRasterize: "ページ全体を強制画像化 (完全ラスタライズ)",
      userProfile: "ユーザプロファイル",
      loadProfile: "プロファイルをロード:",
      selectOption: "選択してください...",
      presetExplanation: "プリセットの説明",
      customExplanation: "カスタムプロファイルがロードされています。現在の詳細設定が有効です。",
      imageSettings: "画像最適化",
      discardSettings: "破棄&クリーンアップ",
      fontSettings: "フォント",
      colorConvPolicy: "カラー変換ポリシー:",
      colorConvOriginal: "元の色を保持",
      colorConvGray: "すべてグレースケール化",
      colorConvMono: "すべて白黒2値化",
      colorImage: "カラー画像",
      grayImage: "グレースケール画像",
      monoImage: "モノクロ二値画像",
      resolution: "解像度 (DPI)",
      threshold: "しきい値DPI",
      quality: "品質 (JPEG Q)",
      binarization: "二値化方式",
      binOtsu: "閾値方式 (Otsu)",
      binFloyd: "ディザリング (FS)",
      discardGroup: "破棄設定",
      discardComments: "注釈とコメントを破棄",
      discardLinks: "リンクを破棄",
      discardBookmarks: "しおりを破棄",
      discardForms: "フォームを破棄",
      discardMetadata: "XMLメタデータを破棄",
      discardJs: "JavaScriptを破棄",
      discardThumbs: "サムネイルを破棄",
      cleanupGroup: "クリーンアップ",
      cleanLinearize: "Web表示用に最適化 (線形化)",
      cleanDeflate: "コンテンツを再圧縮 (Deflate)",
      cleanSanitize: "CropBox外データの物理消去",
      fontSubset: "すべての埋め込みフォントをサブセット化",
      fontUnembedTitle: "埋め込み解除 (Unembed) フォント設定",
      fontUnembedDesc: "ダブルクリックして、埋め込みを解除するフォントを指定。",
      maintainEmbed: "埋め込み維持",
      targetUnembed: "解除対象",
      saveCurrentProfile: "現在の設定を保存",
      profileNamePlaceholder: "プロファイル名を入力...",
      saveBtn: "プロファイルを保存",
      savedProfilesTitle: "保存済みプロファイル",
      noSavedProfiles: "保存されたカスタムプロファイルはありません。",
      deleteBtn: "削除",
      resetBtn: "初期値に戻す",
      compressAllBtn: "一括最適化を開始",
      compressingBtn: "最適化中...",
      startingOptimization: "最適化を開始しています...",
      logPlaceholder: "最適化を開始するとログが表示されます...",
      selectPdfAlert: "最適化するPDFファイルを追加してください。",
      profileSavedLog: "プロファイルを正常に保存しました。",
      profileDeletedLog: "プロファイルを削除しました。",
      settingsResetLog: "設定を初期値にリセットしました。",
      done: "完了しました",
      failed: "失敗しました",
      tooltipColorConv: "画像の色変換ポリシー。白黒化（Mono）は白と黒の2色のみに落とし込むことでサイズを極限まで縮小します。",
      tooltipDpi: "画像の目標解像度(DPI)。手入力の他、50/72/100/150/200/300/450のリストからも選べます。",
      tooltipThreshold: "画像の元のDPIがこの値を超える場合にリサイズ処理を適用します。リスト（75/100/150/225/300/450）選択と手入力に対応。",
      tooltipQuality: "JPEGの圧縮品質。35(高圧縮), 50(軽量), 65(標準), 75(高画質)等から選択・手入力できます。",
      tooltipMonoMethod: "モノクロ変換方式。Otsu（しきい値法）は文字認識（OCR）などに最適。FS（ディザリング）は写真などの階調を疑似表現します。",
      tooltipDiscardComments: "ハイライトやメモ、注釈オブジェクトをPDFから完全に除去します。",
      tooltipDiscardLinks: "ページ内のハイパーリンクURL情報を削除し、ただのテキストにします。",
      tooltipDiscardBookmarks: "目次ツリー（アウトライン）をPDFから完全に削除します。",
      tooltipDiscardForms: "インタラクティブな入力フォームやチェックボックスをただの表示テキストにフラット化し、フォームオブジェクトを破棄します。",
      tooltipDiscardMetadata: "作成者、作成ソフト、作成日時などの不要なXMLメタデータ（XMPデータ）を削除します。",
      tooltipDiscardJs: "セキュリティ上の懸念や動作軽量化のため、PDF内に記述されたJavaScriptやアクションを消去します。",
      tooltipDiscardThumbs: "Acrobat等で表示されるページの縮小サムネイル画像を消去します。（通常は再生成されるため安全です）",
      tooltipCleanLinearize: "Web閲覧時に、全体をダウンロードする前に最初の数ページを即座に表示できるようPDFの構造を再整列します。",
      tooltipCleanDeflate: "PDF内のすべてのストリームデータに対してDeflate圧縮を再適用し、隠れた無駄データを圧縮します。",
      tooltipCleanSanitize: "ページの表示枠（CropBox）の外側にある不要な画像データや余白データを物理的に消去して容量を削ります。",
      tooltipFontSubset: "PDF内で実際に使用されている文字のフォントデータのみを抽出し、フォントファイルのサイズを劇的に軽量化して埋め込みます。",
      tooltipForceRasterize: "すべてのページを画像としてレンダリングします。文字選択はできなくなりますが容量削減に有効です。",
      auditTitle: "容量の調査 (PDF Audit)",
      auditBtn: "容量調査",
      auditLoading: "PDFのオブジェクト構造を解析中...",
      auditBefore: "最適化前 (Original)",
      auditAfter: "最適化後 (Optimized)",
      auditDiff: "差分 / 削減量",
      auditColCategory: "PDF オブジェクト種別",
      auditColCount: "カウント",
      auditColBytes: "バイト数",
      auditColPercent: "割合 (%)",
      auditColDiff: "削減量",
      auditTotal: "トータル",
      auditNoOptYet: "（最適化実行後に前後比較データが表示されます）",
      auditClose: "閉じる",
    },
    en: {
      targetFiles: "Target Files",
      dropZone: "Drag & drop PDF files here, or click to select",
      addFiles: "+ Add PDF",
      removeSelected: "Remove",
      clearAll: "Clear All",
      easyMode: "Easy Mode",
      detailMode: "Detail Mode",
      profileMode: "Profile Management",
      presets: "Compression Presets",
      presetHigh: "High Quality (Print draft)",
      presetStandard: "Standard (E-book / Reading)",
      presetLow: "Low Quality (Web / Email)",
      presetMono: "Monochrome (Scanned doc / FAX)",
      presetRaster: "Force Rasterize (Full Page Image)",
      forceRasterize: "Force rasterize all pages to images",
      userProfile: "User Profile",
      loadProfile: "Load Profile:",
      selectOption: "Select an option...",
      presetExplanation: "Preset Explanation",
      customExplanation: "Custom profile loaded. Current detail settings apply.",
      imageSettings: "Image Optimization",
      discardSettings: "Discard & Clean",
      fontSettings: "Fonts",
      colorConvPolicy: "Color Conversion Policy:",
      colorConvOriginal: "Keep Original Colors",
      colorConvGray: "Convert All to Grayscale",
      colorConvMono: "Convert All to Monochrome",
      colorImage: "Color Image",
      grayImage: "Grayscale Image",
      monoImage: "Monochrome Image",
      resolution: "Resolution (DPI)",
      threshold: "Threshold DPI",
      quality: "Quality (JPEG Q)",
      binarization: "Binarization Method",
      binOtsu: "Thresholding (Otsu)",
      binFloyd: "Dithering (Floyd-Steinberg)",
      discardGroup: "Discard Settings",
      discardComments: "Discard Annotations & Comments",
      discardLinks: "Discard Links",
      discardBookmarks: "Discard Bookmarks",
      discardForms: "Discard Interactive Forms",
      discardMetadata: "Discard XML Metadata",
      discardJs: "Discard JavaScript",
      discardThumbs: "Discard Thumbnails",
      cleanupGroup: "Cleanup Options",
      cleanLinearize: "Optimize for Fast Web View (Linearize)",
      cleanDeflate: "Recompress Streams (Deflate)",
      cleanSanitize: "Sanitize Content Outside CropBox",
      fontSubset: "Subset All Embedded Fonts",
      fontUnembedTitle: "Unembed Font Settings",
      fontUnembedDesc: "Double-click to toggle font unembedding.",
      maintainEmbed: "Maintain Embedded",
      targetUnembed: "Target Unembed",
      saveCurrentProfile: "Save Current Settings",
      profileNamePlaceholder: "Enter profile name...",
      saveBtn: "Save Profile",
      savedProfilesTitle: "Saved Profiles",
      noSavedProfiles: "No saved custom profiles.",
      deleteBtn: "Delete",
      resetBtn: "Reset to Default",
      compressAllBtn: "Start Optimization",
      compressingBtn: "Optimizing...",
      startingOptimization: "Starting optimization...",
      logPlaceholder: "Logs will be displayed here...",
      selectPdfAlert: "Please add PDF files to optimize.",
      profileSavedLog: "Profile saved successfully",
      profileDeletedLog: "Profile deleted",
      settingsResetLog: "Settings reset to defaults",
      done: "Completed successfully",
      failed: "Failed",
      tooltipColorConv: "Policy for image colors. Monochrome reduces sizes drastically.",
      tooltipDpi: "Target image resolution (DPI).",
      tooltipThreshold: "Downsample images only if DPI exceeds threshold.",
      tooltipQuality: "JPEG compression quality Q.",
      tooltipMonoMethod: "Monochrome binarization method.",
      tooltipDiscardComments: "Remove highlights, notes, and annotations.",
      tooltipDiscardLinks: "Remove hyperlinked URL actions.",
      tooltipDiscardBookmarks: "Remove bookmark tree outlines.",
      tooltipDiscardForms: "Flatten interactive form fields into static text.",
      tooltipDiscardMetadata: "Remove XMP metadata.",
      tooltipDiscardJs: "Remove embedded JavaScript.",
      tooltipDiscardThumbs: "Remove page thumbnail images.",
      tooltipCleanLinearize: "Reorder PDF structure for fast web display.",
      tooltipCleanDeflate: "Re-apply Deflate compression to streams.",
      tooltipCleanSanitize: "Physically remove content outside CropBox.",
      tooltipFontSubset: "Subset all embedded fonts to include used glyphs only.",
      tooltipForceRasterize: "Render all pages as high-resolution images.",
      auditTitle: "Audit Space Usage",
      auditBtn: "Audit",
      auditLoading: "Analyzing PDF object structure...",
      auditBefore: "Original",
      auditAfter: "Optimized",
      auditDiff: "Difference",
      auditColCategory: "PDF Object Category",
      auditColCount: "Count",
      auditColBytes: "Bytes",
      auditColPercent: "Percentage (%)",
      auditColDiff: "Reduction",
      auditTotal: "Total",
      auditNoOptYet: "(Optimized breakdown will be available after optimization)",
      auditClose: "Close",
    }
  };

  function formatBytes(bytes: number): string {
    if (bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    const val = (bytes / Math.pow(k, i)).toFixed(2);
    return `${val} ${sizes[i]}`;
  }

  async function openAuditModal(filePath: string, existingAudit?: PdfAuditComparison) {
    let targetPath = filePath;
    if (!targetPath) {
      if (files.length > 0) {
        targetPath = files[0];
      } else {
        alert(t[currentLang].selectPdfAlert);
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

  const presetExplanations: Record<"ja" | "en", Record<string, string>> = {
    ja: {
      high_quality: "画像品質を極力落とさずに軽くします。印刷用の下書きや画質を最優先したい文書に最適です (300 DPI, Quality 85)。",
      standard: "最もバランスの取れた標準設定です。画面閲覧や電子書籍、通常のオフィス文書に推奨されます (150 DPI, Quality 70)。",
      low_quality: "解像度と品質を大幅に下げてサイズ縮小を優先します。メール添付やWeb公開用に適しています (72 DPI, Quality 50)。",
      mono: "すべての画像を白黒2値（1-bit CCITT G4）に変換します。スキャン文書やモノクロFAX文書で劇的な圧縮効果を発揮します。",
      rasterize: "PDFの全ページを画質最適化画像として再生成します。フォントの埋め込みトラブルや破損PDFを確実に軽量化できます。"
    },
    en: {
      high_quality: "Preserves high visual quality while reducing size. Ideal for draft printing (300 DPI, Quality 85).",
      standard: "Best balanced preset for desktop viewing, ebooks, and office documents (150 DPI, Quality 70).",
      low_quality: "Prioritizes size reduction for email attachments and web publishing (72 DPI, Quality 50).",
      mono: "Converts images to 1-bit monochrome (CCITT G4). Provides maximum compression for scanned documents.",
      rasterize: "Renders every page into an image PDF. Resolves font corruption and complex vector layout issues."
    }
  };

  onMount(() => {
    fetchProfiles();
    
    // 1. Tauri Native Drag & Drop Event Listeners
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

    // 2. Optimization Progress & Log Listeners
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

    const unlistenLog = listen<{ file_path: string; success: boolean; orig_size: number; opt_size: number; message: string }>(
      "optimization-log",
      (event) => {
        const payload = event.payload;
        logs = [payload.message, ...logs];
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
      const res = await invoke<Record<string, typeof config>>("load_profiles_cmd");
      savedProfiles = Object.keys(res);
    } catch (e) {
      console.error("Failed to load profiles:", e);
    }
  }

  async function selectPreset(preset: string) {
    selectedPreset = preset;
    loadedProfile = "";
    try {
      const pConfig = await invoke<typeof config>("get_preset_config_cmd", { preset });
      config = pConfig;
    } catch (e) {
      console.error("Failed to fetch preset config:", e);
    }
  }

  async function loadProfile(name: string) {
    if (!name) return;
    try {
      const res = await invoke<Record<string, typeof config>>("load_profiles_cmd");
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
      logs = [`${t[currentLang].profileSavedLog}: ${newProfileName}`, ...logs];
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
      logs = [`${t[currentLang].profileDeletedLog}: ${name}`, ...logs];
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
    logs = [t[currentLang].settingsResetLog, ...logs];
  }

  async function selectPdfFiles() {
    try {
      const selected = await openFileDialog({
        multiple: true,
        filters: [{ name: 'PDF Document', extensions: ['pdf'] }],
        title: 'Select PDF Files'
      });
      if (selected) {
        // selected is string[] (multiple) or string (single) or null
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

  async function startOptimize() {
    if (files.length === 0) {
      alert(t[currentLang].selectPdfAlert);
      return;
    }

    isProcessing = true;
    progressPercent = 0;
    progressText = t[currentLang].startingOptimization;

    const strategy = config.force_rasterize ? "rasterize" : "text";

    try {
      await invoke("optimize_pdf_cmd", {
        inputPaths: $state.snapshot(files),
        config: $state.snapshot(config),
        strategy
      });
      progressPercent = 100;
      progressText = t[currentLang].done;
    } catch (e) {
      logs = [`Error: ${e}`, ...logs];
      progressText = t[currentLang].failed;
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
  <section class="top-drop-bar">
    {#if files.length === 0}
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div class="drop-zone-slim" onclick={selectPdfFiles} style="cursor: pointer;">
        <span class="icon">📄</span>
        <span class="drop-text">{t[currentLang].dropZone}</span>
      </div>
    {:else}
      <div class="file-summary-bar">
        <div class="file-info-badge">
          <span class="icon">📄</span>
          <span class="file-name-main">{files[0].split(/[\\/]/).pop()}</span>
          {#if files.length > 1}
            <span class="badge">+{files.length - 1}</span>
          {/if}
        </div>
        <div class="top-actions">
          {#if files.length > 0}
            <button
              type="button"
              class="btn btn-sm btn-info-outline"
              onclick={(e) => {
                e.stopPropagation();
                const path = files[selectedFileIdx >= 0 && selectedFileIdx < files.length ? selectedFileIdx : 0];
                openAuditModal(path);
              }}
            >
              📊 {t[currentLang].auditBtn}
            </button>
          {/if}
          <button class="btn btn-sm btn-secondary" onclick={selectPdfFiles}>{t[currentLang].addFiles}</button>
          <button class="btn btn-sm btn-danger-outline" onclick={clearFiles}>{t[currentLang].clearAll}</button>
        </div>
      </div>
    {/if}
  </section>

  <!-- Middle Section: Mode Navigation & Active Tab Settings -->
  <section class="main-panel">
    <header class="nav-bar">
      <nav class="tab-group">
        <button class="tab-btn" class:active={activeTab === 'easy'} onclick={() => activeTab = 'easy'}>{t[currentLang].easyMode}</button>
        <button class="tab-btn" class:active={activeTab === 'detail'} onclick={() => activeTab = 'detail'}>{t[currentLang].detailMode}</button>
        <button class="tab-btn" class:active={activeTab === 'profile'} onclick={() => activeTab = 'profile'}>{t[currentLang].profileMode}</button>
      </nav>
      <select class="lang-select-box" bind:value={currentLang}>
        <option value="en">English (EN)</option>
        <option value="ja">日本語 (JA)</option>
      </select>
    </header>

    <div class="content-body">
      <!-- Easy Mode -->
      {#if activeTab === 'easy'}
        <div class="card-panel easy-panel">
          <h3 class="panel-title">{t[currentLang].presets}</h3>
          <div class="preset-grid">
            <label class="radio-card" class:active={selectedPreset === 'high_quality'}>
              <input type="radio" name="preset" checked={selectedPreset === 'high_quality'} onchange={() => selectPreset('high_quality')} />
              <span>{t[currentLang].presetHigh}</span>
            </label>
            <label class="radio-card" class:active={selectedPreset === 'standard'}>
              <input type="radio" name="preset" checked={selectedPreset === 'standard'} onchange={() => selectPreset('standard')} />
              <span>{t[currentLang].presetStandard}</span>
            </label>
            <label class="radio-card" class:active={selectedPreset === 'low_quality'}>
              <input type="radio" name="preset" checked={selectedPreset === 'low_quality'} onchange={() => selectPreset('low_quality')} />
              <span>{t[currentLang].presetLow}</span>
            </label>
            <label class="radio-card" class:active={selectedPreset === 'mono'}>
              <input type="radio" name="preset" checked={selectedPreset === 'mono'} onchange={() => selectPreset('mono')} />
              <span>{t[currentLang].presetMono}</span>
            </label>
            <label class="radio-card" class:active={selectedPreset === 'rasterize'}>
              <input type="radio" name="preset" checked={selectedPreset === 'rasterize'} onchange={() => selectPreset('rasterize')} />
              <span>{t[currentLang].presetRaster}</span>
            </label>
            <div class="profile-inline-box">
              <label class="radio-card" class:active={selectedPreset === 'user_profile'}>
                <input
                  type="radio"
                  name="preset"
                  checked={selectedPreset === 'user_profile'}
                  disabled={savedProfiles.length === 0}
                  onchange={() => {
                    if (loadedProfile) {
                      loadProfile(loadedProfile);
                    } else if (savedProfiles.length > 0) {
                      loadProfile(savedProfiles[0]);
                    }
                  }}
                />
                <span>{t[currentLang].userProfile}</span>
              </label>
              <select
                class="select-box slim-select"
                value={loadedProfile}
                onchange={(e) => loadProfile(e.currentTarget.value)}
                disabled={savedProfiles.length === 0}
              >
                <option value="">{t[currentLang].selectOption}</option>
                {#each savedProfiles as p}
                  <option value={p}>{p}</option>
                {/each}
              </select>
            </div>
          </div>
          <div class="preset-info-box">
            <span class="info-label">{t[currentLang].presetExplanation}:</span>
            <span class="info-text">
              {selectedPreset === 'user_profile'
                ? t[currentLang].customExplanation
                : presetExplanations[currentLang][selectedPreset] || ''}
            </span>
          </div>
        </div>
      {/if}

      <!-- Detail Mode -->
      {#if activeTab === 'detail'}
        <div class="detail-panel">
          <div class="sub-nav-bar">
            <button class="sub-nav-btn" class:active={detailSubTab === 'image'} onclick={() => detailSubTab = 'image'}>{t[currentLang].imageSettings}</button>
            <button class="sub-nav-btn" class:active={detailSubTab === 'discard'} onclick={() => detailSubTab = 'discard'}>{t[currentLang].discardSettings}</button>
            <button class="sub-nav-btn" class:active={detailSubTab === 'font'} onclick={() => detailSubTab = 'font'}>{t[currentLang].fontSettings}</button>
          </div>

          <div class="sub-content">
            <!-- Image Settings Sub-tab (3-Column Cards) -->
            {#if detailSubTab === 'image'}
              <div class="policy-row">
                <div class="field-item">
                  <span class="field-label">{t[currentLang].colorConvPolicy}</span>
                  <div class="tooltip-wrapper tip-left">
                    <select class="select-box" bind:value={config.color_conv}>
                      <option value="original">{t[currentLang].colorConvOriginal}</option>
                      <option value="gray">{t[currentLang].colorConvGray}</option>
                      <option value="mono">{t[currentLang].colorConvMono}</option>
                    </select>
                    <span class="tooltip-text">{t[currentLang].tooltipColorConv}</span>
                  </div>
                </div>

                <label class="checkbox-field tooltip-wrapper tip-right">
                  <input type="checkbox" bind:checked={config.force_rasterize} />
                  <span>{t[currentLang].forceRasterize}</span>
                  <span class="tooltip-text">{t[currentLang].tooltipForceRasterize}</span>
                </label>
              </div>

              <!-- 3-Column Grid (Color, Grayscale, Monochrome) -->
              <div class="image-cards-3col">
                <!-- Color Image Card -->
                <div class="image-card">
                  <h4 class="card-head">{t[currentLang].colorImage}</h4>
                  <div class="card-controls">
                    <div class="ctrl-row tooltip-wrapper tip-left">
                      <span>{t[currentLang].resolution}:</span>
                      <div class="combo-box-wrapper">
                        <input type="number" class="combo-input-box" bind:value={config.color_img.target_dpi} />
                        <select 
                          class="combo-dropdown-btn" 
                          onchange={(e) => { if (e.currentTarget.value) { config.color_img.target_dpi = Number(e.currentTarget.value); e.currentTarget.value = ""; } }}
                        >
                          <option value="" disabled selected></option>
                          <option value="50">50</option>
                          <option value="72">72</option>
                          <option value="100">100</option>
                          <option value="150">150</option>
                          <option value="200">200</option>
                          <option value="300">300</option>
                        </select>
                      </div>
                      <span class="tooltip-text">{t[currentLang].tooltipDpi}</span>
                    </div>
                    <div class="ctrl-row tooltip-wrapper tip-left">
                      <span>{t[currentLang].threshold}:</span>
                      <div class="combo-box-wrapper">
                        <input type="number" class="combo-input-box" bind:value={config.color_img.threshold_dpi} />
                        <select 
                          class="combo-dropdown-btn" 
                          onchange={(e) => { if (e.currentTarget.value) { config.color_img.threshold_dpi = Number(e.currentTarget.value); e.currentTarget.value = ""; } }}
                        >
                          <option value="" disabled selected></option>
                          <option value="75">75</option>
                          <option value="100">100</option>
                          <option value="150">150</option>
                          <option value="225">225</option>
                          <option value="300">300</option>
                          <option value="450">450</option>
                        </select>
                      </div>
                      <span class="tooltip-text">{t[currentLang].tooltipThreshold}</span>
                    </div>
                    <div class="ctrl-row tooltip-wrapper tip-left">
                      <span>{t[currentLang].quality}:</span>
                      <div class="combo-box-wrapper">
                        <input type="number" class="combo-input-box" bind:value={config.color_img.quality} />
                        <select 
                          class="combo-dropdown-btn" 
                          onchange={(e) => { if (e.currentTarget.value) { config.color_img.quality = Number(e.currentTarget.value); e.currentTarget.value = ""; } }}
                        >
                          <option value="" disabled selected></option>
                          <option value="35">35</option>
                          <option value="50">50</option>
                          <option value="65">65</option>
                          <option value="75">75</option>
                        </select>
                      </div>
                      <span class="tooltip-text">{t[currentLang].tooltipQuality}</span>
                    </div>
                  </div>
                </div>

                <!-- Grayscale Image Card -->
                <div class="image-card">
                  <h4 class="card-head">{t[currentLang].grayImage}</h4>
                  <div class="card-controls">
                    <div class="ctrl-row tooltip-wrapper tip-center">
                      <span>{t[currentLang].resolution}:</span>
                      <div class="combo-box-wrapper">
                        <input type="number" class="combo-input-box" bind:value={config.gray_img.target_dpi} />
                        <select 
                          class="combo-dropdown-btn" 
                          onchange={(e) => { if (e.currentTarget.value) { config.gray_img.target_dpi = Number(e.currentTarget.value); e.currentTarget.value = ""; } }}
                        >
                          <option value="" disabled selected></option>
                          <option value="50">50</option>
                          <option value="72">72</option>
                          <option value="100">100</option>
                          <option value="150">150</option>
                          <option value="200">200</option>
                          <option value="300">300</option>
                        </select>
                      </div>
                      <span class="tooltip-text">{t[currentLang].tooltipDpi}</span>
                    </div>
                    <div class="ctrl-row tooltip-wrapper tip-center">
                      <span>{t[currentLang].threshold}:</span>
                      <div class="combo-box-wrapper">
                        <input type="number" class="combo-input-box" bind:value={config.gray_img.threshold_dpi} />
                        <select 
                          class="combo-dropdown-btn" 
                          onchange={(e) => { if (e.currentTarget.value) { config.gray_img.threshold_dpi = Number(e.currentTarget.value); e.currentTarget.value = ""; } }}
                        >
                          <option value="" disabled selected></option>
                          <option value="75">75</option>
                          <option value="100">100</option>
                          <option value="150">150</option>
                          <option value="225">225</option>
                          <option value="300">300</option>
                          <option value="450">450</option>
                        </select>
                      </div>
                      <span class="tooltip-text">{t[currentLang].tooltipThreshold}</span>
                    </div>
                    <div class="ctrl-row tooltip-wrapper tip-center">
                      <span>{t[currentLang].quality}:</span>
                      <div class="combo-box-wrapper">
                        <input type="number" class="combo-input-box" bind:value={config.gray_img.quality} />
                        <select 
                          class="combo-dropdown-btn" 
                          onchange={(e) => { if (e.currentTarget.value) { config.gray_img.quality = Number(e.currentTarget.value); e.currentTarget.value = ""; } }}
                        >
                          <option value="" disabled selected></option>
                          <option value="35">35</option>
                          <option value="50">50</option>
                          <option value="65">65</option>
                          <option value="75">75</option>
                        </select>
                      </div>
                      <span class="tooltip-text">{t[currentLang].tooltipQuality}</span>
                    </div>
                  </div>
                </div>

                <!-- Monochrome Image Card -->
                <div class="image-card">
                  <h4 class="card-head">{t[currentLang].monoImage}</h4>
                  <div class="card-controls">
                    <div class="ctrl-row tooltip-wrapper tip-right">
                      <span>{t[currentLang].resolution}:</span>
                      <div class="combo-box-wrapper">
                        <input type="number" class="combo-input-box" bind:value={config.mono_img.target_dpi} />
                        <select 
                          class="combo-dropdown-btn" 
                          onchange={(e) => { if (e.currentTarget.value) { config.mono_img.target_dpi = Number(e.currentTarget.value); e.currentTarget.value = ""; } }}
                        >
                          <option value="" disabled selected></option>
                          <option value="50">50</option>
                          <option value="72">72</option>
                          <option value="100">100</option>
                          <option value="150">150</option>
                          <option value="200">200</option>
                          <option value="300">300</option>
                        </select>
                      </div>
                      <span class="tooltip-text">{t[currentLang].tooltipDpi}</span>
                    </div>
                    <div class="ctrl-row tooltip-wrapper tip-right">
                      <span>{t[currentLang].threshold}:</span>
                      <div class="combo-box-wrapper">
                        <input type="number" class="combo-input-box" bind:value={config.mono_img.threshold_dpi} />
                        <select 
                          class="combo-dropdown-btn" 
                          onchange={(e) => { if (e.currentTarget.value) { config.mono_img.threshold_dpi = Number(e.currentTarget.value); e.currentTarget.value = ""; } }}
                        >
                          <option value="" disabled selected></option>
                          <option value="75">75</option>
                          <option value="100">100</option>
                          <option value="150">150</option>
                          <option value="225">225</option>
                          <option value="300">300</option>
                          <option value="450">450</option>
                        </select>
                      </div>
                      <span class="tooltip-text">{t[currentLang].tooltipThreshold}</span>
                    </div>
                    <div class="ctrl-row tooltip-wrapper tip-right">
                      <span>{t[currentLang].binarization}:</span>
                      <select class="select-box slim-select-box" bind:value={config.mono_img.mono_method}>
                        <option value="threshold">{t[currentLang].binOtsu}</option>
                        <option value="dither">{t[currentLang].binFloyd}</option>
                      </select>
                      <span class="tooltip-text">{t[currentLang].tooltipMonoMethod}</span>
                    </div>
                  </div>
                </div>
              </div>
            {/if}

            <!-- Discard Settings Sub-tab -->
            {#if detailSubTab === 'discard'}
              <div class="discard-two-col">
                <div class="discard-box">
                  <h4 class="section-subhead">{t[currentLang].discardGroup}</h4>
                  <div class="checkbox-list">
                    <label class="checkbox-field tooltip-wrapper tip-left">
                      <input type="checkbox" bind:checked={config.discard_comments} /> {t[currentLang].discardComments}
                      <span class="tooltip-text">{t[currentLang].tooltipDiscardComments}</span>
                    </label>
                    <label class="checkbox-field tooltip-wrapper tip-left">
                      <input type="checkbox" bind:checked={config.discard_links} /> {t[currentLang].discardLinks}
                      <span class="tooltip-text">{t[currentLang].tooltipDiscardLinks}</span>
                    </label>
                    <label class="checkbox-field tooltip-wrapper tip-left">
                      <input type="checkbox" bind:checked={config.discard_bookmarks} /> {t[currentLang].discardBookmarks}
                      <span class="tooltip-text">{t[currentLang].tooltipDiscardBookmarks}</span>
                    </label>
                    <label class="checkbox-field tooltip-wrapper tip-left">
                      <input type="checkbox" bind:checked={config.discard_forms} /> {t[currentLang].discardForms}
                      <span class="tooltip-text">{t[currentLang].tooltipDiscardForms}</span>
                    </label>
                    <label class="checkbox-field tooltip-wrapper tip-left">
                      <input type="checkbox" bind:checked={config.discard_metadata} /> {t[currentLang].discardMetadata}
                      <span class="tooltip-text">{t[currentLang].tooltipDiscardMetadata}</span>
                    </label>
                    <label class="checkbox-field tooltip-wrapper tip-left">
                      <input type="checkbox" bind:checked={config.discard_js} /> {t[currentLang].discardJs}
                      <span class="tooltip-text">{t[currentLang].tooltipDiscardJs}</span>
                    </label>
                    <label class="checkbox-field tooltip-wrapper tip-left">
                      <input type="checkbox" bind:checked={config.discard_thumbs} /> {t[currentLang].discardThumbs}
                      <span class="tooltip-text">{t[currentLang].tooltipDiscardThumbs}</span>
                    </label>
                  </div>
                </div>

                <div class="discard-box">
                  <h4 class="section-subhead">{t[currentLang].cleanupGroup}</h4>
                  <div class="checkbox-list">
                    <label class="checkbox-field tooltip-wrapper tip-right">
                      <input type="checkbox" bind:checked={config.clean_linearize} /> {t[currentLang].cleanLinearize}
                      <span class="tooltip-text">{t[currentLang].tooltipCleanLinearize}</span>
                    </label>
                    <label class="checkbox-field tooltip-wrapper tip-right">
                      <input type="checkbox" bind:checked={config.clean_deflate} /> {t[currentLang].cleanDeflate}
                      <span class="tooltip-text">{t[currentLang].tooltipCleanDeflate}</span>
                    </label>
                    <label class="checkbox-field tooltip-wrapper tip-right">
                      <input type="checkbox" bind:checked={config.clean_sanitize} /> {t[currentLang].cleanSanitize}
                      <span class="tooltip-text">{t[currentLang].tooltipCleanSanitize}</span>
                    </label>
                  </div>
                </div>
              </div>
            {/if}

            <!-- Font Settings Sub-tab -->
            {#if detailSubTab === 'font'}
              <div class="font-panel-box">
                <label class="checkbox-field tooltip-wrapper tip-left">
                  <input type="checkbox" bind:checked={config.font_subset} /> {t[currentLang].fontSubset}
                  <span class="tooltip-text">{t[currentLang].tooltipFontSubset}</span>
                </label>
                
                <h4 class="section-subhead" style="margin-top: 10px;">{t[currentLang].fontUnembedTitle}</h4>
                <p class="subtext">{t[currentLang].fontUnembedDesc}</p>
                <div class="font-scroll-box">
                  <ul>
                    {#each detectedFonts as font}
                      <!-- svelte-ignore a11y_no_noninteractive_element_to_interactive_role -->
                      <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
                      <li class="font-row" class:unembedded={config.unembed_fonts.includes(font)} ondblclick={() => toggleFontUnembed(font)}>
                        <span>{font}</span>
                        <span class="tag">
                          {config.unembed_fonts.includes(font) ? t[currentLang].targetUnembed : t[currentLang].maintainEmbed}
                        </span>
                      </li>
                    {/each}
                  </ul>
                </div>
              </div>
            {/if}
          </div>
        </div>
      {/if}

      <!-- Profile Tab -->
      {#if activeTab === 'profile'}
        <div class="profile-panel">
          <div class="profile-card">
            <h4 class="section-subhead">{t[currentLang].saveCurrentProfile}</h4>
            <div class="input-inline">
              <input type="text" class="text-input" placeholder={t[currentLang].profileNamePlaceholder} bind:value={newProfileName} />
              <button class="btn btn-sm btn-primary" onclick={handleSaveProfile} disabled={!newProfileName}>{t[currentLang].saveBtn}</button>
            </div>
          </div>

          <div class="profile-card">
            <h4 class="section-subhead">{t[currentLang].savedProfilesTitle}</h4>
            {#if savedProfiles.length === 0}
              <p class="subtext">{t[currentLang].noSavedProfiles}</p>
            {:else}
              <ul class="profile-list">
                {#each savedProfiles as p}
                  <li class="profile-item">
                    <span>{p}</span>
                    <button class="btn-text-danger" onclick={() => handleDeleteProfile(p)}>{t[currentLang].deleteBtn}</button>
                  </li>
                {/each}
              </ul>
            {/if}
          </div>
        </div>
      {/if}
    </div>

    <!-- Status, Progress & Log Footer Area -->
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
          <span class="log-placeholder">{t[currentLang].logPlaceholder}</span>
        {:else if logItems.length > 0}
          {#each logItems as log}
            <div class="log-entry-row">
              <span class={log.success ? "log-success" : "log-fail"}>{log.message}</span>
              {#if log.success && log.file_path}
                <button
                  type="button"
                  class="btn-log-audit"
                  onclick={() => openAuditModal(log.file_path, log.audit)}
                >
                  📊 {t[currentLang].auditBtn}
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
      <button class="btn btn-sm btn-muted" onclick={handleReset} disabled={isProcessing}>{t[currentLang].resetBtn}</button>
      <button class="btn btn-primary" onclick={startOptimize} disabled={isProcessing || files.length === 0}>
        {isProcessing ? t[currentLang].compressingBtn : t[currentLang].compressAllBtn}
      </button>
    </footer>
  </section>
</main>

<!-- Audit Space Usage Modal Dialog -->
{#if isAuditModalOpen}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="modal-backdrop" onclick={closeAuditModal}>
    <div class="modal-container" onclick={(e) => e.stopPropagation()}>
      <header class="modal-header">
        <div class="modal-title-box">
          <span class="modal-icon">📊</span>
          <h3 class="modal-title">{t[currentLang].auditTitle}</h3>
        </div>
        <button class="modal-close-btn" onclick={closeAuditModal}>×</button>
      </header>

      <div class="modal-body">
        {#if isAuditLoading}
          <div class="audit-loading-box">
            <div class="spinner"></div>
            <span>{t[currentLang].auditLoading}</span>
          </div>
        {:else if auditErrorMessage}
          <div class="audit-error-box">
            ❌ {auditErrorMessage}
          </div>
        {:else if currentAuditData}
          <!-- File Info Header -->
          <div class="audit-file-header">
            <div class="file-name">{currentAuditData.original.file_name}</div>
            <div class="file-size-summary">
              <span class="size-orig">{t[currentLang].auditBefore}: <strong>{formatBytes(currentAuditData.original.total_bytes)}</strong></span>
              {#if currentAuditData.optimized}
                <span class="arrow">➔</span>
                <span class="size-opt">{t[currentLang].auditAfter}: <strong>{formatBytes(currentAuditData.optimized.total_bytes)}</strong></span>
                {@const diff = currentAuditData.original.total_bytes - currentAuditData.optimized.total_bytes}
                {@const pct = (diff / currentAuditData.original.total_bytes) * 100}
                <span class="badge-saving" class:negative={diff < 0}>
                  {diff >= 0 ? `-${formatBytes(diff)} (-${pct.toFixed(1)}%)` : `+${formatBytes(-diff)} (+${(-pct).toFixed(1)}%)`}
                </span>
              {:else}
                <span class="audit-no-opt-hint">{t[currentLang].auditNoOptYet}</span>
              {/if}
            </div>
          </div>

          <!-- Stacked Bar Charts (Visual Comparison) -->
          <div class="audit-stacked-bar-section">
            <div class="stacked-bar-row">
              <span class="bar-label">{t[currentLang].auditBefore}</span>
              <div class="stacked-bar">
                {#each currentAuditData.original.categories as cat}
                  {#if cat.percentage > 0}
                    <div
                      class="bar-segment"
                      style="width: {cat.percentage}%; background-color: {categoryColors[cat.key] || '#64748b'};"
                      title="{currentLang === 'ja' ? cat.name_ja : cat.name_en}: {formatBytes(cat.bytes)} ({cat.percentage.toFixed(1)}%)"
                    ></div>
                  {/if}
                {/each}
              </div>
            </div>

            {#if currentAuditData.optimized}
              <div class="stacked-bar-row">
                <span class="bar-label">{t[currentLang].auditAfter}</span>
                <div class="stacked-bar">
                  {#each currentAuditData.optimized.categories as cat}
                    {#if cat.percentage > 0}
                      <div
                        class="bar-segment"
                        style="width: {cat.percentage}%; background-color: {categoryColors[cat.key] || '#64748b'};"
                        title="{currentLang === 'ja' ? cat.name_ja : cat.name_en}: {formatBytes(cat.bytes)} ({cat.percentage.toFixed(1)}%)"
                      ></div>
                    {/if}
                  {/each}
                </div>
              </div>
            {/if}

            <!-- Bar Legend -->
            <div class="audit-legend">
              {#each currentAuditData.original.categories as cat}
                <div class="legend-item">
                  <span class="legend-color" style="background-color: {categoryColors[cat.key] || '#64748b'};"></span>
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
                  <th>{t[currentLang].auditColCategory}</th>
                  <th colspan="3">{t[currentLang].auditBefore}</th>
                  {#if currentAuditData.optimized}
                    <th colspan="3">{t[currentLang].auditAfter}</th>
                    <th>{t[currentLang].auditColDiff}</th>
                  {/if}
                </tr>
                <tr class="subhead-row">
                  <th></th>
                  <th>{t[currentLang].auditColCount}</th>
                  <th>{t[currentLang].auditColBytes}</th>
                  <th>%</th>
                  {#if currentAuditData.optimized}
                    <th>{t[currentLang].auditColCount}</th>
                    <th>{t[currentLang].auditColBytes}</th>
                    <th>%</th>
                    <th>{t[currentLang].auditDiff}</th>
                  {/if}
                </tr>
              </thead>
              <tbody>
                {#each currentAuditData.original.categories as origCat, idx}
                  {@const optCat = currentAuditData.optimized ? currentAuditData.optimized.categories[idx] : null}
                  {@const diffBytes = optCat ? origCat.bytes - optCat.bytes : 0}
                  <tr>
                    <td class="cat-name">
                      <span class="row-color-dot" style="background-color: {categoryColors[origCat.key] || '#64748b'};"></span>
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
                  <td>{t[currentLang].auditTotal}</td>
                  <td class="num">
                    {currentAuditData.original.categories.reduce((a, b) => a + b.count, 0)}
                  </td>
                  <td class="num">{formatBytes(currentAuditData.original.total_bytes)}</td>
                  <td class="num-pct">100%</td>
                  {#if currentAuditData.optimized}
                    <td class="num">
                      {currentAuditData.optimized.categories.reduce((a, b) => a + b.count, 0)}
                    </td>
                    <td class="num">{formatBytes(currentAuditData.optimized.total_bytes)}</td>
                    <td class="num-pct">100%</td>
                    {@const totalDiff = currentAuditData.original.total_bytes - currentAuditData.optimized.total_bytes}
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
        <button class="btn btn-secondary" onclick={closeAuditModal}>{t[currentLang].auditClose}</button>
      </footer>
    </div>
  </div>
{/if}

<style>
  /* Slate / Zinc Minimal Clean Theme */
  :root {
    --bg-base: #0f172a;        /* Deep Slate */
    --bg-surface: #1e293b;     /* Dark Slate Panel */
    --bg-input: #334155;       /* Input Fill */
    --border-color: #334155;   /* Subtle Border */
    --border-highlight: #475569;
    --text-main: #f8fafc;      /* High contrast white */
    --text-muted: #94a3b8;     /* Slate Muted Subtext */
    --accent-blue: #3b82f6;    /* Sleek Active Blue */
    --accent-blue-hover: #60a5fa;
    --accent-emerald: #10b981; /* Success Emerald */
    --accent-danger: #ef4444;  /* Soft Red */
    
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

  /* Completely hide HTML5 spin buttons (number up/down arrows) */
  :global(input::-webkit-outer-spin-button),
  :global(input::-webkit-inner-spin-button) {
    -webkit-appearance: none !important;
    margin: 0 !important;
  }
  :global(input[type=number]) {
    -moz-appearance: textfield !important;
    appearance: textfield !important;
  }

  /* Global Select & Option dark styling reset for Linux GTK3 / WebKitGTK & Windows */
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
    width: 680px;
    height: 530px;
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

  /* Top Section: Slim Drop & File Select Area */
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

  .top-actions {
    display: flex;
    gap: 6px;
  }

  /* Main Panel Layout */
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

  /* Easy Mode Layout */
  .card-panel {
    background-color: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    padding: 10px 12px;
  }

  .easy-panel {
    padding: 10px 12px;
  }

  .panel-title {
    margin: 0 0 8px 0;
    font-size: 0.85rem;
    color: var(--accent-blue);
  }

  .preset-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 6px;
    margin-bottom: 10px;
  }

  .radio-card {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 8px;
    background-color: rgba(15, 23, 42, 0.4);
    border: 1px solid var(--border-color);
    border-radius: 6px;
    cursor: pointer;
    font-size: 0.78rem;
    transition: border-color 0.2s, background-color 0.2s;
  }

  .radio-card:hover {
    border-color: var(--border-highlight);
  }

  .radio-card.active {
    border-color: var(--accent-blue);
    background-color: rgba(59, 130, 246, 0.1);
  }

  .radio-card input[type="radio"] {
    accent-color: var(--accent-blue);
  }

  .profile-inline-box {
    display: flex;
    gap: 6px;
    align-items: center;
  }

  /* Detail Mode Layout */
  .detail-panel {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .sub-nav-bar {
    display: flex;
    gap: 4px;
    border-bottom: 1px solid var(--border-color);
    padding-bottom: 4px;
  }

  .sub-nav-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    padding: 3px 8px;
    font-size: 0.78rem;
    cursor: pointer;
    border-radius: 4px;
    transition: background-color 0.2s, color 0.2s;
  }

  .sub-nav-btn.active {
    background-color: var(--border-color);
    color: var(--text-main);
    font-weight: 600;
  }

  .policy-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    background-color: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: 6px;
    padding: 6px 10px;
    margin-bottom: 6px;
  }

  .field-item {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .field-label {
    font-size: 0.78rem;
    color: var(--text-muted);
  }

  /* Restored 3-Column Side-by-Side Image Cards */
  .image-cards-3col {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 8px;
  }

  .image-card {
    background-color: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: 6px;
    padding: 8px 10px;
  }

  .card-head {
    margin: 0 0 6px 0;
    font-size: 0.8rem;
    color: var(--accent-blue);
    border-bottom: 1px solid rgba(51, 65, 85, 0.5);
    padding-bottom: 4px;
  }

  .card-controls {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .ctrl-row {
    display: flex;
    flex-direction: column;
    gap: 2px;
    font-size: 0.74rem;
  }



  .slim-select-box {
    width: 100%;
    box-sizing: border-box;
    font-size: 0.78rem;
    padding: 3px 6px;
  }

  /* Custom Integrated Combo Box Styles */
  .combo-box-wrapper {
    display: flex;
    align-items: center;
    width: 100%;
  }

  .combo-input-box {
    flex: 1;
    min-width: 0;
    background-color: var(--bg-base);
    border: 1px solid var(--border-color);
    border-right: none;
    border-top-left-radius: 4px;
    border-bottom-left-radius: 4px;
    color: var(--text-main);
    padding: 3px 6px;
    font-size: 0.78rem;
    outline: none;
    box-sizing: border-box;
  }

  .combo-input-box:focus {
    border-color: var(--accent-blue);
  }

  .combo-dropdown-btn {
    appearance: none;
    -webkit-appearance: none;
    -moz-appearance: none;
    background-color: #1e293b !important;
    border: 1px solid var(--border-color);
    border-top-right-radius: 4px;
    border-bottom-right-radius: 4px;
    color: #94a3b8 !important;
    padding: 3px 2px;
    font-size: 0.72rem;
    cursor: pointer;
    outline: none;
    width: 22px;
    height: 100%;
    text-align: center;
    box-sizing: border-box;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='10' height='10' viewBox='0 0 24 24' fill='none' stroke='%2394a3b8' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpolyline points='6 9 12 15 18 9'%3E%3C/polyline%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: center;
  }

  .combo-dropdown-btn:hover {
    background-color: var(--border-color) !important;
    color: #f8fafc !important;
  }

  /* Discard Two Column Layout */
  .discard-two-col {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 8px;
  }

  .discard-box {
    background-color: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: 6px;
    padding: 8px 10px;
  }

  .section-subhead {
    margin: 0 0 6px 0;
    font-size: 0.8rem;
    color: var(--accent-blue);
  }

  .checkbox-list {
    display: flex;
    flex-direction: column;
    gap: 5px;
  }

  .checkbox-field {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 0.76rem;
    cursor: pointer;
  }

  .checkbox-field input[type="checkbox"] {
    accent-color: var(--accent-blue);
  }

  /* Font Panel */
  .font-panel-box {
    background-color: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: 6px;
    padding: 8px 10px;
  }

  .font-scroll-box {
    height: 90px;
    background-color: var(--bg-base);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    overflow-y: auto;
    padding: 4px;
  }

  .font-scroll-box ul {
    list-style: none;
    margin: 0;
    padding: 0;
  }

  .font-row {
    display: flex;
    justify-content: space-between;
    padding: 3px 6px;
    font-size: 0.73rem;
    border-bottom: 1px solid rgba(51, 65, 85, 0.5);
    cursor: pointer;
  }

  .font-row.unembedded {
    background-color: rgba(239, 68, 68, 0.15);
  }

  .tag {
    font-size: 0.68rem;
    color: var(--text-muted);
  }

  /* Profile Panel */
  .profile-panel {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .profile-card {
    background-color: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: 6px;
    padding: 8px 10px;
  }

  .input-inline {
    display: flex;
    gap: 6px;
  }

  .text-input {
    flex: 1;
    background-color: var(--bg-base);
    border: 1px solid var(--border-color);
    color: var(--text-main);
    padding: 4px 6px;
    border-radius: 4px;
    font-size: 0.78rem;
  }

  .profile-list {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .profile-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    background-color: var(--bg-base);
    padding: 4px 8px;
    border-radius: 4px;
    font-size: 0.75rem;
  }

  /* Status Footer & Log Area */
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

  /* Action Footer */
  .bottom-actions {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  /* Tooltip Balloon */
  .tooltip-wrapper {
    position: relative;
  }

  .tooltip-text {
    visibility: hidden;
    width: 190px;
    background-color: #1e293b;
    color: #f8fafc;
    border: 1px solid #475569;
    border-radius: 6px;
    padding: 6px 8px;
    position: absolute;
    z-index: 9999;
    bottom: 125%;
    opacity: 0;
    transition: opacity 0.15s, visibility 0.15s;
    font-size: 0.71rem;
    line-height: 1.38;
    box-shadow: 0 4px 14px rgba(0, 0, 0, 0.6);
    pointer-events: none;
    font-weight: normal;
  }

  .tip-left .tooltip-text {
    left: 0;
    transform: none;
  }
  .tip-left .tooltip-text::after {
    content: "";
    position: absolute;
    top: 100%;
    left: 14px;
    border-width: 5px;
    border-style: solid;
    border-color: #475569 transparent transparent transparent;
  }

  .tip-center .tooltip-text {
    left: 50%;
    transform: translateX(-50%);
  }
  .tip-center .tooltip-text::after {
    content: "";
    position: absolute;
    top: 100%;
    left: 50%;
    margin-left: -5px;
    border-width: 5px;
    border-style: solid;
    border-color: #475569 transparent transparent transparent;
  }

  .tip-right .tooltip-text {
    right: 0;
    transform: none;
  }
  .tip-right .tooltip-text::after {
    content: "";
    position: absolute;
    top: 100%;
    right: 14px;
    border-width: 5px;
    border-style: solid;
    border-color: #475569 transparent transparent transparent;
  }

  .tooltip-wrapper:hover .tooltip-text {
    visibility: visible;
    opacity: 1;
  }

  /* Form controls & Buttons */
  .select-box {
    appearance: none;
    -webkit-appearance: none;
    -moz-appearance: none;
    background-color: #1e293b !important;
    border: 1px solid var(--border-color);
    color: #f8fafc !important;
    padding: 3px 22px 3px 8px;
    border-radius: 4px;
    font-size: 0.78rem;
    outline: none;
    cursor: pointer;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='10' height='10' viewBox='0 0 24 24' fill='none' stroke='%2394a3b8' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpolyline points='6 9 12 15 18 9'%3E%3C/polyline%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: right 6px center;
  }

  .select-box:hover {
    border-color: var(--accent-blue);
  }

  .slim-select {
    padding: 2px 20px 2px 6px;
    font-size: 0.74rem;
    max-width: 140px;
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

  .btn-secondary {
    background-color: var(--bg-surface);
    border: 1px solid var(--border-color);
    color: var(--text-main);
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

  .btn-text-danger {
    background: none;
    border: none;
    color: var(--accent-danger);
    cursor: pointer;
    font-size: 0.73rem;
  }

  .btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
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

  .log-entry-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    line-height: 1.35;
    padding: 1px 0;
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

  /* Modal Backdrop & Overlay Window */
  :global(.modal-backdrop) {
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

  :global(.modal-container) {
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

  :global(.modal-header) {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 6px 14px;
    background-color: #0f172a;
    border-bottom: 1px solid var(--border-color);
  }

  :global(.modal-title-box) {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  :global(.modal-icon) {
    font-size: 1.0rem;
  }

  :global(.modal-title) {
    margin: 0;
    font-size: 0.90rem;
    font-weight: 600;
    color: var(--text-main);
  }

  :global(.modal-close-btn) {
    background: none;
    border: none;
    color: var(--text-muted);
    font-size: 1.2rem;
    line-height: 1;
    cursor: pointer;
    padding: 0 4px;
  }

  :global(.modal-close-btn:hover) {
    color: #ffffff;
  }

  :global(.modal-body) {
    padding: 8px 12px;
    overflow-y: auto;
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  /* Thin sleek scrollbar for modal body */
  :global(.modal-body::-webkit-scrollbar) {
    width: 6px;
  }
  :global(.modal-body::-webkit-scrollbar-track) {
    background: #0f172a;
  }
  :global(.modal-body::-webkit-scrollbar-thumb) {
    background: #475569;
    border-radius: 3px;
  }
  :global(.modal-body::-webkit-scrollbar-thumb:hover) {
    background: #64748b;
  }

  :global(.modal-footer) {
    padding: 6px 14px;
    background-color: #0f172a;
    border-top: 1px solid var(--border-color);
    display: flex;
    justify-content: flex-end;
  }

  /* Audit Header & Summary */
  :global(.audit-file-header) {
    background-color: #0f172a;
    border: 1px solid var(--border-color);
    border-radius: 5px;
    padding: 6px 10px;
  }

  :global(.audit-file-header .file-name) {
    font-weight: 600;
    font-size: 0.82rem;
    color: #ffffff;
    margin-bottom: 2px;
    word-break: break-all;
  }

  :global(.file-size-summary) {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 0.72rem;
    color: var(--text-muted);
    flex-wrap: wrap;
  }

  :global(.file-size-summary strong) {
    color: var(--text-main);
  }

  :global(.arrow) {
    color: var(--accent-blue);
    font-weight: bold;
  }

  :global(.badge-saving) {
    background-color: rgba(16, 185, 129, 0.2);
    color: #34d399;
    border: 1px solid rgba(16, 185, 129, 0.4);
    padding: 0 6px;
    border-radius: 10px;
    font-weight: 600;
    font-size: 0.70rem;
  }

  :global(.badge-saving.negative) {
    background-color: rgba(239, 68, 68, 0.2);
    color: #f87171;
    border-color: rgba(239, 68, 68, 0.4);
  }

  :global(.audit-no-opt-hint) {
    font-size: 0.70rem;
    color: #64748b;
    font-style: italic;
  }

  /* Stacked Bar Section */
  :global(.audit-stacked-bar-section) {
    display: flex;
    flex-direction: column;
    gap: 4px;
    background-color: #0f172a;
    border: 1px solid var(--border-color);
    border-radius: 5px;
    padding: 6px 10px;
  }

  :global(.stacked-bar-row) {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  :global(.bar-label) {
    width: 95px;
    font-size: 0.70rem;
    color: var(--text-muted);
    font-weight: 500;
  }

  :global(.stacked-bar) {
    flex: 1;
    height: 11px;
    background-color: #1e293b;
    border-radius: 3px;
    display: flex;
    overflow: hidden;
  }

  :global(.bar-segment) {
    height: 100%;
    transition: width 0.3s;
  }

  :global(.bar-segment:hover) {
    filter: brightness(1.25);
  }

  :global(.audit-legend) {
    display: flex;
    flex-wrap: wrap;
    gap: 3px 10px;
    margin-top: 2px;
    padding-top: 4px;
    border-top: 1px dashed rgba(51, 65, 85, 0.6);
  }

  :global(.legend-item) {
    display: flex;
    align-items: center;
    gap: 4px;
    font-size: 0.67rem;
    color: var(--text-muted);
  }

  :global(.legend-color) {
    width: 8px;
    height: 8px;
    border-radius: 2px;
  }

  /* Audit Table */
  :global(.audit-table-container) {
    border: 1px solid var(--border-color);
    border-radius: 5px;
    overflow: hidden;
  }

  :global(.audit-table) {
    width: 100%;
    border-collapse: collapse;
    font-size: 0.70rem;
    text-align: left;
  }

  :global(.audit-table th), :global(.audit-table td) {
    padding: 3px 6px;
    border-bottom: 1px solid rgba(51, 65, 85, 0.4);
  }

  :global(.audit-table thead th) {
    background-color: #0f172a;
    color: var(--text-main);
    font-weight: 600;
    text-align: center;
    border-bottom: 1px solid var(--border-color);
  }

  :global(.audit-table thead tr.subhead-row th) {
    font-size: 0.65rem;
    color: var(--text-muted);
    font-weight: normal;
    padding: 1px 4px;
  }

  :global(.audit-table thead th:first-child), :global(.audit-table tbody td:first-child) {
    text-align: left;
  }

  :global(.cat-name) {
    display: flex;
    align-items: center;
    gap: 5px;
    font-weight: 500;
  }

  :global(.row-color-dot) {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    display: inline-block;
  }

  :global(.num) {
    text-align: right;
    font-family: monospace;
  }

  :global(.num-pct) {
    text-align: right;
    font-family: monospace;
    color: var(--text-muted);
  }

  :global(.num-diff) {
    text-align: right;
    font-family: monospace;
    font-weight: 600;
  }

  :global(.num-diff.positive) {
    color: #34d399;
  }

  :global(.num-diff.negative) {
    color: #f87171;
  }

  :global(.total-row td) {
    background-color: #0f172a;
    font-weight: bold;
    border-bottom: none;
    border-top: 1px solid var(--border-color);
  }

  /* Loading & Error Box */
  :global(.audit-loading-box) {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 20px;
    gap: 10px;
    color: var(--text-muted);
  }

  :global(.spinner) {
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

  :global(.audit-error-box) {
    background-color: rgba(239, 68, 68, 0.15);
    border: 1px solid rgba(239, 68, 68, 0.3);
    color: #f87171;
    padding: 10px;
    border-radius: 5px;
    font-size: 0.78rem;
  }
</style>
