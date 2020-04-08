type Matrix2 = [[f32; 2]; 2];
type Matrix3 = [[f32; 3]; 3];
type Matrix4 = [[f32; 4]; 4];

pub fn mm4(m1: Matrix4, m2: Matrix4) -> Matrix4 {
    let mut m = [[0.0; 4]; 4];
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                m[i][j] += m1[i][k] * m2[k][j];
            }
        }
    }
    m
}

#[cfg(test)]
mod matrix_tests {
    use self::super::Matrix4;

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
}
