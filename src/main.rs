// Copyright © 2019 Liam Rotchford, Simon Barton

// Originally inspired by the
// Mandelbrot example from Blandy & Orendorff, ch 1.
// I/O Help
// https://www.reddit.com/r/rust/comments/41hgwq/help_needed_user_input_in_rust/

//! Generate a fractal based on command line args and user input

#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod auto_random;
mod barnsley;
mod julia_sets;
mod julias;
mod mandelbrot;
mod menu;
mod util;

use crate::auto_random::*;
use rocket_contrib::templates::Template;

/// Start a web server and display a cool fractal on root page
/// Allow user to send a GET to create a new fractal as much as they want
#[get("/", format = "text/html")]
fn index() -> Template {
    let context = TemplateContext();
    Template::render("index", &context)
}

#[get("/gen")]
fn generate() -> Template {
    auto_random(1, "cool");
    let context = TemplateContext();
    Template::render("gen", &context)
}

fn main() {
    rocket::ignite().mount("/", rocket::routes![index, generate])
    .attach(Template::fairing())
    .launch();
}
