// Copyright © 2019 Liam Rotchford, Simon Barton

// Originally inspired by the
// Mandelbrot example from Blandy & Orendorff, ch 1.

/// Compute and generate Mandelbrot, julia sets, multi-julia set, 
/// and barnsley fern fractals into a .png image. 

mod barnsley;
mod julia_sets;
mod mandelbrot;
mod multi_julia_set;

use crate::barnsley::*;
use crate::julia_sets::*;
use crate::mandelbrot::*;
use crate::multi_julia_set::*;
use std::str::FromStr;
use std::string::String;

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
    if args.len() != 5 {    //only accept 5 args in the command line string
        usage()
    }

    //parse command line contents given by user
    let pixel_dims = parse_pair(&args[3], 'x').expect("bad image dimensions");
    args[1] = args[1].to_lowercase();

    //pull and set dimensions to be used by fractal genertors
    let imgx = pixel_dims.0;
    let imgy = pixel_dims.1;
    let filename = &args[2];
    let scheme = &args[4];

    //determine which fractal to use

    match args[1].as_str() {
        "barnsley" => barnsley_fern(imgx, imgy, filename, scheme),
        "julia" => julia_fractal(imgx, imgy, filename, scheme),
        "mandelbrot" => mandelbrot_fractal(imgx, imgy, filename, scheme),
        "multi-julia" => multi_julia(imgx, imgy, filename, scheme),
        _ => usage(),
    }
}
