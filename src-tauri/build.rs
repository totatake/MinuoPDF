use std::process::Command;

fn main() {
    // Ensure PDFium shared library is present in resources/ before build
    let status = Command::new("python")
        .args(["../scripts/ensure_pdfium.py"])
        .status();

    if let Err(e) = status {
        println!("cargo:warning=Failed to run ensure_pdfium.py: {}", e);
    }

    tauri_build::build();
}

