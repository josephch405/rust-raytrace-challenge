use crate::tuple::Tuple;

pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

pub fn ray(origin: Tuple, direction: Tuple) -> Ray {
    assert!(origin.is_point());
    assert!(direction.is_vector());
    Ray {
        origin,
        direction,
    }
}

impl Ray {
    pub fn position(&self, time: f32) -> Tuple {
        self.origin.add(self.direction.multiply(time))
    }
}

#[cfg(test)]
mod matrix_tests {
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
}
