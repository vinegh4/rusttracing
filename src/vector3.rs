use std::ops;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

pub fn magnitude(vector: Vector3) -> f64 {
   (vector.x.powf(2.0) + vector.y.powf(2.0) + vector.z.powf(2.0)).sqrt()
}

pub fn unit_vector(vector: Vector3) -> Vector3 {
    vector / magnitude(vector)
}

pub fn dot(lhs: Vector3, rhs: Vector3) -> f64 {
    (lhs.x * rhs.x) + (lhs.y * rhs.y) + (lhs.z * rhs.z)
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, _rhs: Vector3) -> Vector3 {
        Vector3 {x: self.x + _rhs.x, y: self.y + _rhs.y, z: self.z + _rhs.z}
    }
}

impl ops::AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, _rhs : Vector3) {
        *self = *self + _rhs;
    }
}

impl ops::Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, _rhs: f64) -> Vector3 {
        Vector3 {x: self.x * _rhs, y: self.y * _rhs, z: self.z * _rhs}    
    }
}

impl ops::MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, _rhs : f64) {
        *self = *self * _rhs;
    }
}

impl ops::Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, _rhs: f64) -> Vector3 {
        let multiplier = 1.0/_rhs;
        self * multiplier
    }
}

impl ops::DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, _rhs : f64) {
        *self = *self / _rhs;
    }
}

impl ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, _rhs: Vector3) -> Vector3 {
        let neg: Vector3 = _rhs * -1.0;
        self + neg
    }
}

impl ops::SubAssign<Vector3> for Vector3 {
    fn sub_assign(&mut self, _rhs: Vector3) {
        *self = *self - _rhs;
    }
}


#[cfg(test)]
mod tests {
    use crate::vector3; 
    use crate::vector3::Vector3;

    #[test]
    fn test_add() {
        let vec1 : Vector3 = Vector3 {x: 1.0, y: 2.0, z: 3.0};
        let vec2 : Vector3 = Vector3 {x: 2.0, y: 3.0, z: 4.0};
        let result = vec1 + vec2;
        assert_eq!(result, Vector3 {x: 3.0, y: 5.0, z: 7.0});
    }

    #[test]
    fn test_add_assign() {
        let mut result : Vector3 = Vector3 {x: 1.0, y: 2.0, z: 3.0};
        let vec2 : Vector3 = Vector3 {x: 2.0, y: 3.0, z: 4.0};
        result += vec2;
        assert_eq!(result, Vector3 {x: 3.0, y: 5.0, z: 7.0});
    }

    #[test]
    fn test_mul() {
        let vec1 : Vector3 = Vector3 {x: 1.0, y: 2.0, z: 3.0};
        let scalar : f64 = 0.5;
        let result : Vector3 = vec1 * scalar;
        assert_eq!(result, Vector3 {x: 0.5, y: 1.0, z: 1.5});
    }

    #[test]
    fn test_mul_assign() {
        let mut result : Vector3 = Vector3 {x: 1.0, y: 2.0, z: 3.0};
        let scalar : f64 = 0.5;
        result *= scalar;
        assert_eq!(result, Vector3 {x: 0.5, y: 1.0, z: 1.5});
    }
    
    #[test]
    fn test_div() {
        let vec1 : Vector3 = Vector3 {x: 1.0, y: 2.0, z: 3.0};
        let scalar : f64 = 2.0;
        let result : Vector3 = vec1 / scalar;
        assert_eq!(result, Vector3 {x: 0.5, y: 1.0, z: 1.5});
    }

    #[test]
    fn test_div_assign() {
        let mut result : Vector3 = Vector3 {x: 1.0, y: 2.0, z: 3.0};
        let scalar : f64 = 2.0;
        result /= scalar;
        assert_eq!(result, Vector3 {x: 0.5, y: 1.0, z: 1.5});
    }

    #[test]
    fn test_magnitude() {
        let vec : Vector3 = Vector3 {x: 2.0, y: 4.0, z: 4.0};
        assert_eq!(vector3::magnitude(vec), 6.0);
    }

    #[test]
    fn test_sub() {
        let vec1 : Vector3 = Vector3 {x: 1.0, y: 2.0, z: 3.0};
        let vec2 : Vector3 = Vector3 {x: 2.0, y: 3.0, z: 4.0};
        let result = vec1 - vec2;
        assert_eq!(result, Vector3 {x: -1.0, y: -1.0, z: -1.0});
    }

    #[test]
    fn test_sub_assign() {
        let mut result : Vector3 = Vector3 {x: 1.0, y: 2.0, z: 3.0};
        let vec2 : Vector3 = Vector3 {x: 2.0, y: 3.0, z: 4.0};
        result -= vec2;
        assert_eq!(result, Vector3 {x: -1.0, y: -1.0, z: -1.0});
    }
    
}
