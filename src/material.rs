use core::f64::math::mul_add;
use std::intrinsics::{minimumf64, powf64};

use crate::{
    camera::{Hit, Ray},
    vec3::Vec3,
};

pub struct Scatter {
    pub ray: Ray,
    pub color: Vec3,
}

pub struct Lambertian {
    color: Vec3,
}

impl Lambertian {
    fn scatter(&self, hit: &Hit) -> Scatter {
        let mut scatter_direction = hit.normal + Vec3::random_unit_vector_with_range(-1.0, 1.0);
        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
        }
        Scatter {
            ray: Ray::new(hit.p, scatter_direction),
            color: self.color,
        }
    }
}

pub struct Metal {
    color: Vec3,
    fuzz: f64,
}

impl Metal {
    fn scatter(&self, hit: &Hit, direction: &Vec3) -> Option<Scatter> {
        let refl = direction.reflect(hit.normal);
        let reflected = refl.unit_vector()
            + (Vec3::splat(self.fuzz) * Vec3::random_unit_vector_with_range(-1., 1.));
        if reflected.dot(&hit.normal) <= 0.0 {
            return None;
        }

        Some(Scatter {
            ray: Ray::new(hit.p, reflected),
            color: self.color,
        })
    }
}
//
pub struct Dilectric {
    refraction_index: f64,
}

impl Dilectric {
    fn scatter(&self, hit: &Hit, direction: &Vec3) -> Scatter {
        let ri = if hit.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = direction.unit_vector();
        let cos_theta = minimumf64(-unit_direction.dot(&hit.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let dir = if ri * sin_theta > 1.0 || Self::reflectance(cos_theta, ri) > rand::random() {
            unit_direction.reflect(hit.normal)
        } else {
            unit_direction.refract(&hit.normal, ri)
        };

        Scatter {
            color: Vec3::ONE,
            ray: Ray::new(hit.p, dir),
        }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let r0 = (1. - refraction_index) / (1. + refraction_index);
        let r = r0 * r0;
        mul_add(1.0 - r, powf64(1.0 - cosine, 5.0), r)
    }
}

pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dilectric),
}

impl Material {
    pub fn scatter(&self, hit: &Hit, direction: &Vec3) -> Option<Scatter> {
        match self {
            Material::Lambertian(l) => Some(l.scatter(hit)),
            Material::Metal(m) => m.scatter(hit, direction),
            Material::Dielectric(d) => Some(d.scatter(hit, direction)),
        }
    }

    pub fn init_lambertian(color: Vec3) -> Material {
        Material::Lambertian(Lambertian { color })
    }

    pub fn init_metal(color: Vec3, fuzz: f64) -> Material {
        Material::Metal(Metal { color, fuzz })
    }

    pub fn init_dielectric(refraction_index: f64) -> Material {
        Material::Dielectric(Dilectric { refraction_index })
    }
}
