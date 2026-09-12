use std::sync::Arc;

use crate::{
    camera::{Hit, Ray},
    material::Material,
    vec3::Vec3,
};

pub struct World {
    spheres: Vec<Sphere>,
}

impl World {
    pub fn new() -> Self {
        Self {
            spheres: Vec::default(),
        }
    }

    pub fn draw_sphere(&mut self, location: Vec3, radius: f64, material: Arc<Material>) {
        self.spheres.push(Sphere {
            location,
            radius,
            material,
        });
    }

    pub fn hit(&self, ray: &mut Ray) {
        Self::hit_spheres(&self.spheres, ray);
    }

    fn hit_spheres(array: &[Sphere], ray: &mut Ray) {
        for sphere in array {
            sphere.hit(ray);
        }
    }
}

struct Sphere {
    location: Vec3,
    radius: f64,
    material: Arc<Material>,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &mut Ray) {
        let oc = self.location - ray.origin;
        let a = ray.direction.dot(&ray.direction);
        let h = ray.direction.dot(&oc);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if !ray.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray.surrounds(root) {
                return;
            };
        }

        ray.max_t = root;
        let p = ray.at(root);
        let normal = (p - self.location) / self.radius;
        ray.hit = Some(Hit::new(normal, p, ray, self.material.clone()));
    }
}

trait Hittable {
    fn hit(&self, ray: &mut Ray);
}
