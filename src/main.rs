#![no_main]

use clap::Parser;
use image::{ImageOutputFormat, Luma};
use rayon::prelude::*;
use std::{
    fs::File,
    io::{self, BufWriter, Write},
    time::Instant,
};

mod complex;
use complex::Complex;

#[derive(Debug, Parser)]
struct Config {
    #[arg(long, default_value_t = 8000)]
    viewport_width: i32,

    #[arg(long, default_value_t = 6000)]
    viewport_height: i32,

    #[arg(long, allow_negative_numbers = true, default_value_t = 0)]
    x_offset: i32,

    #[arg(long, allow_negative_numbers = true, default_value_t = -2000)]
    y_offset: i32,

    #[arg(long, default_value_t = 0.0005)]
    scale: f32,

    #[arg(long, default_value_t = 255)]
    depth_max: u8,
}

#[no_mangle]
pub fn main() {
    let config = Config::parse();

    let mut stderr = io::stderr().lock();

    stderr.write(b"[info] rendering started.\n").unwrap();
    let timer = Instant::now();

    let mut output = (-config.viewport_height / 2..0)
        .into_par_iter()
        .flat_map_iter(|row| {
            (-config.viewport_width / 2 + config.x_offset
                ..config.viewport_width / 2 + config.x_offset)
                .map(move |col| {
                    render(
                        (col + config.y_offset) as f32 * config.scale,
                        row as f32 * config.scale,
                        config.depth_max,
                    )
                })
        })
        .collect::<Vec<_>>();

    output.reserve_exact(config.viewport_width as usize * config.viewport_height as usize);

    (0..config.viewport_height as usize / 2)
        .rev()
        .for_each(|start| {
            output.extend_from_within(
                start * config.viewport_width as usize
                    ..(start + 1) * config.viewport_width as usize,
            )
        });

    stderr.write(b"[info] rendering took ").unwrap();
    stderr
        .write(timer.elapsed().as_secs_f32().to_string().as_bytes())
        .unwrap();
    stderr
        .write(b" seconds, writing to output file...\n")
        .unwrap();

    let mut writer = BufWriter::new(File::create("image.jpeg").unwrap());

    image::ImageBuffer::<Luma<u8>, _>::from_vec(
        config.viewport_width as u32,
        config.viewport_height as u32,
        output,
    )
    .unwrap()
    .write_to(&mut writer, ImageOutputFormat::Jpeg(100))
    .unwrap();

    stderr.write(b"[info] done.\n").unwrap();
}

#[inline(always)]
fn render(x: f32, y: f32, depth_max: u8) -> u8 {
    let c = Complex::new(x, y);
    let mut z2 = c;

    #[allow(unused_assignments)]
    let mut z1 = z2;

    for iter in (1..depth_max).step_by(2) {
        // 1,2 -> 3,4 -> ..
        z1 = z2.sqr() + c;
        z2 = z1.sqr() + c;

        match (z1.re.is_nan(), z2.re.is_nan()) {
            (_, true) => return depth_max - iter,
            (true, _) => return depth_max - iter - 1,
            _ => continue,
        }
    }

    0
}
