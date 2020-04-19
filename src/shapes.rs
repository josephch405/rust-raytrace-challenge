use crate::ray::Ray;
use crate::tuple::Tuple;

pub trait Shape {
    fn intersects(&self, t: Ray) -> Vec<f32>;
}

pub struct Sphere {}

pub fn sphere(origin: Tuple, radius: f32) -> Sphere {
    Sphere {}
}

// impl Sphere {
// }

impl Shape for Sphere {
    fn intersects(&self, r: Ray) -> Vec<f32> {
        let sphere_to_ray = r.origin.subtract(Tuple::point(0., 0., 0.));
        let a = r.direction.dot(r.direction.clone());
        let b = 2. * r.direction.dot(sphere_to_ray.clone());
        let c = sphere_to_ray.dot(sphere_to_ray.clone()) - 1.;
        let d = b * b - 4. * a * c;
        if d < 0. {
            vec![]
        } else {
            vec![(-b - d.sqrt()) / (2. * a), (-b + d.sqrt()) / (2. * a)]
        }
    }
}

#[cfg(test)]
mod sphere_tests {
    use crate::tuple::Tuple;
    use crate::shapes;
    use crate::shapes::Shape;
    use crate::ray::ray;

    #[test]
    fn sphere_test_1() {
        let s = shapes::sphere(Tuple::point(0., 0., 0.), 1.);
        let r = ray(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let r_thru_s = s.intersects(r);
        assert_eq!(r_thru_s.len(), 2);
        assert_eq!(r_thru_s[0], 4.);
        assert_eq!(r_thru_s[1], 6.);
    }

    #[test]
    fn sphere_test_2() {
        let s = shapes::sphere(Tuple::point(0., 0., 0.), 1.);
        let r = ray(Tuple::point(0., 1., -5.), Tuple::vector(0., 0., 1.));
        let r_thru_s = s.intersects(r);
        assert_eq!(r_thru_s.len(), 2);
        assert_eq!(r_thru_s[0], 5.);
        assert_eq!(r_thru_s[1], 5.);
    }

    #[test]
    fn sphere_test_3() {
        let s = shapes::sphere(Tuple::point(0., 0., 0.), 1.);
        let r = ray(Tuple::point(0., 2., -5.), Tuple::vector(0., 0., 1.));
        let r_thru_s = s.intersects(r);
        assert_eq!(r_thru_s.len(), 0);
    }

    #[test]
    fn sphere_test_4() {
        let s = shapes::sphere(Tuple::point(0., 0., 0.), 1.);
        let r = ray(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
        let r_thru_s = s.intersects(r);
        assert_eq!(r_thru_s.len(), 2);
        assert_eq!(r_thru_s[0], -1.);
        assert_eq!(r_thru_s[1], 1.);
    }

    #[test]
    fn sphere_test_5() {
        let s = shapes::sphere(Tuple::point(0., 0., 0.), 1.);
        let r = ray(Tuple::point(0., 0., 5.), Tuple::vector(0., 0., 1.));
        let r_thru_s = s.intersects(r);
        assert_eq!(r_thru_s.len(), 2);
        assert_eq!(r_thru_s[0], -6.);
        assert_eq!(r_thru_s[1], -4.);
    }
}

