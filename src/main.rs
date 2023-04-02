use std::fs::File;
use std::io::Write;

mod ray;
mod vec3;

fn hit_sphere(center: &vec3::Vec3, radius: f64, ray: &ray::Ray) -> bool {
    let oc = ray.origin - *center;
    let a = vec3::dot(&ray.dir, &ray.dir);
    let b = vec3::dot(&oc, &ray.dir) * 2.0;
    let c = vec3::dot(&oc, &oc) - radius * radius;
    let d = b * b - 4.0 * a * c;
    d > 0.0
}

fn vertical_gradient(r: ray::Ray) -> vec3::Vec3 {
    let center = vec3::Vec3 {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };
    let red = vec3::Vec3 {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    if hit_sphere(&center, 0.2, &r) {
        return red;
    }
    let white = vec3::Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    let blue = vec3::Vec3 {
        x: 0.5,
        y: 0.7,
        z: 1.0,
    };
    let unitdir = vec3::unit(&r.dir);
    let t = 0.5 * (unitdir.y + 1.0);
    white * (1.0 - t) + blue * t
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

    let aspect_ratio = 16.0 / 9.0;
    let height = 400;
    let width = (height as f64 * aspect_ratio) as i32;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = vec3::Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let horizontal = vec3::Vec3 {
        x: viewport_width,
        y: 0.0,
        z: 0.0,
    };
    let vertical = vec3::Vec3 {
        x: 0.0,
        y: viewport_height,
        z: 0.0,
    };
    let lower_left_corner = origin
        - horizontal / 2.0
        - vertical / 2.0
        - vec3::Vec3 {
            x: 0.0,
            y: 0.0,
            z: focal_length,
        };

    write_to_file(format!("P3\n{} {}\n255\n", width, height), &mut file);
    for j in (0..width).rev() {
        print!("\rScanlines remaining: {} ", j);
        for i in 0..width {
            let u = i as f64 / (width - 1) as f64;
            let v = j as f64 / (width - 1) as f64;
            let r = ray::Ray {
                origin,
                dir: lower_left_corner + horizontal * u + vertical * v - origin,
            };
            let pixelcolor = vertical_gradient(r);
            write_to_file(pixelcolor.to_string(), &mut file);
        }
    }
    println!("\nDone");
}
