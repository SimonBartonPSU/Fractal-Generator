// Copyright © 2019 Liam Rotchford, Simon Barton

#![allow(dead_code)]

use crate::util::Color::*;
use image::imageops::*;
use image::*;
use rand::Rng;
use std::str::FromStr;

const SMOOTH_KERNEL: [f32; 9] = [1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0];
const SHARPEN_KERNEL: [f32; 9] = [-1.0, -1.0, -1.0, -1.0, 9.0, -1.0, -1.0, -1.0, -1.0];
const RAISED_KERNEL: [f32; 9] = [0.0, 0.0, -2.0, 0.0, 2.0, 0.0, 1.0, 0.0, 0.0];
const COLORS: [&str; 8] = [
    "red", "blue", "green", "orange", "yellow", "violet", "black", "white",
];
const TRANSFORMS: [&str; 11] = [
    "blur",
    "brighten",
    "contrast",
    "huerotate",
    "invert",
    "rotate90",
    "rotate180",
    "rotate270",
    "smooth filter",
    "sharpen filter",
    "raised filter",
];

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
pub struct Scheme {
    pub fractal: String,
    /// Actual color of the fractal
    pub color: Color,
    pub fancy_background: bool,
    pub bg_color: Color,
    pub bg_color_2: Color,
    pub random: bool,
}

impl Default for Scheme {
    fn default() -> Scheme {
        Scheme {
            fractal: "mandelbrot".to_string(),
            color: Green,
            fancy_background: false,
            bg_color: Black,
            bg_color_2: Red,
            random: false,
        }
    }
}

/// Helper to return three u8s based on parsed color
/// u8s function as RGB data
pub fn color_to_rgb(color: &Color) -> [u8; 3] {
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
pub fn apply_background(imgbuf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, scheme: &Scheme) {
    let color: [u8; 3] = color_to_rgb(&scheme.bg_color);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let xc: u8 = (0.3 * x as f32) as u8;
        let yc: u8 = (0.3 * y as f32) as u8;
        if scheme.fancy_background {
            match scheme.bg_color {
                Red => match scheme.bg_color_2 {
                    Blue => *pixel = Rgb([xc, 0, yc]),
                    Green => *pixel = Rgb([xc, yc, 0]),
                    _ => println!("Unsupported bg_color_2"),
                },
                Green => match scheme.bg_color_2 {
                    Blue => *pixel = Rgb([0, xc, yc]),
                    Red => *pixel = Rgb([xc, yc, 0]),
                    _ => println!("Unsupported bg_color_2"),
                },
                Blue => match scheme.bg_color_2 {
                    Red => *pixel = Rgb([xc, 0, yc]),
                    Green => *pixel = Rgb([0, xc, yc]),
                    _ => println!("Unsupported bg_color_2"),
                },
                _ => println!("Unsupported bg_color"),
            }
        } else {
            *pixel = Rgb([color[0], color[1], color[2]]);
        }
    }
}

/// Image processing functions supplied by image crate
/// to be used for fun and randomization
/// imageops functions

/// Range for blur 0.75 to 5.0
/// Range for brighten -50 to 80
/// Range for contrast -20.0 to 200.0
/// Range for huerotate 5 to 355
pub fn process_image(filename: &str, transformation: &str) {
    let mut image = image::open(filename).unwrap();

    match transformation {
        "blur" => blur(&image, 2.0_f32).save(filename).unwrap(),
        "brighten" => brighten(&image, 50).save(filename).unwrap(),
        "contrast" => contrast(&image, 40.0_f32).save(filename).unwrap(),
        "huerotate" => huerotate(&image, 270).save(filename).unwrap(),
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

pub fn randomize(scheme: &mut Scheme) {
    scheme.random = true;
    let fractal_num;
    if scheme.fractal == "barnsley".to_string() {
        fractal_num = rand::thread_rng().gen_range(0, 8);
    } else {
        fractal_num = rand::thread_rng().gen_range(0, 3);
    }
    scheme.color = str_to_color(COLORS[fractal_num]);
    let bg_num = rand::thread_rng().gen_range(0, 8);
    scheme.bg_color = str_to_color(COLORS[bg_num]);
}

pub fn random_transforms(filename: &str) {
    let transform_num = rand::thread_rng().gen_range(0, 9);
    let transform = TRANSFORMS[transform_num];
    process_image(filename, transform);
}

/// Helper to parse a string as a pair of values separated
/// by a separator char.
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
