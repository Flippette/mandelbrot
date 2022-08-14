use rayon::prelude::*;
use std::time::Instant;

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

#[inline]
fn render(x: Float, y: Float) -> u8 {
    let c = Complex(x, y);
    let mut z = c;

    ITER_MAX
        - (1..ITER_MAX)
            .find(|_| {
                z = z.sqr() + c;
                z.0.is_nan()
            })
            .unwrap_or(ITER_MAX)
}
