// Copyright © 2019 Liam Rotchford, Simon Barton

use ::image::Rgb;
//use rand::Rng;

pub fn julia_fractal(imgy: u32, imgx: u32, filename: &str, scheme: &str) {
    // https://crates.io/crates/image
    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // let mut rng = rand::thread_rng();
    // let randR: f32 = rng.gen_range(0, 255) as f32;
    //let randB: f32 = rng.gen_range(0, 255) as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;

        *pixel = Rgb([r, 0, b]);
    }

    for x in 0..imgx {
        for y in 0..imgy {
            let cx = y as f32 * scalex - 1.5; //determines position in frame
            let cy = x as f32 * scaley - 1.5;

            let c = num::Complex::new(-0.4, 0.6);
            let mut z = num::Complex::new(cx, cy);

            let mut i = 0;
            while i < 255 && z.norm() <= 2.0 {
                //the julia fractal
                z = z * z + c;
                i += 1;
            }

            let pixel = imgbuf.get_pixel_mut(x, y);

            let Rgb(data) = *pixel;

            if scheme == "color" {
                *pixel = Rgb([data[0], i as u8, data[2]]);
            }else {
                *pixel = Rgb([i as u8, i as u8, i as u8]);
            }
        }
    }

    // Save the image
    imgbuf.save(filename).unwrap();
}

pub fn multi_julia(imgy: u32, imgx: u32, filename: &str, scheme: &str) {
    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = Rgb([r, 0, b]);
    }

    let iterations = 110; //altering these will change the image
    let cx = -0.9;
    let cy = 0.27015;

    for x in 0..imgx {
        for y in 0..imgy {
            let inner_height = imgy as f32;
            let inner_width = imgx as f32;
            let inner_y = y as f32;
            let inner_x = x as f32;

            let mut zx = 3.0 * (inner_x - 0.5 * inner_width) / (inner_width);
            let mut zy = 2.0 * (inner_y - 0.5 * inner_height) / (inner_height);

            let mut i = iterations;

            while zx * zx + zy * zy < 4.0 && i > 1 {
                let tmp = zx * zx - zy * zy + cx;
                zy = 2.0 * zx * zy + cy;
                zx = tmp;
                i -= 1;
            }

            let pixel = imgbuf.get_pixel_mut(x, y);

            let Rgb(data) = *pixel;

            if scheme == "color" {
                *pixel = Rgb([data[0], i as u8, data[2]]);
            }else {
                *pixel = Rgb([i as u8, i as u8, i as u8]);
            }
        }
    }

    imgbuf.save(filename).unwrap();
}
