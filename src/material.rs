use ray::Ray;
use hitable::HitRecord;
use cgmath::Vector3;
use cgmath::prelude::*;
use sphere::Sphere;
use std::sync::Arc;
use std::fmt::Debug;

type V = Vector3<f64>;

#[derive(Debug)]
pub struct Scattered {
    pub attenuation: V,
    pub scattered: Ray
}

pub trait Material: Debug {
    fn new(albedo: V) -> Arc<Self> where Self: Sized;
    fn scatter(&self, r: &Ray, hr: &HitRecord) -> Option<Scattered>;
}

#[derive(Debug)]
pub struct Lambertian {
    pub albedo: V
}

impl Material for Lambertian {
    fn new(albedo: V) -> Arc<Lambertian> {
        Arc::new(Lambertian { albedo })
    }

    fn scatter(&self, r: &Ray, hr: &HitRecord) -> Option<Scattered> {
        let target: V = hr.p + hr.n + Sphere::random_in_unit_sphere();
        Some(Scattered {
            attenuation: self.albedo,
            scattered: Ray::new(hr.p, target - hr.p)
        })
    }
}

#[derive(Debug)]
pub struct Metal {
    pub albedo: V
}

impl Metal {
    pub fn reflect(&self, v: V, n: V) -> V {
        v - 2.0 * v.dot(n) * n
    }
}

impl Material for Metal {
    fn new(albedo: Vector3<f64>) -> Arc<Metal> {
        Arc::new(Metal { albedo })
    }

    fn scatter(&self, r: &Ray, hr: &HitRecord) -> Option<Scattered> {
        let reflected: V = self.reflect(r.direction().normalize(), hr.n);
        let _scattered = Ray::new(hr.p, reflected);
        if _scattered.direction().dot(hr.n) > 0.0 {
            Some(Scattered {
                attenuation: self.albedo,
                scattered: _scattered
            })
        }
        else {
            None
        }

    }
}