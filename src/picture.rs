use cgmath::{ Vector3 };
use rayon::prelude::*;
use camera::Camera;
use hitable_list::HitableList;
use std::sync::Arc;
use sphere::Sphere;
use rand::prelude::*;
use rand::prelude::*;
use rand::distributions::{Standard};
use std::iter;
use std::cell::RefCell;


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

    fn create_random_tuples(&self, x: u32, y: u32, r: RefCell<SmallRng>) -> (f64, f64) {
        let (a, b): (f64, f64) = r.borrow_mut().sample(Standard);
        let u: f64 = (x as f64 + a) / self.x as f64;
        let v: f64 = (y as f64 + b) / self.y as f64;
        println!("Random tuples created");
        (u, v)
    }

    fn compute_pixel(&self, x: u32, y: u32, rd: RefCell<SmallRng>) -> Vector3<f64> {
        let mut cum_col: Vector3<f64> = (0..self.ns)
            .map( |_| self.create_random_tuples(x, y, rd.clone()) )
            .map( |(u, v)| self.c.get_ray(u, v, rd.clone()) )
            .map( |r| r.color( &self.world.clone(), 0.0, rd.clone()) )
            .sum();
        println!("Computation of one pixel completed");
        cum_col /= (self.ns + 1) as f64;

        cum_col[0] = cum_col[0].sqrt();
        cum_col[1] = cum_col[1].sqrt();
        cum_col[2] = cum_col[2].sqrt();

        cum_col
    }

    pub fn generate_picture(&self) -> Vec<(u32, u32, Vector3<f64>)> {
        let pixels_idx: Vec<(u32, u32)> = (0..(self.x)-1)
            .flat_map( |x| {
                (0..(self.y-1))
                    .map( move |y| (x, y))
            }).collect();

        let mut smallrngs = thread_rng();
        let mut rngs : Vec<RefCell<SmallRng>> = iter::repeat(())
            .map(|()| RefCell::new(SmallRng::from_rng(&mut smallrngs).unwrap()))
            .take(((self.x-1) * (self.y-1)) as usize)
            .collect();

        println!("{}", pixels_idx.len());
        println!("{}", rngs.len());
        println!("Start computation of pixels");

        let res :Vec<(u32, u32, Vector3<f64>)> =
            pixels_idx.par_iter()
                .zip(rngs.par_iter_mut())
                .map( |(&(x, y), r)| (x, y, self.compute_pixel(x, y, r.clone())))
                .collect();

        /* let res: Vec<(u32, u32, Vector3<f64>)> = (0..(self.x)-1)
            .into_par_iter()
            .flat_map( |x| {
                (0..(self.y-1))
                    .into_par_iter()
                    .map( move |y| (x, y, self.compute_pixel(x, y)))
            }).collect();*/

        res
    }
}