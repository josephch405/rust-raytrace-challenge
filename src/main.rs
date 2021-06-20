mod canvas;
mod color;
mod matrix;
mod ray;
mod shapes;
mod tuple;

use crate::color::Color;
use crate::matrix::matrix::translation;
use crate::ray::ray;
use crate::shapes::Shape;
use crate::tuple::Tuple;
use canvas::Canvas;
use indicatif::ProgressBar;
use std::fs::File;
use std::io::Write;

const EPSILON: f32 = 0.0001;

pub fn equals(a: f32, b: f32) -> bool {
    a - b < EPSILON && b - a < EPSILON
}

fn main() {
    // TODO: determine wall dims from canvas dims
    let width: u64 = 200;
    let height: u64 = 200;
    let mut canvas = Canvas::new(width, height);
    let mut file = File::create("output.ppm").expect("Cannot open file");
    let s = shapes::sphere(translation(0., 0., 0.));
    let camera = Tuple::point(0., 0., -5.);
    let wall_z = 10.;
    let wall_width = 7.;
    let wall_height = 7.;
    let pixel_size_w = wall_width / width as f32;
    let half_w = wall_width / 2.;
    let pixel_size_h = wall_height / height as f32;
    let half_h = wall_height / 2.;

    let bar = ProgressBar::new(width);
    for x in 0..width {
        let world_x = -half_w + pixel_size_w * x as f32;
        for y in 0..height {
            let world_y = -half_h + pixel_size_h * y as f32;
            let ray = ray(
                camera.clone(),
                Tuple::vector(world_x, world_y, wall_z)
                    .subtract(camera.clone().as_vector())
                    .unit(),
            );
            let r = if s.intersects(ray).len() > 0 {
                255.
            } else {
                0.
            };
            canvas.set(x, y, Color { r, g: 0., b: 0. })
        }
        bar.inc(1);
    }
    file.write(canvas.to_ppm().as_bytes())
        .expect("write failed");
}
