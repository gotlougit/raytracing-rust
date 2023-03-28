use std::fs::File;
use std::io::Write;
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

fn write_to_file(data: String, fd: &mut File) {
    match fd.write_all(data.as_bytes()) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("FATAL ERROR while writing to file: {}", e);
        }
    }
}

fn main() {
    let filename = "out.ppm";
    let mut file = File::create(filename).unwrap();
    let width = 512;
    let height = 512;
    write_to_file(format!("P3\n{} {}\n255\n", width, height), &mut file);
    for j in (0..width).rev() {
        print!("\rScanlines remaining: {}", j);
        for i in 0..width {
            let r: f64 = i as f64 / (width - 1) as f64;
            let g: f64 = j as f64 / (width - 1) as f64;
            let b: f64 = 0.25;
            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;
            write_to_file(format!("{} {} {}\n", ir, ig, ib), &mut file);
        }
    }
    println!("\nDone");
}
