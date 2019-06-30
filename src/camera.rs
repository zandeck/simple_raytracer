use crate::ray::Ray;

use cgmath::prelude::*;
use cgmath::{Vector2, Vector3};
use rand::distributions::Standard;
use rand::prelude::*;
use std::f64;

use serde::{Deserialize, Serialize};

type V = Vector3<f64>;

#[allow(dead_code)]
pub struct Camera {
    lower_left_corner: V,
    horizontal: V,
    vertical: V,
    origin: V,
    lens_radius: f64,
    u: V,
    v: V,
    w: V,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CameraBuilder {
    look_from: V,
    look_at: V,
    vup: V,
    vfov: f64,
    aspect: f64,
    aperture: f64,
    focus_distance: f64,
}

impl CameraBuilder {
    #[allow(dead_code)]
    pub fn new(
        look_from: V,
        look_at: V,
        vup: V,
        vfov: f64,
        aspect: f64,
        aperture: f64,
        focus_distance: f64,
    ) -> Self {
        CameraBuilder {
            look_from,
            look_at,
            vup,
            vfov,
            aspect,
            aperture,
            focus_distance,
        }
    }

    pub fn build(&self) -> Camera {
        let theta = f64::consts::PI * self.vfov / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = self.aspect * half_height;
        let w = (self.look_from - self.look_at).normalize();
        let u = self.vup.cross(w);
        let v = w.cross(u);
        Camera {
            lower_left_corner: self.look_from
                - half_width * self.focus_distance * u
                - half_height * self.focus_distance * v
                - self.focus_distance * w,
            horizontal: 2.0 * half_width * self.focus_distance * u,
            vertical: 2.0 * half_height * self.focus_distance * v,
            origin: self.look_from,
            lens_radius: self.aperture / 2.0,
            u,
            v,
            w,
        }
    }
}

impl Camera {
    fn _random(&self) -> cgmath::Vector2<f64> {
        let p: Vector2<f64> = thread_rng().sample(Standard);
        2.0 * p - cgmath::vec2(1.0, 1.0)
    }

    fn random_in_unit_disk(&self) -> Vector3<f64> {
        let mut p = cgmath::vec2(999.99, 999.99);
        while p.magnitude2() >= 1.0 {
            p = self._random();
        }
        cgmath::vec3(p.x, p.y, 0.0)
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = self.lens_radius * self.random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
        )
    }
}
