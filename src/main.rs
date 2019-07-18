// Copyright © 2019 Liam Rotchford, Simon Barton

// Originally inspired by the
// Mandelbrot example from Blandy & Orendorff, ch 1.
// Compute and display a Mandelbrot set.

mod mandelbrot;

use crate::mandelbrot::*;
use std::str::FromStr;
use std::string::String;
use rand::Rng;



/*

fn mandelbrot_fractal(arg_set: &mut [String]) {

    
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
*/
/// Show a usage message and exit.
fn usage() -> ! {
    eprintln!("\n\n\tusage: <fractal-type> <file-name> <width>x<height> <viewul>x<viewlr> [<threads>] \n\n");
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

    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    let mut rng = rand::thread_rng();
    let randR: f32 = rng.gen_range(0, 255) as f32;
    let randB: f32 = rng.gen_range(0, 255) as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        


        let r = (randR * x as f32) as u8;
        let b = (randB * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    // A redundant loop to demonstrate reading image data
    for x in 0..imgx {
        for y in 0..imgy {
            let cx = y as f32 * scalex - 1.5;
            let cy = x as f32 * scaley - 1.5;

            let c = num::Complex::new(-0.4, 0.6);
            let mut z = num::Complex::new(cx, cy);

            let mut i = 0;
            while i < 255 && z.norm() <= 2.0 {      //the julia fractal
                z = z * z + c;
                i += 1;
            }

            let pixel = imgbuf.get_pixel_mut(x, y);
            let data = (*pixel as image::Rgb<u8>).data;
            *pixel = image::Rgb([data[0], i as u8, data[2]]);
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save(&args[2]).unwrap();


 /*   match args[1].as_str() {
        "mandelbrot" => mandelbrot_fractal(args.as_mut_slice()),
        //"dragoncurve" =>
        //"levyccurve" =>
        _ => usage()

    }
*/
}
