# Fractal-Generator

Copyright (c) 2019 
Liam Rotchford, Simon Barton
rliam@pdx.edu, Simon5@pdx.edu

This project was inspired and based on the edited Mandelbrot image generator by Bart Massey which in turn was inspired O'Reilly Programming Rust book by Blandy and Orendorff found here: https://github.com/pdx-cs-rust/programming-rust-mandelbrot. With the insiration of Bart's project and his statment of wanting to apply color to his original project we took upon the challange ourselves. For more information on the four fractals we implemented in the project please view the sources section at the bottom of the page.

This program extends the original code supplied by Bart Massey to include RGB color schemes to the fractal images as the original code leaves it in black and white. The program also includes other fractal image choices for a variety of options and experimentation. The program will utilize the same general system for determineing the "shade" of a pixel and now apply it to the RGB color scheme included in the newest version of the image crate.

The following flags can be used to specify how an image is made
* `--Mandelbrot`: Creates the Original Mandelbrot image
* `--Julia Set`:  Creates the Julia Set fractal image
* `--Multi-Julia`: Creates a Multi-Julia or Multibrot Set fractual image
* `--Barnsley`: Creates the Barnsley fern fractal image

## Build and Run

Build this program and library with `cargo build`. You can
run the program with `cargo run`. However, to allow the program to correctly generate a image to your specifications you will need to supply the following data:
    
    <fractal_type>
    <file_name>
    <width>x<height>
   

`--` before the program flag: for example,


    cargo run -- julia julia.png 800x800

To build or run an optimized version, use `cargo --release`.

Run `cargo test` to do some simple testing. Though it should be noted that the main form of testing for this project is human visual, i.e does the image generate in the way we expect visually. 

## License

This program is licensed under the "MIT License".  Please
see the file `LICENSE` in the source distribution of this
software for license terms.

### Sources
https://en.wikipedia.org/wiki/List_of_fractals_by_Hausdorff_dimension
https://en.wikipedia.org/wiki/Barnsley_fern
https://en.wikipedia.org/wiki/Mandelbrot_set
https://en.wikipedia.org/wiki/Julia_set
https://en.wikipedia.org/wiki/Multibrot_set
