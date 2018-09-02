use cgmath::{Vector3};
use cgmath::prelude::*;
use cgmath;
use image;
use hitable_list;
use hitable::{ HitRecord, Hitable };
use std::f64;

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

    pub fn color<T>( &self, world: &hitable_list::HitableList<T> ) -> image::Rgb<u8> where T: Hitable {
        let hr = world.hit( self, 0.0, f64::INFINITY );
        match hr {
            Some(h) => {
                let c = 255.99 * 0.5 * (cgmath::vec3(1.0, 1.0, 1.0) + h.n);
                image::Rgb([c.x as u8, c.y as u8, c.z as u8])
            }
            None => {
                let unit_direction = self.direction();
                let t = 0.5 * (unit_direction.y + 1.0);
                let c = 255.99 * ((1.0 - t) * cgmath::vec3(1.0, 1.0, 1.0) + t * cgmath::vec3(0.5, 0.7, 1.0));
                image::Rgb([c.x as u8, c.y as u8, c.z as u8])
            }
        }

    }

}
