use std::io::{self, Write};
use std::path::Path;
use crate::optimizer::optimize_pdf;
use crate::rasterizer::rasterize_pdf;
use crate::profile::get_default_config;

/// ユーザーがキーを押すまで待機する
pub fn wait_exit() {
    println!("\nPress Enter to exit / Enterキーを押して終了...");
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
}

/// 選択肢をコンソールに表示して入力を促す (0-indexed)
fn prompt_selection(title: &str, options: &[&str]) -> usize {
    loop {
        println!("\n{}", title);
        for (i, opt) in options.iter().enumerate() {
            println!("  [{}] {}", i + 1, opt);
        }
        print!("Enter choice (1-{}): ", options.len());
        let _ = io::stdout().flush();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            if let Ok(num) = input.trim().parse::<usize>() {
                if num >= 1 && num <= options.len() {
                    return num - 1;
                }
            }
        }
        println!("Invalid selection. Please try again. / 選択が無効です。もう一度入力してください。");
    }
}

/// 数値入力を促す
fn prompt_int(title: &str, min: u8, max: u8, default: u8) -> u8 {
    loop {
        print!("{} [default: {}]: ", title, default);
        let _ = io::stdout().flush();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            let trimmed = input.trim();
            if trimmed.is_empty() {
                return default;
            }
            if let Ok(num) = trimmed.parse::<u8>() {
                if num >= min && num <= max {
                    return num;
                }
            }
        }
        println!("Invalid input. Please enter a number between {} and {}. / 無効な入力です。{}から{}の数字を入力してください。", min, max, min, max);
    }
}

/// プログレスバーをコンソールに描画する
fn print_progress_bar(current: usize, total: usize) {
    let width = 30;
    let total_val = if total == 0 { 1 } else { total };
    let ratio = current as f32 / total_val as f32;
    let filled = (ratio * width as f32).round() as usize;
    let filled = filled.min(width);
    let empty = width - filled;
    
    let bar = format!(
        "\r[{}{}] {}% ({}/{})",
        "#".repeat(filled),
        "-".repeat(empty),
        (ratio * 100.0) as u32,
        current,
        total
    );
    let _ = io::stdout().write_all(bar.as_bytes());
    let _ = io::stdout().flush();
}

/// CLI対話モードのメインルーチン
pub fn run_cli(input_path_str: &str) {
    let input_path = crate::normalize_pdf_path(input_path_str);
    if !input_path.exists() {
        println!("Error: Input file does not exist / 入力ファイルが存在しません: {}", input_path_str);
        wait_exit();
        return;
    }

    println!("\n=========================================");
    println!("        PDF Optimizer - CLI Mode");
    println!("=========================================");
    
    // 0. 言語選択
    let lang_choice = prompt_selection(
        "Select Language / 言語を選択してください:",
        &[
            "English",
            "日本語"
        ]
    );
    let is_ja = lang_choice == 1;

    let msg_input_file = if is_ja { "入力ファイル" } else { "Input File" };
    println!("{}: {}", msg_input_file, input_path.display());

    // 1. カラーモード選択
    let msg_select_color = if is_ja { "カラーモードを選択してください:" } else { "Select Color Mode:" };
    let opt_colors = if is_ja {
        vec!["カラー (元の色を保持)", "グレースケール化", "白黒二値化 (1-bit)"]
    } else {
        vec!["Color (Keep original colors)", "Gray (Grayscale)", "Mono (Black & White 1-bit)"]
    };
    let color_mode = prompt_selection(msg_select_color, &opt_colors);

    // 2. DPI選択
    let msg_select_dpi = if is_ja { "目標解像度 (DPI) を選択してください:" } else { "Select DPI Resolution:" };
    let opt_dpis = if is_ja {
        vec![
            "50 DPI (極めて低画質、最小サイズ)",
            "72 DPI (標準的な画面表示用)",
            "100 DPI (Web閲覧に最適)",
            "150 DPI (標準閲覧・アーカイブ推奨)",
            "300 DPI (印刷用の下書き画質)"
        ]
    } else {
        vec![
            "50 DPI (Extremely low quality, smallest size)",
            "72 DPI (Standard screen layout)",
            "100 DPI (Web reading optimized)",
            "150 DPI (Standard reading & archiving quality)",
            "300 DPI (Printable draft quality)"
        ]
    };
    let dpi_idx = prompt_selection(msg_select_dpi, &opt_dpis);
    let dpi = match dpi_idx {
        0 => 50,
        1 => 72,
        2 => 100,
        3 => 150,
        _ => 300,
    };

    // 3. 詳細設定値の適用
    let mut config = get_default_config();
    config.color_conv = match color_mode {
        0 => "original".to_string(),
        1 => "gray".to_string(),
        _ => "mono".to_string(),
    };

    if color_mode == 2 {
        // Mono の場合は二値化手法
        let msg_select_bin = if is_ja { "二値化手法を選択してください:" } else { "Select Binarization Method:" };
        let opt_bins = if is_ja {
            vec![
                "大津の二値化 (自動閾値、テキスト向け)",
                "ディザリング (グラデーション・写真向け)"
            ]
        } else {
            vec![
                "Otsu Thresholding (Auto threshold, clean text)",
                "Floyd-Steinberg Dithering (Error diffusion, preserves grayscale/photos)"
            ]
        };
        let mono_idx = prompt_selection(msg_select_bin, &opt_bins);
        let method = if mono_idx == 0 { "threshold" } else { "dither" };
        config.mono_img.target_dpi = dpi;
        config.mono_img.threshold_dpi = (dpi as f32 * 1.5) as u32;
        config.mono_img.mono_method = Some(method.to_string());
    } else {
        // Color/Gray の場合はJPEGクオリティ
        let msg_enter_quality = if is_ja { "JPEG画質を入力してください (10-100)" } else { "Enter JPEG Quality (10-100)" };
        let quality = prompt_int(msg_enter_quality, 10, 100, 70);
        if color_mode == 1 {
            config.gray_img.target_dpi = dpi;
            config.gray_img.threshold_dpi = (dpi as f32 * 1.5) as u32;
            config.gray_img.quality = quality;
        } else {
            config.color_img.target_dpi = dpi;
            config.color_img.threshold_dpi = (dpi as f32 * 1.5) as u32;
            config.color_img.quality = quality;
        }
    }

    // 4. 圧縮戦略選択
    let msg_select_strat = if is_ja { "圧縮戦略を選択してください:" } else { "Select Optimization Strategy:" };
    let opt_strats = if is_ja {
        vec![
            "テキスト保持モード (画像のみ再圧縮、テキストとフォント維持)",
            "完全ラスタライズモード (ページ全体を画像化、検索不可になるが確実に圧縮)"
        ]
    } else {
        vec![
            "Text-Preserving Mode (Replaces image bytes, keeps texts & fonts)",
            "Full Rasterization Mode (Renders pages to images, absolute compression, no searchable text)"
        ]
    };
    let strategy = prompt_selection(msg_select_strat, &opt_strats);

    // 出力パス生成
    let ext = input_path.extension().and_then(|s| s.to_str()).unwrap_or("pdf");
    let stem = input_path.file_stem().and_then(|s| s.to_str()).unwrap_or("output");
    let out_dir = input_path.parent().unwrap_or_else(|| Path::new("."));
    let output_path = out_dir.join(format!("{}_optimized.{}", stem, ext));

    let msg_optimizing = if is_ja { "PDFを最適化しています..." } else { "Optimizing PDF..." };
    let msg_saving_to = if is_ja { "出力ファイル保存先:" } else { "output will be saved to:" };
    println!("\n-----------------------------------------");
    println!("{}", msg_optimizing);
    println!("{} >> {}", msg_saving_to, output_path.display());
    println!("-----------------------------------------");

    let progress_cb = |current, total| {
        print_progress_bar(current, total);
    };

    // 実行
    let result = if strategy == 0 {
        optimize_pdf(&input_path, &output_path, &config, progress_cb)
    } else {
        rasterize_pdf(&input_path, &output_path, &config, progress_cb)
    };

    println!(); // 改行

    let msg_success = if is_ja { "成功! 完了しました" } else { "Success! Finished" };
    let msg_orig_size = if is_ja { "元のサイズ" } else { "Original size" };
    let msg_opt_size = if is_ja { "最適化後のサイズ" } else { "Optimized size" };
    let msg_savings = if is_ja { "容量削減率" } else { "Size reduction" };
    let msg_failed = if is_ja { "最適化に失敗しました" } else { "Optimization Failed" };

    match result {
        Ok((orig, opt)) => {
            println!("\n=========================================");
            println!("           {}", msg_success);
            println!("=========================================");
            println!("{}:  {:.2} MB ({} bytes)", msg_orig_size, orig as f64 / 1024.0 / 1024.0, orig);
            println!("{}: {:.2} MB ({} bytes)", msg_opt_size, opt as f64 / 1024.0 / 1024.0, opt);
            if orig > 0 {
                let savings = (1.0 - (opt as f64 / orig as f64)) * 100.0;
                println!("{}: {:.1}%", msg_savings, savings);
            }
        }
        Err(e) => {
            println!("\n{}: {}", msg_failed, e);
        }
    }

    wait_exit();
}

/// 非対話（バッチ・スクリプト・CI用）CLI実行ルーチン
pub fn run_batch_cli(
    input_path_str: &str,
    output_path_opt: Option<&str>,
    preset_opt: Option<&str>,
    profile_opt: Option<&str>,
    strategy_opt: Option<&str>,
) -> Result<(), String> {
    let input_path = crate::normalize_pdf_path(input_path_str);
    if !input_path.exists() {
        return Err(format!("Input file does not exist: {}", input_path_str));
    }

    let mut config = if let Some(profile_name) = profile_opt {
        let profiles = crate::profile::load_all_profiles();
        profiles
            .get(profile_name)
            .cloned()
            .ok_or_else(|| format!("Profile '{}' not found in profiles/ directory", profile_name))?
    } else if let Some(preset_name) = preset_opt {
        crate::profile::get_preset_config(preset_name)
    } else {
        crate::profile::get_default_config()
    };

    let use_rasterize = if let Some(strat) = strategy_opt {
        strat.to_lowercase() == "rasterize"
    } else {
        config.force_rasterize
    };
    config.force_rasterize = use_rasterize;

    let output_path = if let Some(out_str) = output_path_opt {
        crate::normalize_pdf_path(out_str)
    } else {
        let ext = input_path.extension().and_then(|s| s.to_str()).unwrap_or("pdf");
        let stem = input_path.file_stem().and_then(|s| s.to_str()).unwrap_or("output");
        let out_dir = input_path.parent().unwrap_or_else(|| Path::new("."));
        out_dir.join(format!("{}_optimized.{}", stem, ext))
    };

    println!("Optimizing '{}' -> '{}'...", input_path.display(), output_path.display());
    let progress_cb = |current, total| {
        print_progress_bar(current, total);
    };

    let result = if use_rasterize {
        rasterize_pdf(&input_path, &output_path, &config, progress_cb)
    } else {
        optimize_pdf(&input_path, &output_path, &config, progress_cb)
    };

    println!();
    match result {
        Ok((orig, opt)) => {
            let savings = if orig > 0 { (1.0 - (opt as f64 / orig as f64)) * 100.0 } else { 0.0 };
            println!(
                "Success: {} bytes ({:.2} MB) -> {} bytes ({:.2} MB) [-{:.1}%]",
                orig, orig as f64 / 1024.0 / 1024.0,
                opt, opt as f64 / 1024.0 / 1024.0,
                savings
            );
            Ok(())
        }
        Err(e) => Err(format!("Optimization failed: {}", e)),
    }
}

