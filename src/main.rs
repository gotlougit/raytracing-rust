use std::fs;
use std::io::Write;

fn main() {
    let filename = "out.ppm";
    let mut file = fs::File::create(filename).unwrap();
    let width = 512;
    let height = 512;
    file.write_all(format!("P3\n{} {}\n255\n", width, height).as_bytes()).unwrap();
    for j in (0..width).rev() {
        print!("\rScanlines remaining: {}", j);
        for i in 0..width {
            let r : f64 = i as f64 / (width-1) as f64;
            let g : f64 = j as f64 / (width-1) as f64;
            let b : f64 = 0.25;
            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;
            file.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes()).unwrap();
        }
    }
    println!("\nDone");
}
