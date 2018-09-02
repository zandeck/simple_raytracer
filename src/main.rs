extern crate image;
extern crate cgmath;

use cgmath::prelude::*;
use cgmath::{ Vector3 };

use image::imageops;
mod ray;
mod sphere;

fn main() {
    let x_dim: u32 = 200;
    let y_dim: u32 = 100;

    let lower_left_corner = cgmath::vec3(-2.0, -1.0, -1.0);
    let horizontal = cgmath::vec3(4.0, 0.0, 0.0);
    let vertical = cgmath::vec3(0.0, 2.0, 0.0);
    let origin = cgmath::vec3(0.0, 0.0, 0.0);

    let mut img = image::ImageBuffer::new(x_dim, y_dim);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let u: f64 = x as f64 / x_dim as f64;
        let v: f64 = y as f64 / y_dim as f64;
        let r = ray::Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
        let c = r.color();

        *pixel = c;
    }
    img = imageops::flip_vertical( &img);
    image::ImageRgb8(img).save("test.png").unwrap();
}
