pub mod pdfium_util;
pub mod image_util;
pub mod optimizer;
pub mod rasterizer;
pub mod profile;
pub mod cli;
pub mod audit;
pub mod font_subsetter;

use tauri::{Emitter, Window};
use std::path::PathBuf;
use std::collections::{HashMap, HashSet};

use optimizer::{OptimizeConfig, optimize_pdf};
use rasterizer::rasterize_pdf;
use profile::{load_all_profiles, save_profile, delete_profile, get_default_config, get_preset_config};
use audit::{audit_pdf, PdfAuditComparison};

#[derive(Clone, serde::Serialize)]
struct ProgressPayload {
    file_path: String,
    current: usize,
    total: usize,
    message: String,
}

#[derive(Clone, serde::Serialize)]
struct LogPayload {
    file_path: String,
    success: bool,
    orig_size: u64,
    opt_size: u64,
    message: String,
    audit: Option<PdfAuditComparison>,
}

/// パスの正規化：WindowsパスのWSL2変換、および相対ファイルの自動探索を行う
pub fn normalize_pdf_path(path_str: &str) -> PathBuf {
    let raw_path = PathBuf::from(path_str);
    if raw_path.exists() {
        return raw_path;
    }

    // 1. WSL2/Linux環境におけるWindowsドライブ文字パス ("D:\..." -> "/mnt/d/...") の変換
    #[cfg(target_os = "linux")]
    {
        let normalized = path_str.replace('\\', "/");
        let bytes = normalized.as_bytes();
        if bytes.len() >= 2 && bytes[1] == b':' {
            let drive = (bytes[0] as char).to_ascii_lowercase();
            let rest = normalized[2..].trim_start_matches('/');
            let wsl_path = PathBuf::from(format!("/mnt/{}/{}", drive, rest));
            if wsl_path.exists() {
                return wsl_path;
            }
        }
    }

    // 2. 相対パス/ファイル名の場合、カレントディレクトリおよび主要ディレクトリから探索
    if let Ok(cwd) = std::env::current_dir() {
        let candidate = cwd.join(path_str);
        if candidate.exists() {
            return candidate;
        }
        if let Some(file_name) = raw_path.file_name() {
            let candidate_fn = cwd.join(file_name);
            if candidate_fn.exists() {
                return candidate_fn;
            }
            for sub in &["tests/fixtures", "test_assets", "tests", "static", "dist"] {
                let sub_candidate = cwd.join(sub).join(file_name);
                if sub_candidate.exists() {
                    return sub_candidate;
                }
            }
        }
    }

    raw_path
}

/// テストおよび処理用のフィクスチャPDFファイルパスを取得する（存在しない場合は動的ダウンロード）
pub fn get_fixture_path(filename: &str) -> PathBuf {
    let filename_only = std::path::Path::new(filename)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(filename);
    
    let base_dir = std::env::var("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| std::env::current_dir().unwrap_or_default());
    
    let mut fixture_path = base_dir.join("../tests/fixtures").join(filename_only);
    if !fixture_path.exists() {
        let alt_path = base_dir.join("tests/fixtures").join(filename_only);
        if alt_path.exists() {
            fixture_path = alt_path;
        }
    }

    if !fixture_path.exists() || std::fs::metadata(&fixture_path).map(|m| m.len() == 0).unwrap_or(true) {
        // scripts/ensure_fixtures.py を呼び出して動的にダウンロード
        let script_path = if base_dir.join("../scripts/ensure_fixtures.py").exists() {
            base_dir.join("../scripts/ensure_fixtures.py")
        } else if base_dir.join("scripts/ensure_fixtures.py").exists() {
            base_dir.join("scripts/ensure_fixtures.py")
        } else {
            PathBuf::from("scripts/ensure_fixtures.py")
        };

        if script_path.exists() {
            let _ = std::process::Command::new("python")
                .arg(&script_path)
                .arg("--file")
                .arg(filename_only)
                .status();
        }
    }

    fixture_path
}

#[tauri::command]
async fn get_default_config_cmd() -> OptimizeConfig {
    get_default_config()
}

#[tauri::command]
async fn get_preset_config_cmd(preset: String) -> OptimizeConfig {
    get_preset_config(&preset)
}

#[tauri::command]
async fn load_profiles_cmd() -> HashMap<String, OptimizeConfig> {
    load_all_profiles()
}

#[tauri::command]
async fn save_profile_cmd(name: String, config: OptimizeConfig) -> Result<(), String> {
    save_profile(&name, &config)
}

#[tauri::command]
async fn delete_profile_cmd(name: String) -> Result<(), String> {
    delete_profile(&name)
}

#[tauri::command]
async fn scan_fonts_cmd(input_paths: Vec<String>) -> Result<Vec<String>, String> {
    let mut all_fonts = HashSet::new();
    for path_str in input_paths {
        let path = normalize_pdf_path(&path_str);
        if path.exists() {
            if let Ok(fonts) = optimizer::scan_pdf_fonts(&path) {
                for font in fonts {
                    all_fonts.insert(font.name);
                }
            }
        }
    }
    let mut result: Vec<String> = all_fonts.into_iter().collect();
    result.sort();
    Ok(result)
}


#[tauri::command]
async fn audit_pdf_cmd(path: String) -> Result<PdfAuditComparison, String> {
    let input_path = normalize_pdf_path(&path);
    if !input_path.exists() {
        return Err(format!("File not found: {}", path));
    }

    let orig_report = audit_pdf(&input_path)?;

    // 最適化済みファイルのパス候補 ([stem]_optimized.pdf) をチェック
    let ext = input_path.extension().and_then(|s| s.to_str()).unwrap_or("pdf");
    let stem = input_path.file_stem().and_then(|s| s.to_str()).unwrap_or("output");
    let out_dir = input_path.parent().unwrap_or_else(|| std::path::Path::new("."));
    let opt_path = out_dir.join(format!("{}_optimized.{}", stem, ext));

    let opt_report = if opt_path.exists() {
        audit_pdf(&opt_path).ok()
    } else {
        None
    };

    Ok(PdfAuditComparison {
        original: orig_report,
        optimized: opt_report,
    })
}

#[tauri::command]
async fn optimize_pdf_cmd(
    window: Window,
    input_paths: Vec<String>,
    config: OptimizeConfig,
    strategy: String,
) -> Result<(), String> {
    for path_str in &input_paths {
        let path = normalize_pdf_path(path_str);
        let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("Unknown").to_string();
        
        let path_str_clone = path_str.clone();
        let window_clone = window.clone();
        
        // 進捗コールバック
        let file_name_for_cb = file_name.clone();
        let progress_cb = move |curr, tot| {
            let _ = window_clone.emit("optimization-progress", ProgressPayload {
                file_path: path_str_clone.clone(),
                current: curr,
                total: tot,
                message: format!("Processing {}... ({}/{})", file_name_for_cb, curr, tot),
            });
        };

        // 変換前容量調査
        let orig_audit = audit_pdf(&path).ok();

        // 出力先を [元ファイル名]_optimized.pdf に設定
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("pdf");
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("output");
        let out_dir = path.parent().unwrap_or_else(|| std::path::Path::new("."));
        let output_path = out_dir.join(format!("{}_optimized.{}", stem, ext));

        // 実行
        let result = if strategy == "rasterize" {
            rasterize_pdf(&path, &output_path, &config, progress_cb)
        } else {
            optimize_pdf(&path, &output_path, &config, progress_cb)
        };

        match result {
            Ok((orig, opt)) => {
                // 変換後容量調査
                let opt_audit = audit_pdf(&output_path).ok();
                let audit_comp = orig_audit.map(|o| PdfAuditComparison {
                    original: o,
                    optimized: opt_audit,
                });

                let _ = window.emit("optimization-log", LogPayload {
                    file_path: path_str.clone(),
                    success: true,
                    orig_size: orig,
                    opt_size: opt,
                    message: format!(
                        "Success {}: {} KB -> {} KB ({:.1}% reduction)",
                        file_name,
                        orig / 1024,
                        opt / 1024,
                        if orig > 0 { (1.0 - (opt as f64 / orig as f64)) * 100.0 } else { 0.0 }
                    ),
                    audit: audit_comp,
                });
            }
            Err(e) => {
                let _ = window.emit("optimization-log", LogPayload {
                    file_path: path_str.clone(),
                    success: false,
                    orig_size: 0,
                    opt_size: 0,
                    message: format!("Failed {}: {}", file_name, e),
                    audit: None,
                });
            }
        }
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            get_default_config_cmd,
            get_preset_config_cmd,
            load_profiles_cmd,
            save_profile_cmd,
            delete_profile_cmd,
            optimize_pdf_cmd,
            scan_fonts_cmd,
            audit_pdf_cmd
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profile::get_profiles_dir;
    use crate::image_util::{apply_otsu, encode_g4};
    use image::{ImageBuffer, Luma};

    #[test]
    fn test_profile_dir_writable() {
        let p_dir = get_profiles_dir();
        assert!(p_dir.exists());
        let test_file = p_dir.join(".test_write_from_unit_test");
        std::fs::write(&test_file, b"test").unwrap();
        assert!(test_file.exists());
        std::fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_otsu_thresholding() {
        let mut img = ImageBuffer::new(10, 10);
        for y in 0..10 {
            for x in 0..10 {
                if x < 5 {
                    img.put_pixel(x, y, Luma([0]));
                } else {
                    img.put_pixel(x, y, Luma([200]));
                }
            }
        }
        let binarized = apply_otsu(&img);
        assert_eq!(binarized.get_pixel(0, 0)[0], 0);
        assert_eq!(binarized.get_pixel(9, 9)[0], 255);
    }

    #[test]
    fn test_g4_encoding() {
        let mut img = ImageBuffer::new(20, 20);
        for y in 0..20 {
            for x in 0..20 {
                img.put_pixel(x, y, Luma([if (x + y) % 2 == 0 { 0 } else { 255 }]));
            }
        }
        let encoded = encode_g4(&img);
        assert!(encoded.is_ok());
        assert!(!encoded.unwrap().is_empty());
    }

    #[test]
    fn test_profile_save_load_delete() {
        use crate::profile::{save_profile, load_all_profiles, delete_profile, get_default_config};
        let name = "___test_profile_from_unit_test___";
        let mut config = get_default_config();
        config.color_img.quality = 99;
        
        // Save
        let res = save_profile(name, &config);
        assert!(res.is_ok());
        
        // Load all and check
        let profiles = load_all_profiles();
        assert!(profiles.contains_key(name));
        let loaded = profiles.get(name).unwrap();
        assert_eq!(loaded.color_img.quality, 99);
        
        // Delete
        let del_res = delete_profile(name);
        assert!(del_res.is_ok());
        
        // Load all again and verify deletion
        let profiles_after = load_all_profiles();
        assert!(!profiles_after.contains_key(name));
    }

    #[test]
    fn test_preset_configs() {
        use crate::profile::get_preset_config;
        let standard = get_preset_config("standard");
        assert_eq!(standard.color_img.target_dpi, 150);
        assert_eq!(standard.version, 2);
        
        let high = get_preset_config("high_quality");
        assert_eq!(high.color_img.target_dpi, 300);
        assert_eq!(high.color_img.quality, 85);
        
        let low = get_preset_config("low_quality");
        assert_eq!(low.color_img.target_dpi, 72);
        assert_eq!(low.color_img.quality, 50);

        let mono = get_preset_config("mono");
        assert_eq!(mono.color_conv, "mono");

        let rasterize = get_preset_config("rasterize");
        assert!(rasterize.force_rasterize);
    }

    #[test]
    fn test_config_version() {
        let default_cfg = get_default_config();
        assert_eq!(default_cfg.version, 2);
    }
}
