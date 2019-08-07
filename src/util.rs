// Copyright © 2019 Liam Rotchford, Simon Barton

#![allow(dead_code)]

use crate::util::Color::*;
use image::imageops::*;
use image::*;
use rand::Rng;
use std::fs;
use std::str::FromStr;

/// 3x3 matrix values and other literals
const SMOOTH_KERNEL: [f32; 9] = [1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0];
const SHARPEN_KERNEL: [f32; 9] = [-1.0, -1.0, -1.0, -1.0, 9.0, -1.0, -1.0, -1.0, -1.0];
const RAISED_KERNEL: [f32; 9] = [0.0, 0.0, -2.0, 0.0, 2.0, 0.0, 1.0, 0.0, 0.0];
const COLORS: [&str; 8] = [
    "red", "blue", "green", "orange", "yellow", "violet", "black", "white",
];
/// Str literals used to call random image operations
const TRANSFORMS: [&str; 9] = [
    "brighten",
    "contrast",
    "huerotate",
    "invert",
    "rotate180",
    "rotate270",
    "smooth filter",
    "sharpen filter",
    "raised filter",
];
const ROTATIONS: [i32; 3] = [90, 180, 270];

/// Supported colors for user input
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Red,
    Orange,
    Yellow,
    Blue,
    Green,
    Violet,
    Black,
    White,
}

/// Container for properties of fractal being built
#[derive(Debug)]
pub struct Scheme {
    pub fractal: String,
    /// Actual color of the fractal
    pub color: Color,
    pub fancy_background: bool,
    pub bg_color: Color,
    pub bg_color_2: Color,
    pub random: bool,
    pub do_transform: bool,
    pub transform: String,
}

/// Reasonable values are set for a default fractal scheme
/// With these values set, nothing can go seriously wrong
impl Default for Scheme {
    fn default() -> Scheme {
        Scheme {
            fractal: "mandelbrot".to_string(),
            color: Green,
            fancy_background: false,
            bg_color: Black,
            bg_color_2: Red,
            random: false,
            do_transform: false,
            transform: String::new(),
        }
    }
}

/// Helper to return three u8s based on parsed color
/// u8s function as RGB data
pub fn color_to_rgb(color: Color) -> [u8; 3] {
    match color {
        Red => [255, 0, 0],
        Orange => [255, 165, 0],
        Yellow => [255, 255, 0],
        Blue => [0, 0, 255],
        Green => [0, 128, 0],
        Violet => [238, 130, 238],
        Black => [0, 0, 0],
        White => [255, 255, 255],
    }
}

/// Convenient conversion from String to a Color
/// Defaults to Blue for invalid input colors
pub fn str_to_color(color: &str) -> Color {
    match color {
        "red" => Red,
        "orange" => Orange,
        "yellow" => Yellow,
        "blue" => Blue,
        "green" => Green,
        "violet" => Violet,
        "black" => Black,
        "white" => White,
        &_ => Black,
    }
}

/// Iterate over the pixels of the image and apply a cool
/// background, which will depend on scheme.
/// Either transitioning from one color to another
/// or just a solid background.
pub fn apply_background(imgbuf: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, scheme: &Scheme) {
    let color: [u8; 3] = color_to_rgb(scheme.bg_color);
    let alpha: u8 = if scheme.fractal == "barnsley" { 75 } else { 30 };

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let xc: u8 = (0.3 * x as f32) as u8;
        let yc: u8 = (0.3 * y as f32) as u8;
        if scheme.fancy_background {
            match scheme.bg_color {
                Red => match scheme.bg_color_2 {
                    Blue => *pixel = Rgba([xc, 0, yc, alpha]),
                    Green => *pixel = Rgba([xc, yc, 0, alpha]),
                    _ => println!("Unsupported bg_color_2"),
                },
                Green => match scheme.bg_color_2 {
                    Blue => *pixel = Rgba([0, xc, yc, alpha]),
                    Red => *pixel = Rgba([xc, yc, 0, alpha]),
                    _ => println!("Unsupported bg_color_2"),
                },
                Blue => match scheme.bg_color_2 {
                    Red => *pixel = Rgba([xc, 0, yc, alpha]),
                    Green => *pixel = Rgba([0, xc, yc, alpha]),
                    _ => println!("Unsupported bg_color_2"),
                },
                _ => println!("Unsupported bg_color"),
            }
        } else {
            //solid bg
            *pixel = Rgba([color[0], color[1], color[2], 50]);
        }
    }
}

/// Image processing functions supplied by image crate
/// to be used for fun and randomization imageops functions
/// Ranges for transforms that give cool results:
/// Range for blur 0.75 to 5.0
/// Range for brighten -50 to 80
/// Range for contrast -20.0 to 200.0
/// Range for huerotate 5 to 355
pub fn process_image(filename: &str, transformation: &str) {
    let mut image = image::open(filename).unwrap();
    let rotate_index = rand::thread_rng().gen_range(0, 3);

    match transformation {
        "blur" => blur(&image, 3.0_f32).save(filename).unwrap(),
        "brighten" => brighten(&image, 70).save(filename).unwrap(),
        "contrast" => contrast(&image, 100.0_f32).save(filename).unwrap(),
        "huerotate" => huerotate(&image, ROTATIONS[rotate_index])
            .save(filename)
            .unwrap(),
        "invert" => {
            invert(&mut image);
            image.save(filename).unwrap()
        }
        "rotate90" => rotate90(&image).save(filename).unwrap(),
        "rotate180" => rotate180(&image).save(filename).unwrap(),
        "rotate270" => rotate270(&image).save(filename).unwrap(),
        "smooth filter" => filter3x3(&image, &SMOOTH_KERNEL).save(filename).unwrap(),
        "sharpen filter" => filter3x3(&image, &SHARPEN_KERNEL).save(filename).unwrap(),
        "raised filter" => filter3x3(&image, &RAISED_KERNEL).save(filename).unwrap(),
        &_ => blur(&image, 0.9_f32).save("dfault_transform.png").unwrap(),
    };
}

/// Generate a random fractal scheme 
/// This includes color, type of background, and background color(s)
/// Barnsley is able to support more colors for its fractal
pub fn randomize(scheme: &mut Scheme) {
    scheme.random = true;
    if rand::thread_rng().gen_range(0, 1) == 0 {
        scheme.fancy_background = true;
    } else { 
        scheme.fancy_background = false;
    }

    let fractal_color;
    if scheme.fractal == "barnsley" {
        fractal_color = rand::thread_rng().gen_range(0, 8);

    } else {
        fractal_color = rand::thread_rng().gen_range(0, 3);
    }
    scheme.color = str_to_color(COLORS[fractal_color]);

    if scheme.fancy_background == true {
        let background_1 = rand::thread_rng().gen_range(0, 3);
        scheme.bg_color = str_to_color(COLORS[background_1]);
        let mut different = true;
        while different {
            let background_2 = rand::thread_rng().gen_range(0, 3);
            scheme.bg_color_2 = str_to_color(COLORS[background_2]);
            if scheme.bg_color_2 != scheme.bg_color {
                different = false;
            }
        }
    } else {
        let bg_num = rand::thread_rng().gen_range(0, 8);
        scheme.bg_color = str_to_color(COLORS[bg_num]);
    }
}

/// Apply a random number of random transformations
/// to some image file - a fractal for this programm
pub fn random_transforms(scheme: &Scheme, filename: &str) {
    let num_transforms = rand::thread_rng().gen_range(1, 7);
    let mut record: Vec<String> = Vec::new();
    for _ in 0..num_transforms {
        let transform_index = rand::thread_rng().gen_range(0, 8);
        let transform = TRANSFORMS[transform_index];
        process_image(filename, transform);
        record.push(transform.to_string());
    }

    log_random(&scheme, filename, record);
}

/// Keep track of what actually happened when a fractal
/// was randomized. In case a randomization happens to look cool
/// and one would like to apply the same characteristics to another fractal.
pub fn log_random(scheme: &Scheme, filename: &str, record: Vec<String>) {
    let transforms = record.join(", \n");

    let data: String = format!(
        "Fractal log\n
            Scheme:\n {:?},
            Transformations:\n {},",
        scheme, transforms
    );

    let write_path = "/tmp/".to_owned() + filename + ".log";
    fs::write(write_path, data).expect("Log write failed...");
}

// From B&O chapter 2, p28 - modified by Bart Massey
/// Helper to parse a string as a pair of values separated
/// by a separator char. For coordinate pairs e.g. "600x600"
pub fn parse_pair<T: FromStr>(s: &str, sep: char) -> Option<(T, T)> {
    let fields: Vec<&str> = s.split(sep).collect();
    if fields.len() != 2 {
        return None;
    }
    match (T::from_str(fields[0]), T::from_str(fields[1])) {
        (Ok(f0), Ok(f1)) => Some((f0, f1)),
        _ => None,
    }
}
