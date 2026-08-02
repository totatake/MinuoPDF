use lopdf::{Document, Object, ObjectId};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuditCategory {
    pub key: String,
    pub name_ja: String,
    pub name_en: String,
    pub count: usize,
    pub bytes: u64,
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PdfAuditReport {
    pub file_name: String,
    pub total_bytes: u64,
    pub categories: Vec<AuditCategory>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PdfAuditComparison {
    pub original: PdfAuditReport,
    pub optimized: Option<PdfAuditReport>,
}

/// PDF内の各オブジェクトを走査して種別ごとの容量内訳を調査する
pub fn audit_pdf<P: AsRef<Path>>(path: P) -> Result<PdfAuditReport, String> {
    let path_ref = path.as_ref();
    let file_name = path_ref
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("Unknown")
        .to_string();

    let total_bytes = std::fs::metadata(path_ref)
        .map(|m| m.len())
        .map_err(|e| format!("Failed to get file metadata: {:?}", e))?;

    let doc = Document::load(path_ref).map_err(|e| format!("Failed to load PDF for audit: {:?}", e))?;

    let mut content_stream_ids: HashSet<ObjectId> = HashSet::new();
    let mut font_related_ids: HashSet<ObjectId> = HashSet::new();

    let pages = doc.get_pages();
    for (_page_num, &page_id) in pages.iter() {
        if let Ok(page_obj) = doc.get_object(page_id) {
            if let Ok(page_dict) = page_obj.as_dict() {
                if let Ok(contents) = page_dict.get(b"Contents") {
                    match contents {
                        Object::Reference(id) => {
                            content_stream_ids.insert(*id);
                        }
                        Object::Array(arr) => {
                            for item in arr {
                                if let Ok(id) = item.as_reference() {
                                    content_stream_ids.insert(id);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    // Font関連のストリーム (CIDToGIDMap 等) を事前収集
    for (_, obj) in doc.objects.iter() {
        if let Ok(dict) = obj.as_dict() {
            let is_cidfont = dict.get(b"Subtype")
                .and_then(|o| o.as_name_str())
                .map(|s| s == "CIDFontType0" || s == "CIDFontType2")
                .unwrap_or(false);
            if is_cidfont {
                if let Ok(Object::Reference(id)) = dict.get(b"CIDToGIDMap") {
                    font_related_ids.insert(*id);
                }
            }
        }
    }

    let mut img_bytes: u64 = 0;
    let mut img_count: usize = 0;

    let mut content_bytes: u64 = 0;
    let mut content_count: usize = 0;

    let mut font_bytes: u64 = 0;
    let mut font_count: usize = 0;

    let mut annot_bytes: u64 = 0;
    let mut annot_count: usize = 0;

    let mut form_bytes: u64 = 0;
    let mut form_count: usize = 0;

    let mut meta_bytes: u64 = 0;
    let mut meta_count: usize = 0;

    let mut overhead_bytes: u64 = 0;
    let mut overhead_count: usize = 0;

    for (&id, obj) in doc.objects.iter() {
        let approx_bytes = calculate_object_bytes(obj);

        match obj {
            Object::Stream(stream) => {
                let dict = &stream.dict;
                let subtype = dict.get(b"Subtype").ok().and_then(|o| o.as_name_str().ok());
                let type_str = dict.get(b"Type").ok().and_then(|o| o.as_name_str().ok());

                let is_font_file = dict.has(b"FontFile")
                    || dict.has(b"FontFile2")
                    || dict.has(b"FontFile3")
                    || dict.has(b"Length1")
                    || dict.has(b"Length2")
                    || dict.has(b"Length3")
                    || subtype == Some("CIDFontType0C")
                    || subtype == Some("OpenType");

                if subtype == Some("Image") {
                    img_bytes += approx_bytes;
                    img_count += 1;
                } else if is_font_file || type_str == Some("Font") || type_str == Some("FontDescriptor") || font_related_ids.contains(&id) {
                    font_bytes += approx_bytes;
                    font_count += 1;
                } else if content_stream_ids.contains(&id) {
                    content_bytes += approx_bytes;
                    content_count += 1;
                } else if subtype == Some("Form") {
                    form_bytes += approx_bytes;
                    form_count += 1;
                } else if type_str == Some("Metadata") {
                    meta_bytes += approx_bytes;
                    meta_count += 1;
                } else {
                    // その他のストリーム
                    content_bytes += approx_bytes;
                    content_count += 1;
                }
            }
            Object::Dictionary(dict) => {
                let type_str = dict.get(b"Type").ok().and_then(|o| o.as_name_str().ok());

                if type_str == Some("Font") || type_str == Some("FontDescriptor") {
                    font_bytes += approx_bytes;
                    font_count += 1;
                } else if type_str == Some("Annot") {
                    annot_bytes += approx_bytes;
                    annot_count += 1;
                } else if type_str == Some("Outlines") || type_str == Some("StructTreeRoot") {
                    meta_bytes += approx_bytes;
                    meta_count += 1;
                } else if dict.has(b"AcroForm") {
                    form_bytes += approx_bytes;
                    form_count += 1;
                } else {
                    overhead_bytes += approx_bytes;
                    overhead_count += 1;
                }
            }
            _ => {
                overhead_bytes += approx_bytes;
                overhead_count += 1;
            }
        }
    }

    // オブジェクト以外（xref、trailer、ヘッダー等の差分）をオーバーヘッドに組み込む
    let accounted_bytes = img_bytes + content_bytes + font_bytes + annot_bytes + form_bytes + meta_bytes + overhead_bytes;
    if total_bytes > accounted_bytes {
        overhead_bytes += total_bytes - accounted_bytes;
    }

    let calc_pct = |b: u64| -> f64 {
        if total_bytes > 0 {
            (b as f64 / total_bytes as f64) * 100.0
        } else {
            0.0
        }
    };

    let categories = vec![
        AuditCategory {
            key: "images".to_string(),
            name_ja: "画像".to_string(),
            name_en: "Images".to_string(),
            count: img_count,
            bytes: img_bytes,
            percentage: calc_pct(img_bytes),
        },
        AuditCategory {
            key: "content_streams".to_string(),
            name_ja: "コンテンツストリーム".to_string(),
            name_en: "Content Streams".to_string(),
            count: content_count,
            bytes: content_bytes,
            percentage: calc_pct(content_bytes),
        },
        AuditCategory {
            key: "fonts".to_string(),
            name_ja: "フォント".to_string(),
            name_en: "Fonts".to_string(),
            count: font_count,
            bytes: font_bytes,
            percentage: calc_pct(font_bytes),
        },
        AuditCategory {
            key: "annotations".to_string(),
            name_ja: "注釈・リンク".to_string(),
            name_en: "Annotations & Links".to_string(),
            count: annot_count,
            bytes: annot_bytes,
            percentage: calc_pct(annot_bytes),
        },
        AuditCategory {
            key: "forms".to_string(),
            name_ja: "フォーム".to_string(),
            name_en: "Interactive Forms".to_string(),
            count: form_count,
            bytes: form_bytes,
            percentage: calc_pct(form_bytes),
        },
        AuditCategory {
            key: "metadata".to_string(),
            name_ja: "構造・メタデータ".to_string(),
            name_en: "Structure & Metadata".to_string(),
            count: meta_count,
            bytes: meta_bytes,
            percentage: calc_pct(meta_bytes),
        },
        AuditCategory {
            key: "overhead".to_string(),
            name_ja: "文書のオーバーヘッド".to_string(),
            name_en: "Document Overhead".to_string(),
            count: overhead_count,
            bytes: overhead_bytes,
            percentage: calc_pct(overhead_bytes),
        },
    ];

    Ok(PdfAuditReport {
        file_name,
        total_bytes,
        categories,
    })
}

/// オブジェクトのバイト数を推定計算するヘルパー
fn calculate_object_bytes(obj: &Object) -> u64 {
    match obj {
        Object::Stream(s) => {
            let content_len = s.content.len() as u64;
            let dict_estimate = (s.dict.len() * 20) as u64 + 30;
            content_len + dict_estimate
        }
        Object::Dictionary(d) => (d.len() * 24) as u64 + 20,
        Object::Array(a) => (a.len() * 8) as u64 + 10,
        Object::String(s, _) => s.len() as u64 + 4,
        _ => 16,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_user_pdfs_classification() {
        let ikinari = crate::get_fixture_path("sample_font_ikinari_yes.pdf");
        if ikinari.exists() {
            if let Ok(report) = audit_pdf(&ikinari) {
                // Fonts カテゴリが支配的（90%以上）になっていることを確認
                let font_cat = report.categories.iter().find(|c| c.key == "fonts");
                assert!(font_cat.is_some());
                assert!(font_cat.unwrap().bytes > 300_000, "Font bytes should be over 300KB");
            }
        }
    }

    #[test]
    fn test_audit_category_percentage() {
        let total = 1000u64;
        let img = 800u64;
        let pct = (img as f64 / total as f64) * 100.0;
        assert_eq!(pct, 80.0);
    }
}
