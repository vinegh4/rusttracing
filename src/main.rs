#![warn(clippy::pedantic)]
mod vector3;
use crate::vector3::*;
mod raytracing;
use crate::raytracing::*;
mod sphere;
use crate::sphere::*;

const IMAGE_WIDTH : u32 = 400;
const ASPECT_RATIO : f64 = 16.0/9.0;
const IMAGE_HEIGHT : u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;


fn main() {

    let viewport_height : f64 = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length : f64 = 1.0;
    
    let origin : Vector3 = Vector3 {x: 0.0, y: 0.0, z:0.0};
    let horizontal : Vector3 = Vector3 {x: viewport_width, y: 0.0, z: 0.0};
    let vertical : Vector3 = Vector3 {x: 0.0, y: viewport_height, z: 0.0};
    let lower_left_corner : Vector3 = origin - (horizontal / 2.0) - (vertical / 2.0) - Vector3 {x: 0.0, y: 0.0, z: focal_length};

    print!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");

    for j in 0..IMAGE_HEIGHT {
        let scanlines_remaining : u32 = IMAGE_HEIGHT-j;
        eprintln!("Scanlines remaining: {scanlines_remaining}");
        for i in 0..IMAGE_WIDTH {
            let u: f64 = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v: f64 = ((IMAGE_HEIGHT - 1) - j) as f64 / (IMAGE_HEIGHT - 1) as f64;
            let ray_origin: Vector3 = origin;
            let ray_direction: Vector3 = lower_left_corner + horizontal * u + vertical * v - origin;
            let ray: Ray = Ray {origin: ray_origin, direction: ray_direction};
            let pixel: Vector3 = ray_color(ray);
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

fn ray_color (ray: Ray) -> Vector3 {
    let sphere: Sphere = Sphere { center: Vector3{x: 0.0, y: 0.0, z: -1.0}, radius: 0.5};
    let hit_result = sphere.hit(&ray, 0.0, 100.0);
    let (hit, record) = hit_result;
    if hit {
        let sphere_normal: Vector3 = record.normal;
        Vector3 {x: sphere_normal.x + 1.0, y: sphere_normal.y + 1.0, z: sphere_normal.z + 1.0} * 0.5
    }    
    else { 
        let unit_direction: Vector3 = unit_vector(ray.direction);
        let t: f64 = 0.5*(unit_direction.y + 1.0);
        Vector3 {x: 1.0, y: 1.0, z: 1.0} * (1.0 - t) + Vector3 {x: 0.5, y: 0.7, z: 1.0} * t
    }
}
               
        
    
