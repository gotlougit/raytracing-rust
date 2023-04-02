use std::fmt;
use std::ops;

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ir = (255.999 * self.x) as i32;
        let ig = (255.999 * self.y) as i32;
        let ib = (255.999 * self.z) as i32;
        write!(f, "{} {} {}\n", ir, ig, ib)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self: Vec3, _rhs: Vec3) -> Vec3 {
        let x = self.x + _rhs.x;
        let y = self.y + _rhs.y;
        let z = self.z + _rhs.z;
        Vec3 { x, y, z }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self: Vec3, _rhs: Vec3) -> Vec3 {
        let x = self.x - _rhs.x;
        let y = self.y - _rhs.y;
        let z = self.z - _rhs.z;
        Vec3 { x, y, z }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3 {
        let x = self.x * rhs;
        let y = self.y * rhs;
        let z = self.z * rhs;
        Vec3 { x, y, z }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Vec3 {
        let x = self.x / rhs;
        let y = self.y / rhs;
        let z = self.z / rhs;
        Vec3 { x, y, z }
    }
}

impl Vec3 {
    pub fn length_squared(&self) -> f64 {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        return x * x + y * y + z * z;
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}

pub fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
    let x1 = v1.x;
    let y1 = v1.y;
    let z1 = v1.z;
    let x2 = v2.x;
    let y2 = v2.y;
    let z2 = v2.z;
    return x1 * x2 + y1 * y2 + z1 * z2;
}

pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
    let x1 = v1.x;
    let y1 = v1.y;
    let z1 = v1.z;
    let x2 = v2.x;
    let y2 = v2.y;
    let z2 = v2.z;

    let x = y1 * z2 - z1 * y2;
    let y = z1 * x2 - x1 * z2;
    let z = x1 * y2 - y1 * x2;
    Vec3 { x, y, z }
}

pub fn unit(v: &Vec3) -> Vec3 {
    let len = v.length();
    let x = v.x / len;
    let y = v.y / len;
    let z = v.z / len;
    Vec3 { x, y, z }
}
