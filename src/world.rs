use crate::{
    camera::{Hit, Ray},
    material::Material,
    vec3::Vec3,
};

pub struct World {
    spheres: Vec<Sphere>,
    materials: Vec<Material>,
}

impl World {
    pub fn new() -> Self {
        Self {
            spheres: Vec::default(),
            materials: Vec::default()
        }
    }

    pub fn add_material(&mut self, material: Material) -> u16 {
        let index = self.materials.len();
        self.materials.push(material);
        index as u16
    }

    pub fn draw_sphere(&mut self, location: Vec3, radius: f64, material_index: u16) {
        self.spheres.push(Sphere {
            location,
            radius,
            material_index,
        });
    }

    pub fn get_material(&self, index: u16) -> Option<&Material> {
        self.materials.get(index as usize)
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
    material_index: u16,
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
        ray.hit = Some(Hit::new(normal, p, ray, self.material_index));
    }
}

trait Hittable {
    fn hit(&self, ray: &mut Ray);
}
