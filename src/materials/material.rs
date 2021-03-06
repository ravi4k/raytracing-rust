use std::sync::Arc;

use crate::geometry::color::Color;
use crate::geometry::pdf::PDF;
use crate::geometry::ray::Ray;
use crate::geometry::vector::{Point, Vector3};
use crate::objects::hittable::HitRecord;

pub struct ScatterRecord {
    pub specular_ray: Option<Ray>,
    pub attenuation: Color,
    pub pdf_ptr: Option<Arc<dyn PDF>>,
}

pub trait Material: Send + Sync {
    fn scatter(&self, in_ray: Ray, hit_rec: &HitRecord) -> Option<ScatterRecord> {
        return Option::None;
    }
    fn scattering_pdf(&self, in_ray: Ray, hit_rec: &HitRecord, scattered_ray: Ray) -> f32 {
        return 0.0;
    }
    fn emitted(&self, ray: Ray, hit_rec: &HitRecord, u: f32, v: f32, intersection: Point) -> Color {
        return Color::BLACK;
    }
}

pub fn reflect_ray(in_direction: Vector3, normal: Vector3) -> Vector3 {
    (in_direction - 2.0 * in_direction.dot(normal) * normal).direction()
}

pub fn refract(in_direction: Vector3, normal: Vector3, ir: f32) -> Vector3 {
    let cos_0 = -in_direction.dot(normal).min(1.0);
    let refract_perp = ir * (in_direction + cos_0 * normal);
    let refract_para = -(1.0 - refract_perp.length_squared()).sqrt() * normal;
    return (refract_perp + refract_para).direction();
}

pub fn reflectance_schlick(cos_0: f32, ir: f32) -> f32 {
    let mut r0 = (1.0 - ir) / (1.0 + ir);
    r0 = r0.powi(2);
    return r0 + (1.0 - r0) * (1.0 - cos_0).powi(5);
}