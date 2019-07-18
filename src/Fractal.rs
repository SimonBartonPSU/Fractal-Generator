// Copyright © 2019 Liam Rotchford, Simon Barton


pub fn julia_fractal(imgy: u32, imgx: u32, scalex: f32, scaley: f32, imgbuf: image::ImageBuffer<Rgb<u8>, Vec<u8>>) {

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

}