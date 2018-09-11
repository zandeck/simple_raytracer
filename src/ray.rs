use cgmath::{Vector3};
use cgmath::prelude::*;
use cgmath;
use hitable_list;
use hitable::{ Hitable };
use std::f64;
use rand::prelude::*;
use std::cell::RefCell;

type V = Vector3<f64>;

#[derive(Debug)]
pub struct Ray {
    a: V,
    b: V
}

impl Ray {
    pub fn new(a: V, b: V) -> Ray {
        Ray { a,b }
    }

    pub fn origin( &self ) -> V {
        self.a
    }

    pub fn direction( &self ) -> V {
        self.b
    }

    pub fn point_at_parameter( &self, t: f64) -> V {
        self.a + t * self.b
    }


    pub fn color<T>( &self, world: &hitable_list::HitableList<T>, depth: f64, r: RefCell<SmallRng> ) -> V where T: Hitable {
        let hr = world.hit( self, 0.0, f64::INFINITY );
        println!("{:?}", hr);
        match hr {
            Some(h) => {
                match (h.material.scatter( &self, &h, r.clone()), depth < 50.0 ) {
                    (Some(m), true) => {
                        println!("Case 1");
                        m.attenuation.mul_element_wise( m.scattered.color( &world, depth + 1.0, r.clone()))
                    }
                    _ => {println!("Case 2");cgmath::vec3(0.0, 0.0, 0.0)}
                }

            }
            None => {
                println!("Case 3");
                let unit_direction = self.direction();
                let t = 0.5 * (unit_direction.y + 1.0);
                let c = (1.0 - t) * cgmath::vec3(1.0, 1.0, 1.0) + t * cgmath::vec3(0.5, 0.7, 1.0);
                c
            }
        }

    }

}
