use crate::vector3::Vector3;

pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3
}

pub fn cast_ray (ray: Ray, dist: f64) -> Vector3 {
        ray.origin + (ray.direction  * dist)
}

pub struct HitRecord {
    pub hitpoint: Vector3,
    pub normal: Vector3,
    pub jt: f64
}

pub trait Hittable {
    pub fn hit(ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
}


