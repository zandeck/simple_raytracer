use cgmath;
use cgmath::Vector3;
use ray::Ray;

type V = Vector3<f64>;

pub struct Camera {
    lower_left_corner: V,
    horizontal: V,
    vertical: V,
    origin: V
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            lower_left_corner: cgmath::vec3(-2.0, -1.0, -1.0),
            horizontal: cgmath::vec3(4.0, 0.0, 0.0),
            vertical: cgmath::vec3(0.0, 2.0, 0.0),
            origin: cgmath::vec3(0.0, 0.0, 0.0)
        }
    }

    pub fn get_ray( &self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical)
    }
}