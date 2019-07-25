// Copyright © 2019 Liam Rotchford, Simon Barton

pub fn julia_fractal(imgx: u32, imgy: u32, filename: &str) {
    // https://crates.io/crates/image
    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;

    // Width by Height image buffer
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // let mut rng = rand::thread_rng();
    // let randR: f32 = rng.gen_range(0, 255) as f32;
    //let randB: f32 = rng.gen_range(0, 255) as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    for x in 0..imgx {
        for y in 0..imgy {
            let cx = y as f32 * scalex - 1.5;
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
            let image::Rgb(data) = *pixel;
            *pixel = image::Rgb([data[0], i as u8, data[2]]);
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save(filename).expect("Image write failed...");
}

