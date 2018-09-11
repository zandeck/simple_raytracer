extern crate image;
extern crate cgmath;
extern crate rand;
extern crate rayon;
extern crate indicatif;

use image::imageops;
use rand::prelude::*;
use cgmath::prelude::*;
mod hitable;
mod hitable_list;
mod ray;
mod sphere;
mod camera;
mod material;
mod scene;
mod picture;
use std::f64;
use picture::Picture;


fn main() {
    let x_dim: u32 = 200;
    let y_dim: u32 = 100;
    let ns = 300;
    let _r = (f64::consts::PI / 4.0).cos();
    let look_from = cgmath::vec3(-2.0, 2.0, 1.0);
    let look_at = cgmath::vec3(0.0, 0.0, -1.0);
    let vup = cgmath::vec3(0.0, 1.0, 0.0);
    let dist_to_focus = (look_from - look_at).magnitude();
    let aperture = 0.0;

    let c = camera::Camera::new(look_from, look_at, vup, 90.0, (x_dim as f64) / (y_dim as f64), aperture, dist_to_focus);

    let world = scene::create_scene();

    let mut img = image::ImageBuffer::new(x_dim, y_dim);

    let p = picture::Picture::new(x_dim, y_dim, c, world.clone(), ns);
    let generated_picture = p.generate_picture();

    generated_picture
        .iter()
        .map(|&(x, y, v)| (x, y, image::Rgb([ (255.99 * v.x) as u8, (255.99 * v.y) as u8, (255.99 * v.z) as u8])))
        .for_each(|(x, y, p)| img.put_pixel(x, y, p) );

    img = imageops::flip_vertical( &img);
    image::ImageRgb8(img).save("test12.png").unwrap();

}
