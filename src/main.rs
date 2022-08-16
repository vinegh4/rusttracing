//use std::fs::File;
//use std::io::prelude::*;
//use std::path::Path;

const IMAGE_HEIGHT : u32 = 256;
const IMAGE_WIDTH : u32 = 256;

fn main() {
    //let path = "../output/
    print!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let r : f64 = i as f64 / (IMAGE_WIDTH as f64 - 1.0);
            let g : f64 = (IMAGE_HEIGHT - j - 1) as f64 / (IMAGE_HEIGHT as f64 - 1.0);
            let b : f64 = 0.25;

            let r_int : u32 = (255.999 * r) as u32;
            let g_int : u32 = (255.999 * g) as u32;
            let b_int : u32 = (255.999 * b) as u32;

            print!("{r_int} {g_int} {b_int}\n");
        }
    }
}
