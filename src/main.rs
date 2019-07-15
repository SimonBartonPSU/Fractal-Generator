// Copyright © 2019 Liam Rotchford, Simon Barton

// Originally inspired by the
// Mandelbrot example from Blandy & Orendorff, ch 1.
// Compute and display a Mandelbrot set.

mod mandelbrot;

use crate::mandelbrot::*;
use std::str::FromStr;
use std::string::String;



fn mandelbrot_fractal(arg_set: &mut [String]) {

    let pixel_dims = parse_pair(&arg_set[3], 'x').expect("bad image dimensions");
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

/// Show a usage message and exit.
fn usage() -> ! {
    eprintln!("\n\n\tusage: <fractal-type> <file-name> <width>x<height> <viewul>x<viewlr> [<threads>] \n\n");
    std::process::exit(1)
}

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    if args.len() < 5 || args.len() > 6 {
        usage()
    }
  
    args[1] = args[1].to_lowercase();
    

    match args[1].as_str() {
        "mandlebrot" => mandelbrot_fractal(args.as_mut_slice()),
        //"dragoncurve" =>
        //"levyccurve" =>
        _ => usage()

    }

}
