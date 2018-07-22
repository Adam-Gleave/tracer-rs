use coord::prelude::*;
use std::vec::Vec;

struct Sphere {
    radius: f32,
    origin: Vec3<f32>
}

pub struct Scene {
    spheres: Vec<Sphere>
}

impl Sphere {
    pub fn new(in_radius: f32, in_origin: Vec3<f32>) -> Sphere {
        Sphere {
            radius: in_radius,
            origin: in_origin
        }
    }
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            spheres: Vec::new()
        }
    }

    pub fn add_sphere(&mut self, in_radius: f32, in_origin: Vec3<f32>) {
        self.spheres.push(Sphere::new(in_radius, in_origin));
    }
}