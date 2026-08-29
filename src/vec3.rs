use core::f64::math::mul_add;
use std::{
    ops::{Add, Mul},
    random::{Rng, SystemRng},
    simd::{Simd, num::SimdFloat},
};

use rand::RngExt;
pub const ZERO: Vec3 = Vec3::new(0.0, 0.0, 0.0);
pub const ONE: Vec3 = Vec3::new(1.0, 1.0, 1.0);

#[derive(Clone, PartialEq)]
pub struct Vec3 {
    pub value: Simd<f64, 3>,
}

impl Vec3 {
    pub fn random_unit_vector_with_range(min: f64, max: f64) -> Self {
        loop {
            let p = Simd::from_array([
                mul_add(rand::random::<f64>(), (max - min), min),
                mul_add(rand::random::<f64>(), (max - min), min),
                mul_add(rand::random::<f64>(), (max - min), min),
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
        const s: Simd<f64, 3> = Simd::splat(1e-8);
        self.value < s
    }

    pub fn reflect(self, rhs: Self) -> Vec3 {
        Self {
            value: self.value - Simd::splat(2.) * Simd::splat(self.dot(&rhs)) * rhs.value,
        }
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

impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            value: self.value + rhs.value,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        self * Self::splat(rhs)
    }
}
