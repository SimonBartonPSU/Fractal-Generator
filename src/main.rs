// Copyright © 2019 Liam Rotchford, Simon Barton

// Originally inspired by the
// Mandelbrot example from Blandy & Orendorff, ch 1.
// Compute and display a Mandelbrot set.

mod barnsley;
mod fractal;
mod mandelbrot;

use crate::barnsley::*;
use crate::fractal::*;
use crate::mandelbrot::*;
use std::string::String;
use std::str::FromStr;

/// Show a usage message and exit.
fn usage() -> ! {
    eprintln!("\n\n\tusage: <fractal-type> <file-name> <width>x<height> <color or gray>\n\n");
    std::process::exit(1)
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

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    if args.len() != 5 {
        usage()
    }

    let pixel_dims = parse_pair(&args[3], 'x').expect("bad image dimensions");
    args[1] = args[1].to_lowercase();

    let imgx = pixel_dims.0;
    let imgy = pixel_dims.1;
    let filename = &args[2];
    let scheme = &args[4];

    //determine which fractal to use

    match args[1].as_str() {
        "barnsley"    => barnsley_fern(imgx, imgy, filename, scheme), 
        "julia"       => julia_fractal(imgx, imgy, filename, scheme),
        "mandelbrot"  => mandelbrot_fractal(imgx, imgy, filename, scheme),
        "multi-julia" => multi_julia(imgx, imgy, filename, scheme),
        _ => usage(),
    }
}
