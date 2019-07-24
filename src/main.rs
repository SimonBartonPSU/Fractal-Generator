// Copyright © 2019 Liam Rotchford, Simon Barton

// Originally inspired by the
// Mandelbrot example from Blandy & Orendorff, ch 1.
// Compute and display a Mandelbrot set.

mod mandelbrot;
mod fractal;

use crate::mandelbrot::*;
use crate::fractal::*;

use std::string::String;
// use rand::Rng;

/// Show a usage message and exit.
fn usage() -> ! {
    eprintln!("\n\n\tusage: <fractal-type> <file-name> <width>x<height>\n\n");
    std::process::exit(1)
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
        "mandelbrot" => mandelbrot_fractal(imgx, imgy, filename),
        //"dragoncurve" =>
        //"levyccurve" =>
        _ => usage()
    }
}
