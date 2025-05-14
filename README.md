# Welcome to Fweh(ame) 🖼️

Fweh is a command-line tool designed for enhancing your images by adding professional-looking fwehs, shadows, rounded corners, and customizable backgrounds. It allows you to easily process images, adjust their aspect ratios, and apply various visual effects to make them stand out.

## Brief Summary

Fweh empowers you to:

*   Add solid color, gradient, or image-based backgrounds.
*   Apply soft drop shadows with configurable offset, color, blur, and opacity.
*   Round the corners of your images.
*   Scale and offset the main image within the new frame.
*   Adjust the final image to a specific aspect ratio.
*   Process images directly from your terminal.

## Get Started

To start using Fweh, you'll first need to [install it](#install). Once installed, you can process images directly from your command line.

## Tutorial

Fweh is controlled via command-line arguments. Here's the basic syntax:

```bash
fweh <input_image_path> [options]
```

### Core Options:

*   `input`: (Required) Path to the input image file.
*   `-o, --output <output_path>`: Specifies the output filename.
    *   Default: `output.png`
    *   Example: `-o ./processed/my_image.png`
*   `-s, --scale <percentage>`: Scales the input image within the fweh. `100.0` means the image touches the edges of its allocated space before padding for aspect ratio.
    *   Default: `110.0` (which means the background will be visible around the image, effectively scaling the image down to fit within `100/110 %` of the space)
    *   Example: `--scale 90.0`
*   `-b, --background <type:value>`: Sets the background.
    *   `colr:<color>`: Solid color. Color can be a name (e.g., `black`, `red`) or hex (e.g., `#FF0000`, `#333`).
        *   Example: `-b colr:lightgray` or `-b colr:#E0E0E0`
    *   `grad:<color1-color2[-...]>`: Linear gradient (top to bottom).
        *   Example: `-b grad:blue-white` or `-b grad:#FF0000-#0000FF`
    *   `imag:/path/to/image.png`: Uses an image as a background. The image will be resized to fill the background dimensions.
        *   Example: `-b imag:/home/user/textures/paper.jpg`
*   `-r, --ratio <W:H>`: Target aspect ratio for the output image.
    *   Example: `-r 16:9` or `-r 1:1`
*   `--roundness <percentage>`: Border radius for the input image, as a percentage of the shortest side of the image (0-100).
    *   Default: `0.0` (no rounding)
    *   Example: `--roundness 10`
*   `--offset <x,y>`: Offsets the input image from the center of the frame in pixels.
    *   Default: `0,0`
    *   Example: `--offset 10,-20` (10px right, 20px up)

### Shadow Options:

To enable a shadow, you must provide at least `--shadow-offset`.

*   `--shadow-offset <x,y>`: Offset of the shadow from the image in pixels.
    *   Example: `--shadow-offset 5,5`
*   `--shadow-color <color>`: Color of the shadow.
    *   Default: `black`
    *   Example: `--shadow-color #808080`
*   `--shadow-radius <radius>`: Blur radius for the shadow.
    *   Default: `25.0`
    *   Example: `--shadow-radius 15.0`
*   `--shadow-opacity <opacity>`: Opacity of the shadow (0.0 to 1.0).
    *   Default: `1.0` (fully opaque)
    *   Example: `--shadow-opacity 0.5`

### Example Usage:

```bash
fweh ./source_images/cat.jpg \
    -o ./output/cat_fwehd.png \
    --scale 95.0 \
    --background colr:#333333 \
    --ratio 16:9 \
    --roundness 8 \
    --shadow-offset 10,10 \
    --shadow-color black \
    --shadow-radius 20 \
    --shadow-opacity 0.6
```

This command will:
1.  Load `cat.jpg`.
2.  Set the background to a dark gray color.
3.  Ensure the final image has a 16:9 aspect ratio.
4.  Round the corners of `cat.jpg` by 8%.
5.  Scale `cat.jpg` to 95% of its available space within the frame.
6.  Add a black shadow, offset by 10px down and 10px right, with a 20px blur and 60% opacity.
7.  Save the result to `./output/cat_framed.png`.

## Design Philosophy

Fweh aims to be a flexible and easy-to-use command-line tool for common image enhancement tasks. It prioritizes:

*   **Simplicity:** Providing clear and understandable command-line options.
*   **Customization:** Offering a good range of parameters to fine-tune the output.
*   **Modularity:** Built with a clean codebase that can be extended.
*   **Performance:** Leveraging efficient Rust libraries for image processing.

## Building and Debugging

You'll need Rust and Cargo installed. The project uses `just` (a command runner) for convenience, but you can use `cargo` commands directly.

1.  **Clone the repository:**
    ```bash
    git clone <repository_url>
    cd fweh
    ```

2.  **Build:**
    *   Using `just` (recommended):
        ```bash
        just build # Debug build
        just build-release # Release build
        ```
    *   Using `cargo`:
        ```bash
        cargo build
        cargo build --release
        ```

3.  **Run/Debug:**
    *   Using `just`:
        ```bash
        just run -- <input_image> [options]
        # For release version:
        just run-release -- <input_image> [options]
        ```
    *   Using `cargo`:
        ```bash
        cargo run -- <input_image> [options]
        # For release version:
        cargo run --release -- <input_image> [options]
        ```
    The compiled binary will be in `target/debug/fweh` or `target/release/fweh`.

## Install

You can install Fweh in a few ways:

1.  **Using `just` (if you have `just` installed and are in the project root):**
    ```bash
    just install
    ```
    This will build the release binary and use `cargo install`.

2.  **Using `cargo install` (from the project root):**
    ```bash
    cargo install --path .
    ```
    This installs the `fweh` binary into your Cargo binary path (usually `~/.cargo/bin/`).

3.  **Pre-compiled Binaries (Recommended for most users):**
    Check the [Releases page](https://github.com/your_username/fweh/releases) <!-- TODO: Update this link --> for pre-compiled binaries for your operating system. Download the archive, extract it, and place the `fweh` executable in a directory included in your system's PATH. The packaged releases also include shell completion scripts.

## Changelog

All notable changes to this project are documented in the [CHANGELOG.md](CHANGELOG.md) file.

## Libraries Used

Fweh is built with Rust and leverages several excellent open-source libraries:

*   [clap](https://crates.io/crates/clap): For command-line argument parsing.
*   [image](https://crates.io/crates/image): For core image loading, manipulation, and saving.
*   [rgb](https://crates.io/crates/rgb): For RGBA color types.
*   [anyhow](https://crates.io/crates/anyhow): For flexible error handling.
*   [thiserror](https://crates.io/crates/thiserror): For creating custom error types.
*   [log](https://crates.io/crates/log): For logging.
*   [env_logger](https://crates.io/crates/env_logger): For configuring logging via environment variables.
*   [tempfile](https://crates.io/crates/tempfile): For creating temporary files if needed.
*   [rayon](https://crates.io/crates/rayon): For data parallelism (used in shadow generation).

## Acknowledgements

*   Thanks to the Rust community and the maintainers of all the libraries used in this project.
*   Inspired by various image processing tools and the need for a simple, scriptable framing tool.

## License

This project is licensed under the [MIT License](LICENSE) 
