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
    fn new(albedo: V, fuzz: Option<f64>) -> Arc<Self> where Self: Sized;
    fn scatter(&self, r: &Ray, hr: &HitRecord) -> Option<Scattered>;
}

#[derive(Debug)]
pub struct Lambertian {
    pub albedo: V
}

impl Material for Lambertian {
    fn new(albedo: V, fuzz: Option<f64>) -> Arc<Lambertian> {
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
    pub albedo: V,
    pub fuzz: f64
}

impl Metal {
    pub fn reflect(&self, v: V, n: V) -> V {
        v - 2.0 * v.dot(n) * n
    }
}

impl Material for Metal {
    fn new(albedo: Vector3<f64>, fuzz: Option<f64>) -> Arc<Metal> {
        Arc::new(Metal { albedo, fuzz: fuzz.unwrap() })
    }

    fn scatter(&self, r: &Ray, hr: &HitRecord) -> Option<Scattered> {
        let reflected: V = self.reflect(r.direction().normalize(), hr.n) + self.fuzz * Sphere::random_in_unit_sphere();
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