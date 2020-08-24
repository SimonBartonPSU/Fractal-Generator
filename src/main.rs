// Copyright © 2019 Liam Rotchford, Simon Barton

// Originally inspired by the
// Mandelbrot example from Blandy & Orendorff, ch 1.
// I/O Help
// https://www.reddit.com/r/rust/comments/41hgwq/help_needed_user_input_in_rust/

//! Display fractals on a webpage using Rust

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
use rocket::response::NamedFile;
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use std::path::Path;

/// Start a web server and display a cool fractal on root page
/// Allow user to send a GET to create a new fractal as much as they want
#[get("/", format = "text/html")]
fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("foo".to_string(), "bar".to_string());
    Template::render("index", &context)
}

#[get("/gen")]
fn generate() -> Template {
    auto_random(1, "cool");
    let mut context = HashMap::new();
    context.insert("foo".to_string(), "baz".to_string());
    Template::render("gen", &context)
}

#[get("/cool.png")]
fn cool() -> Option<NamedFile> {
    let path = Path::new("resources/cool.png");
    NamedFile::open(&path).ok()
}

#[get("/cool0.png")]
fn cool0() -> Option<NamedFile> {
    let path = Path::new("cool0.png");
    NamedFile::open(&path).ok()
}

#[get("/resources/cool.css", format = "text/css")]
fn style() -> Option<NamedFile> {
    let path = Path::new("resources/cool.css");
    NamedFile::open(&path).ok()
}

#[get("/favicon.ico")]
fn favicon() -> Option<NamedFile> {
    let path = Path::new("favicon.ico");
    NamedFile::open(&path).ok()
}

fn main() {
    rocket::ignite()
        .mount("/", rocket::routes![index, generate, cool, cool0, style, favicon])
        .attach(Template::fairing())
        .launch();
}
