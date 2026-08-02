use pdfium_render::prelude::*;

/// 実行ファイルのパスやカレントディレクトリから pdfium.dll / libpdfium.so を探索し、
/// またはシステムライブラリ（Linux等）から Pdfium をロードします。
pub fn load_pdfium() -> Result<Pdfium, String> {
    let current_dir = std::env::current_dir().ok();
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|parent| parent.to_path_buf()));

    let candidates: &[&str] = if cfg!(target_os = "windows") {
        &["pdfium.dll"]
    } else if cfg!(target_os = "macos") {
        &["libpdfium.dylib", "pdfium.dylib"]
    } else {
        &["libpdfium.so", "pdfium.so"]
    };

    let find_in_dir = |dir: &std::path::Path| -> Option<std::path::PathBuf> {
        for candidate in candidates {
            let p = dir.join(candidate);
            if p.exists() {
                return Some(p);
            }
        }
        None
    };

    let mut dll_path = None;

    // 1. カレントディレクトリ直下
    if let Some(ref cur) = current_dir {
        dll_path = find_in_dir(cur);
    }

    // 2. 実行ファイルと同じディレクトリ直下
    if dll_path.is_none() {
        if let Some(ref dir) = exe_dir {
            dll_path = find_in_dir(dir);
        }
    }

    // 3. 実行ファイルの resources/
    if dll_path.is_none() {
        if let Some(ref dir) = exe_dir {
            dll_path = find_in_dir(&dir.join("resources"));
        }
    }

    // 4. カレントディレクトリの resources/
    if dll_path.is_none() {
        if let Some(ref cur) = current_dir {
            dll_path = find_in_dir(&cur.join("resources"));
        }
    }

    let bindings = if let Some(path) = dll_path {
        // ロードの安定化のために確実に絶対パス化する
        let abs_path = if path.is_absolute() {
            path
        } else {
            std::env::current_dir().unwrap_or_default().join(path)
        };
        let path_str = abs_path.to_string_lossy().to_string();
        Pdfium::bind_to_library(&path_str)
    } else {
        Pdfium::bind_to_system_library()
    };

    match bindings {
        Ok(b) => Ok(Pdfium::new(b)),
        Err(e) => Err(format!(
            "Failed to load Pdfium library: {:?}. Ensure pdfium.dll/libpdfium.so is installed/placed properly.",
            e
        )),
    }
}
