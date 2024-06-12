use crate::{Matrix, Vec3};

#[test]
fn from_vec() {
    let mat_2x1 = Matrix::from(vec![vec![1.1], vec![2.1]]);
    assert_eq!(mat_2x1.get((1, 1)), 1.1);
    assert_eq!(mat_2x1.get((2, 1)), 2.1);
}

#[test]
fn sqr4() {
    let sqr4_mat = Matrix::sqr4();
    assert_eq!(sqr4_mat.get((1, 1)), 0.0);
    assert_eq!(sqr4_mat.get((4, 4)), 0.0);
}

#[test]
fn id4() {
    let id4_mat = Matrix::id4();
    assert_eq!(id4_mat.get((1, 2)), 0.0);
    assert_eq!(id4_mat.get((1, 1)), 1.0);
    assert_eq!(id4_mat.get((2, 2)), 1.0);
    assert_eq!(id4_mat.get((3, 3)), 1.0);
    assert_eq!(id4_mat.get((4, 4)), 1.0);
}

#[test]
fn mat_from_v3() {
    let v3 = Vec3(1.1, 2.1, 3.1);
    let mat = Matrix::from(v3);
    assert_eq!(mat.get((1, 1)), 1.1);
    assert_eq!(mat.get((2, 1)), 2.1);
    assert_eq!(mat.get((3, 1)), 3.1);
}

#[test]
fn mat_get_row() {
    let mat = Matrix::id4();
    assert_eq!(mat.get_row(1), vec![1.0, 0.0, 0.0, 0.0]);
}

#[test]
fn mat_get_col() {
    let mat = Matrix::id4();
    assert_eq!(mat.get_col(1), vec![1.0, 0.0, 0.0, 0.0]);
}

#[test]
fn mat_mul() {
    /*
    [1, 2] * [5, 6]   [1*5 + 2*7, 1*6 + 2*8]   [19, 22]
    [3, 4]   [7, 8] = [3*5 + 4*7, 3*6 + 4*8] = [43, 50]
    */
    let mat1 = Matrix::from(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);

    let mat2 = Matrix::from(vec![vec![5.0, 6.0], vec![7.0, 8.0]]);

    let result = &mat1 * &mat2;
    assert_eq!(
        result,
        Matrix::from(vec![vec![19.0, 22.0], vec![43.0, 50.0]])
    );

    let mut scaling = Matrix::id4();
    scaling.set((1, 1), 2.0);
    scaling.set((2, 2), 0.5);
    scaling.set((3, 3), 3.0);

    let v = Vec3(2.0, 4.0, 3.0);
    let mat_result = &scaling * &v.as_mat4(1.0);
    assert_eq!(
        mat_result,
        Matrix::from(vec![vec![4.0], vec![2.0], vec![9.0], vec![1.0]])
    );
    assert_eq!(Vec3::from(mat_result), Vec3(4.0, 2.0, 9.0));
}

#[test]
fn matrix_chain_multiply() {
    let mut scale = Matrix::id4();
    scale.set((1, 1), 2.0);
    scale.set((2, 2), 0.5);
    scale.set((3, 3), 3.0);

    let mut translate = Matrix::id4();
    translate.set((1, 4), 5.0);
    translate.set((2, 4), 10.0);
    translate.set((3, 4), 2.0);

    let vec_mat4 = Vec3(5.0, 6.0, 7.0).as_mat4(1.0);

    let result = translate * scale * vec_mat4;
    assert_eq!(Vec3::from(result), Vec3(10.0 + 5.0, 3.0 + 10.0, 21.0 + 2.0));
}
