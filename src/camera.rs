use cgmath;
use cgmath::prelude::*;
use cgmath::Vector3;
use ray::Ray;
use std::f64;

type V = Vector3<f64>;

pub struct Camera {
    lower_left_corner: V,
    horizontal: V,
    vertical: V,
    origin: V
}

impl Camera {
    pub fn new(look_from: V, look_at: V, vup: V, vfov: f64, aspect: f64) -> Camera {
        let theta = f64::consts::PI * vfov / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (look_from - look_at).normalize();
        let u = vup.cross(w);
        let v = w.cross(u);
        Camera {
            lower_left_corner: look_from - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
            origin: look_from
        }
    }

    pub fn get_ray( &self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin)
    }
}