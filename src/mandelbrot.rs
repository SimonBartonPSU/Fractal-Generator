// Copyright © 2019 Liam Rotchford, Simon Barton

/// Mandelbrot - fractal pattern representing the escape time of
/// a complex number being squared plus some constant to infinity.
use num::Complex;
use std::str::FromStr;

pub fn mandelbrot_fractal(imgx: u32, imgy: u32, filename: &str) {
    let complex_x_min = -2_f32;
    let complex_x_max = 1_f32;
    let complex_y_min = -2_f32;
    let complex_y_max = 1_f32;
    let scalex = (complex_x_max - complex_x_min) / imgx as f32;
    let scaley = (complex_y_max - complex_y_min) / imgx as f32;

    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let cx = complex_x_min + x as f32 * scalex;
        let cy = complex_y_min + y as f32 * scaley;

        let c = Complex::new(cx, cy);
        let mut z = Complex::new(0_f32, 0_f32);

        let mut i = 0;
        for t in 0..256 {
            if z.norm() > 2.0 {
                break;
            }
            z = z * z + c;
            i = t;
        }
        *pixel = image::Rgb([i as u8, 0_u8, i as u8]);
    }
    
    imgbuf.save(filename).expect("Image write failed...");
}

// Helpers
/// Parse a string as a pair of values separated by a
/// separator char.
pub fn parse_pair<T: FromStr>(s: &str, sep: char) -> Option<(T, T)> {
    let fields: Vec<&str> = s.split(sep).collect();
    if fields.len() != 2 {
        return None;
    }
    match (T::from_str(fields[0]), T::from_str(fields[1])) {
        (Ok(f0), Ok(f1)) => Some((f0, f1)),
        _ => None,
    }
}
