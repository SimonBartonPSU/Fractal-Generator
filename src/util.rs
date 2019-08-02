// Copyright © 2019 Liam Rotchford, Simon Barton

#![allow(dead_code)]

use crate::util::Color::*;
use image::*;
use std::str::FromStr;

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
    // enum Transformation
}

impl Default for Scheme {
    fn default() -> Scheme {
        Scheme {
            fractal: "mandelbrot".to_string(),
            color: Green,
            fancy_background: false,
            bg_color: Black,
            bg_color_2: Red,
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
pub fn process_image(filename: &str, transformation: &str) { 
    let image = image::open(filename).unwrap();
    
    match transformation {
        "blur" => image::imageops::blur(&image, 0.9_f32),
        "brighten" => image::imageops::brighten(&image, 0),
        &_ => image::imageops::blur(&image, 0.9_f32),
    };
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
