use crate::vector3::Vector3;

pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3
}


fn cast_ray (ray: Ray, dist: f64) -> Vector3 {
        ray.origin + (ray.direction  * dist)
}


