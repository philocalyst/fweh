//! Fweh - A tool for framing images with shadows, rounded corners, and backgrounds
//!
//! This application allows you to process images by adding backgrounds, shadows, and
//! rounded corners, as well as resizing to specific aspect ratios.

mod args;
mod background;
mod error;
mod image_processing;
mod shadow;
mod utils;

use anyhow::Result;
use args::parse_args;
use image_processing::process_image;
use log::{error, info};

fn main() -> Result<()> {
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::default().filter_or("RUST_LOG", "info"));

    // Parse command line arguments
    let args = match parse_args() {
        Ok(args) => args,
        Err(e) => {
            error!("Failed to parse arguments: {}", e);
            return Err(e.into());
        }
    };

    info!("Processing image: {}", args.input.display());

    // Process the image
    match process_image(&args.input.clone(), &args.output.clone(), args.into()) {
        Ok(output_path) => {
            info!("Successfully processed image: {}", output_path.display());
            println!("{}", output_path.display());
            Ok(())
        }
        Err(e) => {
            error!("Failed to process image: {}", e);
            Err(e)
        }
    }
}
