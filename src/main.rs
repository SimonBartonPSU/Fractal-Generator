// Copyright © 2019 Liam Rotchford, Simon Barton

// Originally inspired by the
// Mandelbrot example from Blandy & Orendorff, ch 1.
// I/O Help
// https://www.reddit.com/r/rust/comments/41hgwq/help_needed_user_input_in_rust/

#![allow(dead_code)]

mod barnsley;
mod julia_sets;
mod mandelbrot;
mod multi_julia_set;
mod util;

use crate::barnsley::*;
use crate::julia_sets::*;
use crate::mandelbrot::*;
use crate::multi_julia_set::*;
use crate::util::*;
use std::string::String;

fn usage() -> ! {
    eprintln!("\n\n\tusage: <fractal-type> <file-name> <width>x<height> <color or gray>\n\n");
    std::process::exit(1)
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
    let color = &args[4];
    let scheme = Scheme::default(); 

    let input: Vec<String> = user_menu();
    //match input[0] {
        //"custom".to_string() =>
        //"random".to_string() =>
    //}


    match args[1].as_str() {
        "barnsley" => barnsley_fern(imgx, imgy, filename, scheme),
        "julia" => julia_fractal(imgx, imgy, filename, color),
        "mandelbrot" => mandelbrot_fractal(imgx, imgy, filename, color),
        "multi-julia" => multi_julia(imgx, imgy, filename, color),
        _ => usage(),
    }
}
