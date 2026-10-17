<script lang="ts">
  import type { OptimizeConfig } from "../types";
  import { translations } from "../utils/i18n";

  let {
    config = $bindable(),
    detectedFonts = [],
    currentLang = "en",
    onToggleFontUnembed,
  }: {
    config: OptimizeConfig;
    detectedFonts: string[];
    currentLang: "ja" | "en";
    onToggleFontUnembed: (font: string) => void;
  } = $props();

  let detailSubTab = $state<"image" | "discard" | "font">("image");
  const t = $derived(translations[currentLang]);
</script>

<div class="detail-panel">
  <div class="sub-nav-bar">
    <button class="sub-nav-btn" class:active={detailSubTab === 'image'} onclick={() => detailSubTab = 'image'}>{t.imageSettings}</button>
    <button class="sub-nav-btn" class:active={detailSubTab === 'discard'} onclick={() => detailSubTab = 'discard'}>{t.discardSettings}</button>
    <button class="sub-nav-btn" class:active={detailSubTab === 'font'} onclick={() => detailSubTab = 'font'}>{t.fontSettings}</button>
  </div>

  <div class="sub-content">
    <!-- Image Settings Sub-tab (3-Column Cards) -->
    {#if detailSubTab === 'image'}
      <div class="policy-row">
        <div class="field-item">
          <span class="field-label">{t.colorConvPolicy}</span>
          <div class="tooltip-wrapper tip-left">
            <select class="select-box" bind:value={config.color_conv}>
              <option value="original">{t.colorConvOriginal}</option>
              <option value="gray">{t.colorConvGray}</option>
              <option value="mono">{t.colorConvMono}</option>
            </select>
            <span class="tooltip-text">{t.tooltipColorConv}</span>
          </div>
        </div>

        <label class="checkbox-field tooltip-wrapper tip-right">
          <input type="checkbox" bind:checked={config.force_rasterize} />
          <span>{t.forceRasterize}</span>
          <span class="tooltip-text">{t.tooltipForceRasterize}</span>
        </label>
      </div>

      <!-- 3-Column Grid (Color, Grayscale, Monochrome) -->
      <div class="image-cards-3col">
        <!-- Color Image Card -->
        <div class="image-card">
          <h4 class="card-head">{t.colorImage}</h4>
          <div class="card-controls">
            <div class="ctrl-row tooltip-wrapper tip-left">
              <span>{t.resolution}:</span>
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
                  <option value="450">450</option>
                </select>
              </div>
              <span class="tooltip-text">{t.tooltipDpi}</span>
            </div>
            <div class="ctrl-row tooltip-wrapper tip-left">
              <span>{t.threshold}:</span>
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
              <span class="tooltip-text">{t.tooltipThreshold}</span>
            </div>
            <div class="ctrl-row tooltip-wrapper tip-left">
              <span>{t.quality}:</span>
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
              <span class="tooltip-text">{t.tooltipQuality}</span>
            </div>
          </div>
        </div>

        <!-- Grayscale Image Card -->
        <div class="image-card">
          <h4 class="card-head">{t.grayImage}</h4>
          <div class="card-controls">
            <div class="ctrl-row tooltip-wrapper tip-center">
              <span>{t.resolution}:</span>
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
                  <option value="450">450</option>
                </select>
              </div>
              <span class="tooltip-text">{t.tooltipDpi}</span>
            </div>
            <div class="ctrl-row tooltip-wrapper tip-center">
              <span>{t.threshold}:</span>
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
              <span class="tooltip-text">{t.tooltipThreshold}</span>
            </div>
            <div class="ctrl-row tooltip-wrapper tip-center">
              <span>{t.quality}:</span>
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
              <span class="tooltip-text">{t.tooltipQuality}</span>
            </div>
          </div>
        </div>

        <!-- Monochrome Image Card -->
        <div class="image-card">
          <h4 class="card-head">{t.monoImage}</h4>
          <div class="card-controls">
            <div class="ctrl-row tooltip-wrapper tip-right">
              <span>{t.resolution}:</span>
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
                  <option value="450">450</option>
                </select>
              </div>
              <span class="tooltip-text">{t.tooltipDpi}</span>
            </div>
            <div class="ctrl-row tooltip-wrapper tip-right">
              <span>{t.threshold}:</span>
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
              <span class="tooltip-text">{t.tooltipThreshold}</span>
            </div>
            <div class="ctrl-row tooltip-wrapper tip-right">
              <span>{t.binarization}:</span>
              <select class="select-box slim-select-box" bind:value={config.mono_img.mono_method}>
                <option value="threshold">{t.binOtsu}</option>
                <option value="dither">{t.binFloyd}</option>
              </select>
              <span class="tooltip-text">{t.tooltipMonoMethod}</span>
            </div>
          </div>
        </div>
      </div>
    {/if}

    <!-- Discard Settings Sub-tab -->
    {#if detailSubTab === 'discard'}
      <div class="discard-two-col">
        <div class="discard-box">
          <h4 class="section-subhead">{t.discardGroup}</h4>
          <div class="checkbox-list">
            <label class="checkbox-field tooltip-wrapper tip-left">
              <input type="checkbox" bind:checked={config.discard_comments} /> {t.discardComments}
              <span class="tooltip-text">{t.tooltipDiscardComments}</span>
            </label>
            <label class="checkbox-field tooltip-wrapper tip-left">
              <input type="checkbox" bind:checked={config.discard_links} /> {t.discardLinks}
              <span class="tooltip-text">{t.tooltipDiscardLinks}</span>
            </label>
            <label class="checkbox-field tooltip-wrapper tip-left">
              <input type="checkbox" bind:checked={config.discard_bookmarks} /> {t.discardBookmarks}
              <span class="tooltip-text">{t.tooltipDiscardBookmarks}</span>
            </label>
            <label class="checkbox-field tooltip-wrapper tip-left">
              <input type="checkbox" bind:checked={config.discard_forms} /> {t.discardForms}
              <span class="tooltip-text">{t.tooltipDiscardForms}</span>
            </label>
            <label class="checkbox-field tooltip-wrapper tip-left">
              <input type="checkbox" bind:checked={config.discard_metadata} /> {t.discardMetadata}
              <span class="tooltip-text">{t.tooltipDiscardMetadata}</span>
            </label>
            <label class="checkbox-field tooltip-wrapper tip-left">
              <input type="checkbox" bind:checked={config.discard_js} /> {t.discardJs}
              <span class="tooltip-text">{t.tooltipDiscardJs}</span>
            </label>
            <label class="checkbox-field tooltip-wrapper tip-left">
              <input type="checkbox" bind:checked={config.discard_thumbs} /> {t.discardThumbs}
              <span class="tooltip-text">{t.tooltipDiscardThumbs}</span>
            </label>
          </div>
        </div>

        <div class="discard-box">
          <h4 class="section-subhead">{t.cleanupGroup}</h4>
          <div class="checkbox-list">
            <label class="checkbox-field tooltip-wrapper tip-right">
              <input type="checkbox" bind:checked={config.clean_linearize} /> {t.cleanLinearize}
              <span class="tooltip-text">{t.tooltipCleanLinearize}</span>
            </label>
            <label class="checkbox-field tooltip-wrapper tip-right">
              <input type="checkbox" bind:checked={config.clean_deflate} /> {t.cleanDeflate}
              <span class="tooltip-text">{t.tooltipCleanDeflate}</span>
            </label>
            <label class="checkbox-field tooltip-wrapper tip-right">
              <input type="checkbox" bind:checked={config.clean_sanitize} /> {t.cleanSanitize}
              <span class="tooltip-text">{t.tooltipCleanSanitize}</span>
            </label>
          </div>
        </div>
      </div>
    {/if}

    <!-- Font Settings Sub-tab -->
    {#if detailSubTab === 'font'}
      <div class="font-panel-box">
        <label class="checkbox-field tooltip-wrapper tip-left">
          <input type="checkbox" bind:checked={config.font_subset} /> {t.fontSubset}
          <span class="tooltip-text">{t.tooltipFontSubset}</span>
        </label>
        
        <h4 class="section-subhead" style="margin-top: 10px;">{t.fontUnembedTitle}</h4>
        <p class="subtext">{t.fontUnembedDesc}</p>
        <div class="font-scroll-box">
          <ul>
            {#each detectedFonts as font}
              <!-- svelte-ignore a11y_no_noninteractive_element_to_interactive_role -->
              <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
              <li class="font-row" class:unembedded={config.unembed_fonts.includes(font)} ondblclick={() => onToggleFontUnembed(font)}>
                <span>{font}</span>
                <span class="tag">
                  {config.unembed_fonts.includes(font) ? t.targetUnembed : t.maintainEmbed}
                </span>
              </li>
            {/each}
          </ul>
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
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

  .font-panel-box {
    background-color: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: 6px;
    padding: 8px 10px;
  }

  .subtext {
    font-size: 0.72rem;
    color: var(--text-muted);
    margin: 0 0 6px 0;
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
</style>

