mod matrix {
    use std::cmp::{max, min};
    use crate::equals;
    use crate::tuple::Tuple;

    pub type Matrix2 = [[f32; 2]; 2];
    pub type Matrix3 = [[f32; 3]; 3];
    pub type Matrix4 = [[f32; 4]; 4];

    fn remove_idx<T: Clone>(v: Vec<T>, idx: usize) -> Vec<T> {
        let mut v = v.to_vec();
        v.remove(idx);
        v
    }

    const I4: Matrix4 = [[1., 0., 0., 0.], [0., 1., 0., 0.], [0., 0., 1., 0.], [0., 0., 0., 1.]];

    pub struct M2 {}

    impl M2 {
        pub fn det(m: Matrix2) -> f32 {
            m[0][0] * m[1][1] - m[0][1] * m[1][0]
        }
    }

    pub struct M3 {}

    impl M3 {
        pub fn sub(m: Matrix3, row: i32, col: i32) -> Matrix2 {
            let row_i = max(0, min(row, 2)) as usize;
            let col_j = max(0, min(col, 2)) as usize;
            let row_vecs = remove_idx(m.to_vec(), row_i);
            let row_vecs: Vec<Vec<f32>> = row_vecs.iter()
                .map(|col| remove_idx(col.to_vec(), col_j))
                .collect();
            [[row_vecs[0][0], row_vecs[0][1]], [row_vecs[1][0], row_vecs[1][1]], ]
        }
        pub fn minor(m: Matrix3, row: i32, col: i32) -> f32 {
            let sub_m = M3::sub(m, row, col);
            M2::det(sub_m)
        }
        pub fn cofactor(m: Matrix3, row: i32, col: i32) -> f32 {
            ((row + col) % 2 * -2 + 1) as f32 * M3::minor(m, row, col)
        }
        pub fn det(m: Matrix3) -> f32 {
            m[0][0] * M3::cofactor(m, 0, 0)
                + m[0][1] * M3::cofactor(m, 0, 1)
                + m[0][2] * M3::cofactor(m, 0, 2)
        }
    }

    pub struct M4 {}

    impl M4 {
        pub fn mm(m1: Matrix4, m2: Matrix4) -> Matrix4 {
            let mut m = [[0.0; 4]; 4];
            for i in 0..4 {
                for j in 0..4 {
                    for k in 0..4 {
                        m[i][j] += m1[i][k] * m2[k][j];
                    }
                }
            }
            m
        }

        pub fn t(m: Matrix4) -> Matrix4 {
            [
                [m[0][0], m[1][0], m[2][0], m[3][0]],
                [m[0][1], m[1][1], m[2][1], m[3][1]],
                [m[0][2], m[1][2], m[2][2], m[3][2]],
                [m[0][3], m[1][3], m[2][3], m[3][3]],
            ]
        }
        pub const I: Matrix4 = I4;

        pub fn sub(m: Matrix4, row: i32, col: i32) -> Matrix3 {
            let row_i = max(0, min(row, 3)) as usize;
            let col_j = max(0, min(col, 3)) as usize;
            let row_vecs = remove_idx(m.to_vec(), row_i);
            let row_vecs: Vec<Vec<f32>> = row_vecs.iter()
                .map(|col| remove_idx(col.to_vec(), col_j))
                .collect();
            [
                [row_vecs[0][0], row_vecs[0][1], row_vecs[0][2]],
                [row_vecs[1][0], row_vecs[1][1], row_vecs[1][2]],
                [row_vecs[2][0], row_vecs[2][1], row_vecs[2][2]],
            ]
        }
        pub fn minor(m: Matrix4, row: i32, col: i32) -> f32 {
            let sub_m = M4::sub(m, row, col);
            M3::det(sub_m)
        }
        pub fn cofactor(m: Matrix4, row: i32, col: i32) -> f32 {
            ((row + col) % 2 * -2 + 1) as f32 * M4::minor(m, row, col)
        }
        pub fn det(m: Matrix4) -> f32 {
            m[0][0] * M4::cofactor(m, 0, 0)
                + m[0][1] * M4::cofactor(m, 0, 1)
                + m[0][2] * M4::cofactor(m, 0, 2)
                + m[0][3] * M4::cofactor(m, 0, 3)
        }
        pub fn invertible(m: Matrix4) -> bool {
            !equals(M4::det(m), 0.)
        }
        pub fn invert(m: Matrix4) -> Matrix4 {
            let mut result: Matrix4 = [[0.; 4]; 4];
            let det = M4::det(m);
            for row in 0..4 {
                for col in 0..4 {
                    let c = M4::cofactor(m, row, col);
                    result[col as usize][row as usize] = c / det;
                }
            }
            result
        }
        pub fn eq(a: Matrix4, b: Matrix4) -> bool {
            (0..16).map(|i| equals(a[i / 4][i % 4], b[i / 4][i % 4])).all(|x| x)
        }
        pub fn dot_tuple(m: Matrix4, t: Tuple) -> Tuple {
            let r: Vec<f32> = m.iter().map(|row| row[0] * t.x + row[1] * t.y + row[2] * t.z + row[3] * t.w).collect();
            Tuple {
                x: r[0],
                y: r[1],
                z: r[2],
                w: r[3],
            }
        }
    }

    pub fn translation(x: f32, y: f32, z: f32) -> Matrix4 {
        [[1., 0., 0., x], [0., 1., 0., y], [0., 0., 1., z], [0., 0., 0., 1.]]
    }
    pub fn scale(x: f32, y: f32, z: f32) -> Matrix4 {
        [[x, 0., 0., 0.], [0., y, 0., 0.], [0., 0., z, 0.], [0., 0., 0., 1.]]
    }

}

#[cfg(test)]
mod matrix_tests {
    use self::super::matrix::*;
    use crate::tuple::Tuple;

    #[test]
    fn eq() {
        let a: Matrix4 = [[1., 2., 3., 4.], [5., 6., 7., 8.], [9., 8., 7., 6.], [5., 4., 3., 2.]];
        let b: Matrix4 = [[1., 2., 3., 4.], [5., 6., 7., 8.], [9., 8., 7., 6.], [5., 4., 3., 2.]];
        assert_eq!(a, b)
    }

    #[test]
    fn dne() {
        let a: Matrix4 = [[1., 2., 3., 4.], [5., 6., 7., 8.], [9., 8., 7., 6.], [5., 4., 3., 2.]];
        let b: Matrix4 = [[2., 2., 3., 4.], [5., 6., 7., 8.], [9., 8., 7., 6.], [5., 4., 3., 2.]];
        assert_ne!(a, b)
    }

    #[test]
    fn mm4_test() {
        let a: Matrix4 = [[1., 2., 3., 4.], [5., 6., 7., 8.], [9., 8., 7., 6.], [5., 4., 3., 2.]];
        let b: Matrix4 = [[-2., 1., 2., 3.], [3., 2., 1., -1.], [4., 3., 6., 5.], [1., 2., 7., 8.]];
        let c: Matrix4 = [[20., 22., 50., 48.], [44., 54., 114., 108.], [40., 58., 110., 102.], [16., 26., 46., 42.]];
        assert_eq!(M4::mm(a, b), c)
    }

    #[test]
    fn id_test() {
        let a: Matrix4 = [[1., 2., 3., 4.], [5., 6., 7., 8.], [9., 8., 7., 6.], [5., 4., 3., 2.]];
        assert_eq!(a, M4::mm(a, M4::I))
    }

    #[test]
    fn transpose() {
        let a: Matrix4 = [[0., 9., 3., 0.], [9., 8., 0., 8.], [1., 8., 5., 3.], [0., 0., 5., 8.]];
        let b: Matrix4 = [[0., 9., 1., 0.], [9., 8., 8., 0.], [3., 0., 5., 5.], [0., 8., 3., 8.]];
        assert_eq!(M4::t(a), b);
        assert_eq!(M4::t(M4::I), M4::I)
    }

    #[test]
    fn det() {
        let a: Matrix2 = [[1., 5.], [-3., 2.]];
        assert_eq!(M2::det(a), 17.);
        let b: Matrix3 = [[1., 2., 6.], [-5., 8., -4.], [2., 6., 4.]];
        assert_eq!(M3::det(b), -196.);
        let c: Matrix4 = [[-2., -8., 3., 5.], [-3., 1., 7., 3.], [1., 2., -9., 6.], [-6., 7., 7., -9.]];
        assert_eq!(M4::det(c), -4071.);
    }

    #[test]
    fn sub() {
        let a: Matrix4 = [[0., 9., 3., 0.], [9., 8., 0., 8.], [1., 8., 5., 3.], [0., 0., 5., 8.]];
        let b: Matrix3 = [[0., 3., 0.], [9., 0., 8.], [0., 5., 8.]];
        assert_eq!(M4::sub(a, 2, 1), b);
        let c: Matrix2 = [[9., 0.], [0., 5.]];
        assert_eq!(M3::sub(b, 0, 2), c)
    }

    #[test]
    fn minor() {
        let a: Matrix3 = [[3., 5., 0.], [2., -1., -7.], [6., -1., 5.]];
        assert_eq!(M3::minor(a, 1, 0), 25.)
    }

    #[test]
    fn cofactor() {
        let a: Matrix3 = [[3., 5., 0.], [2., -1., -7.], [6., -1., 5.]];
        assert_eq!(M3::minor(a, 0, 0), -12.);
        assert_eq!(M3::cofactor(a, 0, 0), -12.);
        assert_eq!(M3::minor(a, 1, 0), 25.);
        assert_eq!(M3::cofactor(a, 1, 0), -25.);
    }

    #[test]
    fn inverse() {
        let a: Matrix4 = [[0., 9., 3., 0.], [9., 8., 0., 8.], [1., 8., 5., 3.], [0., 0., 5., 8.]];
        let b: Matrix4 = [[-2., -8., 3., 5.], [-3., 1., 7., 3.], [1., 2., -9., 6.], [-6., 7., 7., -9.]];
        let c = M4::mm(a, b);
        assert!(M4::eq(M4::mm(c, M4::invert(b)), a));
    }

    #[test]
    fn translate() {
        let a = translation(5., -3., 2.);
        let t = Tuple::point(-3., 4., 5.);
        assert!(Tuple::point(2., 1., 7.).eq(&M4::dot_tuple(a, t.clone())));
        let i_a = M4::invert(a);
        assert!(Tuple::point(-8., 7., 3.).eq(&M4::dot_tuple(i_a, t)));
        let v = Tuple::vector(-3., 4., 5.);
        assert!(v.clone().eq(&M4::dot_tuple(a, v)));
    }

    #[test]
    fn scale_test() {
        let a = scale(2., 3., 4.);
        let t = Tuple::point(-4., 6., 8.);
        assert!(Tuple::point(-8., 18., 32.).eq(&M4::dot_tuple(a, t.clone())));
        let i_a = M4::invert(a);
        assert!(Tuple::point(-2., 2., 2.).eq(&M4::dot_tuple(i_a, t)));
        let v = Tuple::vector(-4., 6., 8.);
        assert!(Tuple::vector(-8., 18., 32.).eq(&M4::dot_tuple(a, v)));
    }

    #[test]
    fn reflect() {
        let a = scale(-1., 1., 1.);
        let t = Tuple::point(-4., 6., 8.);
        assert!(Tuple::point(4., 6., 8.).eq(&M4::dot_tuple(a, t.clone())));
    }
}
