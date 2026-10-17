<script lang="ts">
  import { translations, presetExplanations } from "../utils/i18n";

  let {
    selectedPreset = $bindable("standard"),
    loadedProfile = $bindable(""),
    savedProfiles = [],
    currentLang = "en",
    onSelectPreset,
    onLoadProfile,
  }: {
    selectedPreset: string;
    loadedProfile: string;
    savedProfiles: string[];
    currentLang: "ja" | "en";
    onSelectPreset: (preset: string) => void;
    onLoadProfile: (profileName: string) => void;
  } = $props();

  const t = $derived(translations[currentLang]);
</script>

<div class="card-panel easy-panel">
  <h3 class="panel-title">{t.presets}</h3>
  <div class="preset-grid">
    <label class="radio-card" class:active={selectedPreset === 'high_quality'}>
      <input type="radio" name="preset" checked={selectedPreset === 'high_quality'} onchange={() => onSelectPreset('high_quality')} />
      <span>{t.presetHigh}</span>
    </label>
    <label class="radio-card" class:active={selectedPreset === 'standard'}>
      <input type="radio" name="preset" checked={selectedPreset === 'standard'} onchange={() => onSelectPreset('standard')} />
      <span>{t.presetStandard}</span>
    </label>
    <label class="radio-card" class:active={selectedPreset === 'low_quality'}>
      <input type="radio" name="preset" checked={selectedPreset === 'low_quality'} onchange={() => onSelectPreset('low_quality')} />
      <span>{t.presetLow}</span>
    </label>
    <label class="radio-card" class:active={selectedPreset === 'mono'}>
      <input type="radio" name="preset" checked={selectedPreset === 'mono'} onchange={() => onSelectPreset('mono')} />
      <span>{t.presetMono}</span>
    </label>
    <label class="radio-card" class:active={selectedPreset === 'rasterize'}>
      <input type="radio" name="preset" checked={selectedPreset === 'rasterize'} onchange={() => onSelectPreset('rasterize')} />
      <span>{t.presetRaster}</span>
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
              onLoadProfile(loadedProfile);
            } else if (savedProfiles.length > 0) {
              onLoadProfile(savedProfiles[0]);
            }
          }}
        />
        <span>{t.userProfile}</span>
      </label>
      <select
        class="select-box slim-select"
        value={loadedProfile}
        onchange={(e) => onLoadProfile(e.currentTarget.value)}
        disabled={savedProfiles.length === 0}
      >
        <option value="">{t.selectOption}</option>
        {#each savedProfiles as p}
          <option value={p}>{p}</option>
        {/each}
      </select>
    </div>
  </div>
  <div class="preset-info-box">
    <span class="info-label">{t.presetExplanation}:</span>
    <span class="info-text">
      {selectedPreset === 'user_profile'
        ? t.customExplanation
        : presetExplanations[currentLang][selectedPreset] || ''}
    </span>
  </div>
</div>

<style>
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

  .preset-info-box {
    background-color: rgba(15, 23, 42, 0.6);
    border: 1px solid var(--border-color);
    border-radius: 5px;
    padding: 6px 8px;
    font-size: 0.73rem;
    display: flex;
    gap: 6px;
  }

  .info-label {
    color: var(--accent-blue);
    font-weight: 600;
    white-space: nowrap;
  }

  .info-text {
    color: var(--text-muted);
    line-height: 1.35;
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

  .slim-select {
    padding: 2px 20px 2px 6px;
    font-size: 0.74rem;
    max-width: 140px;
  }
</style>

