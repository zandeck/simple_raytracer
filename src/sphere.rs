use hitable;
use cgmath;
use cgmath::prelude::*;
use ray::Ray;

type V = cgmath::Vector3<f64>;

struct Sphere {
    center: V,
    radius: f64
}

impl hitable::Hitable for Sphere {
    fn hit(&self, r: Ray, tmin: f64, tmax: f64 ) -> Option<hitable::HitRecord> {
        None
    }
}