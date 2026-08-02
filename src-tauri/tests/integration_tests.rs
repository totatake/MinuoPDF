use std::path::{Path, PathBuf};
use minuopdf_lib::optimizer::optimize_pdf;
use minuopdf_lib::rasterizer::rasterize_pdf;
use minuopdf_lib::profile::get_preset_config;

fn ensure_test_input_pdf(path: &Path) {
    if path.exists() {
        return;
    }
    
    use lopdf::{Document, Object, Dictionary, Stream};

    let mut doc = Document::with_version("1.5");
    let pages_id = doc.new_object_id();
    let font_id = doc.new_object_id();

    // 1x1 のダミーJPEG画像オブジェクト作成
    let img_id = doc.new_object_id();
    // 1x1 赤色JPEGの最小バイナリ
    let tiny_jpeg: Vec<u8> = vec![
        0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01, 0x01, 0x01, 0x00, 0x60,
        0x00, 0x60, 0x00, 0x00, 0xFF, 0xDB, 0x00, 0x43, 0x00, 0x08, 0x06, 0x06, 0x07, 0x06, 0x05, 0x08,
        0x07, 0x07, 0x07, 0x09, 0x09, 0x08, 0x0A, 0x0C, 0x14, 0x0D, 0x0C, 0x0B, 0x0B, 0x0C, 0x19, 0x12,
        0x13, 0x0F, 0x14, 0x1D, 0x1A, 0x1F, 0x1E, 0x1D, 0x1A, 0x1C, 0x1C, 0x20, 0x24, 0x2E, 0x27, 0x20,
        0x22, 0x2C, 0x23, 0x1C, 0x1C, 0x28, 0x37, 0x29, 0x2C, 0x30, 0x31, 0x34, 0x34, 0x34, 0x1F, 0x27,
        0x39, 0x3D, 0x38, 0x32, 0x3C, 0x2E, 0x33, 0x34, 0x32, 0xFF, 0xC0, 0x00, 0x0B, 0x08, 0x00, 0x01,
        0x00, 0x01, 0x01, 0x01, 0x11, 0x00, 0xFF, 0xC4, 0x00, 0x1F, 0x00, 0x00, 0x01, 0x05, 0x01, 0x01,
        0x01, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04,
        0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0xFF, 0xDA, 0x00, 0x08, 0x01, 0x01, 0x00, 0x00, 0x3F,
        0x00, 0xBE, 0x00, 0x3F, 0xFF, 0xD9
    ];

    let mut img_dict = Dictionary::new();
    img_dict.set("Type", Object::Name(b"XObject".to_vec()));
    img_dict.set("Subtype", Object::Name(b"Image".to_vec()));
    img_dict.set("Width", Object::Integer(1));
    img_dict.set("Height", Object::Integer(1));
    img_dict.set("ColorSpace", Object::Name(b"DeviceRGB".to_vec()));
    img_dict.set("BitsPerComponent", Object::Integer(8));
    img_dict.set("Filter", Object::Name(b"DCTDecode".to_vec()));
    doc.objects.insert(img_id, Object::Stream(Stream::new(img_dict, tiny_jpeg)));

    // Font
    let mut font_dict = Dictionary::new();
    font_dict.set("Type", Object::Name(b"Font".to_vec()));
    font_dict.set("Subtype", Object::Name(b"Type1".to_vec()));
    font_dict.set("BaseFont", Object::Name(b"Helvetica".to_vec()));
    doc.objects.insert(font_id, Object::Dictionary(font_dict));

    // Page Content
    let content_id = doc.new_object_id();
    let content_bytes = b"BT /F1 12 Tf 50 750 Td (Hello World Test PDF) Tj ET q 100 0 0 100 50 600 cm /Im1 Do Q".to_vec();
    doc.objects.insert(content_id, Object::Stream(Stream::new(Dictionary::new(), content_bytes)));

    // Page
    let page_id = doc.new_object_id();
    let mut page_dict = Dictionary::new();
    page_dict.set("Type", Object::Name(b"Page".to_vec()));
    page_dict.set("Parent", Object::Reference(pages_id));
    page_dict.set("MediaBox", vec![0.into(), 0.into(), 595.into(), 842.into()]);
    page_dict.set("Contents", Object::Reference(content_id));

    let mut resources = Dictionary::new();
    let mut fonts_map = Dictionary::new();
    fonts_map.set("F1", Object::Reference(font_id));
    resources.set("Font", Object::Dictionary(fonts_map));

    let mut xobjects_map = Dictionary::new();
    xobjects_map.set("Im1", Object::Reference(img_id));
    resources.set("XObject", Object::Dictionary(xobjects_map));

    page_dict.set("Resources", Object::Dictionary(resources));
    doc.objects.insert(page_id, Object::Dictionary(page_dict));

    // Catalog & Pages
    let mut pages_dict = Dictionary::new();
    pages_dict.set("Type", Object::Name(b"Pages".to_vec()));
    pages_dict.set("Count", Object::Integer(1));
    pages_dict.set("Kids", vec![Object::Reference(page_id)]);
    doc.objects.insert(pages_id, Object::Dictionary(pages_dict));

    let catalog_id = doc.new_object_id();
    let mut catalog_dict = Dictionary::new();
    catalog_dict.set("Type", Object::Name(b"Catalog".to_vec()));
    catalog_dict.set("Pages", Object::Reference(pages_id));
    doc.objects.insert(catalog_id, Object::Dictionary(catalog_dict));

    doc.trailer.set("Root", Object::Reference(catalog_id));
    let _ = doc.save(path);
}

// ヘルパー関数: テスト用の入力PDFのパスを取得する
fn get_test_input_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../test_input.pdf");
    ensure_test_input_pdf(&path);
    path
}

// ヘルパー関数: テスト用の出力PDFのパスを取得する
fn get_test_output_path(filename: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("target");
    path.push(filename);
    path
}

// ヘルパー関数: 空の進捗コールバック
fn empty_progress_callback(_curr: usize, _tot: usize) {}

#[test]
fn test_integration_color_replace() {
    let input = get_test_input_path();
    assert!(input.exists(), "test_input.pdf does not exist.");
    
    let output = get_test_output_path("test_integration_color_replace.pdf");
    if output.exists() {
        let _ = std::fs::remove_file(&output);
    }
    
    let config = get_preset_config("high_quality");
    let res = optimize_pdf(&input, &output, &config, empty_progress_callback);
    assert!(res.is_ok(), "optimize_pdf color replace failed: {:?}", res.err());
    
    let (orig_size, opt_size) = res.unwrap();
    println!("COLOR REPLACE: Orig: {} bytes, Opt: {} bytes", orig_size, opt_size);
    assert!(orig_size > 0 && opt_size > 0);
    assert!(output.exists());
    
    let verify_doc = lopdf::Document::load(&output);
    assert!(verify_doc.is_ok());
}

#[test]
fn test_integration_color_rasterize() {
    let input = get_test_input_path();
    assert!(input.exists());
    
    let output = get_test_output_path("test_integration_color_rasterize.pdf");
    if output.exists() {
        let _ = std::fs::remove_file(&output);
    }
    
    let config = get_preset_config("rasterize");
    let res = rasterize_pdf(&input, &output, &config, empty_progress_callback);
    assert!(res.is_ok(), "rasterize_pdf color failed: {:?}", res.err());
    
    let (orig_size, opt_size) = res.unwrap();
    assert!(orig_size > 0 && opt_size > 0);
    assert!(output.exists());
    
    let verify_doc = lopdf::Document::load(&output);
    assert!(verify_doc.is_ok());
}

#[test]
fn test_integration_gray_replace() {
    let input = get_test_input_path();
    assert!(input.exists());
    
    let output = get_test_output_path("test_integration_gray_replace.pdf");
    if output.exists() {
        let _ = std::fs::remove_file(&output);
    }
    
    let mut config = get_preset_config("standard");
    config.color_conv = "gray".to_string();
    
    let res = optimize_pdf(&input, &output, &config, empty_progress_callback);
    assert!(res.is_ok(), "optimize_pdf gray replace failed: {:?}", res.err());
    
    let (orig_size, opt_size) = res.unwrap();
    println!("GRAY REPLACE: Orig: {} bytes, Opt: {} bytes", orig_size, opt_size);
    assert!(orig_size > 0 && opt_size > 0);
    assert!(output.exists());
    
    let verify_doc = lopdf::Document::load(&output);
    assert!(verify_doc.is_ok());
}

#[test]
fn test_integration_gray_rasterize() {
    let input = get_test_input_path();
    assert!(input.exists());
    
    let output = get_test_output_path("test_integration_gray_rasterize.pdf");
    if output.exists() {
        let _ = std::fs::remove_file(&output);
    }
    
    let mut config = get_preset_config("rasterize");
    config.color_conv = "gray".to_string();
    
    let res = rasterize_pdf(&input, &output, &config, empty_progress_callback);
    assert!(res.is_ok(), "rasterize_pdf gray failed: {:?}", res.err());
    
    let (orig_size, opt_size) = res.unwrap();
    assert!(orig_size > 0 && opt_size > 0);
    assert!(output.exists());
    
    let verify_doc = lopdf::Document::load(&output);
    assert!(verify_doc.is_ok());
}

#[test]
fn test_integration_mono_otsu_replace() {
    let input = get_test_input_path();
    assert!(input.exists());
    
    let output = get_test_output_path("test_integration_mono_otsu_replace.pdf");
    if output.exists() {
        let _ = std::fs::remove_file(&output);
    }
    
    let config = get_preset_config("mono");
    let res = optimize_pdf(&input, &output, &config, empty_progress_callback);
    assert!(res.is_ok(), "optimize_pdf mono otsu failed: {:?}", res.err());
    
    let (orig_size, opt_size) = res.unwrap();
    println!("MONO OTSU: Orig: {} bytes, Opt: {} bytes", orig_size, opt_size);
    assert!(orig_size > 0 && opt_size > 0);
    assert!(output.exists());
    
    let verify_doc = lopdf::Document::load(&output);
    assert!(verify_doc.is_ok());
}

#[test]
fn test_integration_mono_otsu_rasterize() {
    let input = get_test_input_path();
    assert!(input.exists());
    
    let output = get_test_output_path("test_integration_mono_otsu_rasterize.pdf");
    if output.exists() {
        let _ = std::fs::remove_file(&output);
    }
    
    let mut config = get_preset_config("rasterize");
    config.color_conv = "mono".to_string();
    config.mono_img.mono_method = Some("threshold".to_string());
    
    let res = rasterize_pdf(&input, &output, &config, empty_progress_callback);
    assert!(res.is_ok(), "rasterize_pdf mono otsu failed: {:?}", res.err());
    
    let (orig_size, opt_size) = res.unwrap();
    assert!(orig_size > 0 && opt_size > 0);
    assert!(output.exists());
    
    let verify_doc = lopdf::Document::load(&output);
    assert!(verify_doc.is_ok());
}

#[test]
fn test_integration_mono_dither_replace() {
    let input = get_test_input_path();
    assert!(input.exists());
    
    let output = get_test_output_path("test_integration_mono_dither_replace.pdf");
    if output.exists() {
        let _ = std::fs::remove_file(&output);
    }
    
    let mut config = get_preset_config("mono");
    config.mono_img.mono_method = Some("dither".to_string());
    
    let res = optimize_pdf(&input, &output, &config, empty_progress_callback);
    assert!(res.is_ok(), "optimize_pdf mono dither failed: {:?}", res.err());
    
    let (orig_size, opt_size) = res.unwrap();
    println!("MONO DITHER: Orig: {} bytes, Opt: {} bytes", orig_size, opt_size);
    assert!(orig_size > 0 && opt_size > 0);
    assert!(output.exists());
    
    let verify_doc = lopdf::Document::load(&output);
    assert!(verify_doc.is_ok());
}

#[test]
fn test_integration_mono_dither_rasterize() {
    let input = get_test_input_path();
    assert!(input.exists());
    
    let output = get_test_output_path("test_integration_mono_dither_rasterize.pdf");
    if output.exists() {
        let _ = std::fs::remove_file(&output);
    }
    
    let mut config = get_preset_config("rasterize");
    config.color_conv = "mono".to_string();
    config.mono_img.mono_method = Some("dither".to_string());
    
    let res = rasterize_pdf(&input, &output, &config, empty_progress_callback);
    assert!(res.is_ok(), "rasterize_pdf mono dither failed: {:?}", res.err());
    
    let (orig_size, opt_size) = res.unwrap();
    assert!(orig_size > 0 && opt_size > 0);
    assert!(output.exists());
    
    let verify_doc = lopdf::Document::load(&output);
    assert!(verify_doc.is_ok());
}

#[test]
fn test_integration_unembed_fonts() {
    let input = get_test_input_path();
    assert!(input.exists());
    
    let output = get_test_output_path("test_integration_unembed_fonts.pdf");
    if output.exists() {
        let _ = std::fs::remove_file(&output);
    }
    
    let mut config = get_preset_config("standard");
    config.unembed_fonts = vec!["Helvetica".to_string()];
    
    let res = optimize_pdf(&input, &output, &config, empty_progress_callback);
    assert!(res.is_ok(), "optimize_pdf unembed fonts failed: {:?}", res.err());
    
    let (orig_size, opt_size) = res.unwrap();
    println!("UNEMBED FONTS: Orig: {} bytes, Opt: {} bytes", orig_size, opt_size);
    assert!(orig_size > 0 && opt_size > 0);
    assert!(output.exists());
    
    let verify_doc = lopdf::Document::load(&output);
    assert!(verify_doc.is_ok());
}

#[test]
fn test_integration_discard_objects() {
    let input = get_test_input_path();
    assert!(input.exists());
    
    let output = get_test_output_path("test_integration_discard_objects.pdf");
    if output.exists() {
        let _ = std::fs::remove_file(&output);
    }
    
    let mut config = get_preset_config("max_compression");
    config.discard_comments = true;
    config.discard_bookmarks = true;
    config.discard_metadata = true;
    config.discard_thumbs = true;
    config.discard_js = true;
    
    let res = optimize_pdf(&input, &output, &config, empty_progress_callback);
    assert!(res.is_ok(), "optimize_pdf discard objects failed: {:?}", res.err());
    
    let (orig_size, opt_size) = res.unwrap();
    println!("DISCARD OBJECTS: Orig: {} bytes, Opt: {} bytes", orig_size, opt_size);
    assert!(orig_size > 0 && opt_size > 0);
    assert!(output.exists());
    
    let verify_doc = lopdf::Document::load(&output);
    assert!(verify_doc.is_ok());
}

#[test]
fn test_integration_custom_dpi_quality() {
    let input = get_test_input_path();
    assert!(input.exists());
    
    let output = get_test_output_path("test_integration_custom_dpi_quality.pdf");
    if output.exists() {
        let _ = std::fs::remove_file(&output);
    }
    
    let mut config = get_preset_config("standard");
    config.color_img.target_dpi = 50;
    config.color_img.quality = 20;
    
    let res = optimize_pdf(&input, &output, &config, empty_progress_callback);
    assert!(res.is_ok(), "optimize_pdf custom dpi quality failed: {:?}", res.err());
    
    let (orig_size, opt_size) = res.unwrap();
    println!("CUSTOM DPI/QUALITY: Orig: {} bytes, Opt: {} bytes", orig_size, opt_size);
    assert!(orig_size > 0 && opt_size > 0);
    assert!(output.exists());
    
    let verify_doc = lopdf::Document::load(&output);
    assert!(verify_doc.is_ok());
}

#[test]
fn test_integration_font_subsetting() {
    let ikinari = minuopdf_lib::get_fixture_path("sample_font_ikinari_yes.pdf");
    if ikinari.exists() {
        let output = get_test_output_path("test_font_subsetted.pdf");
        if output.exists() {
            let _ = std::fs::remove_file(&output);
        }

        let mut config = get_preset_config("standard");
        config.font_subset = true;

        let res = optimize_pdf(&ikinari, &output, &config, empty_progress_callback);
        assert!(res.is_ok(), "optimize_pdf font subsetting failed: {:?}", res.err());

        let (orig_size, opt_size) = res.unwrap();
        println!("FONT SUBSETTING IKINARI: Orig: {} bytes, Opt: {} bytes (Saved: {:.1}%)", 
            orig_size, opt_size, (1.0 - (opt_size as f64 / orig_size as f64)) * 100.0);
        
        assert!(output.exists());
        let verify_doc = lopdf::Document::load(&output);
        assert!(verify_doc.is_ok(), "Output subsetted PDF should be valid");
    }
}
