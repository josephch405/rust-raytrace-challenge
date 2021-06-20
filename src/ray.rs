use crate::matrix::matrix;
use crate::tuple::Tuple;

#[derive(Clone)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

pub fn ray(origin: Tuple, direction: Tuple) -> Ray {
    assert!(origin.is_point());
    assert!(direction.is_vector());
    Ray { origin, direction }
}

impl Ray {
    pub fn position(&self, time: f32) -> Tuple {
        self.origin.add(self.direction.multiply(time))
    }
    pub fn transform(&self, m: matrix::Matrix4) -> Ray {
        Ray {
            origin: matrix::M4::dot_tuple(m, self.origin.clone()),
            direction: matrix::M4::dot_tuple(m, self.direction.clone()),
        }
    }
}

#[cfg(test)]
mod matrix_tests {
    use crate::matrix::matrix::{scale, translation};
    use crate::ray::ray;
    use crate::tuple::Tuple;

    #[test]
    fn ray_test() {
        let ray = ray(Tuple::point(2., 3., 4.), Tuple::vector(1., 0., 0.));
        assert!(ray.position(0.).eq(&Tuple::point(2., 3., 4.)));
        assert!(ray.position(1.).eq(&Tuple::point(3., 3., 4.)));
        assert!(ray.position(-1.).eq(&Tuple::point(1., 3., 4.)));
        assert!(ray.position(2.5).eq(&Tuple::point(4.5, 3., 4.)));
    }

    #[test]
    fn ray_translate() {
        let r1 = ray(Tuple::point(1., 2., 3.), Tuple::vector(0., 1., 0.));
        let m = translation(3., 4., 5.);
        let r2 = r1.transform(m);
        assert!(r2.origin.eq(&Tuple::point(4., 6., 8.)));
        assert!(r2.direction.eq(&Tuple::vector(0., 1., 0.)));
    }

    #[test]
    fn ray_scale() {
        let r1 = ray(Tuple::point(1., 2., 3.), Tuple::vector(0., 1., 0.));
        let m = scale(2., 3., 4.);
        let r2 = r1.transform(m);
        assert!(r2.origin.eq(&Tuple::point(2., 6., 12.)));
        assert!(r2.direction.eq(&Tuple::vector(0., 3., 0.)));
    }
}
