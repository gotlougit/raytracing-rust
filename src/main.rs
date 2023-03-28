use std::fs::File;
use std::io::Write;
use std::ops;

struct Vec3 {
    data: [u8; 3]
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self: Vec3, _rhs: Vec3) -> Vec3 {
        let mut newarr = [0u8; 3];
        newarr[0] = self.data[0] + _rhs.data[0];
        newarr[1] = self.data[1] + _rhs.data[1];
        newarr[2] = self.data[2] + _rhs.data[2];
        Vec3 {
            data: newarr
        }
    }
}

fn write_to_file(data: String, fd: &mut File) {
    match fd.write_all(data.as_bytes()) {
        Ok(_) => {},
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
            let r : f64 = i as f64 / (width-1) as f64;
            let g : f64 = j as f64 / (width-1) as f64;
            let b : f64 = 0.25;
            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;
            write_to_file(format!("{} {} {}\n", ir, ig, ib), &mut file);
        }
    }
    println!("\nDone");
}
