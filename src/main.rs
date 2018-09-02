extern crate image;
extern crate cgmath;
extern crate rand;

use cgmath::prelude::*;
use cgmath::{ Vector3 };
use rand::prelude::*;
use image::imageops;
mod hitable;
mod hitable_list;
mod ray;
mod sphere;
mod camera;

fn main() {
    let x_dim: u32 = 200;
    let y_dim: u32 = 100;
    let ns = 100;

    let c = camera::Camera::new();
    let obj1 = sphere::Sphere { center: cgmath::vec3(0.0, 0.0, -1.0), radius: 0.5 };
    let obj2 = sphere::Sphere { center: cgmath::vec3(0.0, -100.5, -1.0), radius: 100.0 };
    let world = hitable_list::HitableList { objects: vec![obj1, obj2] };

    let mut img = image::ImageBuffer::new(x_dim, y_dim);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let mut cum_col = [ 0, 0, 0];
        for i in 0..ns {
            let u: f64 = (x as f64 + random::<f64>()) / x_dim as f64;
            let v: f64 = (y as f64 + random::<f64>()) / y_dim as f64;
            let r = c.get_ray(u, v);

            let c_temp = r.color(&world);
            cum_col[0] += c_temp[0] as i32;
            cum_col[1] += c_temp[1] as i32;
            cum_col[2] += c_temp[2] as i32;

        }

        *pixel = image::Rgb( [ (cum_col[0]/ ns ) as u8, (cum_col[1]/ ns) as u8, (cum_col[2]/ ns) as u8]);
    }
    img = imageops::flip_vertical( &img);
    image::ImageRgb8(img).save("test2.png").unwrap();
}
