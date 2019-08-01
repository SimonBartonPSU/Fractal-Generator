// Copyright © 2019 Liam Rotchford, Simon Barton

use image::Rgb;
use crate::util::*;
use crate::util::Color::*;

//use rand::Rng;

pub fn pixel_setter((complex_x, complex_y): (f32, f32), mut iteration: u64) -> u64 {
    let complex_num = num::Complex::new(-0.4, 0.6);
    let mut value = num::Complex::new(complex_x, complex_y);

    while iteration < 255 && value.norm() <= 2.0 {
        //the julia fractal
        value = value * value + complex_num;

        //different styles of julia sets
        //z = z * z + 0.279;
        //z = z * z * z + 0.400;

        //let quad = z * z * z * z;
        //  z = quad + 0.494;

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

    //let mut rng = rand::thread_rng();
    //let randR: f32 = rng.gen_range(0, 255) as f32;
    //let randB: f32 = rng.gen_range(0, 255) as f32;

    apply_background(&mut imgbuf, &scheme);

    // Iterate over the coordinates and pixels of the image
    /*
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        //R                     //G         //B
        *pixel = Rgb([((0.3 * x as f32) as u8), 0, ((0.3 * y as f32) as u8)]);
    }
    */
    for x in 0..imgx {
        for y in 0..imgy {
            let complex_pos = ((y as f32 * scaleset.0 - 1.5), (x as f32 * scaleset.1 - 1.5)); //determines position in frame

            let result = pixel_setter(complex_pos, 0);

            let pixel = imgbuf.get_pixel_mut(x, y);

            let Rgb(data) = *pixel;

            match scheme.color {
                Red   => *pixel = Rgb([result as u8, data[1], data[2]]),
                Green => *pixel = Rgb([data[0], result as u8, data[2]]),
                Blue  => *pixel = Rgb([data[0], data[1], result as u8]),
                White => *pixel = Rgb([result as u8, result as u8, result as u8]),
                _ => panic!("Unsupported color"),
            }
        }
    }

    // Save the image
    imgbuf.save(filename).unwrap();
}
