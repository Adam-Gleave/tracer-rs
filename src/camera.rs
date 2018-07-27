use ray::Ray;
use coord::prelude::*;

pub struct Camera {
    pub lower_left: Vec3<f32>,
    pub origin: Vec3<f32>,
    pub horizontal: Vec3<f32>,
    pub vertical: Vec3<f32>
}

impl Default for Camera {
    fn default() -> Camera {
        Camera {
            lower_left: vec3!(-2.0, -1.0, -1.0),
            origin: vec3!(0.0, 0.0, 0.0),
            horizontal: vec3!(4.0, 0.0, 0.0),
            vertical: vec3!(0.0, 2.0, 0.0),
        }
    }
}

impl Camera {
    pub fn ray_at(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin, self.lower_left
            + vec3!(u*self.horizontal.x, u*self.horizontal.y, u*self.horizontal.z)
            + vec3!(v*self.vertical.x, v*self.vertical.y, v*self.vertical.z)
            - self.origin)
    }
}