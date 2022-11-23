#![no_main]

use image::{ImageOutputFormat, Luma};
use rayon::prelude::*;
use std::{
    fs::File,
    io::{self, BufWriter, Write},
    time::Instant,
};

mod complex;
use complex::Complex;

const ITER_MAX: u8 = 255;

const VIEWPORT_WIDTH: i32 = 40000;
const VIEWPORT_HEIGHT: i32 = 30000;
const Y_OFFSET: i32 = 0;
const X_OFFSET: i32 = -VIEWPORT_HEIGHT / 4;
const SCALE: f32 = 0.0001;

#[no_mangle]
pub fn main() {
    let mut stderr = io::stderr().lock();

    stderr.write(b"[info] rendering started.\n").unwrap();
    let timer = Instant::now();

    let mut output = (-VIEWPORT_HEIGHT / 2..0)
        .into_par_iter()
        .flat_map_iter(|row| {
            (-VIEWPORT_WIDTH / 2 + X_OFFSET..VIEWPORT_WIDTH / 2 + X_OFFSET)
                .map(move |col| render((col + Y_OFFSET) as f32 * SCALE, row as f32 * SCALE))
        })
        .collect::<Vec<_>>();

    output.reserve_exact(VIEWPORT_WIDTH as usize * VIEWPORT_HEIGHT as usize);

    (0..VIEWPORT_HEIGHT as usize / 2).rev().for_each(|start| {
        output.extend_from_within(
            start * VIEWPORT_WIDTH as usize..(start + 1) * VIEWPORT_WIDTH as usize,
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
    let image: image::ImageBuffer<Luma<u8>, _> =
        image::ImageBuffer::from_vec(VIEWPORT_WIDTH as u32, VIEWPORT_HEIGHT as u32, output)
            .unwrap();

    image
        .write_to(&mut writer, ImageOutputFormat::Jpeg(100))
        .unwrap();

    stderr.write(b"[info] done.\n").unwrap();
}

#[inline(always)]
fn render(x: f32, y: f32) -> u8 {
    let c = Complex::new(x, y);
    let mut z2 = c;

    #[allow(unused_assignments)]
    let mut z1 = z2;

    for iter in (1..ITER_MAX).step_by(2) {
        // 1,2 -> 3,4 -> ..
        z1 = z2.sqr() + c;
        z2 = z1.sqr() + c;

        match (z1.re.is_nan(), z2.re.is_nan()) {
            (_, true) => return ITER_MAX - iter,
            (true, _) => return ITER_MAX - iter - 1,
            _ => continue,
        }
    }

    0
}
