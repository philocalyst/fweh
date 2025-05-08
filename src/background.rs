//! Background generation

use anyhow::{anyhow, Result};
use image::RgbaImage;
use log::debug;
use rgb::{Rgba, RGBA8};

use crate::image_processing::to_image_rgba;

/// Types of backgrounds supported by the image framer
#[derive(Debug, Clone)]
pub enum BackgroundType {
    /// Solid color background (e.g. "black", "#FF0000")
    Color(String),

    /// Gradient background (e.g. "blue-red", "linear:red-green-blue")
    Gradient(String),

    /// Image background (path to an image file)
    Image(String),
}

/// Create a background image with the given parameters
pub fn create_background(
    new_width: u32,
    new_height: u32,
    background: &BackgroundType,
) -> Result<RgbaImage> {
    debug!(
        "Creating background of type {:?} with dimensions {}x{}",
        background, new_width, new_height
    );

    match background {
        BackgroundType::Color(color) => create_color_background(new_width, new_height, color),
        BackgroundType::Gradient(gradient) => {
            create_gradient_background(new_width, new_height, gradient)
        }
        BackgroundType::Image(path) => create_image_background(new_width, new_height, path),
    }
}

/// Create a solid color background
fn create_color_background(width: u32, height: u32, color: &str) -> Result<RgbaImage> {
    debug!("Creating color background: {}", color);

    // Parse the color
    let rgba = parse_color(color)?;

    // Create a new image with the specified color
    let mut img = RgbaImage::from_pixel(width, height, to_image_rgba(rgba));
    for pixel in img.pixels_mut() {
        *pixel = to_image_rgba(rgba);
    }

    Ok(img)
}

/// Create a gradient background
fn create_gradient_background(width: u32, height: u32, gradient: &str) -> Result<RgbaImage> {
    debug!("Creating gradient background: {}", gradient);

    // Parse gradient specification
    let colors = parse_gradient(gradient)?;
    if colors.len() < 2 {
        return Err(anyhow!("Gradient needs at least two colors").into());
    }

    // Create a new image
    let mut img = RgbaImage::new(width, height);

    // Simple linear gradient from top to bottom
    for y in 0..height {
        let progress = y as f32 / height as f32;
        let index = (progress * (colors.len() - 1) as f32) as usize;
        let next_index = (index + 1).min(colors.len() - 1);
        let local_progress = progress * (colors.len() - 1) as f32 - index as f32;

        let color = interpolate_color(colors[index], colors[next_index], local_progress);

        for x in 0..width {
            img.put_pixel(x, y, to_image_rgba(color));
        }
    }

    Ok(img)
}

/// Create an image background from an existing image file
fn create_image_background(width: u32, height: u32, path: &str) -> Result<RgbaImage> {
    debug!("Creating image background from: {}", path);

    // Load the background image
    let bg_image = image::open(path)?;

    // Resize the image to fit the new dimensions
    let resized = bg_image.resize_to_fill(width, height, image::imageops::FilterType::Lanczos3);

    // Convert to RGBA
    let rgba = resized.to_rgba8();

    Ok(rgba)
}

/// Parse a color string to an RGBA value
pub fn parse_color(color: &str) -> Result<RGBA8> {
    // Handle hex colors
    if color.starts_with('#') {
        let hex = color.trim_start_matches('#');

        match hex.len() {
            6 => {
                // RGB format
                let r = u8::from_str_radix(&hex[0..2], 16)?;
                let g = u8::from_str_radix(&hex[2..4], 16)?;
                let b = u8::from_str_radix(&hex[4..6], 16)?;
                Ok(rgb::Rgba {
                    r: (r),
                    g: (g),
                    b: (b),
                    a: 255,
                })
            }
            8 => {
                // RGBA format
                let r = u8::from_str_radix(&hex[0..2], 16)?;
                let g = u8::from_str_radix(&hex[2..4], 16)?;
                let b = u8::from_str_radix(&hex[4..6], 16)?;
                let a = u8::from_str_radix(&hex[6..8], 16)?;
                Ok(rgb::Rgba {
                    r: (r),
                    g: (g),
                    b: (b),
                    a: (a),
                })
            }
            3 => {
                // Short RGB format
                let r = u8::from_str_radix(&hex[0..1], 16)? * 17;
                let g = u8::from_str_radix(&hex[1..2], 16)? * 17;
                let b = u8::from_str_radix(&hex[2..3], 16)? * 17;
                Ok(rgb::Rgba {
                    r: (r),
                    g: (g),
                    b: (b),
                    a: 255,
                })
            }
            _ => Err(anyhow!("Invalid hex color format: {}", color)),
        }
    } else {
        // Handle named colors
        match color.to_lowercase().as_str() {
            "black" => Ok(Rgba {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            }),
            "white" => Ok(Rgba {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            }),
            "red" => Ok(Rgba {
                r: 255,
                g: 0,
                b: 0,
                a: 255,
            }),
            "green" => Ok(Rgba {
                r: 0,
                g: 255,
                b: 0,
                a: 255,
            }),
            "blue" => Ok(Rgba {
                r: 0,
                g: 0,
                b: 255,
                a: 255,
            }),
            "yellow" => Ok(Rgba {
                r: 255,
                g: 255,
                b: 0,
                a: 255,
            }),
            "cyan" => Ok(Rgba {
                r: 0,
                g: 255,
                b: 255,
                a: 255,
            }),
            "magenta" => Ok(Rgba {
                r: 255,
                g: 0,
                b: 255,
                a: 255,
            }),
            "transparent" => Ok(Rgba {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            }),
            other => Err(anyhow!("Unknown color name: {}", other)),
        }
    }
}

/// Parse a gradient specification into a list of colors
fn parse_gradient(gradient: &str) -> Result<Vec<RGBA8>> {
    let parts = gradient.split('-').collect::<Vec<_>>();
    let mut colors = Vec::with_capacity(parts.len());
    for part in parts {
        colors.push(parse_color(part)?);
    }
    Ok(colors)
}

/// Interpolate between two colors
fn interpolate_color(color1: RGBA8, color2: RGBA8, t: f32) -> RGBA8 {
    let lerp = |a: u8, b: u8, t: f32| -> u8 { (a as f32 * (1.0 - t) + b as f32 * t).round() as u8 };
    Rgba {
        r: lerp(color1.r, color2.r, t),
        g: lerp(color1.g, color2.g, t),
        b: lerp(color1.b, color2.b, t),
        a: lerp(color1.a, color2.a, t),
    }
}
