use crate::{vecx, Vec3, Vec4};
use std::ops::{self};

type FloatMat = Vec<Vec<f64>>;
type MatIndex = (usize, usize);

#[derive(Debug)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    content: FloatMat,
}

impl ops::Mul<&Matrix> for &Matrix {
    type Output = Matrix;
    fn mul(self, rhs: &Matrix) -> Self::Output {
        Matrix::multiply(self, rhs)
    }
}

impl ops::Mul<Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Self::Output {
        Matrix::multiply(&self, &rhs)
    }
}

impl ops::Mul<&Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: &Matrix) -> Self::Output {
        Matrix::multiply(&self, rhs)
    }
}

impl ops::Mul<Matrix> for &Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Self::Output {
        Matrix::multiply(self, &rhs)
    }
}

impl PartialEq<Matrix> for Matrix {
    fn eq(&self, other: &Matrix) -> bool {
        let same_dimensions = self.rows == other.rows && self.cols == other.cols;
        if !same_dimensions {
            return false;
        }

        for row in 0..self.rows {
            if self.content[row] != other.content[row] {
                return false;
            }
        }
        return true;
    }
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            rows,
            cols,
            content: vec![vec![0.0; cols]; rows],
        }
    }

    pub fn multiply(m1: &Matrix, m2: &Matrix) -> Matrix {
        if m1.cols != m2.rows {
            panic!("Invalid Matrix multiplication: {}x{} by {}x{}, columns of left need to equal rows of right", m1.rows, m1.cols, m2.rows, m2.cols);
        }

        let mut result = Matrix::new(m1.rows, m2.cols);
        for row in 1..=result.rows {
            for col in 1..=result.cols {
                let value = vecx::vec_dot(m1.get_row(row), m2.get_col(col));
                result.set((row, col), value);
            }
        }
        result
    }

    pub fn sqr4() -> Self {
        Matrix::new(4, 4)
    }

    pub fn id4() -> Self {
        let mut id4_mat = Self::sqr4();
        id4_mat.set((1, 1), 1.0);
        id4_mat.set((2, 2), 1.0);
        id4_mat.set((3, 3), 1.0);
        id4_mat.set((4, 4), 1.0);
        id4_mat
    }

    /// Scale 4x4 matrix
    ///
    /// \[sx, 0, 0, 0]
    ///
    /// \[0, sy, 0, 0]
    ///
    /// \[0, 0, sz, 0]
    ///
    /// \[0, 0, 0, 1]
    pub fn m4_scale(scale: Vec3) -> Matrix {
        let mut scale_mat = Matrix::id4();
        scale_mat.set((1, 1), scale.x());
        scale_mat.set((2, 2), scale.y());
        scale_mat.set((3, 3), scale.z());

        scale_mat
    }

    /// Translate 4x4 matrix
    ///
    /// \[1, 0, 0, tx]
    ///
    /// \[0, 1, 0, ty]
    ///
    /// \[0, 0, 1, tz]
    ///
    /// \[0, 0, 0, 1]
    pub fn m4_translate(translation: Vec3) -> Matrix {
        let mut translate_mat = Matrix::id4();
        translate_mat.set((1, 4), translation.x());
        translate_mat.set((2, 4), translation.y());
        translate_mat.set((3, 4), translation.z());

        translate_mat
    }

    /// Z left-handed rotation matrix 4x4
    /// angle a
    /// c = cos(a)
    /// s = sin(a)
    ///
    /// \[c, s, 0, 0]
    ///
    /// \[-s, c, 0, 0]
    ///
    /// \[0, 0, 1, 0]
    ///
    /// \[0, 0, 0, 1]
    pub fn m4_rotate_z(angle: f64) -> Matrix {
        let cos = f64::cos(angle);
        let sin = f64::sin(angle);

        let mut rotate_z = Matrix::id4();
        rotate_z.set((1, 1), cos);
        rotate_z.set((1, 2), sin);
        rotate_z.set((2, 1), -sin);
        rotate_z.set((2, 2), cos);

        rotate_z
    }

    /// X left-handed rotation matrix 4x4
    /// angle a
    /// c = cos(a)
    /// s = sin(a)
    ///
    /// \[1, 0, 0, 0]
    ///
    /// \[0, c, s, 0]
    ///
    /// \[0, -s, c, 0]
    ///
    /// \[0, 0, 0, 1]
    pub fn m4_rotate_x(angle: f64) -> Matrix {
        let cos = f64::cos(angle);
        let sin = f64::sin(angle);

        let mut rotate_x = Matrix::id4();
        rotate_x.set((2, 2), cos);
        rotate_x.set((2, 3), -sin);
        rotate_x.set((3, 2), sin);
        rotate_x.set((3, 3), cos);

        rotate_x
    }

    /// Y left-handed rotation matrix 4x4
    /// angle a
    /// c = cos(a)
    /// s = sin(a)
    ///
    /// \[c, 0, -s, 0]
    ///
    /// \[0, 1, 0, 0]
    ///
    /// \[s, 0, c, 0]
    ///
    /// \[0, 0, 0, 1]
    pub fn m4_rotate_y(angle: f64) -> Matrix {
        let cos = f64::cos(angle);
        let sin = f64::sin(angle);

        let mut rotate_y = Matrix::id4();
        rotate_y.set((1, 1), cos);
        rotate_y.set((1, 3), sin);
        rotate_y.set((3, 1), -sin);
        rotate_y.set((3, 3), cos);

        rotate_y
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn set(&mut self, rc: MatIndex, value: f64) {
        self.verify_mat_index(rc);

        let vec_index = Self::mat_index_to_vec_index(rc);
        self.content[vec_index.0][vec_index.1] = value;
    }

    pub fn get(&self, rc: MatIndex) -> f64 {
        self.verify_mat_index(rc);
        let vec_index = Self::mat_index_to_vec_index(rc);
        self.content[vec_index.0][vec_index.1]
    }

    pub fn get_row(&self, index: usize) -> Vec<f64> {
        self.verify_row_index(index);
        self.content[index - 1].clone()
    }

    pub fn get_col(&self, col: usize) -> Vec<f64> {
        self.verify_col_index(col);
        let col_index = col - 1;
        self.content.iter().map(|row| row[col_index]).collect()
    }

    pub fn set_content(&mut self, mat: FloatMat) {
        self.verify_content(&mat);
        self.content = mat;
    }

    fn mat_index_to_vec_index(rc: MatIndex) -> (usize, usize) {
        (rc.0 - 1, rc.1 - 1)
    }

    pub fn verify_row_index(&self, row: usize) {
        if row < 1 || row > self.rows {
            panic!(
                "Invalid row index {} for {}x{} matrix",
                row, self.rows, self.cols
            );
        }
    }

    pub fn verify_col_index(&self, col: usize) {
        if col < 1 || col > self.cols {
            panic!(
                "Invalid column index {} for {}x{} matrix",
                col, self.rows, self.cols
            );
        }
    }

    pub fn verify_mat_index(&self, rc: MatIndex) {
        let row = rc.0;
        let col = rc.1;

        self.verify_row_index(row);
        self.verify_col_index(col);
    }

    pub fn verify_content(&self, content: &FloatMat) {
        let content_rows = content.len();
        let content_cols = content[0].len();
        if content_rows != self.rows || content_cols != self.cols {
            panic!(
                "Attempt to set {}x{} content into a {}x{} matrix",
                content_rows, content_cols, self.rows, self.cols
            );
        }
    }
}

impl From<FloatMat> for Matrix {
    fn from(mat: FloatMat) -> Self {
        let mut matrix = Matrix::new(mat.len(), mat[0].len());
        matrix.set_content(mat);
        matrix
    }
}

impl From<Vec3> for Matrix {
    fn from(v: Vec3) -> Self {
        Matrix::from(vec![vec![v.x()], vec![v.y()], vec![v.z()]])
    }
}

impl From<Vec4> for Matrix {
    fn from(v: Vec4) -> Self {
        Matrix::from(vec![vec![v.x()], vec![v.y()], vec![v.z()], vec![v.w()]])
    }
}
