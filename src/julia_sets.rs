// Copyright © 2019 Liam Rotchford, Simon Barton
//Generate 3 a randomly chosen julia set fractal as a .png image
//base code credited to: https://crates.io/crates/image
//resource on julia_set fractals: https://en.wikipedia.org/wiki/Julia_set#Pseudocode_for_normal_Julia_sets

use crate::util::Color::*;
use crate::util::*;
use image::Rgb;
use rand::Rng;

/// Julia Set Fractal - "the Julia set consists of values such that an arbitrarily
/// small perturbation can cause drastic changes in the sequence of iterated function values.
/// Thus the behavior of the Julia set is "chaotic"."

/// Each pixel in the user specified dimensions runs through
/// the loop that calculates the Julia set formula of (f(z) = z^2 + c), and will continue to
/// do so until the value is outside the appropriate range where it can still generate
/// correctly. The int value that is broken out of the function is returned
/// and used for the color shade of the currently specfied pixel.

pub fn pixel_setter((complex_x, complex_y): (f32, f32), mut iteration: u64, randjulia: u64) -> u64 {
    //determine which julia_set fractal will be generated (On the wiki page source under "Quadraic polynomials")
    let complex_num = match randjulia {
        //every stage of the julia set is listed as a possible option
        1 => num::Complex::new(-0.8, 0.0),
        2 => num::Complex::new(0.285, 0.0),
        3 => num::Complex::new(-0.4, 0.6),
        4 => num::Complex::new(0.45, 0.1428),
        5 => num::Complex::new(0.285, 0.01),
        6 => num::Complex::new(-0.70176, -0.3842),
        7 => num::Complex::new(-0.835, -0.2321),
        8 => num::Complex::new(-0.8, 0.156),
        9 => num::Complex::new(-0.7269, 0.1889),
        10 => num::Complex::new(0.0, -0.8),
        _ => num::Complex::new(-0.4, 0.6),
    };

    let mut value = num::Complex::new(complex_x, complex_y);

    while iteration < 255 && value.norm() <= 2.0 {
        //the julia fractal formula (f(z) = z^2 + c)
        value = value * value + complex_num;
        iteration += 1;
    }

    iteration
}

pub fn julia_fractal(imgy: u32, imgx: u32, filename: &str, scheme: &mut Scheme) {
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy); // Create a new ImgBuf and apply our dimensions to it.
    let scaleset = ((3.0 / imgx as f32), (3.0 / imgy as f32));

    let mut rng = rand::thread_rng(); //determine random value that will choose which julia set will be generated
    let randjulia = rng.gen_range(1, 11);
    apply_background(&mut imgbuf, &scheme); //set the intial background of the image based on the users choice

    //cycle through every pixel, send to fractal formula function pixel_setter, set the pixel based on result of that function
    for x in 0..imgx {
        for y in 0..imgy {
            let complex_pos = ((y as f32 * scaleset.0 - 1.5), (x as f32 * scaleset.1 - 1.5)); //determines position in frame

            let result = pixel_setter(complex_pos, 0, randjulia); //run pixel through fractal formula

            let pixel = imgbuf.get_pixel_mut(x, y); //pull out pixel data
            let Rgb(data) = *pixel; //set pixel data onto the rgb array

            match scheme.color {
                //apply the pixel shade on the result from pixel setter
                Red => *pixel = Rgb([result as u8, data[1], data[2]]), //apply it to the channel the user chose
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
