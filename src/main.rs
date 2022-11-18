use image::{ImageOutputFormat, Luma};
use rayon::prelude::*;
use std::{fs::File, io::BufWriter, time::Instant};

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
        .collect::<Vec<_>>();

    let mut output = Vec::with_capacity(VIEWPORT_WIDTH as usize * VIEWPORT_HEIGHT as usize);

    (0..VIEWPORT_HEIGHT as usize / 2)
        .map(|i| &upper_half[i * VIEWPORT_WIDTH as usize..(i + 1) * VIEWPORT_WIDTH as usize])
        .for_each(|slice| output.extend(slice.iter()));

    (0..VIEWPORT_HEIGHT as usize / 2)
        .rev()
        .map(|i| &upper_half[i * VIEWPORT_WIDTH as usize..(i + 1) * VIEWPORT_WIDTH as usize])
        .for_each(|slice| output.extend(slice.iter()));

    eprintln!(
        "[info] rendering took {:.4} seconds, writing to output file...",
        timer.elapsed().as_secs_f32()
    );

    let mut writer = BufWriter::new(File::create("image.jpeg").unwrap());
    let image: image::ImageBuffer<Luma<u8>, Vec<_>> =
        image::ImageBuffer::from_vec(VIEWPORT_WIDTH as u32, VIEWPORT_HEIGHT as u32, output)
            .unwrap();

    image
        .write_to(&mut writer, ImageOutputFormat::Jpeg(100))
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
        last_z = z;
        z = z.sqr() + c;
        if z.re.is_nan() {
            return ITER_MAX - iter - last_z.re.is_nan() as u8;
        }
    }

    0
}
