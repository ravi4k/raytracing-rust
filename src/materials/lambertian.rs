use crate::geometry::color::Color;
use crate::geometry::vector::{Vector3, Point};
use crate::materials::material::Material;
use crate::textures::texture::Texture;
use std::sync::Arc;

pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}

impl Material for Lambertian {
    fn scatter(&self, _in_direction: Vector3, normal: Vector3) -> Vector3 {
        let mut scatter_direction = normal + Vector3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = normal;
        }
        scatter_direction.direction()
    }

    fn color(&self, u: f32, v: f32, intersection: Point) -> Color {
        self.albedo.color(u, v, intersection)
    }
}