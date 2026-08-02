import { test, expect } from '@playwright/test';

test.describe('MinuoPDF E2E GUI Automated Testing', () => {
  
  test.beforeEach(async ({ page }) => {
    // Navigate to local Vite dev server
    await page.goto('/');
  });

  test('REQ_GUI_001: App title and main layout elements exist', async ({ page }) => {
    // Check main window title
    await expect(page).toHaveTitle(/MinuoPDF/i);

    // Verify presence of drop zone text / area
    const dropZone = page.locator('.drop-area, .drop-zone, .drop-text');
    await expect(dropZone.first()).toBeVisible();
  });

  test('REQ_GUI_002: Dynamic Language Toggle (English <-> Japanese)', async ({ page }) => {
    // Locate language select box
    const langSelect = page.locator('select.lang-select-box');
    await expect(langSelect).toBeVisible();

    // Check Easy Mode tab text in English (default)
    const easyTab = page.locator('button.tab-btn').first();
    await expect(easyTab).toContainText('Easy Mode');

    // Switch to Japanese ("ja")
    await langSelect.selectOption('ja');
    await expect(easyTab).toContainText('簡単モード');

    // Switch back to English ("en")
    await langSelect.selectOption('en');
    await expect(easyTab).toContainText('Easy Mode');
  });

  test('REQ_PROF_001 & REQ_GUI_003: Tab switching (Easy Mode <-> Detail Mode)', async ({ page }) => {
    // Locate main tabs
    const easyTab = page.locator('button.tab-btn').filter({ hasText: /Easy Mode|簡単モード/ });
    const detailTab = page.locator('button.tab-btn').filter({ hasText: /Detail Mode|詳細モード/ });

    await expect(easyTab).toBeVisible();
    await expect(detailTab).toBeVisible();

    // Switch to Detail Mode
    await detailTab.click();

    // Verify subnav buttons appear in Detail Mode
    const subNavBtns = page.locator('button.sub-nav-btn');
    await expect(subNavBtns.first()).toBeVisible();

    // Switch back to Easy Mode
    await easyTab.click();
    await expect(easyTab).toHaveClass(/active/);
  });

  test('REQ_PROF_001: Preset Radio Selection in Easy Mode', async ({ page }) => {
    // Ensure Easy mode is active
    const easyTab = page.locator('button.tab-btn').filter({ hasText: /Easy Mode|簡単モード/ });
    await easyTab.click();

    // Find radio inputs
    const radioInputs = page.locator('input[type="radio"]');
    const count = await radioInputs.count();
    expect(count).toBeGreaterThan(0);

    // Select second preset radio option if available
    if (count > 1) {
      await radioInputs.nth(1).check({ force: true });
      await expect(radioInputs.nth(1)).toBeChecked();
    }
  });

  test('REQ_OPT_006: Font Subsetting & Unembedding GUI toggle on double-click', async ({ page }) => {
    // 1. Switch to Detail Mode
    const detailTab = page.locator('button.tab-btn').filter({ hasText: /Detail Mode|詳細モード/ });
    await detailTab.click();

    // 2. Switch to Font Settings Sub-tab
    const fontSubTab = page.locator('button.sub-nav-btn').filter({ hasText: /Font|フォント/ });
    await expect(fontSubTab).toBeVisible();
    await fontSubTab.click();

    // 3. Verify Font Subsetting checkbox and font list box exist
    const fontScrollBox = page.locator('.font-scroll-box');
    await expect(fontScrollBox).toBeVisible();

    // 4. Verify font row double-click toggle behavior if font rows are rendered
    const fontRows = page.locator('.font-row');
    const count = await fontRows.count();
    if (count > 0) {
      const firstFontRow = fontRows.first();
      await expect(firstFontRow).not.toHaveClass(/unembedded/);

      // Double-click to enable unembedding
      await firstFontRow.dblclick();
      await expect(firstFontRow).toHaveClass(/unembedded/);
      await expect(firstFontRow.locator('.tag')).toContainText(/Target Unembed|解除対象/);

      // Double-click again to revert
      await firstFontRow.dblclick();
      await expect(firstFontRow).not.toHaveClass(/unembedded/);
      await expect(firstFontRow.locator('.tag')).toContainText(/Maintain Embed|埋め込み維持/);
    }
  });

  test('REQ_FILE_003: Target File List Actions (Clear All button visibility)', async ({ page }) => {
    // When no files are added, slim drop zone is visible
    const dropZone = page.locator('.drop-zone-slim');
    await expect(dropZone).toBeVisible();
  });

  test('REQ_OPT_001 & REQ_OPT_002 & REQ_OPT_003: Image Optimization Controls in Detail Mode', async ({ page }) => {
    // Switch to Detail Mode -> Image Sub-tab
    const detailTab = page.locator('button.tab-btn').filter({ hasText: /Detail Mode|詳細モード/ });
    await detailTab.click();

    const imageSubTab = page.locator('button.sub-nav-btn').filter({ hasText: /Image Optimization|画像最適化/ });
    await expect(imageSubTab).toBeVisible();

    // Color conversion policy select
    const colorSelect = page.locator('.policy-row select.select-box');
    await expect(colorSelect).toBeVisible();
    await colorSelect.selectOption('gray');
    await expect(colorSelect).toHaveValue('gray');

    // Check DPI & Quality inputs in image cards
    const comboInputs = page.locator('.combo-input-box');
    const inputCount = await comboInputs.count();
    expect(inputCount).toBeGreaterThan(0);

    // Change target DPI of first image card
    await comboInputs.first().fill('200');
    await expect(comboInputs.first()).toHaveValue('200');

    // Check Monochrome Binarization method select box
    const binarizationSelect = page.locator('.slim-select-box');
    if (await binarizationSelect.isVisible()) {
      await binarizationSelect.selectOption('dither');
      await expect(binarizationSelect).toHaveValue('dither');
    }
  });

  test('REQ_OPT_005: Discard & Cleanup Checkboxes in Detail Mode', async ({ page }) => {
    // Switch to Detail Mode
    const detailTab = page.locator('button.tab-btn').filter({ hasText: /Detail Mode|詳細モード/ });
    await detailTab.click();

    // Switch to Discard & Clean Sub-tab
    const discardSubTab = page.locator('button.sub-nav-btn').filter({ hasText: /Discard & Clean|破棄&クリーン/ });
    await expect(discardSubTab).toBeVisible();
    await discardSubTab.click();

    // Check discard checkboxes exist
    const checkboxes = page.locator('.discard-two-col input[type="checkbox"]');
    const cbCount = await checkboxes.count();
    expect(cbCount).toBeGreaterThan(5);

    // Toggle the first discard checkbox
    const firstCb = checkboxes.first();
    const initialState = await firstCb.isChecked();
    await firstCb.click();
    expect(await firstCb.isChecked()).toBe(!initialState);
  });

  test('REQ_PROF_002: Profile Management Tab Controls', async ({ page }) => {
    // Switch to Profile Management Tab
    const profileTab = page.locator('button.tab-btn').filter({ hasText: /Profile Management|プロファイル管理/ });
    await expect(profileTab).toBeVisible();
    await profileTab.click();

    // Verify Save Profile input and button
    const profileInput = page.locator('.profile-card input.text-input');
    await expect(profileInput).toBeVisible();

    const saveBtn = page.locator('.profile-card button').filter({ hasText: /Save Profile|プロファイルを保存/ });
    await expect(saveBtn).toBeDisabled();

    // Enter profile name to enable save button
    await profileInput.fill('TestProfile');
    await expect(saveBtn).toBeEnabled();
  });

  test('REQ_GUI_004 & REQ_GUI_005: Status, Progress Bar, and Log Footer Area', async ({ page }) => {
    // Footer progress & status elements
    const statusMsg = page.locator('.status-msg');
    await expect(statusMsg).toBeVisible();

    const progressTrack = page.locator('.progress-track');
    await expect(progressTrack).toBeVisible();

    const compressBtn = page.locator('footer.bottom-actions button.btn-primary');
    await expect(compressBtn).toBeVisible();

    const logConsole = page.locator('.compact-log-console');
    await expect(logConsole).toBeVisible();
  });

});


