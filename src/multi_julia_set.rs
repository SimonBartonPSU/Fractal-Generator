// Copyright © 2019 Liam Rotchford, Simon Barton
/*
use image::Rgb;
use crate::util::*;

pub fn pixel_set_multi((imgx, imgy): (f32, f32), (loop_x, loop_y): (f32, f32), mut i: u64) -> u64 {
    let mut val_x = 3.0 * (loop_x - 0.5 * imgx) / (imgx);
    let mut val_y = 2.0 * (loop_y - 0.5 * imgy) / (imgy);
    let complex_x = -0.9;
    let complex_y = 0.27015;

    while (val_x * val_x + val_y * val_y) < 4.0 && i > 1 {
        let holder = (val_x * val_x) - (val_y * val_y) + complex_x;
        val_y = 2.0 * (val_x * val_y) + complex_y;
        val_x = holder;
        i -= 1;
    }

    i
}

pub fn multi_julia(imgy: u32, imgx: u32, filename: &str, scheme: Scheme) {
    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        //R                     //G         //B
        *pixel = Rgb([((0.3 * x as f32) as u8), 0, ((0.3 * y as f32) as u8)]);
    }

    //let imgx_edit = 8000;
    //let imgy_edit = 6000;

    let img = (imgx as f32, imgy as f32);

    for x in 0..imgx {
        for y in 0..imgy {
            let loop_val = (x as f32, y as f32);

            let result = pixel_set_multi(img, loop_val, 110);

            let pixel = imgbuf.get_pixel_mut(x, y);

            let Rgb(data) = *pixel;

            if scheme == "color" {
                *pixel = Rgb([data[0], result as u8, data[2]]);
            } else {
                *pixel = Rgb([result as u8, result as u8, result as u8]);
            }
        }
    }

    imgbuf.save(filename).unwrap();
}
*/
