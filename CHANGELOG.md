# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] – 2025-05-09

### Added

-   **Core Application Rewrite (New Foundation):**
    -   Established the `frame` application for image processing, including a project description and author information in `Cargo.toml`.
    -   Introduced command-line argument parsing (`args.rs`) using `clap` for input/output paths, scale, background, aspect ratio, roundness, offsets, and shadow options.
    -   Implemented core image processing logic (`image_processing.rs`):
        -   Image loading, scaling, and aspect ratio adjustments.
        -   Corner rounding functionality, correctly handling radius percentage.
        -   Image compositing.
    -   Added background generation module (`background.rs`):
        -   Support for solid color, gradient, and image-file backgrounds.
        -   Color parsing (hex, named colors).
        -   Gradient parsing and interpolation.
    -   Implemented shadow generation module (`shadow.rs`):
        -   Functionality to add drop shadows with configurable offset, color, blur radius, and opacity.
        -   Utilizes the `image` crate's Gaussian blur implementation.
        -   Corrected shadow opacity calculations, removing an erroneous minimum value.
    -   Defined custom error types (`error.rs`) using `thiserror` for robust error handling.
    -   Included utility functions (`utils.rs`) for points, aspect ratio calculation, padding, and color blending.
    -   Fixed an issue where background creation used original image dimensions instead of the new calculated dimensions.
-   Added `Justfile` for comprehensive project task management, including recipes for building, checking, running, testing, formatting, linting, documentation generation, cleaning, and releasing.
-   Enhanced logging throughout the application using `env_logger` for better debugging and traceability.

### Changed

-   Refactored shadow creation:
    -   Utilizes `rayon` for parallel processing of the alpha mask, enhancing performance.
    -   Updated shadow offset logic for more accurate positioning.
-   Adopted the `rgb` crate's `Rgba` struct for color representation internally, replacing raw `u8` arrays for improved type safety and clarity.
-   Simplified background creation logic.
-   Switched from custom image overlay methods to `image::imageops::overlay` for more robust and optimized image compositing in both image processing and shadow application.
-   Updated Rust edition from 2024 to 2021 in `Cargo.toml`.

### Removed

-   Removed previous experimental dependencies and their transitive dependencies that were part of an earlier iteration or temporary experimentation (e.g., `photon-rs`, `wgpu`, `wasm-bindgen`).
-   Removed dependencies briefly introduced for transformation experiments, which are not part of this release (e.g., `ab_glyph`, `approx`, `nalgebra`, `imageproc`, `quad-to-quad-transformer`, `rav1e`, `ravif`, and their related transitive dependencies as reflected in the final `Cargo.lock`).
-   Cleaned up unused imports and functions across several modules (e.g., `From_image_rgba`, internal `blend` functions that were superseded by `imageops::overlay`).

---

[Unreleased]: https://github.com/your-org/your-repo/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/philocalyst/infat/compare/...v0.1.0  

