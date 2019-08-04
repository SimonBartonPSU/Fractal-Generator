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
    \t\t2) Custom: User choice colored background\n
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
        input = color_options_extensive();

    } else {
        scheme_type = false;
        input = color_options_rgb();
    }

    input = color_determine(input, scheme_type);

    let color: &str = &input;

    scheme.color = str_to_color(color);

    println!("\n========================================================================================================================================\n");
}

/// The custom option allows a user to fine tune properties
/// of the fractal art image.
pub fn custom_menu(mut scheme: &mut Scheme) {
    
    let mut input = String::new();  
    
    print!(
        "\n\no BACKGROUND STYLE MENU: \n
        \to Which background coloring style would you like? Please select from the options below \n
        \t\t1) Solid: solid colored background\n
        \t\t2) Transitional: multiple color transition background\n
        \to Input: "
    );
   
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).ok();
    let mut trimmed: &str = &input.trim().to_lowercase();
    
    println!("\n========================================================================================================================================\n");

    match trimmed {
        "1" => {

            println!("\n\no SOLID BACKGROUND COLOR MENU: \n
                \to What solid color background would you like? Please select from the options below \n ");
            
            input = color_options_extensive();

            input = color_determine(input, true);

            let color: &str = &input;

            scheme.bg_color = str_to_color(color);
        }

        "2" => {
            scheme.fancy_background = true;

            input.clear();

            println!("Choose your first color: red, green, blue: ");

            io::stdin().read_line(&mut input).ok();

            scheme.bg_color = str_to_color(input.trim());

            input.clear();

            println!("Choose one of the remaining two: red, green, blue: ");

            io::stdin().read_line(&mut input).ok();

            scheme.bg_color_2 = str_to_color(input.trim());

            println!("Assummed good input...");
        }

        
        _ => println!("Invalid input: {:?}. Enter a number (1, 2, ..)", input),
    }
        
    input.clear();
}

pub fn color_options_extensive() -> String {
    let mut input = String::new();  

    print!("
        \t1) Red\n
        \t2) Orange\n
        \t3) Yellow\n
        \t4) Green\n
        \t5) Blue\n
        \t6) Violet\n
        \t7) White\n
        o Input: "
    );
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).ok();
    
    input.to_string()
}

pub fn color_options_rgb() -> String {
    let mut input = String::new();  

    print!("
        \t1) Red\n
        \t2) Green\n
        \t3) Blue\n
        \t4) White\n
        o Input: "
    );
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).ok();
    
    input.to_string()
}

pub fn color_determine(input: String, scheme_type: bool) -> String {

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

    trimmed.to_string()
}

pub fn _randomize(_scheme: &mut Scheme) {}
