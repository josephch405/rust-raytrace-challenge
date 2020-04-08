use self::super::color::Color;

pub struct Canvas {
    pub width: i32,
    pub height: i32,
    pixels: Vec<Vec<Color>>,
    // NB: adopting NP [row][col] convention, so [y][x]
}

fn float_to_255(f: f32) -> i32 {
    let f = f * 255.0;
    if f < 0.0 {
        0
    } else if f > 255.0 {
        255
    } else {
        f.round() as i32
    }
}

fn color_to_255_str(c: &Color) -> String {
    format!("{} {} {}", float_to_255(c.r), float_to_255(c.g), float_to_255(c.b))
}

fn limit_string_to_70(s: String) -> String {
    let mut o = String::new();
    let mut i = 0;
    let s = s.split(" ");
    for num in s {
        let num_length = num.len();
        if i + num_length + 1 > 70 {
            o.pop();
            o.push('\n');
            i = 0;
        }
        i += num_length + 1;
        o.push_str(&num);
        o.push(' ');
    };
    o.pop();
    o
}

impl Canvas {
    pub fn new(w: i32, h: i32) -> Canvas {
        Canvas {
            width: w,
            height: h,
            pixels: vec![vec![Color::default(); w as usize]; h as usize],
        }
    }
    pub fn get(&self, w: i32, h: i32) -> Color {
        self.pixels[h as usize][w as usize].clone()
    }
    pub fn set(&mut self, w: i32, h: i32, c: Color) {
        let row = &mut self.pixels[h as usize];
        let w = w as usize;
        row.remove(w);
        row.insert(w, c);
    }
    fn get_ppm_from_row(&self, row: &Vec<Color>) -> String {
        let col_strs: Vec<String> = row.iter().map(color_to_255_str).collect();
        limit_string_to_70(col_strs.join(" "))
    }
    pub fn to_ppm(&self) -> String {
        let header = String::from(format!("P3\n\
        {} {}\n\
        255\n\
        ", self.width, self.height));
        let mapped_cols: Vec<String> = self.pixels.iter().map(|x| self.get_ppm_from_row(x)).collect();
        let content = mapped_cols.join("\n");
        format!("{}{}\n", header, content)
    }
}

#[cfg(test)]
mod canvas_tests {
    use super::*;

    #[test]
    fn empty_canvas() {
        let c = Canvas::new(20, 40);
        assert!(c.get(10, 30) == Color::default())
    }

    #[test]
    fn write_canvas() {
        let mut c = Canvas::new(20, 40);
        c.set(12, 22, Color { r: 0.2, g: 0.3, b: 0.4 });
        assert!(c.get(10, 30) == Color::default());
        assert!(c.get(12, 22) == Color { r: 0.2, g: 0.3, b: 0.4 })
    }

    #[test]
    fn ppm() {
        let mut c = Canvas::new(5, 3);
        c.set(0, 0, Color { r: 1.5, g: 0.0, b: 0.0 });
        c.set(2, 1, Color { r: 0.0, g: 0.5, b: 0.0 });
        c.set(4, 2, Color { r: -0.5, g: 0.0, b: 1.0 });
        assert_eq!(c.to_ppm(), String::from(
            "P3\n\
            5 3\n\
            255\n\
            255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n\
            0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n\
            0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n"
        ));
    }

    #[test]
    fn line_limit() {
        assert_eq!(
            limit_string_to_70(
                String::from("123 456 78 90 \
                123 45 67 89 0 \
                123 45 67 89 0 \
                123 45 67 89 0 \
                12 345 678 90")),
                "123 456 78 90 123 45 67 89 0 123 45 67 89 0 123 45 67 89 0 12 345 678\n90"
        )
    }
}
