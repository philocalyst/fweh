//! Utility functions and types

use anyhow::Result;
use image::Rgba;
use tempfile::NamedTempFile;

/// A 2D point with floating-point coordinates
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    /// Create a new point
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Convert to integer coordinates
    pub fn to_i32(&self) -> (i32, i32) {
        (self.x as i32, self.y as i32)
    }

    /// Convert to unsigned integer coordinates
    pub fn to_u32(&self) -> (u32, u32) {
        (self.x.max(0.0) as u32, self.y.max(0.0) as u32)
    }
}

/// Create a temporary file with the given extension
pub fn create_temp_file(extension: &str) -> Result<NamedTempFile> {
    let temp_file = tempfile::Builder::new()
        .suffix(&format!(".{}", extension))
        .tempfile()?;
    Ok(temp_file)
}

/// Type representing corner radii for all four corners
/// Ordered as: (top_left, top_right, bottom_right, bottom_left)
pub type CornerRadii = (u32, u32, u32, u32);

/// Blend two colors based on alpha
pub fn blend_color(color1: Rgba<u8>, color2: Rgba<u8>, alpha: f32) -> Rgba<u8> {
    let blend =
        |a: u8, b: u8, alpha: f32| -> u8 { (a as f32 * (1.0 - alpha) + b as f32 * alpha) as u8 };

    Rgba([
        blend(color1[0], color2[0], alpha),
        blend(color1[1], color2[1], alpha),
        blend(color1[2], color2[2], alpha),
        blend(color1[3], color2[3], alpha),
    ])
}

/// Calculate padding to maintain aspect ratio
pub fn calculate_padding(
    width: u32,
    height: u32,
    target_ratio: f32,
    scale: f32,
) -> (u32, u32, u32, u32, u32, u32) {
    let original_ratio = width as f32 / height as f32;
    let scale_factor = scale / 100.0;

    let (new_width, new_height);

    if target_ratio > original_ratio {
        new_height = (height as f32 * scale_factor) as u32;
        new_width = (new_height as f32 * target_ratio) as u32;
    } else {
        new_width = (width as f32 * scale_factor) as u32;
        new_height = (new_width as f32 / target_ratio) as u32;
    }

    let pad_width = new_width.saturating_sub((width as f32 * scale_factor) as u32);
    let pad_height = new_height.saturating_sub((height as f32 * scale_factor) as u32);

    let pad_left = pad_width / 2;
    let pad_right = pad_width - pad_left;
    let pad_top = pad_height / 2;
    let pad_bottom = pad_height - pad_top;

    (
        new_width, new_height, pad_left, pad_right, pad_top, pad_bottom,
    )
}

/// Calculate the greatest common divisor
fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Calculate aspect ratio from width and height
pub fn calculate_aspect_ratio(width: u32, height: u32) -> (u32, u32) {
    let divisor = gcd(width, height);
    (width / divisor, height / divisor)
}
