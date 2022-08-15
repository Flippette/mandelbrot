use rayon::prelude::*;
use std::{mem, time::Instant};

mod complex;
use complex::{Complex, Float};

const ITER_MAX: u8 = 255;

fn main() {
    let viewport_width: i32 = 8000;
    let viewport_height: i32 = 6000;
    let y_offset: i32 = 0;
    let x_offset: i32 = -viewport_width / 4;
    let scale: Float = 0.0005;

    eprintln!("[info] rendering started.");
    let timer = Instant::now();

    let output = (-viewport_height / 2 + y_offset..viewport_height / 2 + y_offset)
        .into_par_iter()
        .flat_map_iter(|row| {
            (-viewport_width / 2 + x_offset..viewport_width / 2 + x_offset)
                .map(move |col| render(col as Float * scale, row as Float * scale))
        })
        .collect::<Vec<u8>>();

    eprintln!(
        "[info] rendering took {:.4} seconds, writing to output file...",
        timer.elapsed().as_secs_f32()
    );

    image::save_buffer(
        "./image.png",
        &output,
        viewport_width as u32,
        viewport_height as u32,
        image::ColorType::L8,
    )
    .unwrap();

    eprintln!("[info] done.");
}

#[inline(always)]
fn render(x: Float, y: Float) -> u8 {
    let c = Complex(x, y);
    let mut z = c;

    #[allow(unused_assignments)]
    let mut last_z = z;

    for iter in (1..ITER_MAX).step_by(2) {
        let z_clone = z;
        last_z = mem::replace(&mut z, z_clone.sqr() + c);
        if z.0.is_nan() {
            return ITER_MAX - iter - if last_z.0.is_nan() { 1 } else { 0 };
        }
    }

    0
}
