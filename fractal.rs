// Copyright © 2019 Liam Rotchford, Simon Barton

use ::image::Rgb;

//use rand::Rng;

pub fn pixel_setter( (a, b): (f32, f32), mut i: u64 ) -> u64 {
    let c = num::Complex::new(-0.4, 0.6);
    let mut z = num::Complex::new(a, b);

    while i < 255 && z.norm() <= 2.0 {
        //the julia fractal
        z = z * z + c;

        //different styles of julia sets
        //z = z * z + 0.279;
        //z = z * z * z + 0.400;

        //let quad = z * z * z * z;
        //  z = quad + 0.494;

        i += 1;
    }
    
    i
}

pub fn julia_fractal(imgy: u32, imgx: u32, filename: &str, scheme: &str) {
    // https://crates.io/crates/image
    let scaleset = ( (3.0 / imgx as f32), (3.0 / imgy as f32) );
   
    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    //let mut rng = rand::thread_rng();
    //let randR: f32 = rng.gen_range(0, 255) as f32;
    //let randB: f32 = rng.gen_range(0, 255) as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
                        //R                     //G         //B
        *pixel = Rgb([ ((0.3 * x as f32) as u8), 0, ((0.3 * y as f32) as u8) ]);
    }

    for x in 0..imgx {
        for y in 0..imgy {
            let complex_pos = ( (y as f32 * scaleset.0 - 1.5), (x as f32 * scaleset.1 - 1.5) );  //determines position in frame
            
            let result = pixel_setter(complex_pos, 0);
            
            let pixel = imgbuf.get_pixel_mut(x, y);

            let Rgb(data) = *pixel;

            if scheme == "color" {
                *pixel = Rgb([data[0], result as u8, data[2]]);
            }else {
                *pixel = Rgb([result as u8, result as u8, result as u8]);
            }
        }
    }

    // Save the image
    imgbuf.save(filename).unwrap();
}



//=======================================================================

pub fn pixel_set_multi((imgx, imgy): (u32, u32), (x, y): (u32, u32), mut iteration: u64) -> u64 {

    let mut zx = 3.0 * (x as f32 - 0.5 * imgx as f32) / (imgx as f32);
    let mut zy = 2.0 * (y as f32 - 0.5 * imgy as f32) / (imgy as f32);
    let cx = -0.9;
    let cy = 0.27015;

    while zx * zx + zy * zy < 4.0 && iteration > 1 {
        let tmp = zx * zx - zy * zy + cx;
        zy = 2.0 * zx * zy + cy;
        zx = tmp;
        iteration -= 1;
    }

    iteration
}

pub fn multi_julia(imgy: u32, imgx: u32, filename: &str, scheme: &str) {
    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
                        //R                     //G         //B
        *pixel = Rgb([ ((0.3 * x as f32) as u8), 0, ((0.3 * y as f32) as u8) ]);
    }

    let img = (imgx, imgy);

    for x in 0..imgx {
        for y in 0..imgy {
            let loop_val = (x, y);

            let result = pixel_set_multi(img, loop_val, 110);

            let pixel = imgbuf.get_pixel_mut(x, y);

            let Rgb(data) = *pixel;

            if scheme == "color" {
                *pixel = Rgb([data[0], result as u8, data[2]]);
            }else {
                *pixel = Rgb([result as u8, result as u8, result as u8]);
            }
        }
    }

    imgbuf.save(filename).unwrap();
}
