# Welcome

A command-line utility for adding frames, shadows, and backgrounds to images.

## Installation

For this tool, you'll need:
    - Luajit
    - Luarocks
    - Imagemagick
    - The libvips shared library

If you're on mac, you can install all in one step:

```bash
brew install vips luajit luarocks magick
```

## Usage

```bash
frame [OPTIONS] <input_image>
```

## Options

- `-h, --help`: Show help message
- `-o, --output FILE`: Output filename (default: output.png)
- `-s, --scale PERCENT`: Scale percentage (default: 110)
- `-b, --background TYPE:VALUE`: Background type and value
- `-r, --ratio W:H`: Target aspect ratio (e.g. 16:9)
- `--roundness RADIUS`: Border radius in pixels (default: 0)
- `--offset X,Y`: Image offset in pixels (default: 0,0)

### Shadow Options

(Only applied when at least one shadow option is specified)

- `--shadow-offset X,Y`: Shadow offset in pixels (default: 25,25)
- `--shadow-color COLOR`: Shadow color (default: black)
- `--shadow-radius RADIUS`: Shadow blur radius (default: 25)
- `--shadow-opacity VALUE`: Shadow opacity (0-1, default: 1)

## Background Types

The background can be specified using the `-b` or `--background` option with the following formats:

1. **Solid Color** (`colr`):

    ```bash
    frame input.jpg -b colr:blue
    frame input.jpg -b colr:#FF0000
    ```

2. **Gradient** (`grad`):

    ```bash
    frame input.jpg -b grad:white-blue
    frame input.jpg -b grad:"#FF0000-#0000FF"
    ```

3. **Preset** (`pre`):
   Not supported.
4. **Image** (`imag`):
    ```bash
    frame input.jpg -b imag:background.jpg
    ```

## Examples

1. Basic usage with default settings:

    ```bash
    frame input.jpg
    ```

2. Custom output file with scaling:

    ```bash
    frame input.jpg -o framed.png -s 120
    ```

3. Add a blue background with rounded corners:

    ```bash
    frame input.jpg -b colr:blue --roundness 20
    ```

4. Create a 16:9 frame with gradient background and shadow:

    ```bash
    frame input.jpg -r 16:9 -b grad:white-black --shadow-offset 30,30 --shadow-radius 15
    ```

5. Custom positioning with offset:

    ```bash
    frame input.jpg --offset 50,30 -s 90
    ```

6. Complex example with multiple options:
    ```bash
    frame input.jpg \
      -o custom_frame.png \
      -s 115 \
      -b grad:"#FF0000-#0000FF" \
      -r 16:9 \
      --roundness 15 \
      --shadow-offset 20,20 \
      --shadow-color "#333333" \
      --shadow-radius 30 \
      --shadow-opacity 0.7
    ```

## Notes

- The scale percentage determines how much of the frame the image occupies
- Shadow options are only applied when at least one shadow parameter is specified
- The gradiant option supports all inputs supported by imagemagick
- Background gradients can use color names or hex values
- Aspect ratio is specified as width:height (e.g., 16:9, 4:3, 1:1)
- Offset values can be specified as a single number to apply to both X and Y, or as X,Y coordinates

## Error Handling

This tool is on your side! Expect rich errors to help me solve your problems :)
