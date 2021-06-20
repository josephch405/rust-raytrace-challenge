use self::super::equals;

#[derive(Default, Clone)]
pub struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        equals(self.x, other.x)
            && equals(self.y, other.y)
            && equals(self.z, other.z)
            && equals(self.w, other.w)
    }
}

impl Tuple {
    pub fn is_point(&self) -> bool {
        equals(self.w, 1.0)
    }
    pub fn is_vector(&self) -> bool {
        equals(self.w, 0.0)
    }
    pub fn vector(x: f32, y: f32, z: f32) -> Tuple {
        Tuple { x, y, z, w: 0.0 }
    }
    pub fn point(x: f32, y: f32, z: f32) -> Tuple {
        Tuple { x, y, z, w: 1.0 }
    }
    pub fn as_vector(&self) -> Tuple {
        Tuple {
            x: self.x,
            y: self.y,
            z: self.z,
            w: 0.,
        }
    }
    pub fn add(&self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
    pub fn subtract(&self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
    pub fn negate(&self) -> Tuple {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
    pub fn multiply(&self, k: f32) -> Tuple {
        Tuple {
            x: k * self.x,
            y: k * self.y,
            z: k * self.z,
            w: k * self.w,
        }
    }
    pub fn divide(&self, k: f32) -> Tuple {
        Tuple {
            x: self.x / k,
            y: self.y / k,
            z: self.z / k,
            w: self.w / k,
        }
    }
    pub fn norm(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }
    pub fn unit(&self) -> Tuple {
        let n = self.norm();
        Tuple {
            x: self.x / n,
            y: self.y / n,
            z: self.z / n,
            w: self.w / n,
        }
    }
    pub fn dot(&self, other: Tuple) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
    pub fn cross(&self, other: Tuple) -> Tuple {
        Tuple::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

#[cfg(test)]
mod tuple_tests {
    use super::{equals, Tuple};

    #[test]
    fn subtract_vec() {
        let p1 = Tuple::vector(1.0, 2.0, -4.0);
        let p2 = Tuple::vector(5.2, 0.0, 1.0);
        assert!(p1.is_vector());
        assert!(p2.is_vector());
        let p3 = p1.subtract(p2);
        assert!(equals(p3.x, -4.2));
        assert!(equals(p3.y, 2.0));
        assert!(equals(p3.z, -5.0));
        assert!(p3.is_vector());
    }

    #[test]
    fn subtract_points() {
        let p1 = Tuple::point(1.0, 2.0, -4.0);
        let p2 = Tuple::point(5.2, 0.0, 1.0);
        assert!(p1.is_point());
        assert!(p2.is_point());
        let p3 = p1.subtract(p2);
        assert!(equals(p3.x, -4.2));
        assert!(equals(p3.y, 2.0));
        assert!(equals(p3.z, -5.0));
        assert!(p3.is_vector());
    }

    #[test]
    fn subtract_vec_from_pt() {
        let p1 = Tuple::point(1.0, 2.0, -4.0);
        let p2 = Tuple::vector(5.2, 0.0, 1.0);
        assert!(p1.is_point());
        assert!(p2.is_vector());
        let p3 = p1.subtract(p2);
        assert!(equals(p3.x, -4.2));
        assert!(equals(p3.y, 2.0));
        assert!(equals(p3.z, -5.0));
        assert!(p3.is_point());
    }

    #[test]
    fn equality() {
        let p1 = Tuple::point(1.0, 2.0, -4.0);
        let p2 = Tuple::point(1.0, 2.0, -4.0);
        let p3 = Tuple::point(0.0, 2.0, -4.0);
        let v1 = Tuple::vector(1.0, 2.0, -4.0);
        let v2 = Tuple::vector(1.0, 2.0, -4.0);
        let v3 = Tuple::vector(0.0, 2.0, -4.0);
        assert!(p1 == p2);
        assert!(v1 == v2);
        assert!(p1 != p3);
        assert!(v1 != v3);
        assert!(p1 != v1);
    }

    #[test]
    fn norm() {
        let v = Tuple::vector(3.0, 0.0, -4.0);
        assert!(equals(v.norm(), 5.0))
    }

    #[test]
    fn unit() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        let u = Tuple::vector(0.26726, 0.53452, 0.80178);
        assert!(v.unit() == u);
    }

    #[test]
    fn dot() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        assert!(equals(a.dot(b), 20.0))
    }

    #[test]
    fn cross() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        let c = Tuple::vector(-1.0, 2.0, -1.0);
        assert!(a.cross(b.clone()) == c);
        assert!(b.cross(a) == c.multiply(-1.0))
    }
}
