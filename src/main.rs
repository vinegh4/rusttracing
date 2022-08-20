#![warn(clippy::pedantic)]
mod vector3;
use crate::vector3::Vector3;
mod raytracing;

const IMAGE_HEIGHT : u32 = 256;
const IMAGE_WIDTH : u32 = 256;

fn main() {
    //let path = "../output/
    print!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");

    for j in 0..IMAGE_HEIGHT {
        let scanlines_remaining : u32 = IMAGE_HEIGHT-j;
        eprintln!("Scanlines remaining: {scanlines_remaining}");
        for i in 0..IMAGE_WIDTH {
            let r : f64 = i as f64 / (IMAGE_WIDTH as f64 - 1.0);
            let g : f64 = (IMAGE_HEIGHT - j - 1) as f64 / (IMAGE_HEIGHT as f64 - 1.0);
            let b : f64 = 0.25;

            let pixel = Vector3 {x: r, y: g, z: b};
            print_pixel_val(pixel);
        }
    }
}

fn print_pixel_val(pixel : Vector3) {
    let r_int : u32 = (255.999 * pixel.x) as u32;
    let g_int : u32 = (255.999 * pixel.y) as u32;
    let b_int : u32 = (255.999 * pixel.z) as u32;

    println!("{r_int} {g_int} {b_int}");
}
    
