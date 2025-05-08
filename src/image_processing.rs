//! Core image processing functions

use anyhow::Result;
use image::{imageops, RgbaImage};
use log::debug;
use rgb;
use std::path::{Path, PathBuf};

use crate::background::{create_background, BackgroundType};
use crate::error::FramerError;
use crate::shadow::{add_drop_shadow, ShadowOptions};
use crate::utils::{calculate_aspect_ratio, calculate_padding, CornerRadii, Point};

/// Options for aspect ratio
#[derive(Debug, Clone, Copy)]
pub struct AspectRatio {
    pub width: u32,
    pub height: u32,
}

impl AspectRatio {
    /// Calculate the ratio as a float
    pub fn as_f32(&self) -> f32 {
        self.width as f32 / self.height as f32
    }
}

/// Options for image processing
#[derive(Debug, Clone)]
pub struct ProcessingOptions {
    /// Scale percentage (default: 110.0)
    pub scale: f32,

    /// Corner roundness percentage (0-100)
    pub roundness: f32,

    /// Offset of the image from center
    pub offset: Point,

    /// Shadow options (None for no shadow)
    pub shadow: Option<ShadowOptions>,

    /// Background type
    pub background: BackgroundType,

    /// Target aspect ratio (None to maintain original)
    pub ratio: Option<AspectRatio>,
}

/// Process an image with the given options
pub fn process_image(
    input_path: &Path,
    output_path: &Path,
    options: ProcessingOptions,
) -> Result<PathBuf> {
    // Load the input image
    let input_image = image::open(input_path).map_err(|e| FramerError::ImageLoadError(e))?;

    let input_rgba = input_image.to_rgba8();
    let (width, height) = input_rgba.dimensions();

    debug!("Loaded input image: {}x{}", width, height);

    // Calculate target aspect ratio
    let target_ratio = options.ratio.map(|r| r.as_f32()).unwrap_or_else(|| {
        let (w, h) = calculate_aspect_ratio(width, height);
        w as f32 / h as f32
    });

    debug!("Target aspect ratio: {}", target_ratio);

    // Apply corner rounding if needed
    let mut processed = input_rgba;

    if options.roundness > 0.0 {
        debug!("Rounding corners with radius {}%", options.roundness);
        processed = round_corners(&processed, options.roundness)?;
    }

    // Apply drop shadow if needed
    let mut with_shadow = processed.clone();
    if let Some(shadow_options) = &options.shadow {
        debug!("Adding drop shadow");
        with_shadow = add_drop_shadow(&processed, shadow_options)?;
    }

    // Calculate dimensions for the background
    let (new_width, new_height, _, _, _, _) =
        calculate_padding(width, height, target_ratio, options.scale);

    debug!("Creating background of size {}x{}", new_width, new_height);

    // Create background
    let mut background = create_background(width, height, &options.background)?;

    // Calculate position to place the image on the background
    let x = (new_width as f32 - width as f32) / 2.0 + options.offset.x;
    let y = (new_height as f32 - height as f32) / 2.0 + options.offset.y;

    debug!("Placing image at position ({}, {})", x, y);

    // Composite the processed image onto the background
    if options.shadow.is_some() {
        imageops::overlay(&mut background, &with_shadow, x as i64, y as i64)
    } else {
        imageops::overlay(&mut background, &processed, x as i64, y as i64);
    };

    // Save the final image
    background
        .save(output_path)
        .map_err(|e| FramerError::ImageSaveError(e.to_string()))?;

    Ok(output_path.to_path_buf())
}

/// Round the corners of an image
fn round_corners(image: &RgbaImage, radius_percentage: f32) -> Result<RgbaImage> {
    let (width, height) = image.dimensions();

    // Calculate corner radius in pixels
    let radius = ((width.min(height) as f32 * radius_percentage) / 100.0) as u32;
    debug!("Rounding corners with pixel radius: {}", radius);

    if radius == 0 {
        return Ok(image.clone());
    }

    // Create a copy of the input image
    let mut result = image.clone();

    // Create corner radii (all corners with the same radius)
    let mut radii = (radius, radius, radius, radius);

    // Apply corner rounding
    round(&mut result, &mut radii);

    Ok(result)
}

/// Round the corners of an image buffer (implementation from the provided code)
fn round(img: &mut RgbaImage, radius: &mut CornerRadii) {
    let (width, height) = img.dimensions();
    assert!(radius.0 + radius.1 <= width);
    assert!(radius.3 + radius.2 <= width);
    assert!(radius.0 + radius.3 <= height);
    assert!(radius.1 + radius.2 <= height);

    // top left
    border_radius(img, radius.0, |x, y| (x - 1, y - 1));
    // top right
    border_radius(img, radius.1, |x, y| (width - x, y - 1));
    // bottom right
    border_radius(img, radius.2, |x, y| (width - x, height - y));
    // bottom left
    border_radius(img, radius.3, |x, y| (x - 1, height - y));
}

/// Apply border radius to a specific corner
fn border_radius(img: &mut RgbaImage, r: u32, coordinates: impl Fn(u32, u32) -> (u32, u32)) {
    if r == 0 {
        return;
    }
    let r0 = r;

    // 16x antialiasing: 16x16 grid creates 256 possible shades, great for u8!
    let r = 16 * r;

    let mut x = 0;
    let mut y = r - 1;
    let mut p: i32 = 2 - r as i32;

    let mut alpha: u16 = 0;
    let mut skip_draw = true;

    let draw = |img: &mut RgbaImage, alpha, x, y| {
        debug_assert!((1..=256).contains(&alpha));
        let pixel_alpha = &mut img[coordinates(r0 - x, r0 - y)].0[3];
        *pixel_alpha = ((alpha * *pixel_alpha as u16 + 128) / 256) as u8;
    };

    'l: loop {
        // (comments for bottom_right case:)
        // remove contents below current position
        {
            let i = x / 16;
            for j in y / 16 + 1..r0 {
                img[coordinates(r0 - i, r0 - j)].0[3] = 0;
            }
        }
        // remove contents right of current position mirrored
        {
            let j = x / 16;
            for i in y / 16 + 1..r0 {
                img[coordinates(r0 - i, r0 - j)].0[3] = 0;
            }
        }

        // draw when moving to next pixel in x-direction
        if !skip_draw {
            draw(img, alpha, x / 16 - 1, y / 16);
            draw(img, alpha, y / 16, x / 16 - 1);
            alpha = 0;
        }

        for _ in 0..16 {
            skip_draw = false;

            if x >= y {
                break 'l;
            }

            alpha += y as u16 % 16 + 1;
            if p < 0 {
                x += 1;
                p += (2 * x + 2) as i32;
            } else {
                // draw when moving to next pixel in y-direction
                if y % 16 == 0 {
                    draw(img, alpha, x / 16, y / 16);
                    draw(img, alpha, y / 16, x / 16);
                    skip_draw = true;
                    alpha = (x + 1) as u16 % 16 * 16;
                }

                x += 1;
                p -= (2 * (y - x) + 2) as i32;
                y -= 1;
            }
        }
    }

    // one corner pixel left
    if x / 16 == y / 16 {
        // column under current position possibly not yet accounted
        if x == y {
            alpha += y as u16 % 16 + 1;
        }
        let s = y as u16 % 16 + 1;
        let alpha = 2 * alpha - s * s;
        draw(img, alpha, x / 16, y / 16);
    }

    // remove remaining square of content in the corner
    let range = y / 16 + 1..r0;
    for i in range.clone() {
        for j in range.clone() {
            img[coordinates(r0 - i, r0 - j)].0[3] = 0;
        }
    }
}

use image::Rgba as ImageRgba;
use rgb::Rgba as RgbRgba;

/// Turn an rgb::Rgba<u8> into an image::Rgba<u8>
pub fn to_image_rgba(px: RgbRgba<u8>) -> ImageRgba<u8> {
    // ImageRgba is a tuple‐struct around [u8;4]
    ImageRgba([px.r, px.g, px.b, px.a])
}

/// Turn an image::Rgba<u8> back into rgb::Rgba<u8>
pub fn from_image_rgba(px: ImageRgba<u8>) -> RgbRgba<u8> {
    // you can either destructure or call .0
    let [r, g, b, a] = px.0;
    RgbRgba { r, g, b, a }
}
