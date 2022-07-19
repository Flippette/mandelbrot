#![feature(portable_simd)]

use anyhow::Result; use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{collections::HashMap, ops::Range, time::Instant};

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
    let render_result = (-viewport_height / 2 + y_offset..viewport_height / 2 + y_offset)
        .into_par_iter()
        .map(|y| {
            render(
                y as u32,
                -viewport_width / 2 + x_offset..viewport_width / 2 + x_offset,
                y,
                scale,
            )
        })
        .collect::<HashMap<u32, Vec<(u8, u8, u8)>>>();

    let mut bytes = vec![];
    (-viewport_height / 2 + y_offset..viewport_height / 2 + y_offset).for_each(|y| {
        for item in render_result.get(&(y as u32)).unwrap() {
            bytes.append(&mut vec![item.0, item.1, item.2]);
        }
    });

    assert_eq!(
        bytes.len(),
        viewport_width as usize * viewport_height as usize * 3
    );

    eprintln!(
        "[info] Rendering took {:.2} seconds, writing to output file...",
        timer.elapsed().as_secs_f64()
    );

    image::save_buffer(
        "./image.png",
        &bytes,
        viewport_width as u32,
        viewport_height as u32,
        image::ColorType::Rgb8,
    )?;

    eprintln!("[info] Done.");

    Ok(())
}

fn render(id: u32, x_block: Range<i32>, y_index: i32, scale: f64) -> (u32, Vec<(u8, u8, u8)>) {
    let render_line = x_block
        .map(|x| {
            let x = x as f64 * scale;
            let y = y_index as f64 * scale;

            let c = Complex(x, y);
            let mut z = Complex(0.0, 0.0);
            let mut iter = 0;

            loop {
                z = z * z + c;
                if z.0.is_nan() || z.1.is_nan() {
                    let color = (ITER_MAX - iter) as u8;
                    return (color, color, color);
                } else if iter == ITER_MAX {
                    return (0, 0, 0);
                }

                iter += 1;
            }
        })
        .collect::<Vec<(u8, u8, u8)>>();

    (id, render_line)
}
