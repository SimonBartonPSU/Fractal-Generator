// Copyright © 2019 Liam Rotchford, Simon Barton

// Originally inspired by the
// Mandelbrot example from Blandy & Orendorff, ch 1.
// Compute and display a Mandelbrot set.


use image::ColorType;
use image::png::PNGEncoder;
use num::Complex;
use std::fs::File;
use std::str::FromStr;

/// Determine if `c` is still a Mandelbrot set candidate
/// after `limit` iterations. If `c` has been eliminated
/// return the iteration count.
fn escape_time(c: Complex<f64>, limit: u64) -> Option<u64> {
    let mut z = Complex {
        re: 0.0,
        im: 0.0,
    };
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
fn parse_pair<T: FromStr>(s: &str, sep: char) -> Option<(T, T)> {
    let fields: Vec<&str> = s.split(sep).collect();
    if fields.len() != 2 {
        return None;
    }
    match (
        T::from_str(fields[0]),
        T::from_str(fields[1]),
    ) {
        (Ok(f0), Ok(f1)) => Some((f0, f1)),
        _ => None,
    }
}

/// Parse a complex number.
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

/// Coordinate map between rectangle of pixels and rectangle
/// of complex numbers.
struct PixelSpace {
    /// Width and height of pixel space.
    pixel_dims: (u64, u64),
    /// Upper-left and lower-right corners of complex space.
    complex_corners: (Complex<f64>, Complex<f64>),
}

impl PixelSpace {
    /// Transform the given pixel coordinate to a
    /// linearly-interpolated complex number.
    fn pixel_to_point(&self, pixel: (u64, u64)) -> Complex<f64> {
        assert!(pixel.0 <= self.pixel_dims.0);
        assert!(pixel.1 <= self.pixel_dims.1);
        let f0 = pixel.0 as f64 / self.pixel_dims.0 as f64;
        let f1 = pixel.1 as f64 / self.pixel_dims.1 as f64;
        let re = self.complex_corners.1.re * f0
            + self.complex_corners.0.re * (1.0 - f0);
        let im = self.complex_corners.1.im * f1
            + self.complex_corners.0.im * (1.0 - f1);
        Complex { re, im }
    }

    /// Render all the pixels in a pixel space as Mandelbrot
    /// points for further processing.
    fn render(&self, result: &mut [u8]) {
        let mut p = 0;
        for row in 0..self.pixel_dims.1 {
            for col in 0..self.pixel_dims.0 {
                let c = self.pixel_to_point((col, row));
                let t = match escape_time(c, 255) {
                    None => 0,
                    Some(t) => 255 - t as u8,
                };
                result[p] = t;
                p += 1;
            }
        }
    }

    /// Render a pixel space to a file.
    fn write_image(
        &self,
        filename: &str,
        nthreads: usize,
    ) -> Result<(), std::io::Error> {
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
        encoder.encode(
            &pixels,
            w as u32,
            h as u32,
            ColorType::Gray(8),
        )
    }

    /// Return a PixelSpace representing a horizontal "band"
    /// of the given space from `h0` to `h1`.
    fn band(&self, h0: u64, h1: u64) -> PixelSpace {
        assert!(h0 <= h1);
        let cul = self.pixel_to_point((0, h0));
        let clr = self.pixel_to_point((self.pixel_dims.0, h1));
        PixelSpace {
            pixel_dims: (self.pixel_dims.0, h1 - h0),
            complex_corners: (cul, clr),
        }
    }
}

#[test]
fn test_pixel_to_point() {
    let ps = PixelSpace {
        pixel_dims: (100, 100),
        complex_corners: (
            Complex {
                re: -1.0,
                im: 1.0,
            },
            Complex {
                re: 1.0,
                im: -1.0,
            },
        ),
    };
    assert_eq!(
        ps.pixel_to_point((25, 75)),
        Complex {
            re: -0.5,
            im: -0.5
        }
    )
}

/// Show a usage message and exit.
fn usage() -> ! {
    eprintln!(
        "usage: mandelbrot <file> <width>x<height> <viewul>x<viewlr> [<threads>]"
    );
    std::process::exit(1)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 || args.len() > 5 {
        usage()
    }
    let pixel_dims =
        parse_pair(&args[2], 'x').expect("bad image dimensions");
    let cs = (&args[3])
        .split('x')
        .collect::<Vec<&str>>();
    let cul = parse_complex(cs[0]).expect("bad complex coordinates");
    let clr = parse_complex(cs[1]).expect("bad complex coordinates");
    let ps = PixelSpace {
        pixel_dims,
        complex_corners: (cul, clr),
    };
    let nthreads =
        if args.len() == 5 {
            usize::from_str(&args[4]).expect("non-number of threads")
        } else {
            1
        };
    ps.write_image(&args[1], nthreads)
        .expect("could not write png")
}
