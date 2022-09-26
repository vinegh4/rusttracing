use crate::raytracing::*;
use crate::vector3::*;

pub struct Sphere {
    pub center: Vector3,
    pub radius: f64
}

impl Hittable for Sphere {
    //essentially, solves for the intersection equation between the surface of the spehre and the ray
    //if the equation has no roots (discriminant is less than 0) we return false
    //otherwise, we return true and a record for the intersection
    //given by the smallest root of the equation (smallest root means the closest intersect to the ray origin)
    fn hit(self, ray: &Ray, t_min: f64, t_max: f64) -> (bool, HitRecord) {
        let offset: Vector3 = ray.origin - self.center;
        let a_quadratic: f64 = square_magnitude(ray.direction);
        let half_b_quadratic: f64 = dot(offset, ray.direction);
        let c_quadratic: f64 = square_magnitude(offset) - self.radius * self.radius;
        let discriminant: f64 = half_b_quadratic * half_b_quadratic - a_quadratic * c_quadratic;
        let mut record: HitRecord = HitRecord::new();

        if discriminant < 0.0 {
            return (false, record);
        }

        let sqrt_discriminant: f64 = discriminant.sqrt();
        
        //check if the closest of the sphere-ray intersection fall within the specified range
        let mut root: f64 = (-1.0 * half_b_quadratic - sqrt_discriminant) / a_quadratic;
        if root < t_min || t_max < root {
            root = (-1.0 * half_b_quadratic + sqrt_discriminant) / a_quadratic;
            if root < t_min || t_max < root {
                return (false, record);
            }
        }

        let hitpoint: Vector3 = cast_ray(ray, root);
        record.hitpoint = hitpoint;
        record.normal = (hitpoint - self.center) / self.radius;
        record.t = root;
        (true, record)
    }
}