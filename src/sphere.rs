use cgmath;
use cgmath::prelude::*;
use crate::ray::Ray;
use crate::hitable::*;
use rand::prelude::*;
use rand::distributions::{Standard};
use crate::material::{Material};
use std::sync::Arc;

type V = cgmath::Vector3<f64>;

#[derive(Debug)]
pub struct Sphere {
    pub center: V,
    pub radius: f64,
    pub material: Arc<Material>
}

impl Sphere {
    fn _random() -> V {
        let p: V = SmallRng::from_entropy().sample(Standard);
        2.0 * p - cgmath::vec3(1.0, 1.0, 1.0)
    }

    pub fn random_in_unit_sphere() -> V {
        let mut p = cgmath::vec3(999.99, 999.99, 999.99);
        while p.magnitude2() >= 1.0 {
            p = Sphere::_random();
        }
        p
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64 ) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().dot(r.direction());
        let b = oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < tmax && temp > tmin {
                Some(HitRecord {
                    t: temp,
                    p: r.point_at_parameter(temp),
                    n: (r.point_at_parameter(temp) - self.center) / self.radius,
                    material: self.material.clone()
                })
            }
                else {
                    let temp = (-b + (b * b - a * c).sqrt()) / a;
                    if temp < tmax && temp > tmin {
                        Some(HitRecord {
                            t: temp,
                            p: r.point_at_parameter(temp),
                            n: (r.point_at_parameter(temp) - self.center) / self.radius,
                            material: self.material.clone()
                        })
                    }
                        else {
                            None
                        }
                }
        }
            else {
                None
            }
    }
}