// Copyright © 2019 Liam Rotchford, Simon Barton

// Originally inspired by the
// Mandelbrot example from Blandy & Orendorff, ch 1.
// Compute and display a Mandelbrot set.

mod mandelbrot;

use crate::mandelbrot::*;
use std::str::FromStr;

/// Show a usage message and exit.
fn usage() -> ! {
    eprintln!("usage: mandelbrot <file> <width>x<height> <viewul>x<viewlr> [<threads>]");
    std::process::exit(1)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 || args.len() > 5 {
        usage()
    }
    let pixel_dims = parse_pair(&args[2], 'x').expect("bad image dimensions");
    let cs = (&args[3]).split('x').collect::<Vec<&str>>();
    let cul = parse_complex(cs[0]).expect("bad complex coordinates");
    let clr = parse_complex(cs[1]).expect("bad complex coordinates");
    let ps = PixelSpace {
        pixel_dims,
        complex_corners: (cul, clr),
    };
    let nthreads = if args.len() == 5 {
        usize::from_str(&args[4]).expect("non-number of threads")
    } else {
        1
    };
    ps.write_image(&args[1], nthreads)
        .expect("could not write png")
}
