use cgmath;
use cgmath::prelude::*;
use ray::Ray;
use hitable::*;

type V = cgmath::Vector3<f64>;

pub struct Sphere {
    pub center: V,
    pub radius: f64
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
                    n: (r.point_at_parameter(temp) - self.center) / self.radius
                })
            }
            else {
                let temp = (-b + (b * b - a * c).sqrt()) / a;
                if temp < tmax && temp > tmin {
                    Some(HitRecord {
                        t: temp,
                        p: r.point_at_parameter(temp),
                        n: (r.point_at_parameter(temp) - self.center) / self.radius
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