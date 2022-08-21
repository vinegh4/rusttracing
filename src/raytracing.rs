use crate::vector3::Vector3;


fn cast_ray (origin: Vector3, direction: Vector3, dist: f64) -> Vector3 {
        origin + (direction  * dist)
}


