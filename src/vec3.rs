use core::f64::math::mul_add;
use std::{
    intrinsics::minimumf64,
    ops::{Add, Div, Mul, Neg, Sub},
    simd::{Simd, num::SimdFloat},
};

#[derive(Clone, PartialEq, Copy)]
pub struct Vec3 {
    pub value: Simd<f64, 3>,
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    pub const ONE: Vec3 = Vec3::new(1.0, 1.0, 1.0);
    pub fn random_unit_vector_with_range(min: f64, max: f64) -> Self {
        loop {
            let p = Simd::from_array([
                mul_add(rand::random::<f64>(), max - min, min),
                mul_add(rand::random::<f64>(), max - min, min),
                mul_add(rand::random::<f64>(), max - min, min),
            ]);

            let length = (p * p).reduce_sum() as f64;
            if 1e-160 < length && length <= 1. {
                return Self {
                    value: p / Simd::splat(length.sqrt()),
                };
            }
        }
    }

    pub fn x(&self) -> f64 {
        self.value[0]
    }

    pub fn y(&self) -> f64 {
        self.value[1]
    }

    pub fn z(&self) -> f64 {
        self.value[2]
    }

    pub fn clamp(&self, min: f64, max: f64) -> Self {
        Self {
            value: self.value.simd_clamp(Simd::splat(min), Simd::splat(max)),
        }
    }

    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            value: Simd::from_array([x, y, z]),
        }
    }

    pub const fn splat(n: f64) -> Self {
        Self {
            value: Simd::from_array([n, n, n]),
        }
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        (self.value * rhs.value).reduce_sum()
    }

    pub fn length(&self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn unit_vector(self) -> Self {
        Self {
            value: self.value / Simd::splat(self.length()),
        }
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        let l = &self.value;
        let r = &rhs.value;

        Self::new(
            l[1] * r[2] - l[2] * r[1],
            l[2] * r[0] - l[0] * r[2],
            l[0] * r[1] - l[1] * r[0],
        )
    }

    pub fn near_zero(&self) -> bool {
        const S: Simd<f64, 3> = Simd::splat(1e-8);
        self.value < S
    }

    pub fn reflect(self, rhs: Self) -> Vec3 {
        Self {
            value: self.value - Simd::splat(2.) * Simd::splat(self.dot(&rhs)) * rhs.value,
        }
    }

    pub fn refract(self, rhs: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = minimumf64(-self.dot(rhs), 1.0);
        let r_out_perp = etai_over_etat * cos_theta * rhs + self;
        -(1.0 - r_out_perp.dot(&r_out_perp)).abs().sqrt() * rhs + r_out_perp
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value * rhs.value,
        }
    }
}

impl Mul for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            value: self.value * rhs.value,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value,
        }
    }
}

impl Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            value: self.value + rhs.value,
        }
    }
}

impl Add<&Vec3> for f64 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            value: Simd::splat(self) + rhs.value,
        }
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            value: Simd::splat(self) * rhs.value,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            value: Simd::splat(self) * rhs.value,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value - rhs.value,
        }
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            value: self.value - rhs.value,
        }
    }
}

impl Sub<&Vec3> for f64 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            value: Simd::splat(self) - rhs.value,
        }
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value / rhs.value,
        }
    }
}

impl Div<&Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            value: self.value / rhs.value,
        }
    }
}

impl Div<&Vec3> for f64 {
    type Output = Vec3;

    fn div(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            value: Simd::splat(self) / rhs.value,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            value: self.value / Simd::splat(rhs),
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 { value: -self.value }
    }
}
