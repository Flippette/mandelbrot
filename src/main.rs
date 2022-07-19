use anyhow::Result;
use image::EncodableLayout;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{ops::Range, time::Instant};

mod complex;
use complex::Complex;

const ITER_MAX: usize = 255;

fn main() -> Result<()> {
    let viewport_width: i32 = 8000;
    let viewport_height: i32 = 6000;
    let y_offset = 0;
    let x_offset = -viewport_width / 4;
    let scale = 0.001;

    eprintln!("[info] Rendering started.");
    let timer = Instant::now();

    let mut output = vec![];

    (-viewport_height / 2 + y_offset..viewport_height / 2 + y_offset)
        .into_par_iter()
        .map(|y| {
            render(
                -viewport_width / 2 + x_offset..viewport_width / 2 + x_offset,
                y,
                scale,
            )
        })
        .collect::<Vec<Vec<u8>>>()
        .iter_mut()
        .for_each(|row| output.append(row));

    eprintln!(
        "[info] Rendering took {:.2} seconds, writing to output file...",
        timer.elapsed().as_secs_f64()
    );

    image::save_buffer(
        "./image.png",
        output.as_bytes(),
        viewport_width as u32,
        viewport_height as u32,
        image::ColorType::L8,
    )?;

    eprintln!("[info] Done.");

    Ok(())
}

fn render(x_block: Range<i32>, y_index: i32, scale: f64) -> Vec<u8> {
    x_block
        .map(|x| {
            let x = x as f64 * scale;
            let y = y_index as f64 * scale;

            let c = Complex(x, y);
            let mut z = Complex(0.0, 0.0);

            if let Some(iter) = (0..ITER_MAX).find(|_| {
                z = z * z + c;
                z.0.is_nan() || z.1.is_nan()
            }) {
                (ITER_MAX - iter) as u8
            } else {
                0
            }
        })
        .collect::<Vec<u8>>()
}
