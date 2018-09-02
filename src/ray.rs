use cgmath::{Vector3};
use cgmath::prelude::*;
use cgmath;
use image;

type V = Vector3<f64>;

pub struct Ray {
    a: V,
    b: V
}

impl Ray {
    pub fn new(a: V, b: V) -> Ray {
        Ray { a,b }
    }

    pub fn origin( &self ) -> V {
        self.a
    }

    pub fn direction( &self ) -> V {
        self.b
    }

    pub fn point_at_parameter( &self, t: f64) -> V {
        self.a + t * self.b
    }

    pub fn color( &self ) -> image::Rgb<u8> {
        let t = self.hit_sphere(cgmath::vec3(0.0, 0.0, -1.0), 0.5);
        if t > 0.0 {
            let n = (self.point_at_parameter(t) - cgmath::vec3(0.0, 0.0, -1.0)).normalize();
            let c = 255.99 * 0.5 * (cgmath::vec3(1.0, 1.0, 1.0) + n);
            image::Rgb([c.x as u8, c.y as u8, c.z as u8])
        }
        else {
            let unit_direction = self.direction();
            let t = 0.5 * (unit_direction.y + 1.0);
            let c = 255.99 * ((1.0 - t) * cgmath::vec3(1.0, 1.0, 1.0) + t * cgmath::vec3(0.5, 0.7, 1.0));
            image::Rgb([c.x as u8, c.y as u8, c.z as u8])
        }

    }

    pub fn hit_sphere( &self, center: V, radius: f64 ) -> f64 {
        let oc = self.origin() - center;
        let a = self.direction().dot(self.direction());
        let b = 2.0 * oc.dot(self.direction());
        let c = oc.dot(oc) - radius * radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            -1.0
        }
        else {
            (-b - discriminant.sqrt()) / (2.0 * a)
        }
    }
}
