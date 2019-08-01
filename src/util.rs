// Copyright © 2019 Liam Rotchford, Simon Barton

#![allow(dead_code)]

use crate::util::Color::*;
use image::*;
use std::io;
use std::str::FromStr;

/// Main mechanism for user interaction
/// Allows user to generate fractal in three ways
///
pub fn user_menu(mut scheme: &mut Scheme) {
    let mut input = String::new();
    println!("normal, custom, or random fractal generation?");
    io::stdin()
        .read_line(&mut input)
        .ok()
        .expect("Expected good input");

    let trimmed: &str = input.trim();

    match trimmed {
        "normal" => normal_menu(&mut scheme),
        "custom" => custom_menu(&mut scheme),
        "random" => _randomize(&mut scheme),
        _ => println!("Unrecognized input... running default."),
    }
}

pub fn normal_menu(mut scheme: &mut Scheme) {
    let mut input = String::new();
    if scheme.fractal == "barnsley".to_string() {
        println!("What color fractal? (ROYGBIV)");
    } else {
        println!("What color fractal? red, green, blue, or white?");
    }
    io::stdin()
        .read_line(&mut input)
        .ok()
        .expect("Expected good input");
    scheme.color = str_to_color(input.trim());
}

pub fn custom_menu(mut scheme: &mut Scheme) {
    let mut input = String::new();
    println!("What color fractal? (ROYGBIV, Black, White)");
    io::stdin()
        .read_line(&mut input)
        .ok()
        .expect("Expected good input");
    scheme.color = str_to_color(&input);
}

pub fn _randomize(_scheme: &mut Scheme) {}

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
        &_ => Blue,
    }
}

/// Iterate over the pixels of the image and apply a cool
/// background transitioning from one color to another
pub fn apply_fancy_background(imgbuf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, scheme: &Scheme) {
    let color_one: [u8; 3] = color_to_rgb(&scheme.bg_color);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {}
}

/// Iterate over the pixels of the image and apply a cool
/// background, which will depend on scheme.
/// Either transitioning from one color to another
/// or just a solid background.
pub fn apply_background(imgbuf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, scheme: &Scheme) {
    let color: [u8; 3] = color_to_rgb(&scheme.bg_color);
    let color_two: [u8; 3] = color_to_rgb(&scheme.bg_color_2);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        if scheme.fancy_background {
            *pixel = Rgb([((0.3 * x as f32) as u8), 0, ((0.3 * y as f32) as u8)]);
        }
        /* TODO
        else {
            *pixel = Rgb([color[0], color[1], color[2]]);
        }
        */
    }
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

// Helper -- map a color onto an rgb pixel
/*
pub fn paint_color(pixel: &mut image::Pixel::Rgb([u8; 3]),
                            scheme: &Scheme, result: u64) {
    let Rgb(data) = *pixel;
    match scheme.color {
        Red   => *pixel = Rgb([result as u8, data[1], data[2]]),
        Green => *pixel = Rgb([data[0], result as u8, data[2]]),
        Blue  => *pixel = Rgb([data[0], data[1], result as u8]),
        White => *pixel = Rgb([result as u8, result as u8, result as u8]),
        _ => panic!("Unsupported color"),
    }
}
*/
