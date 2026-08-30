#![allow(internal_features)]
#![feature(portable_simd, core_float_math, core_intrinsics)]

use std::rc::Rc;

use crate::{camera::Camera, material::Material, vec3::Vec3, world::World};

mod camera;
mod color;
mod material;
mod vec3;
mod world;

fn main() {
    let camera = Camera::new();
    let ground = Rc::new(Material::init_lambertian(Vec3::new(0.8, 0.8, 0.0)));
    let center = Rc::new(Material::init_lambertian(Vec3::new(0.1, 0.2, 0.5)));
    let left = Rc::new(Material::init_dielectric(1.5));
    let bubble = Rc::new(Material::init_dielectric(1.0 / 1.5));
    let right = Rc::new(Material::init_metal(Vec3::new(0.8, 0.6, 0.2), 1.0));

    let mut world = World::new();
    world.draw_sphere(Vec3::new(0.0, -100.5, -1.0), 100.0, ground);
    world.draw_sphere(Vec3::new(0.0, -0.0, -1.2), 0.5, center);
    world.draw_sphere(Vec3::new(-1.0, 0.0, -1.0), 0.5, left);
    world.draw_sphere(Vec3::new(-1.0, 0.0, -1.0), 0.4, bubble);
    world.draw_sphere(Vec3::new(1.0, 0.0, -1.0), 0.5, right);
    camera.render(&world);
}
