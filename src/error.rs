//! Error types

use std::io;
use std::path::PathBuf;
use thiserror::Error;

/// Error types for the image framer
#[derive(Error, Debug)]
pub enum FramerError {
    #[error("Failed to load image: {0}")]
    ImageLoadError(#[from] image::ImageError),

    #[error("Failed to save image: {0}")]
    ImageSaveError(String),

    #[error("Failed to resize image: {0}")]
    ResizeError(String),

    #[error("Failed to create background: {0}")]
    BackgroundError(String),

    #[error("Failed to add shadow: {0}")]
    ShadowError(String),

    #[error("Failed to round corners: {0}")]
    RoundingError(String),

    #[error("Input file not found: {0}")]
    InputFileNotFound(PathBuf),

    #[error("Output directory not found: {0}")]
    OutputDirectoryNotFound(PathBuf),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    #[error("Other error: {0}")]
    Other(String),
}
