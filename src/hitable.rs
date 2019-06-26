use crate::ray::Ray;
use cgmath;
use crate::material::Material;
use std::sync::Arc;

type V = cgmath::Vector3<f64>;

pub struct HitRecord {
    pub t: f64,
    pub p: V,
    pub n: V,
    pub material: Arc<Material>
}

pub trait Hitable {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64 ) -> Option<HitRecord>;
}