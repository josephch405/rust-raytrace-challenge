mod tuple;
mod color;
mod canvas;
mod matrix;

use canvas::Canvas;
use std::fs::File;
use std::io::Write;
use crate::color::Color;

const EPSILON: f32 = 0.0001;

pub fn equals(a:f32, b:f32) -> bool {
    a - b < EPSILON && b - a < EPSILON
}

fn main() {
    let mut canvas = Canvas::new(300, 200);
    let mut file = File::create("output.ppm").expect("Cannot open file");
    for x in 0..300 {
        for y in 0..200 {
            canvas.set(x, y, Color {
                r: x as f32 / 255.0,
                g: y as f32 / 200.0,
                b: (200 - x) as f32 / 200.0 * (200 - y) as f32 / 200.0
            })
        }
    }
    file.write(canvas.to_ppm().as_bytes());
}
