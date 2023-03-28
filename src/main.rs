fn main() {
    let width = 512;
    let height = 512;
    println!("P3\n{} {}\n255", width,height);
    for j in (0..width).rev() {
        for i in 0..width {
            let r : f64 = i as f64 / (width-1) as f64;
            let g : f64 = j as f64 / (width-1) as f64;
            let b : f64 = 0.25;
            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
