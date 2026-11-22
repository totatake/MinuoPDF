use std::env;

#[cfg(target_os = "windows")]
mod win32 {
    use std::ffi::c_void;
    type HWND = *mut c_void;

    #[link(name = "kernel32")]
    extern "system" {
        pub fn GetConsoleWindow() -> HWND;
        pub fn GetConsoleProcessList(lpdwProcessList: *mut u32, dwProcessCount: u32) -> u32;
        pub fn FreeConsole() -> i32;
    }

    #[link(name = "user32")]
    extern "system" {
        pub fn ShowWindow(hWnd: HWND, nCmdShow: i32) -> i32;
    }
}

#[cfg(target_os = "windows")]
fn hide_console() {
    unsafe {
        let mut pids = [0u32; 2];
        let process_count = win32::GetConsoleProcessList(pids.as_mut_ptr(), 2);
        if process_count == 1 {
            let hwnd = win32::GetConsoleWindow();
            if !hwnd.is_null() {
                win32::ShowWindow(hwnd, 0); // SW_HIDE = 0
            }
        }
        let _ = win32::FreeConsole();
    }
}

#[cfg(not(target_os = "windows"))]
fn hide_console() {}

fn print_help() {
    println!("MinuoPDF - Non-GPL / Lightweight PDF Optimization Tool");
    println!("MinuoPDF - 非GPL / 軽量PDF最適化ツール");
    println!("\nUsage:");
    println!("  MinuoPDF                       (GUI Mode)");
    println!("  MinuoPDF <input.pdf>           (Interactive CUI Mode / D&D)");
    println!("  MinuoPDF <input.pdf...> [opts] (Non-interactive Batch Mode)");
    println!("\nOptions:");
    println!("  -o, --output <FILE|DIR>        Output PDF file path (or dir for multiple inputs)");
    println!("  -p, --preset <PRESET>          Preset: high_quality | standard | low_quality | light | mono | rasterize");
    println!("  --profile <NAME>               Saved JSON profile name in profiles/");
    println!("  -s, --strategy <STRATEGY>      Strategy: text | keep-text | rasterize");
    println!("  -b, --batch [DIR]              Force batch mode (optional output directory)");
    println!("  -h, --help                     Print this help message");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        // Any occurrence of -h / --help prints help and exits
        if args.iter().skip(1).any(|a| a == "--help" || a == "-h" || a == "help") {
            print_help();
            return;
        }

        let mut input_paths: Vec<String> = Vec::new();
        let mut output_path = None;
        let mut preset = None;
        let mut profile = None;
        let mut strategy = None;
        let mut force_batch = false;

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "-o" | "--output" => {
                    if i + 1 < args.len() && !args[i + 1].starts_with('-') {
                        output_path = Some(args[i + 1].clone());
                        i += 1;
                    }
                }
                "-p" | "--preset" => {
                    if i + 1 < args.len() && !args[i + 1].starts_with('-') {
                        preset = Some(args[i + 1].clone());
                        i += 1;
                    }
                }
                "--profile" => {
                    if i + 1 < args.len() && !args[i + 1].starts_with('-') {
                        profile = Some(args[i + 1].clone());
                        i += 1;
                    }
                }
                "-s" | "--strategy" => {
                    if i + 1 < args.len() && !args[i + 1].starts_with('-') {
                        strategy = Some(args[i + 1].clone());
                        i += 1;
                    }
                }
                "-b" | "--batch" => {
                    force_batch = true;
                    if i + 1 < args.len() && !args[i + 1].starts_with('-') {
                        // Optional directory argument for -b
                        output_path = Some(args[i + 1].clone());
                        i += 1;
                    }
                }
                other => {
                    if !other.starts_with('-') {
                        input_paths.push(other.to_string());
                    }
                }
            }
            i += 1;
        }

        if input_paths.is_empty() {
            eprintln!("Error: Missing input PDF file. Use --help for usage.");
            std::process::exit(1);
        }

        // Single file without batch flag and without options -> Interactive CLI
        if input_paths.len() == 1
            && !force_batch
            && output_path.is_none()
            && preset.is_none()
            && profile.is_none()
            && strategy.is_none()
        {
            minuopdf_lib::cli::run_cli(&input_paths[0]);
            return;
        }

        // Batch execution (single or multiple files)
        let is_multi = input_paths.len() > 1;
        let mut has_err = false;

        for in_file in &input_paths {
            let target_out = if let Some(ref out_base) = output_path {
                let out_p = std::path::Path::new(out_base);
                if is_multi || out_p.is_dir() || out_base.ends_with('/') || out_base.ends_with('\\') {
                    let in_p = std::path::Path::new(in_file);
                    let stem = in_p.file_stem().and_then(|s| s.to_str()).unwrap_or("output");
                    let ext = in_p.extension().and_then(|s| s.to_str()).unwrap_or("pdf");
                    let _ = std::fs::create_dir_all(out_p);
                    Some(out_p.join(format!("{}_optimized.{}", stem, ext)).to_string_lossy().to_string())
                } else {
                    Some(out_base.clone())
                }
            } else {
                None
            };

            if let Err(e) = minuopdf_lib::cli::run_batch_cli(
                in_file,
                target_out.as_deref(),
                preset.as_deref(),
                profile.as_deref(),
                strategy.as_deref(),
            ) {
                eprintln!("Error processing '{}': {}", in_file, e);
                has_err = true;
            }
        }

        if has_err {
            std::process::exit(1);
        }
    } else {
        // GUI モード
        hide_console();
        minuopdf_lib::run();
    }
}
