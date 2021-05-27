use std::ops::{Add, Sub, Mul, Div, AddAssign, Neg, MulAssign};

pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn direction(self) -> Self {
        (1.0 / self.length()) * self
    }

    pub fn dot(&self, _rhs: Self) -> f32 {
        self.x * _rhs.x + self.y * _rhs.y + self.z * _rhs.z
    }

    pub fn cross(&self, _rhs: Self) -> Self {
        Self {
            x: self.y * _rhs.z - self.z * _rhs.y,
            y: self.z * _rhs.x - self.x * _rhs.z,
            z: self.x * _rhs.y - self.y * _rhs.x,
        }
    }

    pub fn length_squared(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn near_zero(&self) -> bool {
        const e: f32= 1e-6;
        self.x.abs() < e && self.y.abs() < e && self.z.abs() < e
    }
}

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self {
        Self { x: self.x + _rhs.x, y: self.y + _rhs.y, z: self.z + _rhs.z }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, _rhs: Self) {
        *self = Self {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        };
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self {
        Self { x: self.x - _rhs.x, y: self.y - _rhs.y, z: self.z - _rhs.z }
    }
}

impl Mul for Vector3 {
    type Output = Self;

    fn mul(self, _rhs: Self) -> Self::Output {
        Self { x: self.x * _rhs.x, y: self.y * _rhs.y, z: self.z * _rhs.z }
    }
}

impl MulAssign for Vector3 {
    fn mul_assign(&mut self, _rhs: Self) {
        *self = Self {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
            z: self.z * _rhs.z,
        };
    }
}

impl Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, _rhs: Vector3) -> Self::Output {
        Self::Output { x: self * _rhs.x, y: self * _rhs.x, z: self * _rhs.x}
    }
}

impl Div for Vector3 {
    type Output = Self;

    fn div(self, _rhs: Self) -> Self::Output {
        Self { x: self.x / _rhs.x, y: self.y / _rhs.y, z: self.z / _rhs.z }
    }
}

pub type Point = Vector3;