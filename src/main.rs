// Copyright © 2019 Liam Rotchford, Simon Barton

// Originally inspired by the
// Mandelbrot example from Blandy & Orendorff, ch 1.
// I/O Help
// https://www.reddit.com/r/rust/comments/41hgwq/help_needed_user_input_in_rust/

mod barnsley;
mod julia_sets;
mod julias;
mod mandelbrot;
mod util;
mod menu;



use crate::barnsley::*;
use crate::julia_sets::*;
use crate::mandelbrot::*;
use crate::menu::*;
use crate::util::*;
use std::string::String;

fn usage() -> ! {
    eprintln!("\n\n\tusage: <fractal-type> <file-name> <width>x<height>\n\n");
    std::process::exit(1)
}

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 3 {
        usage()
    }

    let pixel_dims = parse_pair(&args[2], 'x').expect("bad image dimensions");
    args[0] = args[0].to_lowercase();

    let imgx = pixel_dims.0;
    let imgy = pixel_dims.1;
    let filename = &args[1];
    let mut scheme = Scheme {
        fractal: args[0].clone(),
        ..Default::default()
    };

    user_menu(&mut scheme);

    match args[0].as_str() {
        "barnsley" => barnsley_fern(imgx, imgy, filename, scheme),
        "julia" | "multi-julia" => julia_fractal(args[0].as_str(), imgx, imgy, filename, scheme),
        "mandelbrot" => mandelbrot_fractal(imgx, imgy, filename, scheme),
        _ => panic!("Unsupported fractal type"),
    }
}
