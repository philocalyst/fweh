use image::ImageBuffer;
use image::ImageReader;
use image::Rgba;

struct CornerRadii(pub u32, pub u32, pub u32, pub u32);

fn main() {
    // testing load/save speed
    let image = ImageReader::open("/Users/philocalyst/Pictures/Screenshot.png")
        .unwrap()
        .decode()
        .unwrap();

    image.save("/Users/philocalyst/bash.png").unwrap();

    use std::path::Path;
    let width = 3000;
    let height = 1500;
    let output_path = Path::new("rounded_image.png");

    // testing corner rounding
    let mut img = ImageBuffer::from_fn(width, height, |_, _| {
        Rgba([0, 0, 255, 255]) // Blue color, fully opaque
    });

    println!("Created a {}x{} blue image.", width, height);

    // Different radii for each corner (if needed)
    let mut radii = CornerRadii(30, 30, 30, 30); // Uniform radius

    round(&mut img, &mut radii);

    match img.save(output_path) {
        Ok(_) => println!("Successfully saved rounded image to {:?}", output_path),
        Err(e) => eprintln!("Error saving image: {}", e),
    }
}

fn round(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, radius: &mut CornerRadii) {
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

fn border_radius(
    img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    r: u32,
    coordinates: impl Fn(u32, u32) -> (u32, u32),
) {
    if r == 0 {
        return;
    }
    let r0 = r;

    // 16x antialiasing: 16x16 grid creates 256 possible shades, great for u8!
    let r = 16 * r;

    let mut x = 0;
    let mut y = r - 1;
    let mut p: i32 = 2 - r as i32;

    // ...

    let mut alpha: u16 = 0;
    let mut skip_draw = true;

    let draw = |img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, alpha, x, y| {
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
