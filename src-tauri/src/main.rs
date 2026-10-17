use std::env;

#[cfg(target_os = "windows")]
mod win32 {
    use std::ffi::c_void;
    type HWND = *mut c_void;

    #[link(name = "kernel32")]
    extern "system" {
        pub fn GetConsoleWindow() -> HWND;
    }

    #[link(name = "user32")]
    extern "system" {
        pub fn ShowWindow(hWnd: HWND, nCmdShow: i32) -> i32;
    }
}

#[cfg(target_os = "windows")]
fn hide_console() {
    unsafe {
        let hwnd = win32::GetConsoleWindow();
        if !hwnd.is_null() {
            win32::ShowWindow(hwnd, 0); // SW_HIDE = 0
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn hide_console() {}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let first_arg = &args[1];
        if first_arg == "--help" || first_arg == "-h" || first_arg == "help" {
            println!("MinuoPDF - Non-GPL / Lightweight PDF Optimization Tool");
            println!("MinuoPDF - 非GPL / 軽量PDF最適化ツール");
            println!("\nUsage:");
            println!("  MinuoPDF                    (GUI Mode)");
            println!("  MinuoPDF <input.pdf>        (Interactive CUI Mode / D&D)");
            println!("  MinuoPDF <input.pdf> [options] (Non-interactive Batch Mode)");
            println!("\nOptions:");
            println!("  -o, --output <FILE>         Output PDF file path");
            println!("  -p, --preset <PRESET>       Preset: high_quality | standard | low_quality | mono | rasterize");
            println!("  --profile <NAME>            Saved JSON profile name in profiles/");
            println!("  -s, --strategy <STRATEGY>   Strategy: text | rasterize");
            println!("  -b, --batch                 Run non-interactively with default/specified settings");
            println!("  -h, --help                  Print this help message");
            return;
        }

        // 引数が1つだけ（PDFファイルのD&DまたはCLI起動）の場合は従来の対話モードを維持
        if args.len() == 2 && !first_arg.starts_with('-') {
            minuopdf_lib::cli::run_cli(first_arg);
            return;
        }

        // 複数引数（フラグ指定）の場合は非対話バッチモードで実行
        let mut input_path = None;
        let mut output_path = None;
        let mut preset = None;
        let mut profile = None;
        let mut strategy = None;

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "-o" | "--output" => {
                    if i + 1 < args.len() { output_path = Some(args[i + 1].clone()); i += 1; }
                }
                "-p" | "--preset" => {
                    if i + 1 < args.len() { preset = Some(args[i + 1].clone()); i += 1; }
                }
                "--profile" => {
                    if i + 1 < args.len() { profile = Some(args[i + 1].clone()); i += 1; }
                }
                "-s" | "--strategy" => {
                    if i + 1 < args.len() { strategy = Some(args[i + 1].clone()); i += 1; }
                }
                "-b" | "--batch" => {}
                other => {
                    if !other.starts_with('-') && input_path.is_none() {
                        input_path = Some(other.to_string());
                    }
                }
            }
            i += 1;
        }

        if let Some(ref in_file) = input_path {
            if let Err(e) = minuopdf_lib::cli::run_batch_cli(
                in_file,
                output_path.as_deref(),
                preset.as_deref(),
                profile.as_deref(),
                strategy.as_deref(),
            ) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        } else {
            eprintln!("Error: Missing input PDF file. Use --help for usage.");
            std::process::exit(1);
        }
    } else {
        // GUI モード
        hide_console();
        minuopdf_lib::run();
    }
}
