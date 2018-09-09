use cgmath;
use cgmath::prelude::*;
use cgmath::{Vector2, Vector3};
use rand::prelude::*;
use rand::distributions::{Standard};
use ray::Ray;
use std::f64;
use sphere::Sphere;

type V = Vector3<f64>;

pub struct Camera {
    lower_left_corner: V,
    horizontal: V,
    vertical: V,
    origin: V,
    lens_radius: f64,
    u: V,
    v: V,
    w: V
}

impl Camera {
    pub fn new(look_from: V, look_at: V, vup: V, vfov: f64, aspect: f64, aperture: f64, focus_dist: f64) -> Camera {
        let theta = f64::consts::PI * vfov / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (look_from - look_at).normalize();
        let u = vup.cross(w);
        let v = w.cross(u);
        Camera {
            lower_left_corner: look_from - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            origin: look_from,
            lens_radius: aperture / 2.0,
            u,
            v,
            w
        }
    }

    fn _random(&self) -> cgmath::Vector2<f64> {
        let p: Vector2<f64>  = SmallRng::from_entropy().sample(Standard);
        2.0 * p - cgmath::vec2(1.0, 1.0)
    }

    fn random_in_unit_disk(&self) -> Vector3<f64>{
        let mut p = cgmath::vec2(999.99, 999.99);
        while p.magnitude2() >= 1.0 {
            p = self._random();
        }
        cgmath::vec3(p.x, p.y, 0.0)
    }

    pub fn get_ray( &self, u: f64, v: f64) -> Ray {
        let rd = self.lens_radius * self.random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(self.origin + offset, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset)
    }
}