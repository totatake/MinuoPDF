<script lang="ts">
  import { translations } from "../utils/i18n";

  let {
    savedProfiles = [],
    newProfileName = $bindable(""),
    currentLang = "en",
    onSaveProfile,
    onDeleteProfile,
  }: {
    savedProfiles: string[];
    newProfileName: string;
    currentLang: "ja" | "en";
    onSaveProfile: () => void;
    onDeleteProfile: (profileName: string) => void;
  } = $props();

  const t = $derived(translations[currentLang]);
</script>

<div class="profile-panel">
  <div class="profile-card">
    <h4 class="section-subhead">{t.saveCurrentProfile}</h4>
    <div class="input-inline">
      <input type="text" class="text-input" placeholder={t.profileNamePlaceholder} bind:value={newProfileName} />
      <button class="btn btn-sm btn-primary" onclick={onSaveProfile} disabled={!newProfileName}>{t.saveBtn}</button>
    </div>
  </div>

  <div class="profile-card">
    <h4 class="section-subhead">{t.savedProfilesTitle}</h4>
    {#if savedProfiles.length === 0}
      <p class="subtext">{t.noSavedProfiles}</p>
    {:else}
      <ul class="profile-list">
        {#each savedProfiles as p}
          <li class="profile-item">
            <span>{p}</span>
            <button class="btn-text-danger" onclick={() => onDeleteProfile(p)}>{t.deleteBtn}</button>
          </li>
        {/each}
      </ul>
    {/if}
  </div>
</div>

<style>
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

  .section-subhead {
    margin: 0 0 6px 0;
    font-size: 0.8rem;
    color: var(--accent-blue);
  }

  .subtext {
    font-size: 0.72rem;
    color: var(--text-muted);
    margin: 0 0 6px 0;
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
    box-sizing: border-box;
  }

  .text-input:focus {
    border-color: var(--accent-blue);
    outline: none;
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

  .btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .btn-text-danger {
    background: none;
    border: none;
    color: var(--accent-danger);
    cursor: pointer;
    font-size: 0.73rem;
  }

  .btn-text-danger:hover {
    filter: brightness(1.2);
  }
</style>

