use std::fmt;
use std::ops;

pub struct Vec3 {
    pub data: [f64; 3],
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ir = (255.999 * self.data[0]) as i32;
        let ig = (255.999 * self.data[1]) as i32;
        let ib = (255.999 * self.data[2]) as i32;
        write!(f, "{} {} {}\n", ir, ig, ib)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self: Vec3, _rhs: Vec3) -> Vec3 {
        let mut newarr = [0f64; 3];
        newarr[0] = self.data[0] + _rhs.data[0];
        newarr[1] = self.data[1] + _rhs.data[1];
        newarr[2] = self.data[2] + _rhs.data[2];
        Vec3 { data: newarr }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self: Vec3, _rhs: Vec3) -> Vec3 {
        let mut newarr = [0f64; 3];
        newarr[0] = self.data[0] - _rhs.data[0];
        newarr[1] = self.data[1] - _rhs.data[1];
        newarr[2] = self.data[2] - _rhs.data[2];
        Vec3 { data: newarr }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3 {
        let mut newarr = [0f64; 3];
        newarr[0] = self.data[0] * rhs;
        newarr[1] = self.data[1] * rhs;
        newarr[2] = self.data[2] * rhs;
        Vec3 { data: newarr }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Vec3 {
        let mut newarr = [0f64; 3];
        newarr[0] = self.data[0] / rhs;
        newarr[1] = self.data[1] / rhs;
        newarr[2] = self.data[2] / rhs;
        Vec3 { data: newarr }
    }
}

impl Vec3 {
    pub fn length_squared(self) -> f64 {
        let x = self.data[0];
        let y = self.data[1];
        let z = self.data[2];
        return x * x + y * y + z * z;
    }
    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }
}

pub fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
    let x1 = v1.data[0];
    let y1 = v1.data[1];
    let z1 = v1.data[2];
    let x2 = v2.data[0];
    let y2 = v2.data[1];
    let z2 = v2.data[2];
    return x1 * x2 + y1 * y2 + z1 * z2;
}

pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
    let x1 = v1.data[0];
    let y1 = v1.data[1];
    let z1 = v1.data[2];
    let x2 = v2.data[0];
    let y2 = v2.data[1];
    let z2 = v2.data[2];
    let mut newarr = [0f64; 3];
    newarr[0] = y1 * z2 - z1 * y2;
    newarr[1] = z1 * x2 - x1 * z2;
    newarr[2] = x1 * y2 - y1 * x2;
    Vec3 { data: newarr }
}

pub fn unit(v: &Vec3) -> Vec3 {
    let ud = [v.data[0]/v.length(), v.data[1]/v.length(), v.data[2]/v.length()];
    Vec3 {
        data: ud
    }
}
