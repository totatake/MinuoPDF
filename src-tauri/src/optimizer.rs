use lopdf::{Document, Object, ObjectId, Dictionary};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use crate::image_util;

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

/// PDFを最適化するメイン処理
pub fn optimize_pdf<P: AsRef<Path>, Q: AsRef<Path>>(
    input_path: P,
    output_path: Q,
    config: &OptimizeConfig,
    progress_callback: impl Fn(usize, usize) + Send + Sync + 'static,
) -> Result<(u64, u64), String> {
    let mut doc = Document::load(&input_path).map_err(|e| format!("Failed to load PDF: {:?}", e))?;
    
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

        // ページのリソース辞書内の XObject を走査
        let resources = match page_dict.get(b"Resources") {
            Ok(Object::Reference(id)) => doc.get_object(*id).ok().and_then(|obj| obj.as_dict().ok()),
            Ok(Object::Dictionary(d)) => Some(d),
            _ => None,
        };

        if let Some(resources_dict) = resources {

            let xobjects = match resources_dict.get(b"XObject") {
                Ok(Object::Reference(id)) => doc.get_object(*id).ok().and_then(|obj| obj.as_dict().ok()),
                Ok(Object::Dictionary(d)) => Some(d),
                _ => None,
            };

            if let Some(xobjects_dict) = xobjects {
                for (_, xobj_ref) in xobjects_dict.iter() {
                    if let Ok(xobj_id) = xobj_ref.as_reference() {
                        if let Ok(xobj_stream) = doc.get_object(xobj_id).and_then(|obj| obj.as_stream()) {
                            if let Ok(Object::Name(ref subtype)) = xobj_stream.dict.get(b"Subtype") {
                                if subtype == b"Image" {
                                    // 一旦そのページのMediaBoxを最大表示サイズとする
                                    let entry = image_sizes.entry(xobj_id).or_insert((0.0, 0.0));
                                    entry.0 = entry.0.max(media_box.0);
                                    entry.1 = entry.1.max(media_box.1);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // 2. 画像の最適化（置換）
    let total_images = image_sizes.len();
    for (img_idx, (&img_id, &(max_w_pt, max_h_pt))) in image_sizes.iter().enumerate() {
        // 画像処理中の進捗をコールバックに通知 (適当に分散させる)
        progress_callback(total_pages + img_idx, total_pages + total_images);

        let mut stream_obj = match doc.get_object(img_id) {
            Ok(Object::Stream(s)) => s.clone(),
            _ => continue,
        };

        // 元の画像サイズ取得
        let width = stream_obj.dict.get(b"Width").and_then(|o| o.as_i64()).unwrap_or(0) as u32;
        let height = stream_obj.dict.get(b"Height").and_then(|o| o.as_i64()).unwrap_or(0) as u32;
        if width == 0 || height == 0 { continue; }

        // 元データの取得 (デコンプレス実行)
        let _ = stream_obj.decompress();
        let data = stream_obj.content.clone();

        // image crate またはカスタムデコーダで読み込み
        let bits_per_component = stream_obj.dict.get(b"BitsPerComponent").and_then(|o| o.as_i64()).unwrap_or(8) as u32;
        let img = match load_raw_pdf_image(&data, width, height, bits_per_component) {
            Some(i) => i,
            None => continue, // デコードできない形式はスキップ
        };

        // 圧縮ポリシーの決定
        let color_space = stream_obj.dict.get(b"ColorSpace").ok();
        let is_originally_mono = match stream_obj.dict.get(b"BitsPerComponent").and_then(|o| o.as_i64()) {
            Ok(1) => true,
            _ => false,
        };
        let is_originally_gray = if let Some(Object::Name(name)) = color_space {
            name == b"DeviceGray"
        } else {
            false
        };

        // 最適化設定の決定 (Color/Gray/Mono のどれとして扱うか)
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

        // 目標DPIに基づくリサイズ計算
        let target_dpi = active_config.target_dpi;
        let threshold_dpi = active_config.threshold_dpi;
        
        // 推定DPI = width / (max_w_pt / 72)
        let est_w_in = max_w_pt / 72.0;
        let est_h_in = max_h_pt / 72.0;
        let current_dpi_w = if est_w_in > 0.0 { width as f32 / est_w_in } else { 300.0 };
        let current_dpi_h = if est_h_in > 0.0 { height as f32 / est_h_in } else { 300.0 };
        let current_dpi = current_dpi_w.max(current_dpi_h);

        let mut processed_img = img;
        let mut resized = false;

        // 解像度が閾値を超えていればリサイズ
        if active_config.downsample != "off" && current_dpi > threshold_dpi as f32 {
            let target_w = (est_w_in * target_dpi as f32).round() as u32;
            let target_h = (est_h_in * target_dpi as f32).round() as u32;
            
            // アスペクト比を維持しつつ縮小
            if target_w < width && target_h < height {
                processed_img = image_util::resize_image(&processed_img, target_w, target_h, &active_config.downsample);
                resized = true;
            }
        }

        // off が指定され、かつリサイズもされていない場合は元のストリームを維持する
        if active_config.format == "off" && !resized {
            continue;
        }

        // フォーマットと色の変換・保存
        let mut new_stream = stream_obj.clone();
        let new_data: Vec<u8>;
        let format_lower = active_config.format.to_lowercase();

        if mode == "mono" {
            // モノクロ二値化
            let gray = processed_img.to_luma8();
            let mono_method = active_config.mono_method.as_deref().unwrap_or("threshold");
            let binarized = if mono_method == "dither" {
                image_util::apply_floyd_steinberg(&gray)
            } else {
                image_util::apply_otsu(&gray)
            };

            if format_lower == "zip" || format_lower == "lzw" {
                // ロスレス圧縮 (FlateDecode)
                let packed_bytes = image_util::pack_mono_image(&binarized);
                if let Ok(deflated) = image_util::compress_deflate(&packed_bytes) {
                    new_data = deflated;
                    new_stream.dict.set("Width", binarized.width() as i64);
                    new_stream.dict.set("Height", binarized.height() as i64);
                    new_stream.dict.set("ColorSpace", Object::Name(b"DeviceGray".to_vec()));
                    new_stream.dict.set("BitsPerComponent", 1);
                    new_stream.dict.set("Filter", Object::Name(b"FlateDecode".to_vec()));
                    new_stream.dict.remove(b"DecodeParms");
                } else {
                    continue;
                }
            } else {
                // CCITT Group 4 エンコード (デフォルトおよび off/ccitt 指定時のフォールバック)
                if let Ok(g4_data) = image_util::encode_g4(&binarized) {
                    new_data = g4_data;
                    new_stream.dict.set("Width", binarized.width() as i64);
                    new_stream.dict.set("Height", binarized.height() as i64);
                    new_stream.dict.set("ColorSpace", Object::Name(b"DeviceGray".to_vec()));
                    new_stream.dict.set("BitsPerComponent", 1);
                    new_stream.dict.set("Filter", Object::Name(b"CCITTFaxDecode".to_vec()));
                    
                    let mut decode_parms = Dictionary::new();
                    decode_parms.set("Columns", binarized.width() as i64);
                    decode_parms.set("Rows", binarized.height() as i64);
                    decode_parms.set("K", -1);
                    new_stream.dict.set("DecodeParms", Object::Dictionary(decode_parms));
                } else {
                    continue; // G4エンコード失敗時はスキップ
                }
            }
        } else if mode == "gray" {
            // グレースケール化
            let gray = processed_img.to_luma8();
            if format_lower == "zip" || format_lower == "lzw" {
                // ロスレス圧縮 (FlateDecode)
                let raw_pixels = gray.as_raw();
                if let Ok(deflated) = image_util::compress_deflate(raw_pixels) {
                    new_data = deflated;
                    new_stream.dict.set("Width", gray.width() as i64);
                    new_stream.dict.set("Height", gray.height() as i64);
                    new_stream.dict.set("ColorSpace", Object::Name(b"DeviceGray".to_vec()));
                    new_stream.dict.set("BitsPerComponent", 8);
                    new_stream.dict.set("Filter", Object::Name(b"FlateDecode".to_vec()));
                    new_stream.dict.remove(b"DecodeParms");
                } else {
                    continue;
                }
            } else {
                // JPEG圧縮
                if let Ok(jpeg_data) = image_util::encode_jpeg_luma(&gray, active_config.quality) {
                    new_data = jpeg_data;
                    new_stream.dict.set("Width", gray.width() as i64);
                    new_stream.dict.set("Height", gray.height() as i64);
                    new_stream.dict.set("ColorSpace", Object::Name(b"DeviceGray".to_vec()));
                    new_stream.dict.set("BitsPerComponent", 8);
                    new_stream.dict.set("Filter", Object::Name(b"DCTDecode".to_vec()));
                    new_stream.dict.remove(b"DecodeParms");
                } else {
                    continue;
                }
            }
        } else {
            // カラー
            if format_lower == "zip" || format_lower == "lzw" {
                // ロスレス圧縮 (FlateDecode)
                let rgb = processed_img.to_rgb8();
                let raw_pixels = rgb.as_raw();
                if let Ok(deflated) = image_util::compress_deflate(raw_pixels) {
                    new_data = deflated;
                    new_stream.dict.set("Width", rgb.width() as i64);
                    new_stream.dict.set("Height", rgb.height() as i64);
                    new_stream.dict.set("ColorSpace", Object::Name(b"DeviceRGB".to_vec()));
                    new_stream.dict.set("BitsPerComponent", 8);
                    new_stream.dict.set("Filter", Object::Name(b"FlateDecode".to_vec()));
                    new_stream.dict.remove(b"DecodeParms");
                } else {
                    continue;
                }
            } else {
                // JPEG圧縮
                if let Ok(jpeg_data) = image_util::encode_jpeg_rgb(&processed_img, active_config.quality) {
                    new_data = jpeg_data;
                    new_stream.dict.set("Width", processed_img.width() as i64);
                    new_stream.dict.set("Height", processed_img.height() as i64);
                    new_stream.dict.set("ColorSpace", Object::Name(b"DeviceRGB".to_vec()));
                    new_stream.dict.set("BitsPerComponent", 8);
                    new_stream.dict.set("Filter", Object::Name(b"DCTDecode".to_vec()));
                    new_stream.dict.remove(b"DecodeParms");
                } else {
                    continue;
                }
            }
        }

        // ストリームのデータを更新
        new_stream.content = new_data;
        doc.set_object(img_id, Object::Stream(new_stream));
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
    let orig_size = std::fs::metadata(&input_path).map(|m| m.len()).unwrap_or(0);
    let opt_size = std::fs::metadata(&output_path).map(|m| m.len()).unwrap_or(0);

    // 完了を通知
    progress_callback(total_pages + total_images, total_pages + total_images);

    Ok((orig_size, opt_size))
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

