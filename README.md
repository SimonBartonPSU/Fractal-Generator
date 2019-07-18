# Fractal-Generator

Copyright (c) 2019 
Liam Rotchford, Simon Barton
rliam@pdx.edu, Simon5@pdx.edu

This program generates fractal .png image patterns based on the list of fractals by Hausdorff dimension located here: https://en.wikipedia.org/wiki/List_of_fractals_by_Hausdorff_dimension  This project was inspired and based on the edited Mandelbrot image generator by Bart Massey which in turn was inspired O'Reilly Programming Rust book by Blandy and Orendorff. The program determines what fractal image to generate based on input flags listed below.

This program extends the original code supplied by Bart Massey to include RGB color schemes to the fractal images as the original code leaves it in black and white. The program also includes other fractal image choices for a variety of options and experimentation. The program will utilize the same general system for determineing the "shade" of a pixel and now apply it to the RGB color scheme included in the newest version of the image crate.

The following flags can be used to specify how an image is made
* `--Mandelbrot`:   Creates the Original Mandelbrot image
* `--Dragoncurve`:  Creates the Dragoncurve fractal image
* `--LevyCcurve`:   Creates the Levy C curve fractual image

## Build and Run

Build this program and library with `cargo build`. You can
run the program with `cargo run`. However, to allow the program to correctly generate a image to your specifications you will need to supply the following data:
    
    <fractal_type>
    <file>
    <width>x<height>

`--` before the program flag: for example,

    cargo run --release mandelbrot.png 1000x1000 
    

To build or run an optimized version, use `cargo --release`.

Run `cargo test` to do some simple testing.

## License

This program is licensed under the "MIT License".  Please
see the file `LICENSE` in the source distribution of this
software for license terms.
