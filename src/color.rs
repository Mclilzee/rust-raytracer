use std::simd::Simd;

use crate::vec3::Vec3;

pub const GREEN: Color = Color { r: 0, g: 255, b: 0 };
pub const RED: Color = Color { r: 255, g: 0, b: 0 };
pub const BLUE: Color = Color { r: 0, g: 0, b: 255 };
pub const WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
};
pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Into<[u8; 3]> for Color {
    fn into(self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

impl Color {
    pub fn init(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }
}

impl From<Vec3> for Color {
    fn from(value: Vec3) -> Self {
        let gamma_vec = Vec3::new(
            linear_to_gamma(value.x()),
            linear_to_gamma(value.y()),
            linear_to_gamma(value.z()),
        )
        .clamp(0., 0.999);

        let color = Vec3::new(256.0, 256.0, 256.0) * gamma_vec;
        return Self {
            r: color.x() as u8,
            g: color.y() as u8,
            b: color.z() as u8,
        };
    }
}

fn linear_to_gamma(n: f64) -> f64 {
    if n > 0. {
        return n.sqrt();
    }

    return 0.;
}
