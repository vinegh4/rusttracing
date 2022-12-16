use crate::vector3::*;

pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3
}

pub fn cast_ray (ray: &Ray, dist: f64) -> Vector3 {
        ray.origin + (ray.direction  * dist)
}

pub struct HitRecord {
    pub hitpoint: Vector3,
    pub normal: Vector3,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {
    pub fn new() -> Self {
        Self {hitpoint: Vector3{x: -1.0, y: -1.0, z: -1.0}, normal: Vector3 { x: 1.0, y: 1.0, z: 1.0 }, t: -1.0, front_face: false}
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vector3) {
        self.front_face = dot(ray.direction, *outward_normal) > 0.0;
        self.normal = *outward_normal;
        if !self.front_face {
            self.normal = self.normal * -1.0;
        }
    }
}

pub trait Hittable {
    fn hit(self, ray: &Ray, t_min: f64, t_max: f64) -> (bool, HitRecord);
}


