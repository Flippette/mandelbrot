#![feature(slice_as_chunks)]

use rayon::prelude::*;
use std::{mem, time::Instant};

mod complex;
use complex::Complex;

const ITER_MAX: u8 = 255;

const VIEWPORT_WIDTH: i32 = 40000;
const VIEWPORT_HEIGHT: i32 = 30000;
const Y_OFFSET: i32 = 0;
const X_OFFSET: i32 = -VIEWPORT_HEIGHT / 4;
const SCALE: f32 = 0.0001;

fn main() {
    eprintln!("[info] rendering started.");
    let timer = Instant::now();

    let upper_half = (-VIEWPORT_HEIGHT / 2..0)
        .into_par_iter()
        .flat_map_iter(|row| {
            (-VIEWPORT_WIDTH / 2 + X_OFFSET..VIEWPORT_WIDTH / 2 + X_OFFSET)
                .map(move |col| render((col + Y_OFFSET) as f32 * SCALE, row as f32 * SCALE))
        })
        .collect::<Vec<u8>>();

    let upper_half = upper_half.as_chunks::<{ VIEWPORT_WIDTH as usize }>();
    let lower_half = upper_half.0.iter().rev().flatten();

    let mut output = Vec::with_capacity(VIEWPORT_WIDTH as usize * VIEWPORT_HEIGHT as usize);

    output.extend(upper_half.0.iter().flatten());
    output.extend(upper_half.1.iter());
    output.extend(lower_half);

    eprintln!(
        "[info] rendering took {:.4} seconds, writing to output file...",
        timer.elapsed().as_secs_f32()
    );

    image::save_buffer(
        "./image.png",
        &output,
        VIEWPORT_WIDTH as u32,
        VIEWPORT_HEIGHT as u32,
        image::ColorType::L8,
    )
    .unwrap();

    eprintln!("[info] done.");
}

#[inline(always)]
fn render(x: f32, y: f32) -> u8 {
    let c = Complex::new(x, y);
    let mut z = c;

    #[allow(unused_assignments)]
    let mut last_z = z;

    for iter in (1..ITER_MAX).step_by(2) {
        let z_clone = z;
        last_z = mem::replace(&mut z, z_clone.sqr() + c);
        if z.re.is_nan() {
            return ITER_MAX - iter - last_z.re.is_nan() as u8;
        }
    }

    0
}
