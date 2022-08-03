use anyhow::Result;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{ops::Range, time::Instant};

mod complex;
use complex::{Complex, Float};

const ITER_MAX: u8 = 255;

fn main() -> Result<()> {
    let viewport_width: i32 = 8000;
    let viewport_height: i32 = 6000;
    let y_offset: i32 = 0;
    let x_offset: i32 = -viewport_width / 4;
    let scale: Float = 0.0005;

    eprintln!("[info] Rendering started.");
    let timer = Instant::now();

    let output = (-viewport_height / 2 + y_offset..viewport_height / 2 + y_offset)
        .into_par_iter()
        .flat_map_iter(|row| {
            render(
                -viewport_width / 2 + x_offset..viewport_width / 2 + x_offset,
                row,
                scale,
            )
        })
        .collect::<Vec<u8>>();

    eprintln!(
        "[info] Rendering took {:.2} seconds, writing to output file...",
        timer.elapsed().as_secs_f32()
    );

    image::save_buffer(
        "./image.png",
        &output,
        viewport_width as u32,
        viewport_height as u32,
        image::ColorType::L8,
    )?;

    eprintln!("[info] Done.");

    Ok(())
}

fn render(x_block: Range<i32>, y_index: i32, scale: Float) -> Box<dyn Iterator<Item = u8>> {
    Box::new(x_block.map(move |x| {
        let x = x as Float * scale;
        let y = y_index as Float * scale;

        let c = Complex(x, y);
        let mut z = c;

        ITER_MAX
            - (1..ITER_MAX)
                .find(|_| {
                    z = z * z + c;
                    z.0.is_nan() || z.1.is_nan()
                })
                .unwrap_or(ITER_MAX)
    }))
}
