// Copyright © 2019 Liam Rotchford, Simon Barton

use crate::util::Color::*;
use crate::util::*;
use image::Rgb;
use num::Complex;

/// Mandelbrot - fractal pattern representing the escape time of
/// a complex number being squared plus some constant to infinity.
pub fn mandelbrot_fractal(imgx: u32, imgy: u32, filename: &str, scheme: &mut Scheme) {
    let complex_x_min = -2_f32;
    let complex_x_max = 1_f32;
    let complex_y_min = -1.4_f32;
    let complex_y_max = 1_f32;
    let scalex = (complex_x_max - complex_x_min) / imgx as f32;
    let scaley = (complex_y_max - complex_y_min) / imgx as f32;

    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    apply_background(&mut imgbuf, &scheme);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let cx = complex_x_min + x as f32 * scalex;
        let cy = complex_y_min + y as f32 * scaley;

        let c = Complex::new(cx, cy);
        let mut z = Complex::new(0_f32, 0_f32);

        let mut i = 0;
        for t in 0..255 {
            if z.norm() > 2.0 {
                break;
            }
            z = z * z + c;
            i = t;
        }

        let Rgb(data) = *pixel;
        match scheme.color {
            Red => *pixel = Rgb([i, data[1], data[2]]), //apply it to the channel the user chose
            Green => *pixel = Rgb([data[0], i, data[2]]),
            Blue => *pixel = Rgb([data[0], data[1], i]),
            White => *pixel = Rgb([i, i, i]),
            _ => panic!("Unsupported color"),
        }
    }

    imgbuf.save(filename).expect("Image write failed...");
}
