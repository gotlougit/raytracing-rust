use std::fs::File;
use std::io::Write;

mod vec3;
mod ray;

fn vertical_gradient(r: ray::Ray) -> vec3::Vec3 {
    let mut data = [1f64; 3];
    let white = vec3::Vec3 {
        data
    };
    data = [0.5, 0.7, 1.0];
    let blue = vec3::Vec3 {
        data
    };
    let unitdir = vec3::unit(&r.dir);
    let t = 0.5*(unitdir.data[1] + 1.0);
    white * (1.0-t) + blue * t
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

    let aspect_ratio = 16.0/ 9.0;
    let height = 720;
    let width = (height as f64 * aspect_ratio) as i32;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let mut data = [0f64; 3];
    let origin = vec3::Vec3 {
        data: data.clone()
    };
    data[0] = viewport_width;
    let horizontal = vec3::Vec3 {
        data: data.clone()
    };
    data[0] = 0.0;
    data[1] = viewport_height;
    let vertical = vec3::Vec3 {
        data: data.clone()
    };
    data[0] = 0.0;
    data[1] = 0.0;
    data[2] = focal_length;
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - vec3::Vec3 {
        data: data.clone()
    };

    write_to_file(format!("P3\n{} {}\n255\n", width, height), &mut file);
    for j in (0..width).rev() {
        print!("\rScanlines remaining: {}", j);
        for i in 0..width {
            let u = i as f64 / (width - 1) as f64;
            let v = j as f64 / (width - 1) as f64;
            let r = ray::Ray {
                origin,
                dir: lower_left_corner + horizontal*u + vertical*v - origin
            };
            let pixelcolor = vertical_gradient(r);
            write_to_file(pixelcolor.to_string(), &mut file);
        }
    }
    println!("\nDone");
}
