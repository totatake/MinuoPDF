use image::{DynamicImage, ImageBuffer, Luma};
use image::codecs::jpeg::JpegEncoder;
use std::io::Cursor;

/// 大津の二値化の最適な閾値を算出する
pub fn otsu_threshold(img: &ImageBuffer<Luma<u8>, Vec<u8>>) -> u8 {
    let mut histogram = [0u32; 256];
    for pixel in img.pixels() {
        histogram[pixel[0] as usize] += 1;
    }

    let total = img.width() * img.height();
    let mut sum = 0.0;
    for (i, &count) in histogram.iter().enumerate() {
        sum += (i as f32) * (count as f32);
    }

    let mut sum_b = 0.0;
    let mut w_b = 0u32;
    let mut max_variance = 0.0;
    let mut threshold = 127u8;

    for t in 0..256 {
        w_b += histogram[t];
        if w_b == 0 { continue; }
        
        let w_f = total - w_b;
        if w_f == 0 { break; }

        sum_b += (t as f32) * (histogram[t] as f32);
        
        let m_b = sum_b / (w_b as f32);
        let m_f = (sum - sum_b) / (w_f as f32);

        // クラス間分散の計算
        let variance = (w_b as f32) * (w_f as f32) * (m_b - m_f) * (m_b - m_f);

        if variance > max_variance {
            max_variance = variance;
            threshold = t as u8;
        }
    }

    threshold
}

/// 大津の二値化を適用して、0か255のグレースケール画像を返す
pub fn apply_otsu(img: &ImageBuffer<Luma<u8>, Vec<u8>>) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let th = otsu_threshold(img);
    let mut out = img.clone();
    for pixel in out.pixels_mut() {
        pixel[0] = if pixel[0] > th { 255 } else { 0 };
    }
    out
}

/// Floyd-Steinberg ディザリングを適用して、0か255の二値画像を作る
pub fn apply_floyd_steinberg(img: &ImageBuffer<Luma<u8>, Vec<u8>>) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let width = img.width();
    let height = img.height();
    let mut buffer: Vec<i32> = img.pixels().map(|p| p[0] as i32).collect();
    let mut out = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) as usize;
            let old_val = buffer[idx];
            let new_val = if old_val > 127 { 255 } else { 0 };
            
            // 出力ピクセルに設定
            out.put_pixel(x, y, Luma([new_val as u8]));

            let error = old_val - new_val;

            // 誤差を周囲の画素に拡散
            if x + 1 < width {
                buffer[idx + 1] += error * 7 / 16;
            }
            if y + 1 < height {
                let next_row_idx = ((y + 1) * width) as usize;
                if x > 0 {
                    buffer[next_row_idx + x as usize - 1] += error * 3 / 16;
                }
                buffer[next_row_idx + x as usize] += error * 5 / 16;
                if x + 1 < width {
                    buffer[next_row_idx + x as usize + 1] += error * 1 / 16;
                }
            }
        }
    }
    out
}

/// 0または255のモノクログレースケール画像をCCITT Group 4圧縮の生バイト列に変換する
pub fn encode_g4(img: &ImageBuffer<Luma<u8>, Vec<u8>>) -> Result<Vec<u8>, String> {
    use fax::Color;
    let width = img.width() as u16;
    let height = img.height() as usize;

    let writer = fax::VecWriter::new();
    let mut encoder = fax::encoder::Encoder::new(writer);

    let pixels: Vec<u8> = img.pixels().map(|p| p[0]).collect();

    for y in 0..height {
        let row_start = y * width as usize;
        let mut row = Vec::with_capacity(width as usize);
        for x in 0..width as usize {
            let val = pixels[row_start + x];
            if val > 127 {
                row.push(Color::White);
            } else {
                row.push(Color::Black);
            }
        }
        encoder.encode_line(row.into_iter(), width).map_err(|e| format!("G4 encoding error: {:?}", e))?;
    }

    let writer = encoder.finish().map_err(|e| format!("G4 finish error: {:?}", e))?;
    Ok(writer.finish())
}

/// カラー画像を JPEG にエンコードする
pub fn encode_jpeg_rgb(img: &DynamicImage, quality: u8) -> Result<Vec<u8>, String> {
    let rgb = img.to_rgb8();
    let mut buf = Vec::new();
    let mut cursor = Cursor::new(&mut buf);
    let mut encoder = JpegEncoder::new_with_quality(&mut cursor, quality);
    encoder.encode_image(&rgb).map_err(|e| format!("JPEG encoding error: {:?}", e))?;
    Ok(buf)
}

/// グレースケール画像を JPEG にエンコードする
pub fn encode_jpeg_luma(img: &ImageBuffer<Luma<u8>, Vec<u8>>, quality: u8) -> Result<Vec<u8>, String> {
    let mut buf = Vec::new();
    let mut cursor = Cursor::new(&mut buf);
    let mut encoder = JpegEncoder::new_with_quality(&mut cursor, quality);
    encoder.encode_image(img).map_err(|e| format!("JPEG encoding error: {:?}", e))?;
    Ok(buf)
}

/// 画像を指定のサイズに指定のフィルタでリサイズする
pub fn resize_image(img: &DynamicImage, width: u32, height: u32, algorithm: &str) -> DynamicImage {
    use image::imageops::FilterType;
    let filter = match algorithm.to_lowercase().as_str() {
        "nearest" => FilterType::Nearest,
        "bilinear" => FilterType::Triangle,
        "bicubic" => FilterType::CatmullRom,
        "lanczos" => FilterType::Lanczos3,
        "average" => FilterType::Gaussian,
        _ => FilterType::CatmullRom, // デフォルトは bicubic
    };
    img.resize_exact(width, height, filter)
}

/// zlib (Flate) 圧縮を行う
pub fn compress_deflate(data: &[u8]) -> Result<Vec<u8>, String> {
    use flate2::write::ZlibEncoder;
    use flate2::Compression;
    use std::io::Write;
    
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).map_err(|e| format!("Deflate compression failed: {:?}", e))?;
    encoder.finish().map_err(|e| format!("Deflate finish failed: {:?}", e))
}

/// 0または255のモノクログレースケール画像を、各行をバイト境界にアラインした1bitパックバイト列に変換する
pub fn pack_mono_image(img: &ImageBuffer<Luma<u8>, Vec<u8>>) -> Vec<u8> {
    let width = img.width() as usize;
    let height = img.height() as usize;
    let row_stride = (width + 7) / 8;
    let mut packed = Vec::with_capacity(row_stride * height);

    for y in 0..height {
        let mut current_byte = 0u8;
        let mut bit_count = 0;
        for x in 0..width {
            let pixel = img.get_pixel(x as u32, y as u32)[0];
            let bit = if pixel > 127 { 1 } else { 0 };
            current_byte = (current_byte << 1) | bit;
            bit_count += 1;
            if bit_count == 8 {
                packed.push(current_byte);
                current_byte = 0;
                bit_count = 0;
            }
        }
        if bit_count > 0 {
            current_byte <<= 8 - bit_count;
            packed.push(current_byte);
        }
    }
    packed
}

