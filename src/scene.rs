use coord::prelude::*;
use std::vec::Vec;

pub struct Sphere {
    radius: f32,
    center: Vec3<f32>
}

pub struct Scene {
    spheres: Vec<Sphere>,
}

impl Default for Sphere {
    fn default() -> Sphere {
        Sphere {
            radius: 0.5,
            center: vec3!(0.0, 0.0, -1.0)
        }
    }
}

impl Sphere {
    pub fn new(in_radius: f32, in_origin: Vec3<f32>) -> Sphere {
        Sphere {
            radius: in_radius,
            center: in_origin
        }
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn center(&self) -> Vec3<f32> {
        self.center
    }
}

impl Default for Scene {
    fn default() -> Scene {
        Scene {
            spheres: vec![
                Sphere::default(),
                Sphere::new(100.0, vec3!(0.0, -100.5, -1.0))
            ]
        }
    }
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            spheres: vec![]
        }
    }

    pub fn add_sphere(&mut self, in_radius: f32, in_origin: Vec3<f32>) {
        self.spheres.push(Sphere::new(in_radius, in_origin));
    }

    pub fn spheres(&self) -> &Vec<Sphere> {
        &self.spheres
    }
}