extern crate image;
extern crate cgmath;
extern crate rand;
extern crate rayon;

use rand::prelude::*;
use image::imageops;
use cgmath::prelude::*;
use rayon::prelude::*;
mod hitable;
mod hitable_list;
mod ray;
mod sphere;
mod camera;
mod material;
mod scene;
use material::*;
use std::f64;

fn main() {
    let x_dim: u32 = 1000;
    let y_dim: u32 = 500;
    let ns = 100;
    let r = (f64::consts::PI / 4.0).cos();
    let look_from = cgmath::vec3(-2.0, 2.0, 1.0);
    let look_at = cgmath::vec3(0.0, 0.0, -1.0);
    let vup = cgmath::vec3(0.0, 1.0, 0.0);
    let dist_to_focus = (look_from - look_at).magnitude();
    let aperture = 2.0;

    let c = camera::Camera::new(look_from, look_at, vup, 90.0, (x_dim as f64) / (y_dim as f64), aperture, dist_to_focus);

    let world = scene::create_scene();

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
    image::ImageRgb8(img).save("test9.png").unwrap();

}
