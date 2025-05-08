//! Command line argument parsing

use anyhow::{anyhow, Result};
use clap::{ArgAction, Parser};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::background::BackgroundType;
use crate::error::FramerError;
use crate::image_processing::{AspectRatio, ProcessingOptions};
use crate::shadow::ShadowOptions;
use crate::utils::Point;

/// Command line arguments for the image framer tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Input image file
    pub input: PathBuf,

    /// Output filename
    #[arg(short, long, default_value = "output.png")]
    pub output: PathBuf,

    /// Scale percentage
    #[arg(short, long, default_value_t = 110.0)]
    pub scale: f32,

    /// Background type and value (e.g. colr:black, grad:blue-red, imag:/path/to/image.png)
    #[arg(short, long)]
    pub background: Option<String>,

    /// Target aspect ratio (e.g. 16:9)
    #[arg(short, long)]
    pub ratio: Option<String>,

    /// Border radius percentage (0-100)
    #[arg(long, default_value_t = 0.0)]
    pub roundness: f32,

    /// Image offset in pixels (e.g. 0,0)
    #[arg(long, default_value = "0,0")]
    pub offset: String,

    /// Shadow offset in pixels (e.g. 25,25)
    #[arg(long)]
    pub shadow_offset: Option<String>,

    /// Shadow color (e.g. black, #000000)
    #[arg(long, default_value = "black")]
    pub shadow_color: String,

    /// Shadow blur radius
    #[arg(long, default_value_t = 25.0)]
    pub shadow_radius: f32,

    /// Shadow opacity (0.0-1.0)
    #[arg(long, default_value_t = 1.0)]
    pub shadow_opacity: f32,
}

impl From<Args> for ProcessingOptions {
    fn from(args: Args) -> Self {
        let offset = parse_point(&args.offset).unwrap_or(Point::new(0.0, 0.0));

        let shadow = if args.shadow_offset.is_some() {
            let shadow_offset = parse_point(args.shadow_offset.as_ref().unwrap())
                .unwrap_or(Point::new(25.0, -25.0));

            Some(ShadowOptions {
                offset: shadow_offset,
                color: args.shadow_color,
                radius: args.shadow_radius,
                opacity: args.shadow_opacity,
            })
        } else {
            None
        };

        let ratio = args.ratio.as_ref().and_then(|r| {
            let parts: Vec<&str> = r.split(':').collect();
            if parts.len() == 2 {
                let width = parts[0].parse::<u32>().ok()?;
                let height = parts[1].parse::<u32>().ok()?;
                Some(AspectRatio { width, height })
            } else {
                None
            }
        });

        let background = args
            .background
            .as_ref()
            .and_then(|bg| {
                let parts: Vec<&str> = bg.split(':').collect();
                if parts.len() == 2 {
                    match parts[0] {
                        "colr" => Some(BackgroundType::Color(parts[1].to_string())),
                        "grad" => Some(BackgroundType::Gradient(parts[1].to_string())),
                        "imag" => Some(BackgroundType::Image(parts[1].to_string())),
                        _ => None,
                    }
                } else {
                    None
                }
            })
            .unwrap_or(BackgroundType::Color("black".to_string()));

        ProcessingOptions {
            scale: args.scale,
            roundness: args.roundness,
            offset,
            shadow,
            background,
            ratio,
        }
    }
}

/// Parse command line arguments
pub fn parse_args() -> Result<Args> {
    Ok(Args::parse())
}

/// Parse a point from a string (e.g. "10,20")
fn parse_point(input: &str) -> Result<Point> {
    let parts: Vec<&str> = input.split(',').collect();

    if parts.len() != 2 {
        return Err(anyhow!("Invalid point format: {}", input));
    }

    let x = parts[0].trim().parse::<f32>()?;
    let y = parts[1].trim().parse::<f32>()?;

    Ok(Point::new(x, y))
}
