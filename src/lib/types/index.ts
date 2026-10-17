export interface ImageConfig {
  downsample: string;
  target_dpi: number;
  threshold_dpi: number;
  format: string;
  quality: number;
  mono_method: string | null;
}

export interface OptimizeConfig {
  version: number;
  color_conv: string;
  color_img: ImageConfig;
  gray_img: ImageConfig;
  mono_img: ImageConfig;
  discard_comments: boolean;
  discard_links: boolean;
  discard_bookmarks: boolean;
  discard_forms: boolean;
  discard_metadata: boolean;
  discard_js: boolean;
  discard_thumbs: boolean;
  clean_linearize: boolean;
  clean_deflate: boolean;
  clean_sanitize: boolean;
  font_subset: boolean;
  unembed_fonts: string[];
  force_rasterize: boolean;
}

export interface AuditCategory {
  key: string;
  name_ja: string;
  name_en: string;
  count: number;
  bytes: number;
  percentage: number;
}

export interface PdfAuditReport {
  file_name: string;
  total_bytes: number;
  categories: AuditCategory[];
}

export interface PdfAuditComparison {
  original: PdfAuditReport;
  optimized: PdfAuditReport | null;
}

export interface LogItem {
  file_path: string;
  success: boolean;
  orig_size: number;
  opt_size: number;
  message: string;
  audit?: PdfAuditComparison;
}

export const CATEGORY_COLORS: Record<string, string> = {
  images: "#10b981",          // Emerald green
  content_streams: "#3b82f6", // Blue
  fonts: "#8b5cf6",           // Purple
  annotations: "#f59e0b",     // Amber
  forms: "#ec4899",           // Pink
  metadata: "#06b6d4",        // Cyan
  overhead: "#64748b",        // Slate gray
};

