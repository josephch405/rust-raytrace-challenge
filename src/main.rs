mod canvas;
mod color;
mod matrix;
mod tuple;
mod ray;
mod shapes;

use crate::color::Color;
use canvas::Canvas;
use std::fs::File;
use std::io::Write;
use crate::shapes::Shape;
use crate::tuple::Tuple;
use crate::ray::ray;
use crate::matrix::matrix::translation;

const EPSILON: f32 = 0.0001;

pub fn equals(a: f32, b: f32) -> bool {
    a - b < EPSILON && b - a < EPSILON
}

fn main() {
    let mut canvas = Canvas::new(100, 100);
    let mut file = File::create("output.ppm").expect("Cannot open file");
    let s = shapes::sphere(translation(2., 0., 0.));
    let camera = Tuple::point(0., 0., -5.);
    let wall_z = 10.;
    let wall_size = 7.;
    let pixel_size = wall_size / 100.;
    let half = wall_size / 2.;
    for x in 0..100 {
        let world_x = -half + pixel_size * x as f32;
        for y in 0..100 {
            let world_y = -half + pixel_size * y as f32;
            let ray = ray(camera.clone(), Tuple::vector(world_x, world_y, wall_z)
                .subtract(camera.clone().as_vector()).unit());
            let r = if s.intersects(ray).len() > 0 {
                255.
            } else {
                0.
            };
            canvas.set(
                x,
                y,
                Color {
                    r,
                    g: 0.,
                    b: 0.,
                },
            )
        }
    }
    file.write(canvas.to_ppm().as_bytes())
        .expect("write failed");
}
