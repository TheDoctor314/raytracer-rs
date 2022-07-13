use std::ops;

use approx::{AbsDiffEq, RelativeEq};

use crate::vec3::Vec3;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Mat4 {
    rows: [[f32; 4]; 4],
}

type Row = [f32; 4];

impl Mat4 {
    pub fn new(row0: Row, row1: Row, row2: Row, row3: Row) -> Self {
        Self {
            rows: [row0, row1, row2, row3],
        }
    }

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

    pub fn transpose(&self) -> Self {
        let mut res = self.clone();

        for i in 0..4 {
            for j in 0..4 {
                res[(i, j)] = self[(j, i)];
            }
        }

        res
    }

    pub fn transposed(&mut self) {
        *self = self.transpose();
    }

    pub fn iter(&self) -> impl Iterator<Item = &f32> {
        self.rows.iter().flatten()
    }
}

impl From<[[f32; 4]; 4]> for Mat4 {
    fn from(rows: [[f32; 4]; 4]) -> Self {
        Self { rows }
    }
}

impl ops::Mul<&Mat4> for &Mat4 {
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
        f32::default_epsilon()
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
