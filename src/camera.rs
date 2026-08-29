use core::f64::math::mul_add;

use crate::vec3::{ONE, Vec3, ZERO};

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
    // min_t: f64 = 0.001,
    // max_t: f64 = f64::INFINITY,
    // hit: Optional<Hit>,
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        self.direction.clone() * t + self.origin.clone()
    }

    // pub fn cast(&mut self, world: *World, depth: u16) -> Vec3 {
    pub fn cast(&mut self, depth: u16) -> Vec3 {
        return ZERO;
        //     if (depth == 0) {
        //         return ZERO;
        //     }
        //
        //     world.hit(self);
        //     if (self.hit) |h| {
        //         const scatter = h.material.scatter(rand, &h, self.direction) orelse return vec.zero;
        //         self.* = scatter.ray;
        //         return scatter.color * self.cast(world, depth - 1);
        //     }
        //
        //     const unit_direction = vec.unitVector(self.direction);
        //     const a = vec.splat(0.5 * (unit_direction[1] + 1.0));
        //     return (vec.one - a) * vec.one + a * Vec3{ 0.5, 0.7, 1.0 };
        // }
        //
        // pub fn contains(self: Self, t: f64) bool {
        //     return self.min_t <= t and t <= self.max_t;
        // }
        //
        // pub fn surrounds(self: Self, t: f64) bool {
        //     return self.min_t < t and t < self.max_t;
        // }
    }
}

// pub const Hit = struct {
//     t: f64,
//     normal: Vec3,
//     p: Vec3,
//     material: *const Material,
//     front_face: bool = false,
//     const Self = @This();
//
//     pub fn init(t: f64, normal: Vec3, p: Vec3, ray: *Ray, m: *const Material) Self {
//         const front_face = vec.dot(ray.direction, normal) < 0;
//         const hit_normal = if (front_face) normal else -normal;
//         return Hit{ .t = t, .normal = hit_normal, .p = p, .front_face = front_face, .material = m };
//     }
// }
