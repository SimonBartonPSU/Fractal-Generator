# Fractal-Generator

Copyright (c) 2019 
Liam Rotchford, Simon Barton
rliam@pdx.edu, Simon5@pdx.edu

## Summary
This project was inspired and based on the edited Mandelbrot image generator by Bart Massey which in turn was inspired O'Reilly Programming Rust book by Blandy and Orendorff found here: https://github.com/pdx-cs-rust/programming-rust-mandelbrot. With the insiration of Bart Massey's project and his statment of wanting to apply color to his original project, we took upon the opportunity to explore colors and new fractals ourselves. For more information on the four fractals we implemented in the project please view the sources section below.

This project implements the generation of the following set of fractals as .png or .jpeg images: Julia Set, Barnsley's Fern, Mandelbrot, Multi-Julia Set / MultiBrot Set. With a simple web UI and some patience for auto-generation the user can view all of the fractals. We have implemented 3 different coloring modes in this project: normal, custom, and random. The normal mode allows only the customization of the fractal color. Custom allows both fractal color and background color, be it one solid color or a transition between two. Random selects all characteristics for you and logs what was selected in the users /tmp/ directory, in the event you would like to examine what was performed and perhaps apply the same characteristics to a different fractal later on.

Visit https://rust-fractals.herokuapp.com/ for a demonstration!

## Fractal Types
The following fractal types are supported:
* `Mandelbrot`: Creates the Original Mandelbrot image
* `Julia`:  Creates the Julia Set fractal image (Has 10 different stages / versions)
* `Multi-Julia`: Creates a Multi-Julia / Multibrot Set fractual image (Has 6 different stages / versions)
* `Barnsley`: Creates the Barnsley fern fractal image

## Build and Run

NOTE: These steps apply only to the `rust-class-finished` branch. The current branch simply functions as a web server.

Build this program and library with `cargo build`. You can run the program with `cargo run`. However, to allow the program to correctly generate an image to your specifications you will need to supply the following data:
    
    cargo run --release -- <fractal_type> <file_name> <width>x<height>
   
For example:

    cargo run --release -- julia julia.png 800x800
    
Note: To correctly produce an image without warping, it is good practice to set the width x height as a perfect square as shown in the example above. Additionally, the image extension must be placed in the filename.
    
## Alternative Build and Run (Randomizer)
This command will allow the automatic generation of some provided number of random fractals with completely random color schemes and random image transformations. Running this option with the --release flag is highly recommended.

    cargo run --release auto-random <file-name> <number to create>

For example:

    cargo run -- auto-random a_filename 10

## Testing
Run the following command
    
    cargo test

This will cover some minor test cases / unit test throughout the program. Keep in mind the main test will be human visual, you may compare the generated image to our source links down below to determine if the fractal itself was created correctly. 

## Documentation
To read the inner documentation on each files functionality and purpose run the following command:

    cargo rustdoc --open -- --document-private-items

## Fractal Generation Examples
![Barnsley](https://i.imgur.com/KPU4MaJ.png)
![Julia](https://i.imgur.com/TzwaN9f.png)
![Mandelbrot](https://i.imgur.com/YiGrLzn.png)
![Multi-julia](https://i.imgur.com/MsDTDte.png)

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
