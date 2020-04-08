mod tuple;
mod color;

use tuple::Tuple;

const EPSILON: f32 = 0.0001;

pub fn equals(a:f32, b:f32) -> bool {
    a - b < EPSILON && b - a < EPSILON
}

fn main() {
    let t = Tuple::vector(1.0, 3.0, -2.0);
    println!("{}", t.x);
    println!("{}", t.y);
    println!("{}", t.z);
    println!("{}", t.w);
    println!("{}", t.is_point());
    println!("{}", t.is_vector());
}
