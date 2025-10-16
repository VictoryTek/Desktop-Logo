use std::path::Path;
use image::{DynamicImage, GenericImage, GenericImageView, ImageFormat, RgbaImage};
use anyhow::{Result, bail, Context};

#[derive(Clone, Copy, Debug)]
pub enum Position { TopLeft, TopRight, BottomLeft, BottomRight }

pub struct CompositeOptions<'a> {
    pub base_path: &'a Path,
    pub logo_path: &'a Path,
    pub output_path: &'a Path,
    pub position: Position,
    pub margin: u32,
    pub max_logo_percent: f32, // relative to shortest side
    pub opacity: f32,          // 0.0 - 1.0
}

pub fn composite_logo(opts: &CompositeOptions) -> Result<()> {
    if !(0.0..=1.0).contains(&opts.opacity) { bail!("opacity must be between 0 and 1"); }
    if !(0.01..=0.9).contains(&opts.max_logo_percent) { bail!("max_logo_percent should be sensible (0.01-0.9)"); }

    let mut base = load_image(opts.base_path).with_context(|| format!("loading base image {:?}", opts.base_path))?;
    let mut logo = load_image(opts.logo_path).with_context(|| format!("loading logo image {:?}", opts.logo_path))?;

    // Scale logo
    let (bw, bh) = base.dimensions();
    let target_side = (bw.min(bh) as f32 * opts.max_logo_percent).round().max(1.0) as u32;
    let (lw, lh) = logo.dimensions();
    let scale = if lw >= lh { target_side as f32 / lw as f32 } else { target_side as f32 / lh as f32 };
    let new_w = (lw as f32 * scale).round().max(1.0) as u32;
    let new_h = (lh as f32 * scale).round().max(1.0) as u32;
    logo = logo.resize_exact(new_w, new_h, image::imageops::FilterType::Lanczos3);

    // Apply opacity if needed
    let mut logo_rgba = logo.to_rgba8();
    if opts.opacity < 1.0 { adjust_opacity(&mut logo_rgba, opts.opacity); }

    let (x, y) = placement_coords(opts.position, bw, bh, new_w, new_h, opts.margin);
    overlay(&mut base, &logo_rgba, x, y);

    // Guess format by extension
    let fmt = infer_format_from_extension(opts.output_path).unwrap_or(ImageFormat::Png);
    base.save_with_format(opts.output_path, fmt).with_context(|| format!("saving composited image to {:?}", opts.output_path))?;
    Ok(())
}

fn load_image(p: &Path) -> Result<DynamicImage> { Ok(image::open(p)?) }

fn adjust_opacity(img: &mut RgbaImage, opacity: f32) {
    for p in img.pixels_mut() { p[3] = ((p[3] as f32) * opacity).round().clamp(0.0, 255.0) as u8; }
}

fn placement_coords(pos: Position, bw: u32, bh: u32, lw: u32, lh: u32, margin: u32) -> (u32,u32) {
    match pos {
        Position::TopLeft => (margin, margin),
        Position::TopRight => (bw.saturating_sub(lw + margin), margin),
        Position::BottomLeft => (margin, bh.saturating_sub(lh + margin)),
        Position::BottomRight => (bw.saturating_sub(lw + margin), bh.saturating_sub(lh + margin)),
    }
}

fn overlay(base: &mut DynamicImage, logo: &RgbaImage, x0: u32, y0: u32) {
    for (lx, ly, p) in logo.enumerate_pixels() {
        let (x, y) = (x0 + lx, y0 + ly);
        if x >= base.width() || y >= base.height() { continue; }
        let mut dst = base.get_pixel(x, y);
        let src_a = p[3] as f32 / 255.0;
        if src_a <= 0.0 { continue; }
        for c in 0..3 { dst[c] = ((p[c] as f32 * src_a) + (dst[c] as f32 * (1.0 - src_a))).round().clamp(0.0,255.0) as u8; }
        // alpha combine (simple max for now)
        dst[3] = 255; // ensure opaque result
        base.put_pixel(x, y, dst);
    }
}

fn infer_format_from_extension(p: &Path) -> Option<ImageFormat> {
    let ext = p.extension()?.to_string_lossy().to_ascii_lowercase();
    match ext.as_str() { "png" => Some(ImageFormat::Png), "jpg"|"jpeg" => Some(ImageFormat::Jpeg), _ => None }
}

#[cfg(test)]
mod tests {
    use super::*; use std::fs; use std::io::Write; use tempfile::tempdir; use image::{Rgba};

    fn write_solid_png(path: &Path, w: u32, h: u32, color: [u8;4]) {
        let mut img = RgbaImage::new(w,h);
        for p in img.pixels_mut() { *p = Rgba(color); }
        img.save(path).unwrap();
    }

    #[test]
    fn basic_composite() {
        let dir = tempdir().unwrap();
        let base = dir.path().join("base.png");
        let logo = dir.path().join("logo.png");
        let out = dir.path().join("out.png");
        write_solid_png(&base, 800, 600, [10,20,30,255]);
        write_solid_png(&logo, 200, 100, [200,50,50,255]);
        let opts = CompositeOptions { base_path: &base, logo_path: &logo, output_path: &out, position: Position::BottomRight, margin: 50, max_logo_percent: 0.2, opacity: 0.8 };
        composite_logo(&opts).unwrap();
        assert!(out.exists());
        let result = image::open(out).unwrap();
        assert_eq!(result.width(), 800);
        assert_eq!(result.height(), 600);
    }
}
