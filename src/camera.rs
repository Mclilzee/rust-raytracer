use std::{
    io::{BufWriter, Stdout, Write},
    rc::Rc,
};

use crate::{color::Color, material::Material, vec3::Vec3, world::World};

const VFOV: f64 = 20.0;
const LOOK_FROM: Vec3 = Vec3::new(-2.0, 2.0, 1.0);
const LOOK_AT: Vec3 = Vec3::new(0.0, 0.0, -1.0);
const VUP: Vec3 = Vec3::new(0.0, 1.0, 0.0);

const THETA: f64 = f64::to_radians(VFOV);
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const ANTI_ALIACING_SAMPLES: usize = 100;
const PIXEL_SAMPLES_SCALE: Vec3 = Vec3::splat(1.0 / ANTI_ALIACING_SAMPLES as f64);
const IMAGE_WIDTH: usize = 400;
const IMAGE_HEIGHT: usize = const {
    let height: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    if height < 1 { 1 } else { height }
};
const MAX_BOUNCE_DEPTH: u16 = 50;
const CENTER: Vec3 = LOOK_FROM;

pub struct Camera {
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let w: Vec3 = (LOOK_FROM - LOOK_AT).unit_vector();
        let u: Vec3 = VUP.cross(&w).unit_vector();
        let v: Vec3 = w.cross(&u);
        let focal_length = (LOOK_FROM - LOOK_AT).length();
        let h: f64 = (THETA / 2.0).tan();
        let viewport_height: f64 = 2.0 * h * focal_length;
        let viewport_width: f64 = viewport_height * IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64;
        let viewport_u: Vec3 = Vec3::splat(viewport_width) * u;
        let viewport_v: Vec3 = Vec3::splat(viewport_height) * -v;
        let pixel_delta_u: Vec3 = viewport_u / Vec3::splat(IMAGE_WIDTH as f64);
        let pixel_delta_v: Vec3 = viewport_v / Vec3::splat(IMAGE_HEIGHT as f64);
        let viewport_upper_left: Vec3 = CENTER
            - (Vec3::splat(focal_length) * w)
            - viewport_u / Vec3::splat(2.0)
            - viewport_v / Vec3::splat(2.0);

        Self {
            pixel00_loc: Vec3::splat(0.5) * (pixel_delta_u + pixel_delta_v) + viewport_upper_left,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: &World) {
        let mut pixels_buffer = vec![Color::BLACK; IMAGE_WIDTH * IMAGE_HEIGHT];
        // pixels_buffer.fill([0u8; 3]);
        let mut writer = std::io::BufWriter::new(std::io::stdout());
        // var threads = try self.alloc.alloc(std.Thread, image_height);
        for j in 0..IMAGE_HEIGHT {
            // threads[j] = try std.Thread.spawn(.{}, renderColumns, .{ world, j, pixels_buffer, &pr });
            self.render_columns(world, j, &mut pixels_buffer);
        }

        draw_pixels(&mut writer, &pixels_buffer);
        let _ = writer.flush();
    }

    fn render_columns(&self, world: &World, row: usize, pixels_buffer: &mut [Color]) {
        for i in 0..IMAGE_WIDTH {
            let mut hit_value = Vec3::ZERO;
            for _ in 0..ANTI_ALIACING_SAMPLES {
                let mut ray = self.get_ray(i as f64, row as f64);
                hit_value = hit_value + ray.cast(world, MAX_BOUNCE_DEPTH);
            }

            pixels_buffer[IMAGE_WIDTH * row + i] = (hit_value * PIXEL_SAMPLES_SCALE).into();
        }
    }

    fn get_ray(&self, i: f64, j: f64) -> Ray {
        let offset = Vec3::new(
            rand::random::<f64>() - 0.5,
            rand::random::<f64>() - 0.5,
            0.0,
        );
        let pixel_sample = self.pixel00_loc
            + (Vec3::splat(i + offset.x()) * self.pixel_delta_u)
            + ((Vec3::splat(j + offset.y())) * self.pixel_delta_v);
        Ray::new(CENTER, pixel_sample - CENTER)
    }
}

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    min_t: f64,
    pub max_t: f64,
    pub hit: Option<Hit>,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction,
            min_t: 0.001,
            max_t: f64::INFINITY,
            hit: None,
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        t * self.direction + self.origin
    }

    pub fn surrounds(&self, t: f64) -> bool {
        self.min_t < t && t < self.max_t
    }

    pub fn cast(&mut self, world: &World, depth: u16) -> Vec3 {
        if depth == 0 {
            return Vec3::ZERO;
        }

        world.hit(self);
        if let Some(hit) = self.hit.as_ref() {
            if let Some(scatter) = hit.material.scatter(hit, &self.direction) {
                *self = scatter.ray;
                return scatter.color * self.cast(world, depth - 1);
            } else {
                return Vec3::ZERO;
            }
        }

        let unit_direction = self.direction.unit_vector();
        let a = Vec3::splat(0.5 * (unit_direction.y() + 1.0));
        (Vec3::ONE - a) * Vec3::ONE + a * Vec3::new(0.5, 0.7, 1.0)
    }
}

pub struct Hit {
    pub normal: Vec3,
    pub p: Vec3,
    pub material: Rc<Material>,
    pub front_face: bool,
}

impl Hit {
    pub fn new(normal: Vec3, p: Vec3, ray: &Ray, m: Rc<Material>) -> Self {
        let front_face = ray.direction.dot(&normal) < 0.;
        let hit_normal = if front_face { normal } else { -normal };
        Hit {
            normal: hit_normal,
            p,
            front_face,
            material: m,
        }
    }
}

fn draw_pixels(writer: &mut BufWriter<Stdout>, pixels_buffer: &[Color]) {
    let _ = writer.write(format!("P6\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes());
    for color in pixels_buffer {
        let bytes: [u8; 3] = color.into();
        _ = writer.write(&bytes);
    }
}
