//! Shadow effects

use anyhow::{anyhow, Result};
use image::{imageops::blur, DynamicImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};
use log::debug;

use crate::background::parse_color;
use crate::error::FramerError;
use crate::utils::Point;

/// Shadow options for the image framer
#[derive(Debug, Clone)]
pub struct ShadowOptions {
    /// Offset of the shadow from the image
    pub offset: Point,

    /// Color of the shadow
    pub color: String,

    /// Blur radius of the shadow
    pub radius: f32,

    /// Opacity of the shadow (0.0-1.0)
    pub opacity: f32,
}

/// Add a drop shadow to an image
pub fn add_drop_shadow(image: &RgbaImage, options: &ShadowOptions) -> Result<RgbaImage> {
    debug!(
        "Adding drop shadow with radius {} and offset ({}, {})",
        options.radius, options.offset.x, options.offset.y
    );

    // Parse shadow color
    let shadow_color = parse_color(&options.color)
        .map_err(|e| FramerError::ShadowError(format!("Invalid shadow color: {}", e)))?;

    // Calculate dimensions for the shadow image
    let shadow_width = image.width() + 2 * options.radius as u32;
    let shadow_height = image.height() + 2 * options.radius as u32;

    // Create alpha mask from original image
    let mut alpha_mask = ImageBuffer::new(shadow_width, shadow_height);

    // Position of the original image in the larger shadow canvas
    let offset_x = options.radius as u32;
    let offset_y = options.radius as u32;

    // Copy alpha channel to create the shadow mask
    for (x, y, pixel) in image.enumerate_pixels() {
        let alpha = pixel[3] as f32 / 255.0;
        let shadow_x = x + offset_x;
        let shadow_y = y + offset_y;

        if shadow_x < shadow_width && shadow_y < shadow_height {
            alpha_mask.put_pixel(
                shadow_x,
                shadow_y,
                Rgba([255, 255, 255, (alpha * 255.0) as u8]),
            );
        }
    }

    // Apply Gaussian blur to create the shadow effect
    image::imageops::blur(&alpha_mask, options.radius);

    // Apply opacity to the blurred mask
    let mut shadow_image = RgbaImage::new(shadow_width, shadow_height);
    for (x, y, pixel) in alpha_mask.enumerate_pixels() {
        let alpha = (pixel[3] as f32 * options.opacity).min(255.0) as u8;
        shadow_image.put_pixel(
            x,
            y,
            Rgba([shadow_color[0], shadow_color[1], shadow_color[2], alpha]),
        );
    }

    // Calculate dimensions for the final image (original + shadow with offset)
    let final_width = shadow_width + options.offset.x.abs() as u32;
    let final_height = shadow_height + options.offset.y.abs() as u32;

    // Create the final image
    let mut final_image = RgbaImage::new(final_width, final_height);

    // Calculate the position of the shadow in the final image
    let shadow_pos_x = if options.offset.x < 0.0 {
        (-options.offset.x) as u32
    } else {
        0
    };
    let shadow_pos_y = if options.offset.y < 0.0 {
        (-options.offset.y) as u32
    } else {
        0
    };

    // Draw the shadow
    for (x, y, pixel) in shadow_image.enumerate_pixels() {
        let final_x = shadow_pos_x + x + options.offset.x.max(0.0) as u32;
        let final_y = shadow_pos_y + y + options.offset.y.max(0.0) as u32;

        if final_x < final_width && final_y < final_height {
            final_image.put_pixel(final_x, final_y, *pixel);
        }
    }

    // Calculate the position of the original image in the final image
    let image_pos_x = shadow_pos_x + offset_x;
    let image_pos_y = shadow_pos_y + offset_y;

    // Draw the original image on top of the shadow
    for (x, y, pixel) in image.enumerate_pixels() {
        let final_x = image_pos_x + x;
        let final_y = image_pos_y + y;

        if final_x < final_width && final_y < final_height && pixel[3] > 0 {
            // Composite the original pixel over the shadow
            let existing = final_image.get_pixel(final_x, final_y);
            let alpha = pixel[3] as f32 / 255.0;
            let result = Rgba([
                blend(existing[0], pixel[0], alpha),
                blend(existing[1], pixel[1], alpha),
                blend(existing[2], pixel[2], alpha),
                blend(existing[3], pixel[3], alpha),
            ]);

            final_image.put_pixel(final_x, final_y, result);
        }
    }

    Ok(final_image)
}

/// Blend two color values based on alpha
fn blend(bg: u8, fg: u8, alpha: f32) -> u8 {
    (bg as f32 * (1.0 - alpha) + fg as f32 * alpha) as u8
}
