// Copyright © 2019 Liam Rotchford, Simon Barton
// Inspired by http://rosettacode.org/wiki/Barnsley_fern

use rand::Rng;

pub fn barnsley_fern(imgx: u32, imgy: u32, filename: &str) {
    let max_iter = 200000_u32;
    let mut rng = rand::thread_rng();
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    let mut x = 0_f64;
    let mut y = 0_f64;

    for _ in 0..max_iter {
        let rand_num = rng.gen::<f32>();
        let cx: f64;
        let cy: f64;
        
        if rand_num <= 0.01 {
            cx = 0.0_f64;
            cy = 0.16 * y;
        }
        else if rand_num <= 0.08 {
            cx = 0.2 * x - 0.26 * y;
            cy = 0.23 * x + 0.22 * y + 1.6;
        }
        else if rand_num <= 0.15 {
            cx = -0.15 * x + 0.28 * y;
            cy = 0.26 * x  + 0.26 * y + 0.44;
        }
        else {
            cx = 0.85 * x + 0.04 * y;
            cy = -0.04 * x  + 0.85 * y + 1.6;
        }
        x = cx;
        y = cy;

        let new_x = ((imgx as f64) / 2. + x * (imgx as f64) / 11.).round() as u32;
        let new_y = ((imgy as f64) - y * (imgy as f64) / 11.).round() as u32;
        let pixel = imgbuf.get_pixel_mut(new_x, new_y);
        *pixel = image::Rgb([50 as u8, 205 as u8, 50 as u8]);
    }

    imgbuf.save(filename).expect("Image write failed...");
}
