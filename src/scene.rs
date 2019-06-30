use crate::hitable_list::HitableList;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::sphere::Sphere;
use cgmath;
use cgmath::prelude::*;
use cgmath::vec3;
use rand::distributions::Standard;
use rand::prelude::*;

use std::sync::Arc;

pub fn create_scene() -> Arc<HitableList<Sphere>> {
    let mut spheres = Vec::new();
    spheres.push(Sphere {
        center: vec3(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Lambertian::new(vec3(0.5, 0.5, 0.5)),
    });
    let mut e = thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = e.sample(Standard);
            let (x, y): (f64, f64) = (e.sample(Standard), e.sample(Standard));
            let center = vec3((a as f64) + 0.9 * x, 0.2, (b as f64) + 0.9 * y);
            if (center - vec3(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                if choose_mat < 0.8 {
                    let u: cgmath::Vector3<f64> = e.sample(Standard);
                    let v: cgmath::Vector3<f64> = e.sample(Standard);
                    let uv: cgmath::Vector3<f64> = u.mul_element_wise(v);
                    spheres.push(Sphere {
                        center,
                        radius: 0.2,
                        material: Lambertian::new(uv),
                    });
                } else if choose_mat < 0.95 {
                    let v: cgmath::Vector3<f64> = e.sample(Standard);
                    let r: f64 = e.sample(Standard);
                    spheres.push(Sphere {
                        center,
                        radius: 0.2,
                        material: Metal::new(0.5 * (vec3(1.0, 1.0, 1.0) + v), 0.5 * r),
                    });
                } else {
                    spheres.push(Sphere {
                        center,
                        radius: 0.2,
                        material: Dielectric::new(1.5),
                    });
                }
            }
        }
    }
    spheres.push(Sphere {
        center: vec3(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Dielectric::new(1.5),
    });
    spheres.push(Sphere {
        center: vec3(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Lambertian::new(vec3(0.4, 0.2, 0.1)),
    });
    spheres.push(Sphere {
        center: vec3(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Metal::new(vec3(0.7, 0.6, 0.5), 0.0),
    });
    Arc::new(HitableList { objects: spheres })
}
