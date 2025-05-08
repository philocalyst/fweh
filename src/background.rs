//! Background generation

use anyhow::{anyhow, Result};
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};
use log::debug;
use std::path::Path;

use crate::error::FramerError;
use crate::utils::{create_temp_file, Point};

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
    width: u32,
    height: u32,
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
    let mut img = RgbaImage::new(width, height);
    for pixel in img.pixels_mut() {
        *pixel = rgba;
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
            img.put_pixel(x, y, color);
        }
    }

    Ok(img)
}

/// Create an image background from an existing image file
fn create_image_background(width: u32, height: u32, path: &str) -> Result<RgbaImage> {
    debug!("Creating image background from: {}", path);

    // Load the background image
    let bg_image = image::open(path).map_err(|e| {
        FramerError::BackgroundError(format!("Failed to load background image: {}", e))
    })?;

    // Resize the image to fit the new dimensions
    let resized = bg_image.resize_to_fill(width, height, image::imageops::FilterType::Lanczos3);

    // Convert to RGBA
    let rgba = resized.to_rgba8();

    Ok(rgba)
}

/// Parse a color string to an RGBA value
pub fn parse_color(color: &str) -> Result<Rgba<u8>> {
    // Handle hex colors
    if color.starts_with('#') {
        let hex = color.trim_start_matches('#');

        match hex.len() {
            6 => {
                // RGB format
                let r = u8::from_str_radix(&hex[0..2], 16)?;
                let g = u8::from_str_radix(&hex[2..4], 16)?;
                let b = u8::from_str_radix(&hex[4..6], 16)?;
                Ok(Rgba([r, g, b, 255]))
            }
            8 => {
                // RGBA format
                let r = u8::from_str_radix(&hex[0..2], 16)?;
                let g = u8::from_str_radix(&hex[2..4], 16)?;
                let b = u8::from_str_radix(&hex[4..6], 16)?;
                let a = u8::from_str_radix(&hex[6..8], 16)?;
                Ok(Rgba([r, g, b, a]))
            }
            3 => {
                // Short RGB format
                let r = u8::from_str_radix(&hex[0..1], 16)? * 17;
                let g = u8::from_str_radix(&hex[1..2], 16)? * 17;
                let b = u8::from_str_radix(&hex[2..3], 16)? * 17;
                Ok(Rgba([r, g, b, 255]))
            }
            _ => Err(anyhow!("Invalid hex color format: {}", color)),
        }
    } else {
        // Handle named colors
        match color.to_lowercase().as_str() {
            "black" => Ok(Rgba([0, 0, 0, 255])),
            "white" => Ok(Rgba([255, 255, 255, 255])),
            "red" => Ok(Rgba([255, 0, 0, 255])),
            "green" => Ok(Rgba([0, 255, 0, 255])),
            "blue" => Ok(Rgba([0, 0, 255, 255])),
            "yellow" => Ok(Rgba([255, 255, 0, 255])),
            "cyan" => Ok(Rgba([0, 255, 255, 255])),
            "magenta" => Ok(Rgba([255, 0, 255, 255])),
            "transparent" => Ok(Rgba([0, 0, 0, 0])),
            _ => Err(anyhow!("Unknown color name: {}", color)),
        }
    }
}

/// Parse a gradient specification into a list of colors
fn parse_gradient(gradient: &str) -> Result<Vec<Rgba<u8>>> {
    let parts: Vec<&str> = gradient.split('-').collect();
    let mut colors = Vec::with_capacity(parts.len());

    for part in parts {
        colors.push(parse_color(part)?);
    }

    Ok(colors)
}

/// Interpolate between two colors
fn interpolate_color(color1: Rgba<u8>, color2: Rgba<u8>, t: f32) -> Rgba<u8> {
    let lerp = |a: u8, b: u8, t: f32| -> u8 { (a as f32 * (1.0 - t) + b as f32 * t).round() as u8 };

    Rgba([
        lerp(color1[0], color2[0], t),
        lerp(color1[1], color2[1], t),
        lerp(color1[2], color2[2], t),
        lerp(color1[3], color2[3], t),
    ])
}
