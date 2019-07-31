// Copyright © 2019 Liam Rotchford, Simon Barton

#![allow(dead_code)]

use std::io;
use std::str::FromStr;
use crate::util::Color::*;

pub fn user_menu() -> Vec<String> {
    let mut input = String::new();
    let mut result = Vec::<String>::new();
    println!("normal, custom, or random fractal generation?");
    io::stdin().read_line(&mut input).ok().expect("Expected good input");

    result.push(input.clone());

    if input == "custom".to_string() {
        custom_menu(&mut result);
    }
    else if input == "random".to_string() {
        println!("random scheme");
        // random_scheme(),
    }
    result
}

pub fn custom_menu(result: &mut Vec<String>) {
    let mut input = String::new();
    println!("What color fractal? (ROYGBIV, Black, White)");
    io::stdin().read_line(&mut input).ok().expect("Expected good input");
    result.push(input);
}

/// Supported colors for user input
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

pub struct Scheme {
    pub color_scheme: String,
    pub color: Color,
    pub background: bool,
    pub bg_color: Color,
    // enum Transformation
}

impl Default for Scheme {
    fn default() -> Scheme {
        Scheme {
            color_scheme: "rgb".to_string(),
            color: Blue,
            background: false,
            bg_color: Black,
        }
    }
}

/// Helper to return three u8s to funcion as an RGB
pub fn color_to_rgb(color: Color) -> [u8; 3] {
    match color {
        Red    => [255, 0, 0],
        Orange => [255, 165, 0],
        Yellow => [255, 255, 0],
        Blue   => [0, 0, 255],
        Green  => [0, 128, 0],
        Violet => [238, 130, 238],
        Black  => [0, 0, 0],
        White  => [255, 255, 255],
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
