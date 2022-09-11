use crate::vector3::Vector3;

pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3
}

pub fn cast_ray (ray: Ray, dist: f64) -> Vector3 {
        ray.origin + (ray.direction  * dist)
}

pub struct HitRecord {
    hitpoint: Vector3,
    normal: Vector3,
    t: f64
}

pub trait Hittable {
    fn hit(ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
}


