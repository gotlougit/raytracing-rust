use crate::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3
}

impl Ray {
    pub fn at(t: f64) -> Vec3 {
        origin + t*dir
    }
}
