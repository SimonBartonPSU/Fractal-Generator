// Copyright © 2019 Liam Rotchford, Simon Barton
// Generate Barnsley's fern saved as an image
// Inspired by http://rosettacode.org/wiki/Barnsley_fern
// Barnsley's IFS: https://en.wikipedia.org/wiki/Barnsley_fern#Construction

use crate::util::*;
use rand::Rng;

/// Plot Barnsley's fern - For some arbitrarily large number of iterations,
/// apply one of four affine transformations. That is, start the x,y coordinate
/// pair at 0,0 then multiply by some values in Barnsley's matrix of
/// constants and adding some constant.
pub fn barnsley_fern(imgx: u32, imgy: u32, filename: &str, scheme: &mut Scheme) {
    let mut rng = rand::thread_rng();
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
    let color: [u8; 3] = color_to_rgb(scheme.color);

    let mut x = 0_f64;
    let mut y = 0_f64;

    apply_background(&mut imgbuf, &scheme);

    for _ in 0..20000_u32 {
        let rand_num = rng.gen::<f32>();
        let cx: f64;
        let cy: f64;

        if rand_num <= 0.01 {
            cx = 0.0_f64;
            cy = 0.16 * y;
        } else if rand_num <= 0.08 {
            cx = 0.2 * x - 0.26 * y;
            cy = 0.23 * x + 0.22 * y + 1.6;
        } else if rand_num <= 0.15 {
            cx = -0.15 * x + 0.28 * y;
            cy = 0.26 * x + 0.26 * y + 0.44;
        } else {
            cx = 0.85 * x + 0.04 * y;
            cy = -0.04 * x + 0.85 * y + 1.6;
        }
        x = cx;
        y = cy;

        let new_x = ((f64::from(imgx)) / 2.0 + x * (f64::from(imgx)) / 11.0).round() as u32;
        let new_y = ((f64::from(imgy)) - y * (f64::from(imgy)) / 11.0).round() as u32;
        let pixel = imgbuf.get_pixel_mut(new_x, new_y);
        *pixel = image::Rgb([color[0], color[1], color[2]]);
    }

    imgbuf.save(filename).expect("Image write failed...");
}
