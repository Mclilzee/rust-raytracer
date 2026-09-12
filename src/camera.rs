use std::{
    io::{BufWriter, Stdout, Write},
    sync::Arc,
};

use crate::{color::Color, material::Material, vec3::Vec3, world::World};

const DEFOCUS_ANGLE: f64 = 0.06;
const FOCUS_DIST: f64 = 10.0;
const VFOV: f64 = 20.0;
const LOOK_FROM: Vec3 = Vec3::new(13., 2.0, 3.0);
const LOOK_AT: Vec3 = Vec3::ZERO;
const VUP: Vec3 = Vec3::new(0.0, 1.0, 0.0);

const THETA: f64 = f64::to_radians(VFOV);
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const ANTI_ALIACING_SAMPLES: usize = 100;
const PIXEL_SAMPLES_SCALE: Vec3 = Vec3::splat(1.0 / ANTI_ALIACING_SAMPLES as f64);
const IMAGE_WIDTH: usize = 1200;
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
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let w: Vec3 = (LOOK_FROM - LOOK_AT).unit_vector();
        let u: Vec3 = VUP.cross(&w).unit_vector();
        let v: Vec3 = w.cross(&u);
        let defocus_radius = FOCUS_DIST * (DEFOCUS_ANGLE / 2.0).to_radians().tan();

        let h: f64 = (THETA / 2.0).tan();
        let viewport_height: f64 = 2.0 * h * FOCUS_DIST;
        let viewport_width: f64 = viewport_height * IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64;
        let viewport_u: Vec3 = Vec3::splat(viewport_width) * u;
        let viewport_v: Vec3 = Vec3::splat(viewport_height) * -v;
        let pixel_delta_u: Vec3 = viewport_u / Vec3::splat(IMAGE_WIDTH as f64);
        let pixel_delta_v: Vec3 = viewport_v / Vec3::splat(IMAGE_HEIGHT as f64);
        let viewport_upper_left: Vec3 = CENTER
            - (Vec3::splat(FOCUS_DIST) * w)
            - viewport_u / Vec3::splat(2.0)
            - viewport_v / Vec3::splat(2.0);

        Self {
            pixel00_loc: Vec3::splat(0.5) * (pixel_delta_u + pixel_delta_v) + viewport_upper_left,
            defocus_disk_u: u * Vec3::splat(defocus_radius),
            defocus_disk_v: v * Vec3::splat(defocus_radius),
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: &World) {
        let mut pixels_buffer = vec![Color::BLACK; IMAGE_WIDTH * IMAGE_HEIGHT];
        let mut writer = std::io::BufWriter::new(std::io::stdout());

        for column in 0..IMAGE_WIDTH {
            self.render_columns(world, column, &mut pixels_buffer);
        }

        draw_pixels(&mut writer, &pixels_buffer);
        let _ = writer.flush();
    }

    fn render_columns(&self, world: &World, column: usize, pixels_buffer: &mut [Color]) {
        for row in 0..IMAGE_HEIGHT {
            let mut hit_value = Vec3::ZERO;
            for _ in 0..ANTI_ALIACING_SAMPLES {
                let mut ray = self.get_ray(column as f64, row as f64);
                let mut color = Vec3::ONE;
                for _ in 0..MAX_BOUNCE_DEPTH {
                    world.hit(&mut ray);
                    if let Some(hit) = ray.hit {
                        let scatter = match hit.material.scatter(&hit, &ray.direction) {
                            Some(v) => v,
                            None => {
                                color = Vec3::ZERO;
                                break;
                            }
                        };

                        ray = scatter.ray;
                        color = color * scatter.color;
                    } else {
                        let unit_direction = ray.direction.unit_vector();
                        let a = Vec3::splat(0.5 * (unit_direction.y() + 1.0));
                        color =
                            color * ((Vec3::ONE - a) * Vec3::ONE + a * Vec3::new(0.5, 0.7, 1.0));
                        break;
                    }
                }

                hit_value = hit_value + color;
            }

            pixels_buffer[IMAGE_WIDTH * row + column] = (hit_value * PIXEL_SAMPLES_SCALE).into();
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
        let origin = if DEFOCUS_ANGLE > 0. {
            self.defocus_disk_sample()
        } else {
            CENTER
        };
        Ray::new(origin, pixel_sample - CENTER)
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = Vec3::random_in_unit_disk(-1., 1.);
        CENTER
            + (Vec3::splat(p.x()) * self.defocus_disk_u)
            + (Vec3::splat(p.y()) * self.defocus_disk_v)
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
}

pub struct Hit {
    pub normal: Vec3,
    pub p: Vec3,
    pub material: Arc<Material>,
    pub front_face: bool,
}

impl Hit {
    pub fn new(normal: Vec3, p: Vec3, ray: &Ray, m: Arc<Material>) -> Self {
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
