use ray::Ray;
use cgmath;

type V = cgmath::Vector3<f64>;

pub struct HitRecord {
    t: f64,
    p: V,
    n: V
}

pub trait Hitable {
    fn hit(&self, r: Ray, tmin: f64, tmax: f64 ) -> Option<HitRecord>;
}