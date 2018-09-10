use ray::Ray;
use hitable::HitRecord;
use cgmath;
use cgmath::Vector3;
use cgmath::prelude::*;
use sphere::Sphere;
use std::sync::Arc;
use std::fmt::Debug;
use rand::prelude::*;
use rand::distributions::{Standard};

type V = Vector3<f64>;

#[derive(Debug)]
pub struct Scattered {
    pub attenuation: V,
    pub scattered: Ray
}

pub trait Material: Debug + Send + Sync {
    fn scatter(&self, r: &Ray, hr: &HitRecord) -> Option<Scattered>;

    fn schlick(&self, cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }

    fn reflect(&self, v: V, n: V) -> V {
        v - 2.0 * v.dot(n) * n
    }

    fn refract(&self, v: V, n: V, ni_over_nt: f64) -> Option<V> {
        let uv = v.normalize();
        let dt = uv.dot(n);
        let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
        if discriminant > 0.0 {
            Some(ni_over_nt * (uv - n * dt) - n * discriminant.sqrt())
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Lambertian {
    pub albedo: V
}

impl Lambertian {
    pub fn new(albedo: V) -> Arc<Lambertian> {
        Arc::new(Lambertian { albedo })
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r: &Ray, hr: &HitRecord) -> Option<Scattered> {
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
    pub fn new(albedo: Vector3<f64>, fuzz: f64) -> Arc<Metal> {
        Arc::new(Metal { albedo, fuzz })
    }
}

impl Material for Metal {

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

#[derive(Debug)]
pub struct Dielectric {
    pub ref_idx: f64
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Arc<Dielectric> {
        Arc::new(Dielectric { ref_idx })
    }
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, hr: &HitRecord) -> Option<Scattered> {
        let outward_normal;
        let ni_over_nt;
        let cosine;
        let reflected = self.reflect(r.direction(), hr.n);
        let attenuation = cgmath::vec3(1.0, 1.0, 1.0);
        if r.direction().dot(hr.n) > 0.0 {
            outward_normal = -hr.n;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * r.direction().dot(hr.n) / r.direction().magnitude();
        } else {
            outward_normal = hr.n;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -r.direction().dot(hr.n) / r.direction().magnitude();
        }



        let (ref_prob, refracted) = match self.refract(r.direction(), outward_normal, ni_over_nt ) {
            Some(refracted) => (self.schlick(cosine, self.ref_idx), Some(refracted)),
            _ => (1.0, None)
        };

        let rand_num: f64 = SmallRng::from_entropy().sample(Standard);

        if rand_num < ref_prob {
            Some(Scattered {
                attenuation,
                scattered: Ray::new(hr.p, reflected)
            })
        } else {
            Some(Scattered {
                attenuation,
                scattered: Ray::new(hr.p, refracted.unwrap())
            })

        }
    }
}