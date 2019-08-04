// Copyright © 2019 Liam Rotchford, Simon Barton

use crate::util::*;
use std::io;
use std::io::Write;

/// Main mechanism for user interaction
/// Allows user to generate fractal in three ways
pub fn user_menu(mut scheme: &mut Scheme) {
    let mut input = String::new();

    if scheme.fractal == "barnsley" {
        print!(
            "
        \n\nUSER MENU: \n
        o What type of background would you like? Please select from the following options. \n
        \t1) Normal: Simple black background\n
        \t2) Custom: User choice colored background\n
        \t3) Random: Random properties of fractal fractal\n
        o Input: "
        );

        io::stdout().flush().unwrap();
        io::stdin()
        .read_line(&mut input)
        .expect("Expected good input");
    } else {
        input = "normal".to_string();
    }
    
    let trimmed: &str = &input.trim().to_lowercase();

    println!("\n========================================================================================================================================\n");

    match trimmed {
        "normal" | "1" => normal_menu(&mut scheme),
        "custom" | "2" => {
            normal_menu(&mut scheme);
            custom_menu(&mut scheme)
        }
        "random" | "3"  => randomize(&mut scheme),
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
        input = color_options_rgb(false);
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
        o Which background coloring style would you like? Please select from the options below \n
        \t1) Solid: solid colored background\n
        \t2) Transitional: multiple color transition background\n
        o Input: "
    );
   
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).ok();
    let trimmed: &str = &input.trim().to_lowercase();
    
    println!("\n========================================================================================================================================\n");

    match trimmed {
        "1" | "solid" => {          
            scheme.fancy_background = false;

            println!("\n\no SOLID BACKGROUND COLOR MENU: \n
            o What solid color background would you like? Keep in mind some fractal colors are easier to see on certain colors\n ");
            
            input = color_options_extensive();
            input = color_determine(input, true);

            let color: &str = &input;

            scheme.bg_color = str_to_color(color);
        }

        "2" | "transition" => {         
            scheme.fancy_background = true;

            input.clear();
            println!("\n\no TRANSITIONAL BACKGROUND COLOR MENU: \n
            o Please select one of the following colors: ");

            input = color_options_rgb(true);
            input = color_determine(input, false);

            let mut color: &str = &input;

            scheme.bg_color = str_to_color(color);

            input.clear();

            println!("\n\to Choose one color that is not the same as the first: ");

            input = color_options_rgb(true);
            input = color_determine(input, false);
            color = &input;

            scheme.bg_color_2 = str_to_color(color);

        }

        _ => {
            println!("Unrecognized input... running default black background.");

            let color: &str = "black";

            scheme.bg_color = str_to_color(color);
        }
    }

    println!("\n========================================================================================================================================\n");
}

//User Prompts and Match statements for color

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
        \t8) Black\n
        o Input: "
    );
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).ok();

    let trimmed: &str = &input.trim().to_lowercase();

    if trimmed == "1" || trimmed == "2" || trimmed == "3" || trimmed == "4" || trimmed == "5" || trimmed == "6" || trimmed == "7" || trimmed == "8" {
       input.to_string()

    } else {
        println!("\n\to Non-allowed option selected, running default color RED \n");
        "1".to_string()
    } 
}

pub fn color_options_rgb(transitional: bool) -> String {
    let mut input = String::new();  

    if transitional {
        print!("
        \t1) Red\n
        \t2) Green\n
        \t3) Blue\n
        o Input: "
        );
    } else {
        print!("
        \t1) Red\n
        \t2) Green\n
        \t3) Blue\n
        \t4) White\n
        o Input: "
        );
    }
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).ok();

    let trimmed: &str = &input.trim().to_lowercase();
    
    if trimmed == "1" || trimmed == "2" || trimmed == "3" || trimmed != "4" {
        input.to_string()

    } else {
        println!("\n\to Non-allowed option selected, running default color RED \n");
        "1".to_string()
    }
}

pub fn color_determine(input: String, scheme_type: bool) -> String {

    let mut trimmed: &str = &input.trim().to_lowercase();

    trimmed = match trimmed {   //if solid case -> used colors_options_extensive then scheme type is true for matching
        "1" => "red",           //else if normal or transitional cases -> used color_options_rgb and scheme type is false for the shorter list
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
        "8" => "black",
        _ => trimmed, //if not a numeric char then just keep input as is
    };

    trimmed.to_string()
}
