use super::vector::{Vector3, Point};

pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
    pub time: f32,
}

impl Ray {
    pub fn at_distance(&self, distance: f32) -> Point {
        self.origin + distance * self.direction
    }
}