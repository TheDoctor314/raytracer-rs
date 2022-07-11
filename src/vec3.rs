use approx::{AbsDiffEq, RelativeEq};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3([f32; 4]);

impl Point3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self([x, y, z, 1.0])
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3([f32; 4]);

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self([x, y, z, 0.0])
    }
}

impl AbsDiffEq for Point3 {
    type Epsilon = <f32 as AbsDiffEq>::Epsilon;
    fn default_epsilon() -> Self::Epsilon {
        f32::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.0
            .into_iter()
            .zip(other.0.into_iter())
            .take(3)
            .all(|(l, r)| f32::abs_diff_eq(&l, &r, epsilon))
    }
}

impl RelativeEq for Point3 {
    fn default_max_relative() -> Self::Epsilon {
        f32::default_max_relative()
    }
    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.0
            .into_iter()
            .zip(other.0.into_iter())
            .take(3)
            .all(|(l, r)| f32::relative_eq(&l, &r, epsilon, max_relative))
    }
}

impl AbsDiffEq for Vec3 {
    type Epsilon = <f32 as AbsDiffEq>::Epsilon;
    fn default_epsilon() -> Self::Epsilon {
        f32::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.0
            .into_iter()
            .zip(other.0.into_iter())
            .take(3)
            .all(|(l, r)| f32::abs_diff_eq(&l, &r, epsilon))
    }
}

impl RelativeEq for Vec3 {
    fn default_max_relative() -> Self::Epsilon {
        f32::default_max_relative()
    }
    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.0
            .into_iter()
            .zip(other.0.into_iter())
            .take(3)
            .all(|(l, r)| f32::relative_eq(&l, &r, epsilon, max_relative))
    }
}
