pub mod matrix;
pub mod vec3;

use std::ops;

pub use approx::relative_eq;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color(image::Rgb<f32>);

impl Color {
    pub fn into_inner(self) -> image::Rgb<f32> {
        self.0
    }

    pub fn blend(&self, rhs: Self) -> Self {
        let mut clone = *self;

        for i in 0..3 {
            clone.0[i] *= rhs.0[i];
        }

        clone
    }
}

impl From<[f32; 3]> for Color {
    fn from(i: [f32; 3]) -> Self {
        Self(i.into())
    }
}

impl ops::Add for Color {
    type Output = Color;
    fn add(mut self, rhs: Self) -> Self::Output {
        for i in 0..3 {
            self.0[i] += rhs.0[i];
        }

        self
    }
}

impl ops::Sub for Color {
    type Output = Color;
    fn sub(mut self, rhs: Self) -> Self::Output {
        for i in 0..3 {
            self.0[i] -= rhs.0[i];
        }

        self
    }
}

impl ops::Mul<f32> for Color {
    type Output = Color;
    fn mul(mut self, rhs: f32) -> Self::Output {
        for i in 0..3 {
            self.0[i] *= rhs;
        }

        self
    }
}

impl approx::AbsDiffEq for Color {
    type Epsilon = <f32 as approx::AbsDiffEq>::Epsilon;
    fn default_epsilon() -> Self::Epsilon {
        f32::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        for i in 0..3 {
            if !f32::abs_diff_eq(&self.0[i], &other.0[i], epsilon) {
                return false;
            }
        }

        true
    }
}

impl approx::RelativeEq for Color {
    fn default_max_relative() -> Self::Epsilon {
        f32::default_max_relative()
    }
    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        for i in 0..3 {
            if !f32::relative_eq(&self.0[i], &other.0[i], epsilon, max_relative) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests;
