// Copyright © 2019 Liam Rotchford, Simon Barton
//Generate 3 a randomly chosen julia set fractal as a .png image
//base code credited to: https://crates.io/crates/image
//resource on julia_set fractals: https://en.wikipedia.org/wiki/Julia_set#Pseudocode_for_normal_Julia_sets

use crate::util::Color::*;
use crate::util::*;
use image::Rgb;
use rand::Rng;

///Julia Set Fractal - Each pixel in the user specified dimensions runs through
/// the loop that calculates the Julia set formula of (f(z) = z^2 + c), and will continue to
/// do so until the value is outside the appropriate range where it can still generate
/// correctly. The int value that is broken out of the function is returned
/// and used for the color shade of the currently specfied pixel.
pub fn pixel_setter((complex_x, complex_y): (f32, f32), mut iteration: u64, randjulia: u64) -> u64 {
    //determine which julia_set fractal will be generated
    let complex_num = match randjulia {
        1 => num::Complex::new(-0.4, 0.6),
        2 => num::Complex::new(0.285, 0.01),
        3 => num::Complex::new(-0.7269, 0.1889),
        _ => num::Complex::new(-0.4, 0.6),
    };

    let mut value = num::Complex::new(complex_x, complex_y);

    while iteration < 255 && value.norm() <= 2.0 {
        //the julia fractal
        value = value * value + complex_num;

        iteration += 1;
    }

    iteration
}

pub fn julia_fractal(imgy: u32, imgx: u32, filename: &str, scheme: Scheme) {
    // https://crates.io/crates/image

    let scaleset = ((3.0 / imgx as f32), (3.0 / imgy as f32));
    let color: [u8; 3] = color_to_rgb(&scheme.color);

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    apply_background(&mut imgbuf, &scheme);

    let mut rng = rand::thread_rng();
    let randjulia = rng.gen_range(1, 4);

    //move through the dimensions of the image size, call the fractal generator to calculate the pixel shade, set pixel
    for x in 0..imgx {
        for y in 0..imgy {
            let complex_pos = ((y as f32 * scaleset.0 - 1.5), (x as f32 * scaleset.1 - 1.5)); //determines position in frame

            let result = pixel_setter(complex_pos, 0, randjulia);

            let pixel = imgbuf.get_pixel_mut(x, y);

            let Rgb(data) = *pixel;

            match scheme.color {
                Red => *pixel = Rgb([result as u8, data[1], data[2]]),
                Green => *pixel = Rgb([data[0], result as u8, data[2]]),
                Blue => *pixel = Rgb([data[0], data[1], result as u8]),
                White => *pixel = Rgb([result as u8, result as u8, result as u8]),
                _ => panic!("Unsupported color"),
            }
        }
    }

    // Save the image
    imgbuf.save(filename).unwrap();
}
