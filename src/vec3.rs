use std::ops;

use approx::{AbsDiffEq, RelativeEq};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3([f32; 4]);

impl Point3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self([x, y, z, 1.0])
    }

    pub fn x(&self) -> f32 {
        self.0[0]
    }

    pub fn y(&self) -> f32 {
        self.0[1]
    }

    pub fn z(&self) -> f32 {
        self.0[2]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec3([f32; 4]);

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self([x, y, z, 0.0])
    }

    pub fn x(&self) -> f32 {
        self.0[0]
    }

    pub fn y(&self) -> f32 {
        self.0[1]
    }

    pub fn z(&self) -> f32 {
        self.0[2]
    }

    pub fn mag(&self) -> f32 {
        self.mag_sq().sqrt()
    }

    pub fn mag_sq(&self) -> f32 {
        self.0.into_iter().map(|v| v * v).sum()
    }

    pub fn normalize(&self) -> Self {
        let mut clone = *self;
        clone.normalized();

        clone
    }

    pub fn normalized(&mut self) {
        let mag = self.mag();
        let (_, slice) = self.0.split_last_mut().unwrap();

        for v in slice {
            *v /= mag;
        }
    }

    pub fn dot(&self, rhs: Self) -> f32 {
        let lhs = &self.0;
        let rhs = &rhs.0;

        lhs.iter().zip(rhs.iter()).map(|(l, r)| l * r).sum()
    }

    pub fn cross(&self, rhs: Self) -> Self {
        let ax = self.x();
        let ay = self.y();
        let az = self.z();

        let bx = rhs.x();
        let by = rhs.y();
        let bz = rhs.z();

        let mut res: Vec3 = Default::default();
        res.0[0] = ay * bz - az * by;
        res.0[1] = az * bx - ax * bz;
        res.0[2] = ax * by - ay * bx;

        res
    }
}

// Vec3 + Vec3 -> Vec3
impl ops::Add for Vec3 {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        let lhs = &mut self.0;
        let rhs = &rhs.0;

        for (lhs, rhs) in lhs.iter_mut().zip(rhs.iter()) {
            *lhs += rhs;
        }

        Self(*lhs)
    }
}

// Point3 + Vec3 -> Point3
impl ops::Add<Vec3> for Point3 {
    type Output = Point3;
    fn add(mut self, rhs: Vec3) -> Self::Output {
        let lhs = &mut self.0;
        let rhs = &rhs.0;

        for (lhs, rhs) in lhs.iter_mut().zip(rhs.iter()) {
            *lhs += rhs;
        }

        Self(*lhs)
    }
}

// Point3 - Point3 -> Vec3
impl ops::Sub for Point3 {
    type Output = Vec3;
    fn sub(mut self, rhs: Self) -> Self::Output {
        let lhs = &mut self.0;
        let rhs = &rhs.0;

        for (lhs, rhs) in lhs.iter_mut().zip(rhs.iter()) {
            *lhs -= rhs;
        }

        Vec3(*lhs)
    }
}

// Point3 - Vec3 -> Point3
impl ops::Sub<Vec3> for Point3 {
    type Output = Self;
    fn sub(mut self, rhs: Vec3) -> Self::Output {
        let lhs = &mut self.0;
        let rhs = &rhs.0;

        for (lhs, rhs) in lhs.iter_mut().zip(rhs.iter()) {
            *lhs -= rhs;
        }

        Self(*lhs)
    }
}

// Vec3 - Vec3 -> Vec3
impl ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(mut self, rhs: Self) -> Self::Output {
        let lhs = &mut self.0;
        let rhs = &rhs.0;

        for (lhs, rhs) in lhs.iter_mut().zip(rhs.iter()) {
            *lhs -= rhs;
        }

        Self(*lhs)
    }
}

impl ops::Neg for Point3 {
    type Output = Point3;
    fn neg(mut self) -> Self::Output {
        let inner = &mut self.0;
        inner[0] = -inner[0];
        inner[1] = -inner[1];
        inner[2] = -inner[2];

        self
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(mut self) -> Self::Output {
        let inner = &mut self.0;
        inner[0] = -inner[0];
        inner[1] = -inner[1];
        inner[2] = -inner[2];

        self
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(mut self, rhs: f32) -> Self::Output {
        let inner = &mut self.0;
        inner[0] *= rhs;
        inner[1] *= rhs;
        inner[2] *= rhs;

        self
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Self::Output {
        let recip = rhs.recip();
        self * recip
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl AbsDiffEq for Point3 {
    type Epsilon = <f32 as AbsDiffEq>::Epsilon;
    fn default_epsilon() -> Self::Epsilon {
        0.0001
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
        0.0001
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
