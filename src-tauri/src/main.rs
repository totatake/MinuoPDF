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
            println!("  MinuoPDF.exe              (GUI Mode)");
            println!("  MinuoPDF.exe <input.pdf>  (CUI Interactive Mode)");
            return;
        }
        // CUI モード
        minuopdf_lib::cli::run_cli(first_arg);
    } else {
        // GUI モード
        hide_console();
        minuopdf_lib::run();
    }
}
