// Copyright © 2019 Liam Rotchford, Simon Barton

use image::png::PNGEncoder;
/// Mandelbrot - fractal pattern representing the escape time of
/// a complex number being squared plus some constant to infinity.
use image::ColorType;
use num::Complex;
use std::fs::File;
use std::str::FromStr;

/*
pub fn mandelbrot_fractal(arg_set: &mut [String]) {

    
    let cs = (&arg_set[4]).split('x').collect::<Vec<&str>>();
    let cul = parse_complex(cs[0]).expect("bad complex coordinates");
    let clr = parse_complex(cs[1]).expect("bad complex coordinates");
    let ps = PixelSpace {
        pixel_dims,
        complex_corners: (cul, clr),
    };
    let nthreads = if arg_set.len() == 6 {
        usize::from_str(&arg_set[5]).expect("non-number of threads")
    } else {
        1
    };
    ps.write_image(&arg_set[2], nthreads)
        .expect("could not write png")
}
*/

/// Determine if `c` is still a Mandelbrot set candidate
/// after `limit` iterations. If `c` has been eliminated
/// return the iteration count.
pub fn escape_time(c: Complex<f64>, limit: u64) -> Option<u64> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}

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

/// Parse a complex number.
pub fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

/// Coordinate map between rectangle of pixels and rectangle
/// of complex numbers.
pub struct PixelSpace {
    /// Width and height of pixel space.
    pub pixel_dims: (u64, u64),
    /// Upper-left and lower-right corners of complex space.
    pub complex_corners: (Complex<f64>, Complex<f64>),
}

impl PixelSpace {
    /// Transform the given pixel coordinate to a
    /// linearly-interpolated complex number.
    pub fn pixel_to_point(&self, pixel: (u64, u64)) -> Complex<f64> {
        assert!(pixel.0 <= self.pixel_dims.0);
        assert!(pixel.1 <= self.pixel_dims.1);
        let f0 = pixel.0 as f64 / self.pixel_dims.0 as f64;
        let f1 = pixel.1 as f64 / self.pixel_dims.1 as f64;
        let re = self.complex_corners.1.re * f0 + self.complex_corners.0.re * (1.0 - f0);
        let im = self.complex_corners.1.im * f1 + self.complex_corners.0.im * (1.0 - f1);
        Complex { re, im }
    }

    /// Render all the pixels in a pixel space as Mandelbrot
    /// points for further processing.
    pub fn render(&self, result: &mut [u8]) {
        let mut p = 0;
        for row in 0..self.pixel_dims.1 {
            for col in 0..self.pixel_dims.0 {
                let c = self.pixel_to_point((col, row));
                
                let t = match escape_time(c, 255) {         //T is what determine the shade on the 
                    None => 0,                              // bit shade choice, if its something then black 255
                    Some(t) => 255 - t as u8,               // otherwise white
                };
                
                result[p] = t;
                p += 1;
            }
        }
    }

    /// Render a pixel space to a file.
    pub fn write_image(&self, filename: &str, nthreads: usize) -> Result<(), std::io::Error> {
        let w = self.pixel_dims.0 as usize;
        let h = self.pixel_dims.1 as usize;
        

        let mut pixels = vec![0u8; w * h];
        
        crossbeam::scope(|spawner| {
            let mut h0 = 0;
            let dh = h / nthreads;
            
            for px in pixels.chunks_mut(w * dh) {
                let h1 = std::cmp::min(h as u64, h0 as u64 + dh as u64);
                let ps = self.band(h0, h1);
                spawner.spawn(move || ps.render(px));
                h0 = h1;
            }
        
        });
        
        let output = File::create(filename)?;
        
        let encoder = PNGEncoder::new(output);
        
        //println!("\n\n{:?}\n\n", pixels);
        
        encoder.encode(&pixels, w as u32, h as u32, ColorType::Gray(8))
        //pixels are my numbers of image, w and h are the dimensions
    }

    /// Return a PixelSpace representing a horizontal "band"
    /// of the given space from `h0` to `h1`.
    pub fn band(&self, h0: u64, h1: u64) -> PixelSpace {
        assert!(h0 <= h1);
        let cul = self.pixel_to_point((0, h0));
        let clr = self.pixel_to_point((self.pixel_dims.0, h1));
        PixelSpace {
            pixel_dims: (self.pixel_dims.0, h1 - h0),
            complex_corners: (cul, clr),
        }
    }
}
