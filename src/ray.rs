use coord::prelude::*;

pub struct Ray {
    origin: Vec3<f32>,
    direction: Vec3<f32>
}

impl Ray {
    pub fn new(origin: Vec3<f32>, direction: Vec3<f32>) -> Ray {
        Ray {
            origin,
            direction
        }
    }

    pub fn colour(&self) -> Vec3<f32> {
        let norm_direction = self.direction.norm();
        let t = 0.5 * (norm_direction.y + 1.0);
        let t2 = 1.0 - t;

        vec3!(1.0*t2, 1.0*t2, 1.0*t2) + vec3!(0.3*t, 0.3*t, 0.3*t)
    }
}