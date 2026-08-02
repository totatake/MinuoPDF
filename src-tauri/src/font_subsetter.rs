use std::collections::HashSet;
use lopdf::{Document, Object};
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::Write;
use allsorts::binary::read::ReadScope;
use allsorts::font_data::FontData;
use allsorts::tables::cmap::{Cmap, CmapSubtable, PlatformId, EncodingId};
use allsorts::tables::FontTableProvider;
use allsorts::subset::subset;

/// PDF内の全埋め込みフォントを抽出し、使用中のグリフのみでサブセット化してストリームを差し替える
pub fn subset_pdf_fonts(doc: &mut Document) -> usize {
    // Phase 1: Content Streamから直接GID候補を収集 (hex文字列の2バイトペア)
    let raw_gids = collect_raw_gids_from_streams(doc);

    // Phase 2: フォント辞書のW配列からCID/GIDを収集 (フォールバック)
    let w_gids = collect_gids_from_w_arrays(doc);

    // Phase 3: 全使用GIDをマージ
    let mut all_gids: HashSet<u16> = HashSet::new();
    all_gids.insert(0); // .notdef は常に含める
    all_gids.extend(&raw_gids);
    all_gids.extend(&w_gids);

    let mut subsetted_count = 0;

    // Phase 4: FontDescriptorを走査してフォントストリームをサブセット化
    let mut font_desc_ids = Vec::new();
    for (&obj_id, obj) in doc.objects.iter() {
        if let Ok(dict) = obj.as_dict() {
            if let Ok(type_name) = dict.get(b"Type").and_then(|o| o.as_name_str()) {
                if type_name == "FontDescriptor" {
                    font_desc_ids.push(obj_id);
                }
            }
        }
    }

    let mut processed_streams = HashSet::new();

    for desc_id in font_desc_ids {
        let mut font_file_ref = None;

        if let Ok(Object::Dictionary(ref dict)) = doc.get_object(desc_id) {
            if let Ok(r) = dict.get(b"FontFile2").and_then(|o| o.as_reference()) {
                font_file_ref = Some((r, "FontFile2"));
            } else if let Ok(r) = dict.get(b"FontFile3").and_then(|o| o.as_reference()) {
                font_file_ref = Some((r, "FontFile3"));
            }
        }

        if let Some((stream_id, font_key)) = font_file_ref {
            if processed_streams.contains(&stream_id) {
                continue;
            }

            // Find all FontDescriptors pointing to this stream_id
            let mut desc_ids_for_stream = Vec::new();
            for (&obj_id, obj) in doc.objects.iter() {
                if let Ok(dict) = obj.as_dict() {
                    if let Ok(type_name) = dict.get(b"Type").and_then(|o| o.as_name_str()) {
                        if type_name == "FontDescriptor" {
                            if dict.get(b"FontFile2").and_then(|o| o.as_reference()).ok() == Some(stream_id)
                                || dict.get(b"FontFile3").and_then(|o| o.as_reference()).ok() == Some(stream_id)
                            {
                                desc_ids_for_stream.push(obj_id);
                            }
                        }
                    }
                }
            }

            // Find all CIDFonts referencing any FontDescriptor for this stream
            let mut referencing_cidfonts = Vec::new();
            for (&obj_id, obj) in doc.objects.iter() {
                if let Ok(dict) = obj.as_dict() {
                    let is_cidfont = dict.get(b"Subtype")
                        .and_then(|o| o.as_name_str())
                        .map(|s| s == "CIDFontType0" || s == "CIDFontType2")
                        .unwrap_or(false);
                    if is_cidfont {
                        if let Ok(fd_ref) = dict.get(b"FontDescriptor").and_then(|o| o.as_reference()) {
                            if desc_ids_for_stream.contains(&fd_ref) {
                                referencing_cidfonts.push(obj_id);
                            }
                        }
                    }
                }
            }

            if process_font_stream(doc, stream_id, font_key, &all_gids, &referencing_cidfonts) {
                subsetted_count += 1;
                processed_streams.insert(stream_id);
            }
        }
    }

    subsetted_count
}

/// Content Streamから全ページのテキスト描画オペレータを解析し、
/// <hex> 文字列の2バイトペアをGIDとして、
/// (...) 文字列の各バイトもGID候補として収集する
fn collect_raw_gids_from_streams(doc: &Document) -> HashSet<u16> {
    let mut gids = HashSet::new();

    for (_page_num, page_id) in doc.get_pages() {
        if let Ok(page_dict) = doc.get_object(page_id).and_then(|o| o.as_dict()) {
            if let Ok(contents) = page_dict.get(b"Contents") {
                let content_streams = get_content_streams(doc, contents);
                for stream_bytes in content_streams {
                    extract_gids_from_stream(&stream_bytes, &mut gids);
                }
            }
        }
    }

    gids
}

fn get_content_streams(doc: &Document, contents_obj: &Object) -> Vec<Vec<u8>> {
    let mut streams = Vec::new();
    match contents_obj {
        Object::Reference(id) => {
            if let Ok(Object::Stream(s)) = doc.get_object(*id) {
                if let Ok(decompressed) = s.decompressed_content() {
                    streams.push(decompressed);
                } else {
                    streams.push(s.content.clone());
                }
            }
        }
        Object::Array(arr) => {
            for item in arr {
                if let Ok(id) = item.as_reference() {
                    if let Ok(Object::Stream(s)) = doc.get_object(id) {
                        if let Ok(decompressed) = s.decompressed_content() {
                            streams.push(decompressed);
                        } else {
                            streams.push(s.content.clone());
                        }
                    }
                }
            }
        }
        _ => {}
    }
    streams
}

/// Content Streamのバイト列からテキスト描画オペレータのオペランドを抽出し、GID候補を収集する。
///
/// Identity-H エンコーディングの CIDFont では、<hex> 文字列の各2バイトが GID に直結する。
/// 通常のフォントでは (...) 文字列の各バイトが文字コードとなる。
fn extract_gids_from_stream(data: &[u8], gids: &mut HashSet<u16>) {
    let text = String::from_utf8_lossy(data);
    for line in text.lines() {
        let trimmed = line.trim();
        // テキスト描画オペレータを検出: Tj, TJ, ', "
        let is_text_op = trimmed.ends_with("Tj") || trimmed.ends_with("TJ")
            || trimmed.ends_with("'") || trimmed.ends_with("\"");
        if !is_text_op {
            continue;
        }

        // <hex> 文字列の解析: Identity-H での GID 直接取得
        extract_hex_gids(trimmed, gids);

        // (...) 文字列の解析: 各バイトを GID 候補として収集
        extract_paren_gids(trimmed, gids);
    }
}

/// `<AABB CCDD ...>` 形式の hex 文字列から 2バイトペアを GID として抽出
fn extract_hex_gids(line: &str, gids: &mut HashSet<u16>) {
    let bytes = line.as_bytes();
    let len = bytes.len();
    let mut i = 0;
    while i < len {
        if bytes[i] == b'<' && (i == 0 || bytes[i.wrapping_sub(1)] != b'<') {
            // 先頭の << (辞書開始) は除外
            if i + 1 < len && bytes[i + 1] == b'<' {
                i += 2;
                continue;
            }
            i += 1; // skip '<'
            let mut hex_bytes = Vec::new();
            while i < len && bytes[i] != b'>' {
                let ch = bytes[i];
                if ch.is_ascii_hexdigit() {
                    hex_bytes.push(ch);
                }
                i += 1;
            }
            // hex_bytes を2文字ずつデコード → バイト列
            let mut decoded = Vec::new();
            let mut j = 0;
            while j + 1 < hex_bytes.len() {
                let hi = hex_val(hex_bytes[j]);
                let lo = hex_val(hex_bytes[j + 1]);
                decoded.push((hi << 4) | lo);
                j += 2;
            }
            // 2バイトペアを GID として収集
            let mut k = 0;
            while k + 1 < decoded.len() {
                let gid = ((decoded[k] as u16) << 8) | (decoded[k + 1] as u16);
                if gid > 0 {
                    gids.insert(gid);
                }
                k += 2;
            }
            // 奇数バイトの場合、単バイトも GID 候補に
            if decoded.len() == 1 {
                gids.insert(decoded[0] as u16);
            }
        }
        i += 1;
    }
}

fn hex_val(b: u8) -> u8 {
    match b {
        b'0'..=b'9' => b - b'0',
        b'a'..=b'f' => b - b'a' + 10,
        b'A'..=b'F' => b - b'A' + 10,
        _ => 0,
    }
}

/// `(text)` 形式の文字列から各バイトを GID 候補として抽出
fn extract_paren_gids(line: &str, gids: &mut HashSet<u16>) {
    let bytes = line.as_bytes();
    let len = bytes.len();
    let mut i = 0;
    let mut depth = 0;
    while i < len {
        if bytes[i] == b'(' {
            if depth == 0 {
                depth = 1;
            } else {
                depth += 1;
                // ネストされた括弧内の文字もGID候補
                gids.insert(bytes[i] as u16);
            }
        } else if bytes[i] == b')' {
            if depth > 0 {
                depth -= 1;
                if depth > 0 {
                    gids.insert(bytes[i] as u16);
                }
            }
        } else if depth > 0 {
            if bytes[i] == b'\\' && i + 1 < len {
                // エスケープシーケンスはスキップ（次の文字を含める）
                i += 1;
                gids.insert(bytes[i] as u16);
            } else {
                gids.insert(bytes[i] as u16);
            }
        }
        i += 1;
    }
}

/// PDF内の全CIDFont辞書の W(Widths) 配列から CID を収集する。
/// Identity-H + Identity CIDToGIDMap の場合、CID = GID となる。
fn collect_gids_from_w_arrays(doc: &Document) -> HashSet<u16> {
    let mut gids = HashSet::new();

    for (_, obj) in doc.objects.iter() {
        if let Ok(dict) = obj.as_dict() {
            // CIDFont辞書かどうかをチェック
            let is_cidfont = dict.get(b"Subtype")
                .and_then(|o| o.as_name_str())
                .map(|s| s == "CIDFontType0" || s == "CIDFontType2")
                .unwrap_or(false);

            if !is_cidfont {
                continue;
            }

            // W 配列を解析
            if let Ok(w_obj) = dict.get(b"W") {
                let w_arr = match w_obj {
                    Object::Array(arr) => Some(arr.clone()),
                    Object::Reference(id) => {
                        doc.get_object(*id).ok().and_then(|o| o.as_array().ok().cloned())
                    },
                    _ => None,
                };

                if let Some(arr) = w_arr {
                    parse_w_array(&arr, &mut gids);
                }
            }
        }
    }

    gids
}

/// CIDFont の W 配列を解析して CID を収集する。
/// W 配列の形式:
///   [cid_first [w1 w2 ...]]   → cid_first, cid_first+1, ... の各CID
///   [cid_first cid_last w]    → cid_first 〜 cid_last の範囲の全CID
fn parse_w_array(arr: &[Object], gids: &mut HashSet<u16>) {
    let mut i = 0;
    while i < arr.len() {
        if let Ok(cid_start) = arr[i].as_i64() {
            if i + 1 < arr.len() {
                match &arr[i + 1] {
                    Object::Array(widths) => {
                        // [cid_start [w1 w2 ...]] 形式
                        for j in 0..widths.len() {
                            let cid = cid_start as u16 + j as u16;
                            gids.insert(cid);
                        }
                        i += 2;
                    }
                    _ => {
                        // [cid_start cid_last w] 形式
                        if i + 2 < arr.len() {
                            if let Ok(cid_end) = arr[i + 1].as_i64() {
                                for cid in (cid_start as u16)..=(cid_end as u16) {
                                    gids.insert(cid);
                                }
                            }
                            i += 3;
                        } else {
                            i += 1;
                        }
                    }
                }
            } else {
                i += 1;
            }
        } else {
            i += 1;
        }
    }
}

fn process_font_stream(
    doc: &mut Document, 
    stream_id: (u32, u16), 
    font_key: &str, 
    all_gids: &HashSet<u16>,
    referencing_cidfonts: &[lopdf::ObjectId],
) -> bool {
    let font_bytes = match doc.get_object(stream_id) {
        Ok(Object::Stream(s)) => {
            if let Ok(decompressed) = s.decompressed_content() {
                decompressed
            } else {
                s.content.clone()
            }
        }
        _ => return false,
    };

    if font_bytes.len() < 100 {
        return false;
    }

    if let Some((subsetted_bytes, used_gids)) = subset_font_binary(&font_bytes, all_gids) {
        if subsetted_bytes.len() < font_bytes.len() {
            let uncompressed_len = subsetted_bytes.len() as u64;

            let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());
            if encoder.write_all(&subsetted_bytes).is_ok() {
                if let Ok(compressed) = encoder.finish() {
                    if let Ok(Object::Stream(ref mut stream)) = doc.get_object_mut(stream_id) {
                        stream.content = compressed;
                        stream.dict.set("Filter", Object::Name(b"FlateDecode".to_vec()));
                        if font_key == "FontFile2" {
                            stream.dict.set("Length1", Object::Integer(uncompressed_len as i64));
                        }
                    }

                    // --- ADD CIDToGIDMap ---
                    if !referencing_cidfonts.is_empty() {
                        let max_cid = *used_gids.last().unwrap_or(&0) as usize;
                        let mut cid_to_gid_map = vec![0u8; (max_cid + 1) * 2];
                        for (new_gid, &old_gid) in used_gids.iter().enumerate() {
                            let old_cid = old_gid as usize;
                            cid_to_gid_map[old_cid * 2] = (new_gid >> 8) as u8;
                            cid_to_gid_map[old_cid * 2 + 1] = (new_gid & 0xFF) as u8;
                        }

                        let mut cmap_encoder = ZlibEncoder::new(Vec::new(), Compression::best());
                        cmap_encoder.write_all(&cid_to_gid_map).unwrap();
                        let cmap_compressed = cmap_encoder.finish().unwrap();

                        let mut cmap_dict = lopdf::Dictionary::new();
                        cmap_dict.set("Filter", Object::Name(b"FlateDecode".to_vec()));

                        let cmap_stream = lopdf::Stream::new(cmap_dict, cmap_compressed);
                        let cmap_obj_id = doc.add_object(Object::Stream(cmap_stream));

                        for &cidfont_id in referencing_cidfonts {
                            if let Ok(Object::Dictionary(ref mut dict)) = doc.get_object_mut(cidfont_id) {
                                dict.set("CIDToGIDMap", Object::Reference(cmap_obj_id));
                            }
                        }
                    }

                    return true;
                }
            }
        }
    }

    false
}

/// フォントバイナリを受け取り、指定された GID のみを残してサブセット化する。
/// all_gids に直接 GID が含まれている場合はそれを使用し、
/// 追加でフォントの cmap テーブルから Unicode→GID マッピングも試みる。
fn subset_font_binary(font_data_bytes: &[u8], all_gids: &HashSet<u16>) -> Option<(Vec<u8>, Vec<u16>)> {
    let scope = ReadScope::new(font_data_bytes);
    let font_data = scope.read::<FontData>().ok()?;
    let provider = font_data.table_provider(0).ok()?;

    let mut glyph_ids: HashSet<u16> = all_gids.clone();
    glyph_ids.insert(0); // .notdef

    // cmap テーブルが存在する場合、Unicode→GID マッピングも収集
    // (all_gids にはバイト値や hex 値が含まれており、一部は Unicode コードポイントと一致する可能性がある)
    if let Ok(cmap_buf) = provider.read_table_data(allsorts::tag::CMAP) {
        let cmap_scope = ReadScope::new(&cmap_buf);
        if let Ok(cmap) = cmap_scope.read::<Cmap>() {
            let record_opt = cmap.find_subtable(PlatformId::UNICODE, EncodingId::UNICODE_FULL)
                .or_else(|| cmap.find_subtable(PlatformId::UNICODE, EncodingId::UNICODE_BMP))
                .or_else(|| cmap.find_subtable(PlatformId::WINDOWS, EncodingId::WINDOWS_UNICODE_BMP_UCS2));

            if let Some(record) = record_opt {
                if let Ok(subtable) = cmap_scope.offset(record.offset as usize).read::<CmapSubtable>() {
                    // all_gids 内の値を Unicode コードポイントとしても試す
                    for &code in all_gids {
                        if let Ok(Some(gid)) = subtable.map_glyph(code as u32) {
                            glyph_ids.insert(gid);
                        }
                    }
                }
            }
        }
    }

    // フォント内に存在する GID のみに絞り込む（フォントの maxp テーブルで上限チェック）
    if let Ok(maxp_buf) = provider.read_table_data(allsorts::tag::MAXP) {
        if maxp_buf.len() >= 6 {
            let num_glyphs = ((maxp_buf[4] as u16) << 8) | (maxp_buf[5] as u16);
            glyph_ids.retain(|&gid| gid < num_glyphs);
        }
    }

    let mut glyph_ids_vec: Vec<u16> = glyph_ids.into_iter().collect();
    glyph_ids_vec.sort_unstable();

    let subsetted_bytes = subset(&provider, &glyph_ids_vec).ok()?;
    Some((subsetted_bytes, glyph_ids_vec))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_hex_gids() {
        let mut gids = HashSet::new();
        // Identity-H CIDFont で使われる形式
        extract_hex_gids("[<08C7>] TJ", &mut gids);
        assert!(gids.contains(&0x08C7u16));
    }

    #[test]
    fn test_extract_hex_gids_multi() {
        let mut gids = HashSet::new();
        extract_hex_gids("[<08C708E5089B>] TJ", &mut gids);
        assert!(gids.contains(&0x08C7u16));
        assert!(gids.contains(&0x08E5u16));
        assert!(gids.contains(&0x089Bu16));
    }

    #[test]
    fn test_extract_paren_gids() {
        let mut gids = HashSet::new();
        extract_paren_gids("(Hello) Tj", &mut gids);
        assert!(gids.contains(&(b'H' as u16)));
        assert!(gids.contains(&(b'e' as u16)));
        assert!(gids.contains(&(b'l' as u16)));
        assert!(gids.contains(&(b'o' as u16)));
    }

    #[test]
    fn test_parse_w_array_bracket_form() {
        // [10 [500 600 700]] → CID 10, 11, 12
        let arr = vec![
            Object::Integer(10),
            Object::Array(vec![
                Object::Integer(500),
                Object::Integer(600),
                Object::Integer(700),
            ]),
        ];
        let mut gids = HashSet::new();
        parse_w_array(&arr, &mut gids);
        assert!(gids.contains(&10u16));
        assert!(gids.contains(&11u16));
        assert!(gids.contains(&12u16));
        assert!(!gids.contains(&13u16));
    }

    #[test]
    fn test_parse_w_array_range_form() {
        // [100 105 500] → CID 100, 101, 102, 103, 104, 105
        let arr = vec![
            Object::Integer(100),
            Object::Integer(105),
            Object::Integer(500),
        ];
        let mut gids = HashSet::new();
        parse_w_array(&arr, &mut gids);
        for cid in 100..=105u16 {
            assert!(gids.contains(&cid));
        }
        assert!(!gids.contains(&106u16));
    }

    #[test]
    fn test_subset_ikinari_pdf_verification() {
        let ikinari = crate::get_fixture_path("sample_font_ikinari_yes.pdf");
        if ikinari.exists() {
            let mut doc = lopdf::Document::load(&ikinari).unwrap();
            let orig_size = std::fs::metadata(&ikinari).unwrap().len();
            let count = subset_pdf_fonts(&mut doc);
            let mut out = Vec::new();
            doc.save_to(&mut out).unwrap();
            let opt_size = out.len();
            println!("\n========================================================");
            println!("IKINARI FONT SUBSETTING RESULT:");
            println!("  - Subsetted fonts count: {}", count);
            println!("  - Original file size: {} bytes ({:.2} KB)", orig_size, orig_size as f64 / 1024.0);
            println!("  - Subsetted file size: {} bytes ({:.2} KB)", opt_size, opt_size as f64 / 1024.0);
            println!("  - Total Reduction: -{:.1}%", (1.0 - (opt_size as f64 / orig_size as f64)) * 100.0);
            println!("========================================================\n");
            assert!(count > 0, "At least one font should be subsetted");
            assert!(opt_size < orig_size as usize, "Subsetted size must be smaller than original");
            // サイズが元の半分以下になることを期待（89%削減が目標）
            assert!(opt_size < (orig_size as usize / 2), "Should achieve significant reduction");
        }
    }
}
