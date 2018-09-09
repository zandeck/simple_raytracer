extern crate image;
extern crate cgmath;
extern crate rand;

use rand::prelude::*;
use image::imageops;
mod hitable;
mod hitable_list;
mod ray;
mod sphere;
mod camera;
mod material;
use material::*;
use std::f64;

fn main() {
    let x_dim: u32 = 200;
    let y_dim: u32 = 100;
    let ns = 100;

    let c = camera::Camera::new();
    let m1 = Lambertian::new( cgmath::vec3(0.1, 0.2, 0.5));
    let m2 = Lambertian::new( cgmath::vec3(0.8, 0.8, 0.0));
    let m3 = Metal::new( cgmath::vec3(0.8, 0.6, 0.2), 0.0);
    let m4 = Dielectric::new( 1.5);
    let obj1 = sphere::Sphere { center: cgmath::vec3(0.0, 0.0, -1.0), radius: 0.5, material: m1.clone()};
    let obj2 = sphere::Sphere { center: cgmath::vec3(0.0, -100.5, -1.0), radius: 100.0, material: m2.clone() };
    let obj3 = sphere::Sphere { center: cgmath::vec3(1.0, 0.0, -1.0), radius: 0.5, material: m3.clone() };
    let obj4 = sphere::Sphere { center: cgmath::vec3(-1.0, 0.0, -1.0), radius: 0.5, material: m4.clone() };
    let obj5 = sphere::Sphere { center: cgmath::vec3(-1.0, 0.0, -1.0), radius: -0.45, material: m4.clone() };


    let world = hitable_list::HitableList { objects: vec![obj1, obj2, obj3, obj4, obj5] };

    let mut img = image::ImageBuffer::new(x_dim, y_dim);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let mut cum_col = cgmath::vec3(0.0, 0.0, 0.0);
        for _i in 0..ns {
            let u: f64 = (x as f64 + random::<f64>()) / x_dim as f64;
            let v: f64 = (y as f64 + random::<f64>()) / y_dim as f64;
            let r = c.get_ray(u, v);

            let c_temp = r.color(&world, 0.0);
            cum_col += c_temp;

        }

        cum_col /= (ns + 1) as f64;

        cum_col[0] = 255.99 * cum_col[0].sqrt();
        cum_col[1] = 255.99 * cum_col[1].sqrt();
        cum_col[2] = 255.99 * cum_col[2].sqrt();


        *pixel = image::Rgb( [ cum_col[0] as u8, cum_col[1]as u8, cum_col[2] as u8]);
    }
    img = imageops::flip_vertical( &img);
    image::ImageRgb8(img).save("test5.png").unwrap();
}
