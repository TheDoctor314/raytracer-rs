//! Implementation of matrices and their various operations.
use std::ops;

use approx::{AbsDiffEq, RelativeEq};

use crate::vec3::{Point3, Vec3};

/// Representation of a 4x4 Matrix.
/// The elements are stored in row-major order.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Mat4 {
    /// Stores the rows of the matrix.
    rows: [[f32; 4]; 4],
}

/// Helper private alias for the matrix.
type Row = [f32; 4];

impl Mat4 {
    /// Constructs a new `Mat4` from the specified rows.
    pub fn new(row0: Row, row1: Row, row2: Row, row3: Row) -> Self {
        Self {
            rows: [row0, row1, row2, row3],
        }
    }

    /// Constructs the identity matrix.
    pub fn identity() -> Self {
        Self {
            rows: [
                [1., 0., 0., 0.],
                [0., 1., 0., 0.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1.],
            ],
        }
    }

    /// Transposes the given matrix.
    pub fn transpose(&self) -> Self {
        let mut res = self.clone();

        for i in 0..4 {
            for j in 0..4 {
                res[(i, j)] = self[(j, i)];
            }
        }

        res
    }

    /// Transposes the given matrix in-place.
    pub fn transposed(&mut self) {
        *self = self.transpose();
    }

    /// Calculates the determinant of the given matrix.
    pub fn determinant(&self) -> f32 {
        let [m00, m01, m02, m03] = self.rows[0];
        let [m10, m11, m12, m13] = self.rows[1];
        let [m20, m21, m22, m23] = self.rows[2];
        let [m30, m31, m32, m33] = self.rows[3];

        // compute the determinant of the 2x2 matrices required
        let d22_33 = det2x2([[m22, m23], [m32, m33]]);
        let d21_33 = det2x2([[m21, m23], [m31, m33]]);
        let d21_32 = det2x2([[m21, m22], [m31, m32]]);
        let d20_33 = det2x2([[m20, m23], [m30, m33]]);
        let d20_32 = det2x2([[m20, m22], [m30, m32]]);
        let d20_31 = det2x2([[m20, m21], [m30, m31]]);

        // compute the determinant of the 3x3 matrices
        m00 * (m11 * d22_33 - m12 * d21_33 + m13 * d21_32)
            - m01 * (m10 * d22_33 - m12 * d20_33 + m13 * d20_32)
            + m02 * (m10 * d21_33 - m11 * d20_33 + m13 * d20_31)
            - m03 * (m10 * d21_32 - m11 * d20_32 + m12 * d20_31)
    }

    /// Returns the inverse of the given matrix.
    /// If the matrix is uninvertible, `None` is returned.
    pub fn inverse(&self) -> Option<Self> {
        let [m00, m01, m02, m03] = self.rows[0];
        let [m10, m11, m12, m13] = self.rows[1];
        let [m20, m21, m22, m23] = self.rows[2];
        let [m30, m31, m32, m33] = self.rows[3];

        let mut res = Self::default();
        res[(0, 0)] = m11 * (m22 * m33 - m23 * m32) - m12 * (m21 * m33 - m23 * m31)
            + m13 * (m21 * m32 - m22 * m31);

        res[(1, 0)] = -m10 * (m22 * m33 - m23 * m32) + m12 * (m20 * m33 - m23 * m30)
            - m13 * (m20 * m32 - m22 * m30);

        res[(2, 0)] = m10 * (m21 * m33 - m23 * m31) - m11 * (m20 * m33 - m23 * m30)
            + m13 * (m20 * m31 - m21 * m30);

        res[(3, 0)] = -m10 * (m21 * m32 - m22 * m31) + m11 * (m20 * m32 - m22 * m30)
            - m12 * (m20 * m31 - m21 * m30);

        res[(0, 1)] = -m01 * (m22 * m33 - m23 * m32) + m02 * (m21 * m33 - m23 * m31)
            - m03 * (m21 * m32 - m22 * m31);

        res[(1, 1)] = m00 * (m22 * m33 - m23 * m32) - m02 * (m20 * m33 - m23 * m30)
            + m03 * (m20 * m32 - m22 * m30);

        res[(2, 1)] = -m00 * (m21 * m33 - m23 * m31) + m01 * (m20 * m33 - m23 * m30)
            - m03 * (m20 * m31 - m21 * m30);

        res[(3, 1)] = m00 * (m21 * m32 - m22 * m31) - m01 * (m20 * m32 - m22 * m30)
            + m02 * (m20 * m31 - m21 * m30);

        res[(0, 2)] = m01 * (m12 * m33 - m13 * m32) - m02 * (m11 * m33 - m13 * m31)
            + m03 * (m11 * m32 - m12 * m31);

        res[(1, 2)] = -m00 * (m12 * m33 - m13 * m32) + m02 * (m10 * m33 - m13 * m30)
            - m03 * (m10 * m32 - m12 * m30);

        res[(2, 2)] = m00 * (m11 * m33 - m13 * m31) - m01 * (m10 * m33 - m13 * m30)
            + m03 * (m10 * m31 - m11 * m30);

        res[(3, 2)] = -m00 * (m11 * m32 - m12 * m31) + m01 * (m10 * m32 - m12 * m30)
            - m02 * (m10 * m31 - m11 * m30);

        res[(0, 3)] = -m01 * (m12 * m23 - m13 * m22) + m02 * (m11 * m23 - m13 * m21)
            - m03 * (m11 * m22 - m12 * m21);

        res[(1, 3)] = m00 * (m12 * m23 - m13 * m22) - m02 * (m10 * m23 - m13 * m20)
            + m03 * (m10 * m22 - m12 * m20);

        res[(2, 3)] = -m00 * (m11 * m23 - m13 * m21) + m01 * (m10 * m23 - m13 * m20)
            - m03 * (m10 * m21 - m11 * m20);

        res[(3, 3)] = m00 * (m11 * m22 - m12 * m21) - m01 * (m10 * m22 - m12 * m20)
            + m02 * (m10 * m21 - m11 * m20);

        // res now contains the adjugate matrix.
        // we determine the determinant from the first row of the input matrix
        // and the first column of the adjugate.
        let det = self[(0, 0)] * res[(0, 0)]
            + self[(0, 1)] * res[(1, 0)]
            + self[(0, 2)] * res[(2, 0)]
            + self[(0, 3)] * res[(3, 0)];

        if det == 0.0 {
            return None;
        }

        for i in 0..4 {
            for j in 0..4 {
                res[(i, j)] /= det;
            }
        }

        Some(res)
    }

    /// Returns an iterator that gives elements in row-major order.
    pub fn iter(&self) -> impl Iterator<Item = &f32> {
        self.rows.iter().flatten()
    }

    /// Constructs a translation transfrom matrix from the given `Vec3`.
    pub fn new_translation(translation: Vec3) -> Self {
        let mut res = Self::identity();

        res[(0, 3)] = translation.x();
        res[(1, 3)] = translation.y();
        res[(2, 3)] = translation.z();

        res
    }

    /// Constructs a scaling transform matrix from the given `Vec3`.
    pub fn new_scaling(scaling: Vec3) -> Self {
        let mut res = Self::default();

        res[(0, 0)] = scaling.x();
        res[(1, 1)] = scaling.y();
        res[(2, 2)] = scaling.z();
        res[(3, 3)] = 1.0;

        res
    }

    /// Constructs a rotation transform matrix that rotates around
    /// the x-axis clockwise with the given angle in radians.
    pub fn new_rotation_x(angle: f32) -> Self {
        let (sin, cos) = angle.sin_cos();

        let mut res = Self::default();

        res[(0, 0)] = 1.0;
        res[(1, 1)] = cos;
        res[(1, 2)] = -sin;
        res[(2, 1)] = sin;
        res[(2, 2)] = cos;
        res[(3, 3)] = 1.0;

        res
    }

    /// Constructs a rotation transform matrix that rotates around
    /// the y-axis clockwise with the given angle in radians.
    pub fn new_rotation_y(angle: f32) -> Self {
        let (sin, cos) = angle.sin_cos();

        let mut res = Self::default();

        res[(0, 0)] = cos;
        res[(0, 2)] = sin;
        res[(1, 1)] = 1.0;
        res[(2, 0)] = -sin;
        res[(2, 2)] = cos;
        res[(3, 3)] = 1.0;

        res
    }

    /// Constructs a rotation transform matrix that rotates around
    /// the z-axis clockwise with the given angle in radians.
    pub fn new_rotation_z(angle: f32) -> Self {
        let (sin, cos) = angle.sin_cos();

        let mut res = Self::default();

        res[(0, 0)] = cos;
        res[(0, 1)] = -sin;
        res[(1, 0)] = sin;
        res[(1, 1)] = cos;
        res[(2, 2)] = 1.0;
        res[(3, 3)] = 1.0;

        res
    }

    /// Constructs a shearing transform matrix.
    pub fn new_shearing(dx_y: f32, dx_z: f32, dy_x: f32, dy_z: f32, dz_x: f32, dz_y: f32) -> Self {
        let mut res = Mat4::identity();

        res[(0, 1)] = dx_y;
        res[(0, 2)] = dx_z;
        res[(1, 0)] = dy_x;
        res[(1, 2)] = dy_z;
        res[(2, 0)] = dz_x;
        res[(2, 1)] = dz_y;

        res
    }

    /// Multiply a translation transform.
    pub fn translate(self, translation: Vec3) -> Self {
        Self::new_translation(translation) * &self
    }

    /// Multiply a scaling transform.
    pub fn scale(self, scaling: Vec3) -> Self {
        Self::new_scaling(scaling) * &self
    }

    /// Multiply a transform which rotates around the x-axis clockwise
    /// with the given angle in radians.
    pub fn rotate_x(self, angle: f32) -> Self {
        Self::new_rotation_x(angle) * &self
    }

    /// Multiply a transform which rotates around the y-axis clockwise
    /// with the given angle in radians.
    pub fn rotate_y(self, angle: f32) -> Self {
        Self::new_rotation_y(angle) * &self
    }

    /// Multiply a transform which rotates around the z-axis clockwise
    /// with the given angle in radians.
    pub fn rotate_z(self, angle: f32) -> Self {
        Self::new_rotation_z(angle) * &self
    }

    /// Multiply a shear transform.
    pub fn shear(self, dx_y: f32, dx_z: f32, dy_x: f32, dy_z: f32, dz_x: f32, dz_y: f32) -> Self {
        Self::new_shearing(dx_y, dx_z, dy_x, dy_z, dz_x, dz_y) * &self
    }

    /// Returns a transform for orienting the view.
    /// We pretend to transform the eye, but transform the world instead.
    ///
    /// Note: The `up` vector does not need to be exactly perpedicular
    /// and can be an approximate direction.
    pub fn view_transform(from: Point3, to: Point3, up: Vec3) -> Self {
        let forward = (to - from).normalize();
        let left = forward.cross(up.normalize());
        let true_up = left.cross(forward);

        let orientation: Mat4 = [
            [left.x(), left.y(), left.z(), 0.0],
            [true_up.x(), true_up.y(), true_up.z(), 0.0],
            [-forward.x(), -forward.y(), -forward.z(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]
        .into();

        let transform = Mat4::new_translation((-from.x(), -from.y(), -from.z()).into());

        orientation * &transform
    }
}

/// Helper function to calculate the determinant of 2x2 matrix.
fn det2x2(mat: [[f32; 2]; 2]) -> f32 {
    mat[0][0] * mat[1][1] - mat[0][1] * mat[1][0]
}

impl From<[[f32; 4]; 4]> for Mat4 {
    fn from(rows: [[f32; 4]; 4]) -> Self {
        Self { rows }
    }
}

impl ops::Mul<&Mat4> for Mat4 {
    type Output = Mat4;
    fn mul(self, rhs: &Mat4) -> Self::Output {
        let mut out: Self::Output = Default::default();

        for row in 0..4 {
            for col in 0..4 {
                out[(row, col)] = self[(row, 0)] * rhs[(0, col)]
                    + self[(row, 1)] * rhs[(1, col)]
                    + self[(row, 2)] * rhs[(2, col)]
                    + self[(row, 3)] * rhs[(3, col)];
            }
        }

        out
    }
}
impl ops::Mul<&Mat4> for &Mat4 {
    type Output = Mat4;
    fn mul(self, rhs: &Mat4) -> Self::Output {
        self.clone() * rhs
    }
}

impl ops::Mul<Vec3> for &Mat4 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        let mut out: Self::Output = Default::default();

        for row in 0..4 {
            out[row] = self[(row, 0)] * rhs[0]
                + self[(row, 1)] * rhs[1]
                + self[(row, 2)] * rhs[2]
                + self[(row, 3)] * rhs[3];
        }

        out
    }
}

impl ops::Mul<Point3> for &Mat4 {
    type Output = Point3;
    fn mul(self, rhs: Point3) -> Self::Output {
        let mut out: Self::Output = Default::default();

        for row in 0..4 {
            out[row] = self[(row, 0)] * rhs[0]
                + self[(row, 1)] * rhs[1]
                + self[(row, 2)] * rhs[2]
                + self[(row, 3)] * rhs[3];
        }

        out
    }
}

impl ops::Index<usize> for Mat4 {
    type Output = Row;
    fn index(&self, index: usize) -> &Self::Output {
        &self.rows[index]
    }
}

impl ops::Index<(usize, usize)> for Mat4 {
    type Output = f32;
    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.rows[i][j]
    }
}

impl ops::IndexMut<usize> for Mat4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.rows[index]
    }
}

impl ops::IndexMut<(usize, usize)> for Mat4 {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        &mut self.rows[i][j]
    }
}

impl AbsDiffEq for Mat4 {
    type Epsilon = <f32 as AbsDiffEq>::Epsilon;
    fn default_epsilon() -> Self::Epsilon {
        0.0001
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.iter()
            .zip(other.iter())
            .all(|(l, r)| f32::abs_diff_eq(l, r, epsilon))
    }
}

impl RelativeEq for Mat4 {
    fn default_max_relative() -> Self::Epsilon {
        f32::default_max_relative()
    }
    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.iter()
            .zip(other.iter())
            .all(|(l, r)| f32::relative_eq(l, r, epsilon, max_relative))
    }
}
