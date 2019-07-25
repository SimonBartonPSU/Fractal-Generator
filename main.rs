// Copyright © 2019 Liam Rotchford, Simon Barton

// Originally inspired by the
// Mandelbrot example from Blandy & Orendorff, ch 1.
// Compute and display a Mandelbrot set.

mod fractal;

use crate::fractal::*;
use std::str::FromStr;
use std::string::String;
// use rand::Rng;

/// Show a usage message and exit.
fn usage() -> ! {
    eprintln!("\n\n\tusage: <fractal-type> <file-name> <width>x<height>\n\n");
    std::process::exit(1)
}

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
    if args.len() != 4 {
        usage()
    }

    let pixel_dims = parse_pair(&args[3], 'x').expect("bad image dimensions");
    args[1] = args[1].to_lowercase();

    let imgx = pixel_dims.0;
    let imgy = pixel_dims.1;
    let filename = &args[2];

    //determine which fractal to use

    match args[1].as_str() {
        "julia" => julia_fractal(imgx, imgy, filename),
        //"mandelbrot" => mandelbrot_fractal(args.as_mut_slice()),
        "multi-julia" => multi_julia(imgx, imgy, filename),
        _ => usage(),
    }
}
