use std::ops;

struct Vec3 {
    data: [f64; 3],
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
    pub fn get_x(self) -> f64 {
        self.data[0]
    }
    pub fn get_y(self) -> f64 {
        self.data[1]
    }
    pub fn get_z(self) -> f64 {
        self.data[2]
    }
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
