// Copyright © 2019 Liam Rotchford, Simon Barton

use crate::util::*;
use std::io;
use std::io::Write;

/// Main mechanism for user interaction
/// Allows user to generate fractal in three ways
pub fn user_menu(mut scheme: &mut Scheme) {
    let mut input = String::new();

    print!(
        "
    \n\nUSER MENU: \n
    \to What type of background would you like? Please select from the following options. \n
    \t\t1) Normal: Simple black background\n
    \t\t2) Custom: User choice color background\n
    \t\t3) Random: Randomly generated background\n
    \to Input: "
    );
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Expected good input");

    let trimmed: &str = &input.trim().to_lowercase();

    println!("\n========================================================================================================================================\n");

    match trimmed {
        "normal" | "1" => normal_menu(&mut scheme),
        "custom" | "2" => {
            println!("We're glad you chose customization message\n\n");
            normal_menu(&mut scheme);
            custom_menu(&mut scheme)
        }
        "random" | "3" => _randomize(&mut scheme),
        _ => println!("Unrecognized input... running default."),
    }
}

/// The normal option allows a user to select only the color
/// of the fractal they will generate.
pub fn normal_menu(mut scheme: &mut Scheme) {
    let mut input = String::new();
    let scheme_type;
    println!(
        "\n\no FRACTAL COLOR MENU: \n
        o What color would you like the fractal to be? Please select from the following option. \n"
    );

    if scheme.fractal == "barnsley" {
        scheme_type = true;
        print!(
            "
        \t1) Red\n
        \t2) Orange\n
        \t3) Yellow\n
        \t4) Green\n
        \t5) Blue\n
        \t6) Violet\n
        \t7) White\n
        o Input: "
        );
    } else {
        scheme_type = false;
        print!(
            "
        \t1) Red\n
        \t2) Green\n
        \t3) Blue\n
        \t4) White\n
        o Input: "
        );
    }

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).ok();

    let mut trimmed: &str = &input.trim().to_lowercase();

    trimmed = match trimmed {
        "1" => "red",
        "2" => {
            if scheme_type {
                "orange"
            } else {
                "green"
            }
        }
        "3" => {
            if scheme_type {
                "yellow"
            } else {
                "blue"
            }
        }
        "4" => {
            if scheme_type {
                "green"
            } else {
                "white"
            }
        }
        "5" => "blue",
        "6" => "violet",
        "7" => "white",
        _ => trimmed, //if not a numeric char then just keep input as is
    };

    scheme.color = str_to_color(trimmed);

    println!("\n========================================================================================================================================\n");
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

pub fn _randomize(_scheme: &mut Scheme) {}
