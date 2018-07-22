use coord::prelude::*;
use scene::{Scene, Sphere};

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

    pub fn colour(&self, scene: &Scene) -> Vec3<f32> {
        let norm_direction = self.direction.norm();
        let t = 0.5 * (norm_direction.y + 1.0);
        let t2 = 1.0 - t;

        for sphere in scene.spheres() {
            if self.collides_with(sphere) {
                return vec3!(1.0, 0.0, 0.0);
            }
        }
        vec3!(1.0*t2, 1.0*t2, 1.0*t2) + vec3!(0.3*t, 0.3*t, 0.3*t)
    }

    pub fn collides_with(&self, sphere: &Sphere) -> bool {
        let o_c = self.origin() - sphere.center();
        let a = dot(self.direction(), self.direction());
        let b = 2.0 * dot(o_c, self.direction());
        let c = dot(o_c, o_c) - sphere.radius()*sphere.radius();

        b*b - 4.0*a*c > 0.0
    }

    pub fn origin(&self) -> Vec3<f32> {
        self.origin
    }

    pub fn direction(&self) -> Vec3<f32> {
        self.direction
    }
}

fn dot(a: Vec3<f32>, b: Vec3<f32>) -> f32 {
    a.x*b.x + a.y*b.y + a.z*b.z
}