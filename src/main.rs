use image::imageops;

mod camera;
mod config;
mod hitable;
mod hitable_list;
mod material;

mod picture;
mod ray;
mod scene;
mod sphere;

use config::ConfigLoader;
use failure::Error;

fn main() -> Result<(), Error> {
    let config = ConfigLoader::new(
        "/Users/matthieuzandecki/Documents/Rust_projects/simple_raytracer/config/picture1.toml",
    )
    .load()?;
    let world = scene::create_scene();

    let mut img = image::ImageBuffer::new(config.params.x_dim, config.params.y_dim);

    let p = picture::Picture::new(
        config.params.x_dim,
        config.params.y_dim,
        config.camera(),
        world.clone(),
        config.params.n,
    );
    let generated_picture = p.generate_picture();

    generated_picture
        .iter()
        .map(|&(x, y, v)| {
            (
                x,
                y,
                image::Rgb([
                    (255.99 * v.x) as u8,
                    (255.99 * v.y) as u8,
                    (255.99 * v.z) as u8,
                ]),
            )
        })
        .for_each(|(x, y, p)| img.put_pixel(x, y, p));

    img = imageops::flip_vertical(&img);
    image::ImageRgb8(img).save("test13.png").unwrap();

    Ok(())
}
