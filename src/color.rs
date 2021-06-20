use self::super::equals;

#[derive(Default, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        equals(self.r, other.r) && equals(self.g, other.g) && equals(self.b, other.b)
    }
}

impl Color {
    pub fn add(&self, other: Color) -> Color {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
    pub fn subtract(&self, other: Color) -> Color {
        Color {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
    pub fn multiply(&self, k: f32) -> Color {
        Color {
            r: k * self.r,
            g: k * self.g,
            b: k * self.b,
        }
    }
    pub fn multiply_color(&self, c: Color) -> Color {
        Color {
            r: self.r * c.r,
            g: self.g * c.g,
            b: self.b * c.b,
        }
    }
}

#[cfg(test)]
mod color_tests {
    use super::Color;

    #[test]
    fn mult_color() {
        let a = Color {
            r: 1.0,
            g: 0.2,
            b: 0.4,
        };
        let b = Color {
            r: 0.9,
            g: 1.0,
            b: 0.1,
        };
        let c = Color {
            r: 0.9,
            g: 0.2,
            b: 0.04,
        };
        assert!(a.multiply_color(b) == c)
    }
}
