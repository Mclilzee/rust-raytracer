#![allow(internal_features)]
#![feature(portable_simd, core_float_math, core_intrinsics)]

use std::sync::Arc;

use crate::{camera::Camera, material::Material, vec3::Vec3, world::World};

mod camera;
mod color;
mod material;
mod vec3;
mod world;

fn main() {
    let camera = Camera::new();
    let mut world = World::new();

    let ground = Arc::new(Material::init_lambertian(Vec3::new(0.5, 0.8, 0.0)));
    world.draw_sphere(Vec3::new(0., -1000., 0.), 1000., ground);
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f64>();
            let center = Vec3::new(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );
            if (center - Vec3::new(4., 0.2, 0.)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Vec3::new(
                        rand::random::<f64>(),
                        rand::random::<f64>(),
                        rand::random::<f64>(),
                    );
                    let sphere_material = Arc::new(Material::init_lambertian(albedo));
                    world.draw_sphere(center, 0.2, sphere_material);
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_unit_vector_with_range(0.5, 1.);
                    let fuzz = rand::random::<f64>() * 0.5;
                    let sphere_material = Arc::new(Material::init_metal(albedo, fuzz));
                    world.draw_sphere(center, 0.2, sphere_material);
                } else {
                    let sphere_material = Arc::new(Material::init_dielectric(1.5));
                    world.draw_sphere(center, 0.2, sphere_material);
                }
            }
        }
    }

    let ground = Arc::new(Material::init_lambertian(Vec3::new(0.5, 0.8, 0.0)));
    world.draw_sphere(Vec3::new(0., -1000., 0.), 1000., ground);
    let material1 = Arc::new(Material::init_dielectric(1.5));
    world.draw_sphere(Vec3::new(0., 1., 0.), 1.0, material1);

    let material2 = Arc::new(Material::init_lambertian(Vec3::new(0.4, 0.2, 0.1)));
    world.draw_sphere(Vec3::new(-4., 1., 0.), 1.0, material2);

    let material3 = Arc::new(Material::init_metal(Vec3::new(0.7, 0.6, 0.5), 0.));
    world.draw_sphere(Vec3::new(4., 1., 0.), 1.0, material3);

    camera.render(&world);
}
