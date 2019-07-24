// Copyright © 2019 Liam Rotchford, Simon Barton

// Originally inspired by the
// Mandelbrot example from Blandy & Orendorff, ch 1.
// Compute and display a Mandelbrot set.

mod fractal;
mod mandelbrot;

use crate::fractal::*;
use crate::mandelbrot::*;

use std::string::String;
// use rand::Rng;

/// Show a usage message and exit.
fn usage() -> ! {
    eprintln!("\n\n\tusage: <fractal-type> <file-name> <width>x<height>\n\n");
    std::process::exit(1)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        usage()
    }
    let pixel_dims = parse_pair(&args[3], 'x').expect("bad image dimensions");

    // determine which fractal to use
    match args[1].to_lowercase().as_str() {
        "julia" => julia_fractal(pixel_dims.0, pixel_dims.1, &args[2]),
        "mandelbrot" => mandelbrot_fractal(pixel_dims.0, pixel_dims.1, &args[2]),
        //"dragoncurve" =>
        //"levyccurve" =>
        _ => usage(),
    }
}
