use lopdf::{Document, Object, Dictionary, Stream};
use pdfium_render::prelude::*;
use std::path::Path;
use crate::pdfium_util;
use crate::image_util;
use crate::optimizer::OptimizeConfig;

/// PDFの全ページをラスタライズし、新しい画像のみのPDFファイルを構築します。
pub fn rasterize_pdf<P: AsRef<Path>, Q: AsRef<Path>>(
    input_path: P,
    output_path: Q,
    config: &OptimizeConfig,
    progress_callback: impl Fn(usize, usize) + Send + Sync + 'static,
) -> Result<(u64, u64), String> {
    let pdfium = pdfium_util::load_pdfium()?;
    let pdfium_doc = pdfium.load_pdf_from_file(input_path.as_ref(), None)
        .map_err(|e| format!("Failed to load PDF in Pdfium: {:?}", e))?;

    let total_pages = pdfium_doc.pages().len() as usize;

    let mut doc = Document::new();
    let pages_id = doc.new_object_id();
    let mut page_ids = Vec::new();

    // 圧縮設定の取得 (完全ラスタライズ時はカラー・グレー・モノクロのどれかを選択)
    let mode = if config.color_conv == "mono" {
        "mono"
    } else if config.color_conv == "gray" {
        "gray"
    } else {
        "color"
    };

    let active_config = match mode {
        "mono" => &config.mono_img,
        "gray" => &config.gray_img,
        _ => &config.color_img,
    };

    let target_dpi = active_config.target_dpi;

    for (idx, page) in pdfium_doc.pages().iter().enumerate() {
        progress_callback(idx, total_pages);

        // ページ物理サイズ取得（ポイント）
        let w_pt = page.width().value;
        let h_pt = page.height().value;

        // 目標ピクセルサイズ算出
        let w_px = ((w_pt / 72.0) * target_dpi as f32).round() as u32;
        let h_px = ((h_pt / 72.0) * target_dpi as f32).round() as u32;

        // レンダー設定
        let render_config = PdfRenderConfig::new()
            .set_target_width(w_px as i32)
            .set_target_height(h_px as i32);

        let bitmap = page.render_with_config(&render_config)
            .map_err(|e| format!("Failed to render page {}: {:?}", idx, e))?;

        // `image` の DynamicImage に変換
        // pdfium-render が返したピクセルデータを取得
        let img = bitmap.as_image();

        let img_bytes: Vec<u8>;
        let final_w: u32;
        let final_h: u32;
        let format_lower = active_config.format.to_lowercase();
        let filter_name: Vec<u8>;
        let bits_per_component: i64;

        if mode == "mono" {
            let gray = img.to_luma8();
            let mono_method = active_config.mono_method.as_deref().unwrap_or("threshold");
            let binarized = if mono_method == "dither" {
                image_util::apply_floyd_steinberg(&gray)
            } else {
                image_util::apply_otsu(&gray)
            };
            
            final_w = binarized.width();
            final_h = binarized.height();

            if format_lower == "zip" || format_lower == "lzw" {
                let packed_bytes = image_util::pack_mono_image(&binarized);
                img_bytes = image_util::compress_deflate(&packed_bytes)
                    .map_err(|e| format!("Deflate compression failed: {:?}", e))?;
                filter_name = b"FlateDecode".to_vec();
                bits_per_component = 1;
            } else {
                img_bytes = image_util::encode_g4(&binarized)
                    .map_err(|e| format!("G4 encoding failed: {:?}", e))?;
                filter_name = b"CCITTFaxDecode".to_vec();
                bits_per_component = 1;
            }
        } else if mode == "gray" {
            let gray = img.to_luma8();
            final_w = gray.width();
            final_h = gray.height();
            if format_lower == "zip" || format_lower == "lzw" {
                let raw_pixels = gray.as_raw();
                img_bytes = image_util::compress_deflate(raw_pixels)
                    .map_err(|e| format!("Deflate compression failed: {:?}", e))?;
                filter_name = b"FlateDecode".to_vec();
                bits_per_component = 8;
            } else {
                img_bytes = image_util::encode_jpeg_luma(&gray, active_config.quality)
                    .map_err(|e| format!("JPEG encoding failed: {:?}", e))?;
                filter_name = b"DCTDecode".to_vec();
                bits_per_component = 8;
            }
        } else {
            final_w = img.width();
            final_h = img.height();
            if format_lower == "zip" || format_lower == "lzw" {
                let rgb = img.to_rgb8();
                let raw_pixels = rgb.as_raw();
                img_bytes = image_util::compress_deflate(raw_pixels)
                    .map_err(|e| format!("Deflate compression failed: {:?}", e))?;
                filter_name = b"FlateDecode".to_vec();
                bits_per_component = 8;
            } else {
                img_bytes = image_util::encode_jpeg_rgb(&img, active_config.quality)
                    .map_err(|e| format!("JPEG encoding failed: {:?}", e))?;
                filter_name = b"DCTDecode".to_vec();
                bits_per_component = 8;
            }
        }

        // 1. 画像オブジェクトを作成して lopdf に追加
        let img_id = doc.new_object_id();
        let mut img_stream = Stream::new(Dictionary::new(), img_bytes);
        img_stream.dict.set("Type", Object::Name(b"XObject".to_vec()));
        img_stream.dict.set("Subtype", Object::Name(b"Image".to_vec()));
        img_stream.dict.set("Width", final_w as i64);
        img_stream.dict.set("Height", final_h as i64);

        let color_space = if mode == "color" { b"DeviceRGB".to_vec() } else { b"DeviceGray".to_vec() };
        img_stream.dict.set("ColorSpace", Object::Name(color_space));
        img_stream.dict.set("BitsPerComponent", bits_per_component);
        img_stream.dict.set("Filter", Object::Name(filter_name.clone()));

        if filter_name == b"CCITTFaxDecode" {
            let mut decode_parms = Dictionary::new();
            decode_parms.set("Columns", final_w as i64);
            decode_parms.set("Rows", final_h as i64);
            decode_parms.set("K", -1);
            img_stream.dict.set("DecodeParms", Object::Dictionary(decode_parms));
        } else {
            img_stream.dict.remove(b"DecodeParms");
        }

        doc.objects.insert(img_id, Object::Stream(img_stream));

        // 2. コンテンツストリームを作成 (ページ全体に画像を拡大描画)
        let content_id = doc.new_object_id();
        let content_str = format!("q {} 0 0 {} 0 0 cm /Im1 Do Q\n", w_pt, h_pt);
        let content_stream = Stream::new(Dictionary::new(), content_str.into_bytes());
        doc.objects.insert(content_id, Object::Stream(content_stream));

        // 3. ページリソースの作成
        let mut resources = Dictionary::new();
        let mut xobjects = Dictionary::new();
        xobjects.set("Im1", Object::Reference(img_id));
        resources.set("XObject", Object::Dictionary(xobjects));

        // 4. ページオブジェクトを作成して追加
        let page_id = doc.new_object_id();
        let mut page_dict = Dictionary::new();
        page_dict.set("Type", Object::Name(b"Page".to_vec()));
        page_dict.set("Parent", Object::Reference(pages_id));
        page_dict.set("Resources", Object::Dictionary(resources));
        page_dict.set("MediaBox", Object::Array(vec![
            Object::Real(0.0),
            Object::Real(0.0),
            Object::Real(w_pt as f32),
            Object::Real(h_pt as f32),
        ]));
        page_dict.set("Contents", Object::Reference(content_id));

        doc.objects.insert(page_id, Object::Dictionary(page_dict));
        page_ids.push(page_id);
    }

    // 5. ページツリーの追加
    let mut pages_dict = Dictionary::new();
    pages_dict.set("Type", Object::Name(b"Pages".to_vec()));
    pages_dict.set("Kids", Object::Array(page_ids.into_iter().map(Object::Reference).collect()));
    pages_dict.set("Count", total_pages as i64);
    doc.objects.insert(pages_id, Object::Dictionary(pages_dict));

    // 6. カタログとRootの追加
    let catalog_id = doc.new_object_id();
    let mut catalog_dict = Dictionary::new();
    catalog_dict.set("Type", Object::Name(b"Catalog".to_vec()));
    catalog_dict.set("Pages", Object::Reference(pages_id));
    doc.objects.insert(catalog_id, Object::Dictionary(catalog_dict));

    doc.trailer.set("Root", Object::Reference(catalog_id));

    // Deflate圧縮
    if config.clean_deflate {
        doc.compress();
    }

    // 保存
    let mut out_file = std::fs::File::create(&output_path)
        .map_err(|e| format!("Failed to create output file: {:?}", e))?;
    doc.save_to(&mut out_file)
        .map_err(|e| format!("Failed to save rasterized PDF: {:?}", e))?;

    let orig_size = std::fs::metadata(&input_path).map(|m| m.len()).unwrap_or(0);
    let opt_size = std::fs::metadata(&output_path).map(|m| m.len()).unwrap_or(0);

    progress_callback(total_pages, total_pages);

    Ok((orig_size, opt_size))
}
