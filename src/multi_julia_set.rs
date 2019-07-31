// Copyright © 2019 Liam Rotchford, Simon Barton
// Generate the Multilevel Julia set fractal
// base code credited to: https://rosettacode.org/wiki/Julia_set#Rust
// knowledge source and pseudo code: https://en.wikipedia.org/wiki/Julia_set#Pseudocode_for_normal_Julia_sets

use ::image::Rgb;

/// Multi-Julia Set Fractal - Each pixel in the user specified dimensions runs through
/// the loop that calculates the Julia set formula of (f(z) = z^n + c), and will continue to
/// do so until the value is outside the appropriate range where it can still generate
/// correctly. The int value that is broken out of the function is returned
/// and used for the color shade of the currently specfied pixel.

pub fn pixel_set_multi((imgx, imgy): (f32, f32), (loop_x, loop_y): (f32, f32), mut i: u64) -> u64 {
    let mut val_x = 3.0 * (loop_x - 0.5 * imgx) / (imgx);
    let mut val_y = 2.0 * (loop_y - 0.5 * imgy) / (imgy);
    let complex_x = -0.9;
    let complex_y = 0.27015;

    //multilevel julia set formula calculation
    while (val_x * val_x + val_y * val_y) < 4.0 && i > 1 {
        let holder = (val_x * val_x) - (val_y * val_y) + complex_x;
        val_y = 2.0 * (val_x * val_y) + complex_y;
        val_x = holder;
        i -= 1;
    }

    i
}

pub fn multi_julia(imgy: u32, imgx: u32, filename: &str, scheme: &str) {
    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        //R                     //G         //B
        *pixel = Rgb([((0.3 * x as f32) as u8), 0, ((0.3 * y as f32) as u8)]);
    }

    let img = (imgx as f32, imgy as f32);

    //move through each pixel, run formala on it, determine pixel shade
    for x in 0..imgx {
        for y in 0..imgy {
            let loop_val = (x as f32, y as f32);

            let result = pixel_set_multi(img, loop_val, 110);

            let pixel = imgbuf.get_pixel_mut(x, y);

            let Rgb(data) = *pixel;

            if scheme == "color" {
                *pixel = Rgb([data[0], result as u8, data[2]]);
            } else {
                *pixel = Rgb([result as u8, result as u8, result as u8]);
            }
        }
    }

    //save image
    imgbuf.save(filename).unwrap();
}
