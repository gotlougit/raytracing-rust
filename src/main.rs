use std::fs::File;
use std::io::Write;

mod vec3;

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
            let data = [r, g, b];
            let color = vec3::Vec3 {
                data
            };
            write_to_file(color.to_string(), &mut file);
        }
    }
    println!("\nDone");
}
