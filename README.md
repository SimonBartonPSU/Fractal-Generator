# Fractal-Generator

Copyright (c) 2019 
Liam Rotchford, Simon Barton
rliam@pdx.edu, Simon5@pdx.edu

This project was inspired and based on the edited Mandelbrot image generator by Bart Massey which in turn was inspired O'Reilly Programming Rust book by Blandy and Orendorff found here: https://github.com/pdx-cs-rust/programming-rust-mandelbrot. With the insiration of Bart's project and his statment of wanting to apply color to his original project we took upon the challange ourselves. For more information on the four fractals we implemented in the project please view the sources section at the bottom of the page.




The following fractal types are supported:
* `Mandelbrot`: Creates the Original Mandelbrot image
* `Julia`:  Creates the Julia Set fractal image
* `Multi-Julia`: Creates a Multi-Julia / Multibrot Set fractual image
* `Barnsley`: Creates the Barnsley fern fractal image

## Build and Run

Build this program and library with `cargo build`. You can run the program with `cargo run`. However, to allow the program to correctly generate a image to your specifications you will need to supply the following data:
    
    cargo run -- <fractal_type> <file_name> <width>x<height>
   
For example:

    cargo run -- julia julia.png 800x800
    
Note: To correctly produce a image without warping it is good practice to set the <width>x<height> as a perfect square as shown in the example above. If you adjust the dimensions vary harshly away from a square you will not see results as expected. 
    
## Alternative Build and Run (Randomizer)

    cargo run -- auto-random <file-name> <number to create>

For example:

    cargo run -- auto-random a_filename 10

To build or run an optimized version, use `cargo --release`.

Run `cargo test` to do some simple testing. Though it should be noted that the main form of testing for this project is human visual, i.e does the image generate in the way we expect visually. 

## Fractal Generation Examples
![Barnsley](https://i.imgur.com/KPU4MaJ.png)
![Julia](https://i.imgur.com/TzwaN9f.png)
![Mandelbrot](https://i.imgur.com/YiGrLzn.png)

## License

This program is licensed under the "MIT License".  Please
see the file `LICENSE` in the source distribution of this
software for license terms.

## Sources
https://en.wikipedia.org/wiki/List_of_fractals_by_Hausdorff_dimension  
https://en.wikipedia.org/wiki/Barnsley_fern  
https://en.wikipedia.org/wiki/Mandelbrot_set  
https://en.wikipedia.org/wiki/Julia_set  
https://en.wikipedia.org/wiki/Multibrot_set  
