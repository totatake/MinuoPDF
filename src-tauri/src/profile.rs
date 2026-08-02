use std::path::PathBuf;
use std::collections::HashMap;
use crate::optimizer::{OptimizeConfig, ImageConfig};

/// デフォルト設定（標準: 電子書籍・閲覧向け）を取得
pub fn get_default_config() -> OptimizeConfig {
    OptimizeConfig {
        version: 2,
        color_conv: "original".to_string(),
        color_img: ImageConfig {
            downsample: "bicubic".to_string(),
            target_dpi: 150,
            threshold_dpi: 225,
            format: "jpeg".to_string(),
            quality: 70,
            mono_method: None,
        },
        gray_img: ImageConfig {
            downsample: "bicubic".to_string(),
            target_dpi: 150,
            threshold_dpi: 225,
            format: "jpeg".to_string(),
            quality: 70,
            mono_method: None,
        },
        mono_img: ImageConfig {
            downsample: "bicubic".to_string(),
            target_dpi: 300,
            threshold_dpi: 450,
            format: "ccitt".to_string(),
            quality: 0,
            mono_method: Some("threshold".to_string()),
        },
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
        unembed_fonts: Vec::new(),
        force_rasterize: false,
    }
}

/// プリセットの定義を返す
pub fn get_preset_config(preset_name: &str) -> OptimizeConfig {
    let mut config = get_default_config();
    match preset_name {
        "high_quality" | "print" => {
            // 高画質 (印刷用下書き)
            config.color_conv = "original".to_string();
            config.color_img.target_dpi = 300;
            config.color_img.threshold_dpi = 450;
            config.color_img.quality = 85;
            config.gray_img.target_dpi = 300;
            config.gray_img.threshold_dpi = 450;
            config.gray_img.quality = 85;
            config.mono_img.target_dpi = 300;
            config.mono_img.threshold_dpi = 450;
            config.mono_img.format = "ccitt".to_string();
            config.mono_img.mono_method = Some("threshold".to_string());
            config.discard_comments = false;
            config.discard_links = false;
            config.discard_bookmarks = false;
            config.discard_forms = false;
            config.discard_metadata = false;
            config.discard_js = false;
            config.discard_thumbs = false;
            config.clean_linearize = false;
            config.clean_deflate = true;
            config.clean_sanitize = false;
            config.font_subset = true;
        }
        "standard" | "ebook" => {
            // 標準 (電子書籍・閲覧向け) - デフォルト通り
        }
        "low_quality" | "screen" => {
            // 軽量化 (Web表示・メール送信用)
            config.color_conv = "original".to_string();
            config.color_img.target_dpi = 72;
            config.color_img.threshold_dpi = 108;
            config.color_img.quality = 50;
            config.gray_img.target_dpi = 72;
            config.gray_img.threshold_dpi = 108;
            config.gray_img.quality = 50;
            config.mono_img.target_dpi = 150;
            config.mono_img.threshold_dpi = 225;
            config.mono_img.format = "ccitt".to_string();
            config.mono_img.mono_method = Some("threshold".to_string());
            config.discard_comments = true;
            config.discard_links = false;
            config.discard_bookmarks = false;
            config.discard_forms = false;
            config.discard_metadata = true;
            config.discard_js = true;
            config.discard_thumbs = true;
            config.clean_linearize = true;
            config.clean_deflate = true;
            config.clean_sanitize = true;
            config.font_subset = true;
        }
        "mono" => {
            // 白黒化 (スキャン文書・FAX向け)
            config.color_conv = "mono".to_string();
            config.color_img.target_dpi = 300;
            config.color_img.threshold_dpi = 450;
            config.color_img.format = "ccitt".to_string();
            config.color_img.quality = 60;
            config.gray_img.target_dpi = 300;
            config.gray_img.threshold_dpi = 450;
            config.gray_img.format = "ccitt".to_string();
            config.gray_img.quality = 60;
            config.mono_img.target_dpi = 300;
            config.mono_img.threshold_dpi = 450;
            config.mono_img.format = "ccitt".to_string();
            config.mono_img.mono_method = Some("threshold".to_string());
            config.discard_comments = true;
            config.discard_links = false;
            config.discard_bookmarks = false;
            config.discard_forms = false;
            config.discard_metadata = false;
            config.discard_js = true;
            config.discard_thumbs = true;
            config.clean_linearize = true;
            config.clean_deflate = true;
            config.clean_sanitize = true;
            config.font_subset = true;
        }
        "rasterize" => {
            // 強制画像化 (他設定で縮まないPDF用)
            config.color_conv = "gray".to_string();
            config.color_img.target_dpi = 150;
            config.color_img.threshold_dpi = 225;
            config.color_img.format = "jpeg".to_string();
            config.color_img.quality = 50;
            config.gray_img.target_dpi = 150;
            config.gray_img.threshold_dpi = 225;
            config.gray_img.format = "jpeg".to_string();
            config.gray_img.quality = 50;
            config.mono_img.target_dpi = 300;
            config.mono_img.threshold_dpi = 450;
            config.mono_img.format = "ccitt".to_string();
            config.mono_img.mono_method = Some("threshold".to_string());
            config.discard_comments = true;
            config.discard_links = true;
            config.discard_bookmarks = true;
            config.discard_forms = true;
            config.discard_metadata = true;
            config.discard_js = true;
            config.discard_thumbs = true;
            config.clean_linearize = true;
            config.clean_deflate = true;
            config.clean_sanitize = true;
            config.font_subset = true;
            config.force_rasterize = true;
        }
        _ => {}
    }
    config
}

/// プロファイルの保存先ディレクトリを取得する。(実行ファイルと同じディレクトリ直下)
pub fn get_profiles_dir() -> PathBuf {
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|parent| parent.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));
    
    let p_dir = exe_dir.join("profiles");
    let _ = std::fs::create_dir_all(&p_dir);
    p_dir
}

/// すべてのプロファイルをロードする
pub fn load_all_profiles() -> HashMap<String, OptimizeConfig> {
    let mut profiles = HashMap::new();
    let p_dir = get_profiles_dir();
    
    if let Ok(entries) = std::fs::read_dir(p_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        if let Ok(config) = serde_json::from_str::<OptimizeConfig>(&content) {
                            profiles.insert(name.to_string(), config);
                        }
                    }
                }
            }
        }
    }
    profiles
}

/// プロファイルを保存する
pub fn save_profile(name: &str, config: &OptimizeConfig) -> Result<(), String> {
    let p_dir = get_profiles_dir();
    let file_path = p_dir.join(format!("{}.json", name));
    
    let content = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize config: {:?}", e))?;
        
    std::fs::write(&file_path, content)
        .map_err(|e| format!("Failed to write profile file: {:?}", e))?;
        
    Ok(())
}

/// プロファイルを削除する
pub fn delete_profile(name: &str) -> Result<(), String> {
    let p_dir = get_profiles_dir();
    let file_path = p_dir.join(format!("{}.json", name));
    
    if file_path.exists() {
        std::fs::remove_file(file_path)
            .map_err(|e| format!("Failed to delete profile: {:?}", e))?;
    }
    Ok(())
}
