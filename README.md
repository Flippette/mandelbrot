# mandelbrot
Simple Mandelbrot fractal renderer.

## Build instructions

Double precision (default):

    cargo build [--release]
    cargo run [--release]

Single precision:

    cargo build [--release] --no-default-features --features single
    cargo run [--release] --no-default-features --features single
