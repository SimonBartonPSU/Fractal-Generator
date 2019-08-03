// Copyright © 2019 Liam Rotchford, Simon Barton

use crate::util::*;
use std::io;

/// Main mechanism for user interaction
/// Allows user to generate fractal in three ways
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
        "custom" => {
            println!("We're glad you chose customization message\n\n");
            normal_menu(&mut scheme);
            custom_menu(&mut scheme)
        }
        "random" => randomize(&mut scheme),
        _ => println!("Unrecognized input... running default."),
    }
}

/// The normal option allows a user to select only the color
/// of the fractal they will generate.
pub fn normal_menu(mut scheme: &mut Scheme) {
    let mut input = String::new();
    if scheme.fractal == "barnsley".to_string() {
        println!("What color fractal? (ROYGBIV)");
    } else {
        println!("What color fractal? red, green, blue, or white?");
    }
    io::stdin().read_line(&mut input).ok();
    scheme.color = str_to_color(input.trim());
}

/// The custom option allows a user to fine tune properties
/// of the fractal art image.
pub fn custom_menu(mut scheme: &mut Scheme) {
    let mut buffer = String::new();
    let std = io::stdin();

    let mut finished: bool = false;
    while !finished {
        println!("Select an item to customize by its ID number:");
        println!("    1. Background color (solid)\n    2. Background color (transition)\n    'quit' to quit\n");
        std.read_line(&mut buffer).ok();

        match buffer.trim() {
            "1" => {
                scheme.fancy_background = false;
                buffer.clear();
                println!("What color background would you like? ");
                io::stdin().read_line(&mut buffer).ok();
                scheme.bg_color = str_to_color(buffer.trim());
            }
            "2" => {
                scheme.fancy_background = true;
                buffer.clear();
                println!("Choose your first color: red, green, blue: ");
                io::stdin().read_line(&mut buffer).ok();
                scheme.bg_color = str_to_color(buffer.trim());
                buffer.clear();
                println!("Choose one of the remaining two: red, green, blue: ");
                io::stdin().read_line(&mut buffer).ok();
                scheme.bg_color_2 = str_to_color(buffer.trim());
                println!("Assummed good input...");
            }
            "quit" => finished = true,
            _ => println!("Invalid input: {:?}. Enter a number (1, 2, ..)", buffer),
        }
        buffer.clear();
    }
}
