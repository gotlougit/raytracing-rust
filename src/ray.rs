use crate::vec3::Vec3;

struct Ray {
    origin: Vec3,
    dir: Vec3
}

impl Ray {
    pub fn at(t: f64) -> Vec3 {
        origin + t*dir
    }
}
