# PDX-Rust-Summer19-Mosaic-Generator

Copyright (c) 2019 Liam Rotchford, Simon Barton
Rliam@pdx.edu, Simon5@pdx.edu

This program generates an interesting, mosaic image. This was inspired
by the Mandelbrot image generator. Its input is a specific flag that
let's the program know what kind of image to generate. [TODO write out
the specific semantics of how an image is created].

The following flags can be used to specify how an image is made
* `--Mandelbrot`: Creates Mandelbrot image
* `--random`: Median

## Build and Run

Build this program and library with `cargo build`. You can
run the program with `cargo run`. You will need to pass a
`--` before the program flag: for example,

    cargo run -- --mean

To build or run an optimized version, use `cargo --release`.

Run `cargo test` to do some simple testing.

## License

This program is licensed under the "MIT License".  Please
see the file `LICENSE` in the source distribution of this
software for license terms.
