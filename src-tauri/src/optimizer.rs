use lopdf::{Document, Object, ObjectId, Dictionary};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use rayon::prelude::*;
use crate::image_util;
use crate::audit::{audit_document, PdfAuditReport};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ImageConfig {
    pub downsample: String,
    pub target_dpi: u32,
    pub threshold_dpi: u32,
    pub format: String,
    pub quality: u8,
    pub mono_method: Option<String>,
}

fn default_version() -> u32 {
    1
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OptimizeConfig {
    #[serde(default = "default_version")]
    pub version: u32,
    pub color_conv: String,
    pub color_img: ImageConfig,
    pub gray_img: ImageConfig,
    pub mono_img: ImageConfig,
    pub discard_comments: bool,
    pub discard_links: bool,
    pub discard_bookmarks: bool,
    pub discard_forms: bool,
    pub discard_metadata: bool,
    pub discard_js: bool,
    pub discard_thumbs: bool,
    pub clean_linearize: bool,
    pub clean_deflate: bool,
    pub clean_sanitize: bool,
    pub font_subset: bool,
    pub unembed_fonts: Vec<String>,
    #[serde(default)]
    pub force_rasterize: bool,
}

/// PDFを最適化し、監査結果（インメモリ計算）も併せて返す
pub fn optimize_pdf_with_audit<P: AsRef<Path>, Q: AsRef<Path>>(
    input_path: P,
    output_path: Q,
    config: &OptimizeConfig,
    progress_callback: impl Fn(usize, usize) + Send + Sync + 'static,
) -> Result<(u64, u64, Option<PdfAuditReport>, Option<PdfAuditReport>), String> {
    let in_path = input_path.as_ref();
    let file_name = in_path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("Unknown")
        .to_string();

    let orig_size = std::fs::metadata(in_path).map(|m| m.len()).unwrap_or(0);
    let mut doc = Document::load(in_path).map_err(|e| format!("Failed to load PDF: {:?}", e))?;
    
    // インメモリ監査（ディスク再ロードを防止）
    let orig_audit = Some(audit_document(&doc, file_name.clone(), orig_size));

    // 1. 各ページの情報を走査し、画像とその表示領域（推定）のマップを作成する
    let page_ids = doc.get_pages();
    let total_pages = page_ids.len();

    // 画像オブジェクトID -> (最大幅 points, 最大高さ points)
    let mut image_sizes: HashMap<ObjectId, (f32, f32)> = HashMap::new();

    for (page_num, (_page_num_key, &page_id)) in page_ids.iter().enumerate() {
        progress_callback(page_num, total_pages);
        
        // clean_sanitize が有効な場合、MediaBox を CropBox に合わせる
        if config.clean_sanitize {
            if let Ok(page_dict) = doc.get_object(page_id).and_then(|obj| obj.as_dict()) {
                if let Ok(crop_box) = page_dict.get(b"CropBox") {
                    let crop_box_clone = crop_box.clone();
                    if let Ok(Object::Dictionary(ref mut page_dict_mut)) = doc.get_object_mut(page_id) {
                        page_dict_mut.set("MediaBox", crop_box_clone);
                    }
                }
            }
        }

        let page_dict = doc.get_object(page_id)
            .and_then(|obj| obj.as_dict())
            .map_err(|e| format!("Page object not dictionary: {:?}", e))?;

        // ページの MediaBox (物理サイズ) を取得 [llx lly urx ury]
        let media_box = if let Ok(arr) = page_dict.get(b"MediaBox").and_then(|obj| obj.as_array()) {
            if arr.len() == 4 {
                let w = (arr[2].as_float().unwrap_or(0.0) - arr[0].as_float().unwrap_or(0.0)).abs();
                let h = (arr[3].as_float().unwrap_or(0.0) - arr[1].as_float().unwrap_or(0.0)).abs();
                (w, h)
            } else {
                (595.27, 841.89) // A4 default
            }
        } else {
            (595.27, 841.89)
        };

        // ページのコンテンツストリームから CTM / Do オペレータを走査して実際の表示サイズ (pt) を推定
        let content_bytes = get_page_content_bytes(&doc, page_dict);
        let mut page_ctm_map = HashMap::new();
        extract_image_ctm_sizes(&content_bytes, &mut page_ctm_map);

        // ページのリソース辞書内の XObject を走査 (Form XObject は再帰走査)
        scan_xobjects_recursive(
            &doc,
            page_dict,
            media_box,
            &page_ctm_map,
            &mut image_sizes,
            &mut HashSet::new(),
        );
    }

    // 2. 画像の最適化（置換）：画像タスクを収集して Rayon で並列処理
    let total_images = image_sizes.len();

    // 抽出に必要なデータを収集
    struct ImageTask {
        img_id: ObjectId,
        max_w_pt: f32,
        max_h_pt: f32,
        stream_obj: lopdf::Stream,
        smask_ref: Option<ObjectId>,
        smask_stream: Option<lopdf::Stream>,
    }

    struct ImageResult {
        img_id: ObjectId,
        encoded: Option<image_util::EncodedPdfImage>,
        smask_ref: Option<ObjectId>,
        smask_data: Option<(u32, u32, Vec<u8>)>,
    }

    let mut tasks = Vec::with_capacity(total_images);
    for (&img_id, &(max_w_pt, max_h_pt)) in image_sizes.iter() {
        if let Ok(Object::Stream(s)) = doc.get_object(img_id) {
            let smask_ref = s.dict.get(b"SMask").and_then(|o| o.as_reference()).ok();
            let smask_stream = if let Some(sr) = smask_ref {
                doc.get_object(sr).ok().and_then(|o| o.as_stream().cloned().ok())
            } else {
                None
            };
            tasks.push(ImageTask {
                img_id,
                max_w_pt,
                max_h_pt,
                stream_obj: s.clone(),
                smask_ref,
                smask_stream,
            });
        }
    }

    // Rayon による画像デコード・リサイズ・再エンコードの並列実行
    let results: Vec<ImageResult> = tasks
        .into_par_iter()
        .map(|task| {
            let width = task.stream_obj.dict.get(b"Width").and_then(|o| o.as_i64()).unwrap_or(0) as u32;
            let height = task.stream_obj.dict.get(b"Height").and_then(|o| o.as_i64()).unwrap_or(0) as u32;
            if width == 0 || height == 0 {
                return ImageResult {
                    img_id: task.img_id,
                    encoded: None,
                    smask_ref: None,
                    smask_data: None,
                };
            }

            let original_compressed_len = task.stream_obj.content.len();
            let mut decompressed_stream = task.stream_obj.clone();
            let _ = decompressed_stream.decompress();
            let data = decompressed_stream.content;

            let bits_per_component = task.stream_obj.dict.get(b"BitsPerComponent").and_then(|o| o.as_i64()).unwrap_or(8) as u32;
            let img = match load_raw_pdf_image(&data, width, height, bits_per_component) {
                Some(i) => i,
                None => {
                    return ImageResult {
                        img_id: task.img_id,
                        encoded: None,
                        smask_ref: None,
                        smask_data: None,
                    };
                }
            };

            let color_space = task.stream_obj.dict.get(b"ColorSpace").ok();
            let is_originally_mono = bits_per_component == 1;
            let is_originally_gray = if let Some(Object::Name(name)) = color_space {
                name == b"DeviceGray"
            } else {
                false
            };

            let mode = if config.color_conv == "mono" || is_originally_mono {
                "mono"
            } else if config.color_conv == "gray" || is_originally_gray {
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
            let threshold_dpi = active_config.threshold_dpi;

            let est_w_in = task.max_w_pt / 72.0;
            let est_h_in = task.max_h_pt / 72.0;
            let current_dpi_w = if est_w_in > 0.0 { width as f32 / est_w_in } else { 300.0 };
            let current_dpi_h = if est_h_in > 0.0 { height as f32 / est_h_in } else { 300.0 };
            let current_dpi = current_dpi_w.max(current_dpi_h);

            let mut processed_img = img;
            let mut resized = false;
            let mut final_target_w = width;
            let mut final_target_h = height;

            if active_config.downsample != "off" && current_dpi > threshold_dpi as f32 {
                let target_w = (est_w_in * target_dpi as f32).round() as u32;
                let target_h = (est_h_in * target_dpi as f32).round() as u32;

                if target_w < width && target_h < height && target_w > 0 && target_h > 0 {
                    processed_img = image_util::resize_image(&processed_img, target_w, target_h, &active_config.downsample);
                    resized = true;
                    final_target_w = target_w;
                    final_target_h = target_h;
                }
            }

            if active_config.format == "off" && !resized {
                return ImageResult {
                    img_id: task.img_id,
                    encoded: None,
                    smask_ref: None,
                    smask_data: None,
                };
            }

            let encoded = match image_util::encode_image_stream(&processed_img, mode, active_config) {
                Ok(enc) => enc,
                Err(_) => {
                    return ImageResult {
                        img_id: task.img_id,
                        encoded: None,
                        smask_ref: None,
                        smask_data: None,
                    };
                }
            };

            // スマートフォールバックガード
            let is_color_converted = (config.color_conv == "mono" && !is_originally_mono)
                || (config.color_conv == "gray" && !is_originally_gray && !is_originally_mono);
            if !is_color_converted && !resized && encoded.data.len() >= original_compressed_len {
                return ImageResult {
                    img_id: task.img_id,
                    encoded: None,
                    smask_ref: None,
                    smask_data: None,
                };
            }

            // SMask 連動リサイズ処理
            let mut smask_data = None;
            if resized {
                if let Some(smask_stream) = task.smask_stream {
                    let mut smask_decomp = smask_stream.clone();
                    let _ = smask_decomp.decompress();
                    let smask_w = smask_stream.dict.get(b"Width").and_then(|o| o.as_i64()).unwrap_or(0) as u32;
                    let smask_h = smask_stream.dict.get(b"Height").and_then(|o| o.as_i64()).unwrap_or(0) as u32;
                    if smask_w > 0 && smask_h > 0 {
                        if let Some(smask_img) = load_raw_pdf_image(&smask_decomp.content, smask_w, smask_h, 8) {
                            let smask_resized = image_util::resize_image(&smask_img, final_target_w, final_target_h, &active_config.downsample);
                            let gray_mask = smask_resized.to_luma8();
                            if let Ok(deflated) = image_util::compress_deflate(gray_mask.as_raw()) {
                                smask_data = Some((final_target_w, final_target_h, deflated));
                            }
                        }
                    }
                }
            }

            ImageResult {
                img_id: task.img_id,
                encoded: Some(encoded),
                smask_ref: task.smask_ref,
                smask_data,
            }
        })
        .collect();

    // 順次 PDF ドキュメントへ適用
    for (img_idx, res) in results.into_iter().enumerate() {
        progress_callback(total_pages + img_idx, total_pages + total_images);
        if let Some(encoded) = res.encoded {
            if let Ok(Object::Stream(ref mut stream)) = doc.get_object_mut(res.img_id) {
                stream.dict.set("Width", encoded.width as i64);
                stream.dict.set("Height", encoded.height as i64);
                stream.dict.set("ColorSpace", Object::Name(encoded.color_space));
                stream.dict.set("BitsPerComponent", encoded.bits_per_component);
                stream.dict.set("Filter", Object::Name(encoded.filter));

                if let Some(dp) = encoded.decode_parms {
                    stream.dict.set("DecodeParms", Object::Dictionary(dp));
                } else {
                    stream.dict.remove(b"DecodeParms");
                }
                stream.content = encoded.data;
            }

            if let (Some(smask_ref), Some((w, h, data))) = (res.smask_ref, res.smask_data) {
                if let Ok(Object::Stream(ref mut smask_stream)) = doc.get_object_mut(smask_ref) {
                    smask_stream.dict.set("Width", w as i64);
                    smask_stream.dict.set("Height", h as i64);
                    smask_stream.dict.set("Filter", Object::Name(b"FlateDecode".to_vec()));
                    smask_stream.content = data;
                }
            }
        }
    }

    // 3. 破棄＆クリーンアップ処理
    if let Ok(catalog_id) = doc.trailer.get(b"Root").and_then(|o| o.as_reference()) {
        // カタログの辞書を取得・編集
        if let Ok(Object::Dictionary(ref mut catalog_dict)) = doc.get_object_mut(catalog_id) {
            if config.discard_metadata {
                catalog_dict.remove(b"Metadata");
            }
            if config.discard_bookmarks {
                catalog_dict.remove(b"Outlines");
            }
            if config.discard_forms {
                catalog_dict.remove(b"AcroForm");
            }
            if config.discard_js {
                catalog_dict.remove(b"Names"); // JavaScriptや埋め込みアクションはここに多い
                catalog_dict.remove(b"OpenAction");
            }
        }
    }

    // ページごとの不要データ削除
    for &page_id in page_ids.values() {
        if config.discard_comments || config.discard_links {
            let annots_arr = if let Ok(Object::Dictionary(ref page_dict)) = doc.get_object(page_id) {
                page_dict.get(b"Annots").ok().and_then(|o| o.as_array().ok().cloned())
            } else {
                None
            };

            if let Some(annots_arr) = annots_arr {
                let mut new_annots = Vec::new();
                for annot_ref in annots_arr.iter() {
                    if let Ok(annot_id) = annot_ref.as_reference() {
                        if let Ok(annot_dict) = doc.get_object(annot_id).and_then(|o| o.as_dict()) {
                            let subtype = annot_dict.get(b"Subtype").and_then(|o| o.as_name()).unwrap_or(&[]);
                            let is_link = subtype == b"Link";
                            
                            if is_link && !config.discard_links {
                                new_annots.push(annot_ref.clone());
                            } else if !is_link && !config.discard_comments {
                                new_annots.push(annot_ref.clone());
                            }
                        }
                    }
                }
                
                if let Ok(Object::Dictionary(ref mut page_dict)) = doc.get_object_mut(page_id) {
                    if new_annots.is_empty() {
                        page_dict.remove(b"Annots");
                    } else {
                        page_dict.set("Annots", Object::Array(new_annots));
                    }
                }
            }
        }

        if config.discard_thumbs {
            if let Ok(Object::Dictionary(ref mut page_dict)) = doc.get_object_mut(page_id) {
                page_dict.remove(b"Thumb");
            }
        }
    }

    // 4. フォント解除 (Unembed)
    if !config.unembed_fonts.is_empty() {
        let unembed_set: HashSet<String> = config.unembed_fonts.iter().cloned().collect();
        let mut font_objects = Vec::new();
        
        // 全オブジェクトを走査して FontDescriptor 内の FontFile を探す
        for (&obj_id, obj) in doc.objects.iter() {
            if let Ok(dict) = obj.as_dict() {
                if let Ok(Object::Name(ref type_name)) = dict.get(b"Type") {
                    if type_name == b"FontDescriptor" {
                        if let Ok(font_name) = dict.get(b"FontName").and_then(|o| o.as_name()) {
                            let name_str = String::from_utf8_lossy(font_name).to_string();
                            // サブセットのプレフィックス (例: "ABCDEF+Arial") を考慮して部分一致でチェック
                            let mut match_found = false;
                            for target in &unembed_set {
                                if name_str.contains(target) {
                                    match_found = true;
                                    break;
                                }
                            }

                            if match_found {
                                font_objects.push(obj_id);
                            }
                        }
                    }
                }
            }
        }

        for font_desc_id in font_objects {
            if let Ok(Object::Dictionary(ref mut dict)) = doc.get_object_mut(font_desc_id) {
                // 埋め込みフォントストリームを削除
                dict.remove(b"FontFile");
                dict.remove(b"FontFile2");
                dict.remove(b"FontFile3");
            }
        }
    }

    // 5. フォントのサブセット化 (Font Subsetting)
    if config.font_subset {
        crate::font_subsetter::subset_pdf_fonts(&mut doc);
    }

    // 6. 未使用オブジェクトの除去 (Linearize / Prune)
    if config.clean_linearize {
        doc.delete_zero_length_streams();
        doc.prune_objects();
    }

    // 7. 保存とクリーンアップ (Deflate圧縮)
    if config.clean_deflate {
        doc.compress();
    }

    // ガベージコレクションを有効にしつつ一時ファイルに保存、サイズを返す
    let mut out_file = std::fs::File::create(&output_path)
        .map_err(|e| format!("Failed to create output file: {:?}", e))?;
    
    doc.save_to(&mut out_file)
        .map_err(|e| format!("Failed to write optimized PDF: {:?}", e))?;

    // ファイルサイズの比較
    let opt_size = std::fs::metadata(&output_path).map(|m| m.len()).unwrap_or(0);

    // インメモリ監査（保存済み Document オブジェクトから直接計算）
    let opt_audit = Some(audit_document(&doc, file_name, opt_size));

    // 完了を通知
    progress_callback(total_pages + total_images, total_pages + total_images);

    Ok((orig_size, opt_size, orig_audit, opt_audit))
}

/// 従来のシグネチャを維持する最適化メイン関数
pub fn optimize_pdf<P: AsRef<Path>, Q: AsRef<Path>>(
    input_path: P,
    output_path: Q,
    config: &OptimizeConfig,
    progress_callback: impl Fn(usize, usize) + Send + Sync + 'static,
) -> Result<(u64, u64), String> {
    optimize_pdf_with_audit(input_path, output_path, config, progress_callback)
        .map(|(orig, opt, _, _)| (orig, opt))
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct FontInfo {
    pub name: String,
    pub font_type: String,
    pub is_embedded: bool,
}

/// PDFファイルをスキャンして含まれるフォント一覧を返す
pub fn scan_pdf_fonts<P: AsRef<Path>>(input_path: P) -> Result<Vec<FontInfo>, String> {
    let doc = Document::load(input_path).map_err(|e| format!("Failed to load PDF: {:?}", e))?;
    let mut fonts = HashMap::new();

    for (&_obj_id, obj) in doc.objects.iter() {
        if let Ok(dict) = obj.as_dict() {
            if let Ok(Object::Name(ref type_name)) = dict.get(b"Type") {
                if type_name == b"Font" {
                    if let Ok(font_name) = dict.get(b"BaseFont").and_then(|o| o.as_name()) {
                        let name_str = String::from_utf8_lossy(font_name).to_string();
                        let is_embedded = check_font_embedded(&doc, dict);
                        let font_type = dict.get(b"Subtype")
                            .and_then(|o| o.as_name())
                            .map(|n| String::from_utf8_lossy(n).to_string())
                            .unwrap_or_else(|_| "Unknown".to_string());

                        // すでにマップにあれば、埋め込み状態が true の方を優先する
                        let entry = fonts.entry(name_str.clone()).or_insert_with(|| FontInfo {
                            name: name_str.clone(),
                            font_type: font_type.clone(),
                            is_embedded: false,
                        });
                        if is_embedded {
                            entry.is_embedded = true;
                        }
                    }
                }
            }
        }
    }
    
    let mut result: Vec<FontInfo> = fonts.into_values().collect();
    result.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(result)
}

fn check_font_embedded(doc: &Document, font_dict: &Dictionary) -> bool {
    if let Ok(fd_ref) = font_dict.get(b"FontDescriptor").and_then(|o| o.as_reference()) {
        if let Ok(fd_dict) = doc.get_object(fd_ref).and_then(|o| o.as_dict()) {
            return fd_dict.get(b"FontFile").is_ok()
                || fd_dict.get(b"FontFile2").is_ok()
                || fd_dict.get(b"FontFile3").is_ok();
        }
    }
    if let Ok(df_arr) = font_dict.get(b"DescendantFonts").and_then(|o| o.as_array()) {
        for df_ref in df_arr {
            if let Ok(df_id) = df_ref.as_reference() {
                if let Ok(df_dict) = doc.get_object(df_id).and_then(|o| o.as_dict()) {
                    if let Ok(fd_ref) = df_dict.get(b"FontDescriptor").and_then(|o| o.as_reference()) {
                        if let Ok(fd_dict) = doc.get_object(fd_ref).and_then(|o| o.as_dict()) {
                            if fd_dict.get(b"FontFile").is_ok()
                                || fd_dict.get(b"FontFile2").is_ok()
                                || fd_dict.get(b"FontFile3").is_ok()
                            {
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

fn load_raw_pdf_image(
    data: &[u8],
    width: u32,
    height: u32,
    bits_per_component: u32,
) -> Option<image::DynamicImage> {
    // 1. Try standard load from memory first (in case it contains standard headers like JPEG/PNG)
    if let Ok(img) = image::load_from_memory(data) {
        return Some(img);
    }

    // 2. If it fails, treat it as raw pixels
    let pixels_count = (width * height) as usize;
    let data_len = data.len();

    if bits_per_component == 1 {
        let row_stride = (width as usize + 7) / 8;
        if data_len >= row_stride * (height as usize) {
            let mut gray_bytes = Vec::with_capacity(pixels_count);
            for y in 0..height as usize {
                let row_start = y * row_stride;
                for x in 0..width as usize {
                    let byte_idx = row_start + (x / 8);
                    let bit_idx = 7 - (x % 8);
                    let bit = (data[byte_idx] >> bit_idx) & 1;
                    // PDF monochrome conventions: 0 is black, 1 is white
                    let pixel_val = if bit == 1 { 255 } else { 0 };
                    gray_bytes.push(pixel_val);
                }
            }
            if let Some(gray_img) = image::ImageBuffer::<image::Luma<u8>, _>::from_raw(width, height, gray_bytes) {
                return Some(image::DynamicImage::ImageLuma8(gray_img));
            }
        }
    } else if bits_per_component == 8 {
        if data_len == pixels_count * 3 {
            // Raw RGB
            if let Some(rgb_img) = image::ImageBuffer::<image::Rgb<u8>, _>::from_raw(width, height, data.to_vec()) {
                return Some(image::DynamicImage::ImageRgb8(rgb_img));
            }
        } else if data_len == pixels_count {
            // Raw Grayscale
            if let Some(gray_img) = image::ImageBuffer::<image::Luma<u8>, _>::from_raw(width, height, data.to_vec()) {
                return Some(image::DynamicImage::ImageLuma8(gray_img));
            }
        } else if data_len == pixels_count * 4 {
            // Raw CMYK -> Convert to RGB
            let mut rgb_bytes = Vec::with_capacity(pixels_count * 3);
            for i in 0..pixels_count {
                let c = data[i * 4] as f32 / 255.0;
                let m = data[i * 4 + 1] as f32 / 255.0;
                let y = data[i * 4 + 2] as f32 / 255.0;
                let k = data[i * 4 + 3] as f32 / 255.0;
                let r = (255.0 * (1.0 - c) * (1.0 - k)).round() as u8;
                let g = (255.0 * (1.0 - m) * (1.0 - k)).round() as u8;
                let b = (255.0 * (1.0 - y) * (1.0 - k)).round() as u8;
                rgb_bytes.push(r);
                rgb_bytes.push(g);
                rgb_bytes.push(b);
            }
            if let Some(rgb_img) = image::ImageBuffer::<image::Rgb<u8>, _>::from_raw(width, height, rgb_bytes) {
                return Some(image::DynamicImage::ImageRgb8(rgb_img));
            }
        }
    }

    None
}

/// ページの Contents から生バイト列を取得する
fn get_page_content_bytes(doc: &Document, page_dict: &Dictionary) -> Vec<u8> {
    let mut bytes = Vec::new();
    if let Ok(contents) = page_dict.get(b"Contents") {
        match contents {
            Object::Reference(id) => {
                if let Ok(Object::Stream(s)) = doc.get_object(*id) {
                    if let Ok(decomp) = s.decompressed_content() {
                        bytes.extend(decomp);
                    } else {
                        bytes.extend(&s.content);
                    }
                }
            }
            Object::Array(arr) => {
                for item in arr {
                    if let Ok(id) = item.as_reference() {
                        if let Ok(Object::Stream(s)) = doc.get_object(id) {
                            if let Ok(decomp) = s.decompressed_content() {
                                bytes.extend(decomp);
                            } else {
                                bytes.extend(&s.content);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
    bytes
}

/// コンテンツストリームを簡易解析し、`cm`（変換行列）と `/Name Do` から画像の描画サイズ (pt) を抽出
fn extract_image_ctm_sizes(content: &[u8], map: &mut HashMap<String, (f32, f32)>) {
    let text = String::from_utf8_lossy(content);
    let mut current_matrix = [1.0f32, 0.0, 0.0, 1.0f32]; // [a, b, c, d]
    let mut matrix_stack = Vec::new();

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed == "q" {
            matrix_stack.push(current_matrix);
        } else if trimmed == "Q" {
            if let Some(m) = matrix_stack.pop() {
                current_matrix = m;
            }
        } else if trimmed.ends_with(" cm") {
            let tokens: Vec<&str> = trimmed.split_whitespace().collect();
            if tokens.len() >= 7 {
                if let (Ok(a), Ok(b), Ok(c), Ok(d)) = (
                    tokens[0].parse::<f32>(),
                    tokens[1].parse::<f32>(),
                    tokens[2].parse::<f32>(),
                    tokens[3].parse::<f32>(),
                ) {
                    current_matrix = [a.abs(), b.abs(), c.abs(), d.abs()];
                }
            }
        } else if trimmed.ends_with(" Do") {
            let tokens: Vec<&str> = trimmed.split_whitespace().collect();
            if tokens.len() >= 2 {
                let name = tokens[0].trim_start_matches('/');
                let width_pt = (current_matrix[0] * current_matrix[0] + current_matrix[1] * current_matrix[1]).sqrt();
                let height_pt = (current_matrix[2] * current_matrix[2] + current_matrix[3] * current_matrix[3]).sqrt();
                if width_pt > 0.0 && height_pt > 0.0 {
                    let entry = map.entry(name.to_string()).or_insert((0.0, 0.0));
                    entry.0 = entry.0.max(width_pt);
                    entry.1 = entry.1.max(height_pt);
                }
            }
        }
    }
}

/// 辞書からリソース内の XObject を再帰走査（Form XObject も対応）
fn scan_xobjects_recursive(
    doc: &Document,
    parent_dict: &Dictionary,
    fallback_size: (f32, f32),
    ctm_map: &HashMap<String, (f32, f32)>,
    image_sizes: &mut HashMap<ObjectId, (f32, f32)>,
    visited_forms: &mut HashSet<ObjectId>,
) {
    let resources = match parent_dict.get(b"Resources") {
        Ok(Object::Reference(id)) => doc.get_object(*id).ok().and_then(|obj| obj.as_dict().ok()),
        Ok(Object::Dictionary(d)) => Some(d),
        _ => None,
    };

    let resources_dict = match resources {
        Some(d) => d,
        None => return,
    };

    let xobjects = match resources_dict.get(b"XObject") {
        Ok(Object::Reference(id)) => doc.get_object(*id).ok().and_then(|obj| obj.as_dict().ok()),
        Ok(Object::Dictionary(d)) => Some(d),
        _ => None,
    };

    let xobjects_dict = match xobjects {
        Some(d) => d,
        None => return,
    };

    for (name_bytes, xobj_ref) in xobjects_dict.iter() {
        if let Ok(xobj_id) = xobj_ref.as_reference() {
            if let Ok(xobj_stream) = doc.get_object(xobj_id).and_then(|obj| obj.as_stream()) {
                if let Ok(Object::Name(ref subtype)) = xobj_stream.dict.get(b"Subtype") {
                    if subtype == b"Image" {
                        let name_str = String::from_utf8_lossy(name_bytes).to_string();
                        let size = if let Some(&(w, h)) = ctm_map.get(&name_str) {
                            (w, h)
                        } else {
                            fallback_size
                        };
                        let entry = image_sizes.entry(xobj_id).or_insert((0.0, 0.0));
                        entry.0 = entry.0.max(size.0);
                        entry.1 = entry.1.max(size.1);
                    } else if subtype == b"Form" && !visited_forms.contains(&xobj_id) {
                        visited_forms.insert(xobj_id);
                        scan_xobjects_recursive(
                            doc,
                            &xobj_stream.dict,
                            fallback_size,
                            ctm_map,
                            image_sizes,
                            visited_forms,
                        );
                    }
                }
            }
        }
    }
}


