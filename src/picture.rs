use cgmath::{ Vector3 };
use rayon::prelude::*;
use crate::camera::Camera;
use crate::hitable_list::HitableList;
use std::sync::{Arc, Mutex};
use crate::sphere::Sphere;
use rand::prelude::*;
use indicatif::ProgressBar;

pub struct Picture {
    pub x: u32,
    pub y: u32,
    pub c: Camera,
    pub world: Arc<HitableList<Sphere>>,
    pub ns: u32
}

impl Picture {
    pub fn new(x: u32, y:u32, c: Camera, world: Arc<HitableList<Sphere>>, ns: u32) -> Picture {
        Picture {x, y, c, world, ns}
    }

    fn create_random_tuples(&self, x: u32, y: u32) -> (f64, f64) {
        let u: f64 = (x as f64 + random::<f64>()) / self.x as f64;
        let v: f64 = (y as f64 + random::<f64>()) / self.y as f64;
        (u, v)
    }

    fn compute_pixel(&self, x: u32, y: u32) -> Vector3<f64> {
        let mut cum_col: Vector3<f64> = (0..self.ns)
            .into_par_iter()
            .map( |_| self.create_random_tuples(x, y) )
            .map( |(u, v)| self.c.get_ray(u, v) )
            .map( |r| r.color( &self.world.clone(), 0.0) )
            .sum();

        cum_col /= (self.ns + 1) as f64;

        cum_col[0] = cum_col[0].sqrt();
        cum_col[1] = cum_col[1].sqrt();
        cum_col[2] = cum_col[2].sqrt();

        cum_col
    }

    pub fn generate_picture(&self) -> Vec<(u32, u32, Vector3<f64>)> {
        let bar = Arc::new(Mutex::new(ProgressBar::new(self.x as u64 * self.y as u64 )));
        let res: Vec<(u32, u32, Vector3<f64>)> = (0..self.x)
            .into_par_iter()
            .flat_map( |x| {
                let b = (&bar).clone();
                (0..self.y)
                    .into_par_iter()
                    .map( move |y| {
                        b.lock().unwrap().inc(1);
                        (x, y, self.compute_pixel(x, y))
                    })
            }).collect();
        bar.lock().unwrap().finish();
        res
    }
}