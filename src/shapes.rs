use crate::matrix::matrix::{Matrix4, M4};
use crate::ray::Ray;
use crate::tuple::Tuple;
use std::sync::atomic::{AtomicIsize, Ordering};

pub trait Shape {
    fn intersects(&self, t: Ray) -> Vec<Intersection>;
    fn id(&self) -> isize;
}

static _MAX_SHAPE_ID: AtomicIsize = AtomicIsize::new(0);

pub struct Sphere {
    transform: Matrix4,
    id: isize,
}

pub fn sphere_unit() -> Sphere {
    Sphere {
        id: _MAX_SHAPE_ID.fetch_add(1, Ordering::SeqCst),
        transform: M4::I,
    }
}

pub fn sphere(transform: Matrix4) -> Sphere {
    Sphere {
        id: _MAX_SHAPE_ID.fetch_add(1, Ordering::SeqCst),
        transform,
    }
}

// impl Sphere {
// }

impl Shape for Sphere {
    fn intersects(&self, r: Ray) -> Vec<Intersection> {
        let r = r.transform(M4::invert(self.transform));
        let sphere_to_ray = r.origin.subtract(Tuple::point(0., 0., 0.));
        let a = r.direction.dot(r.direction.clone());
        let b = 2. * r.direction.dot(sphere_to_ray.clone());
        let c = sphere_to_ray.dot(sphere_to_ray.clone()) - 1.;
        let d = b * b - 4. * a * c;
        if d < 0. {
            vec![]
        } else {
            vec![
                Intersection {
                    t: (-b - d.sqrt()) / (2. * a),
                    object: self,
                },
                Intersection {
                    t: (-b + d.sqrt()) / (2. * a),
                    object: self,
                },
            ]
        }
    }
    fn id(&self) -> isize {
        self.id
    }
}

#[derive(Clone)]
pub struct Intersection<'a> {
    t: f32,
    object: &'a dyn Shape,
}

pub fn hit(intersections: Vec<Intersection>) -> Option<Intersection> {
    if intersections.first().is_none() {
        return None;
    }
    let mut best_i = intersections.first().expect("must exist").clone();
    for i in intersections {
        if best_i.t < 0. || (i.t >= 0. && i.t < best_i.t) {
            best_i = i.clone();
        }
    }
    if best_i.t >= 0. {
        Some(best_i.clone())
    } else {
        None
    }
}

#[cfg(test)]
mod sphere_tests {
    use crate::matrix::matrix::{scale, translation};
    use crate::ray::ray;
    use crate::shapes;
    use crate::shapes::{hit, Intersection, Shape};
    use crate::tuple::Tuple;

    #[test]
    fn sphere_test_1() {
        let s = shapes::sphere_unit();
        let r = ray(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let r_thru_s = s.intersects(r);

        assert_eq!(r_thru_s.len(), 2);
        assert_eq!(r_thru_s[0].t, 4.);
        assert_eq!(r_thru_s[0].object.id(), s.id);
        assert_eq!(r_thru_s[1].t, 6.);
        assert_eq!(r_thru_s[1].object.id(), s.id);
    }

    #[test]
    fn sphere_test_2() {
        let s = shapes::sphere_unit();
        let r = ray(Tuple::point(0., 1., -5.), Tuple::vector(0., 0., 1.));
        let r_thru_s = s.intersects(r);
        assert_eq!(r_thru_s.len(), 2);
        assert_eq!(r_thru_s[0].t, 5.);
        assert_eq!(r_thru_s[1].t, 5.);
    }

    #[test]
    fn sphere_test_3() {
        let s = shapes::sphere_unit();
        let r = ray(Tuple::point(0., 2., -5.), Tuple::vector(0., 0., 1.));
        let r_thru_s = s.intersects(r);
        assert_eq!(r_thru_s.len(), 0);
    }

    #[test]
    fn sphere_test_4() {
        let s = shapes::sphere_unit();
        let r = ray(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
        let r_thru_s = s.intersects(r);
        assert_eq!(r_thru_s.len(), 2);
        assert_eq!(r_thru_s[0].t, -1.);
        assert_eq!(r_thru_s[1].t, 1.);
    }

    #[test]
    fn sphere_test_5() {
        let s = shapes::sphere_unit();
        let r = ray(Tuple::point(0., 0., 5.), Tuple::vector(0., 0., 1.));
        let r_thru_s = s.intersects(r);
        assert_eq!(r_thru_s.len(), 2);
        assert_eq!(r_thru_s[0].t, -6.);
        assert_eq!(r_thru_s[1].t, -4.);
    }

    #[test]
    fn intersect_test() {
        let s = shapes::sphere_unit();
        let i = shapes::Intersection { t: 3.5, object: &s };
        assert_eq!(i.t, 3.5);
        assert_eq!(s.id, i.object.id());
    }

    #[test]
    fn hit_tests() {
        let s = shapes::sphere_unit();
        let i1 = Intersection { t: 1., object: &s };
        let i2 = Intersection { t: 2., object: &s };
        assert_eq!(i1.t, hit(vec![i2, i1.clone()]).expect("should exist").t);
        let i2 = Intersection { t: -1., object: &s };
        assert_eq!(
            i1.t,
            hit(vec![i2.clone(), i1.clone()]).expect("should exist").t
        );
        let i1 = Intersection { t: -2., object: &s };
        assert!(hit(vec![i2, i1]).is_none());

        let i1 = Intersection { t: 5., object: &s };
        let i2 = Intersection { t: 7., object: &s };
        let i3 = Intersection { t: -3., object: &s };
        let i4 = Intersection { t: 2., object: &s };
        assert_eq!(
            i4.t,
            hit(vec![i1, i2, i3, i4.clone()]).expect("should exist").t
        );
    }

    #[test]
    fn sphere_transform_intersect_test() {
        let s = shapes::sphere(scale(2., 2., 2.));
        let r = ray(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let x = s.intersects(r);
        assert_eq!(x.len(), 2);
        assert_eq!(x[0].t, 3.);
        assert_eq!(x[1].t, 7.);
    }

    #[test]
    fn sphere_transform_intersect_test_2() {
        let s = shapes::sphere(translation(5., 0., 0.));
        let r = ray(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let x = s.intersects(r);
        assert_eq!(x.len(), 0);
    }
}
