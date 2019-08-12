// Copyright © 2019 Liam Rotchford, Simon Barton

///This file contains all user interaction (UI) menus to utilize the fractal generator program. Covering user input for fractal color,
///background colors, background color styling, image trasformation additions, and general user notification on the process of the program.
use crate::util::*;
use std::io;
use std::io::Write;

///The user_menu function is the first entry point for the user on the firstly deciding on the initial style of background they will wish to use,
///normal which limits you to only changing the background color on a black background, custom that allows the fractal color, background color & styling,
///and random which gives no user customization and instead randomly selects the input styling attributes and creates a random styled fractal
///of which can be determined in its log file. Reference README.md
pub fn user_menu(mut scheme: &mut Scheme) {
    let mut input = String::new();

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

    let trimmed: &str = &input.trim().to_lowercase();

    println!("\n========================================================================================================================================\n");

    match trimmed {
        "normal" | "1" => normal_menu(&mut scheme),
        "custom" | "2" => {
            normal_menu(&mut scheme);
            custom_menu(&mut scheme)
        }
        "random" | "3" => randomize(&mut scheme),
        _ => println!("Unrecognized input... running default."),
    }

    println!("\n\to Constructing your fractal image");
}

///The normal menu option allows a user to select only the color of the fractal they will generate upon a black background.
///It should be known that the julia sets, multi julia sets, and mandelbrot fractals are given a limited fractal color optionality due to implementation constrictions.
pub fn normal_menu(mut scheme: &mut Scheme) {
    let mut input;
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

///The custom menu option allows a user to select the background styleing between solid color or transitional between two colors of which are user selected.
///After the user moves through those options they are also prompted if they wish to apply image atrributes which can alter the final image.
///We have a default set to just make the background a solid black for unrecognized input.
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

            scheme.bg_color = str_to_color(&input);
        }

        "2" | "transition" => {
            scheme.fancy_background = true;

            input.clear();
            println!(
                "\n\no TRANSITIONAL BACKGROUND COLOR MENU: \n
            o Please select one of the following colors: "
            );

            input = color_options_rgb(true);
            input = color_determine(input, false);

            scheme.bg_color = str_to_color(&input);

            input.clear();

            println!("\n\to Choose one color that is not the same as the first: ");

            input = color_options_rgb(true);
            input = color_determine(input, false);

            scheme.bg_color_2 = str_to_color(&input);
        }

        _ => {
            println!("Unrecognized input... running default black background.");

            let color: &str = "black";

            scheme.bg_color = str_to_color(color);
        }
    }

    println!("\n\to Would you like to perform a single transform? (y/n)");
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).ok();
    let trimmed: &str = &input.trim().to_lowercase();

    if trimmed == "y" || trimmed == "yes" {
        scheme.do_transform = true;
        transform_options(&mut scheme);
    }

    println!("\n========================================================================================================================================\n");
}

///color_options_extensive is utilized to print the repeativly used color print options based on the fractal type. The extensive menu is used
///for the barnsley fractal color and the background colors for the solid background on the custom menu.

pub fn color_options_extensive() -> String {
    let mut input = String::new();

    print!(
        "
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

    if trimmed == "1"
        || trimmed == "2"
        || trimmed == "3"
        || trimmed == "4"
        || trimmed == "5"
        || trimmed == "6"
        || trimmed == "7"
        || trimmed == "8"
    {
        input.to_string()
    } else {
        println!("\n\to Non-allowed option selected, running default color RED \n");
        "1".to_string()
    }
}

///color_options_rgb is used for the julia, multi-julia set, and mandelbrot fractal colors, and the transitional background style menu
pub fn color_options_rgb(transitional: bool) -> String {
    let mut input = String::new();

    if transitional {
        print!(
            "
        \t1) Red\n
        \t2) Green\n
        \t3) Blue\n
        o Input: "
        );
    } else {
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

    let trimmed: &str = &input.trim().to_lowercase();

    if trimmed == "1" || trimmed == "2" || trimmed == "3" || trimmed == "4" {
        input.to_string()
    } else {
        println!("\n\to Non-allowed option selected, running default color RED \n");
        "1".to_string()
    }
}

///color_determine returns the str to match on what option the user selected in the menus, typically by number, can converts it to the appropriate
///string to match the color on the Color enum in util.rs. This way the user wont have to write out the name of the color each time and avoid common
///typo error that would direct into defaulted match for error handling.
pub fn color_determine(input: String, scheme_type: bool) -> String {
    let mut trimmed: &str = &input.trim().to_lowercase();

    trimmed = match trimmed {
        //if solid case -> used colors_options_extensive then scheme type is true for matching
        "1" => "red", //else if normal or transitional cases -> used color_options_rgb and scheme type is false for the shorter list
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

///transform_options is just the prompt and user input menu if the user wishes to apply any image transformations to the fractal.
fn transform_options(scheme: &mut Scheme) {
    let mut input = String::new();
    println!(
        "\n\no TRANSFORMATIONS MENU: \n
        o What transformation would you like to perform? Please select from the following option. \n"
    );

    print!(
        "
        \t1) brighten\n
        \t2) constrast\n
        \t3) huerotate\n
        \t4) invert\n
        \t5) rotate90\n
        \t6) rotate180\n
        \t7) rotate270\n
        \t8) blur\n
        \t9) smooth filter\n
        \t10) sharpen filter\n
        \t11) raised filter\n
        
        
        o Input: "
    );

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).ok();

    let trimmed: &str = &input.trim().to_lowercase();

    let transform: &str = match trimmed {
        "1" | "brighten" => "brighten",
        "2" | "contrast" => "contrast",
        "3" | "huerotate" => "huerotate",
        "4" | "invert" => "invert",
        "5" | "rotate90" => "rotate90",
        "6" | "rotate180" => "rotate180",
        "7" | "rotate270" => "rotate270",
        "8" | "blur" => "blur",
        "9" | "smooth filter" => "smooth filter",
        "10" | "sharpen filter" => "sharpen filter",
        "11" | "raised filter" => "raised filter",
        &_ => "contrast",
    };

    scheme.transform = transform.to_string();
    println!("\n========================================================================================================================================\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_determine_number_input() {
        assert_eq!(
            "violet".to_string(),
            color_determine("6".to_string(), false)
        );
    }

    #[test]
    fn test_color_determine_caps_word_input() {
        assert_eq!(
            "blue".to_string(),
            color_determine("BLUE".to_string(), false)
        );
    }

    #[test]
    fn test_color_determine_bad_input() {
        assert_eq!("huh".to_string(), color_determine("Huh".to_string(), false));
    }
}
