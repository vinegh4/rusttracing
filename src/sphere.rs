use crate::raytracing::*;
use crate::vector3::*;

#[derive(Hittable)]
pub struct Sphere {
    pub center: Vector3,
    pub radius: f64
}

impl Hittable for Sphere {
    //essentially, solves for the intersection equation between the surface of the spehre and the ray
    //if the equation has no roots (discriminant is less than 0) we return false
    //otherwise, we return true and a record for the intersection
    //given by the smallest root of the equation (smallest root means the closest intersect to the ray origin)
    pub fn hit(sphere: Sphere, ray: &Ray, t_min: f64, t_max: f64) -> (bool, HitRecord) {
        let offset: Vector3 = ray.origin - sphere.center;
        let a_quadratic: f64 = dot(ray.direction, ray.direction);
        let half_b_quadratic: f64 = dot(ray_offset, ray.direction);
        let c_quadratic: f64 = dot(ray_offset, ray_offset) - radius * radius;
        let discriminant: f64 = half_b_quadratic * half_b_quadratic - a_quadratic * c_quadratic;

        if(discriminant < 0) {
            false
        }

        sqrt_discriminant = sqrtf(discriminant);
        
        //check if the closest of the sphere-ray intersection fall within the specified range
        
    }
}