use coord::prelude::*;
use std::vec::Vec;

pub struct Sphere {
    radius: f32,
    center: Vec3<f32>
}

pub struct Scene {
    spheres: Vec<Sphere>,
    pub lower_left: Vec3<f32>,
    pub origin: Vec3<f32>,
    pub horizontal: Vec3<f32>,
    pub vertical: Vec3<f32>
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
        let mut scene = Scene {
            spheres: vec![],
            lower_left: vec3!(-2.0, -1.0, -1.0),
            origin: vec3!(0.0, 0.0, 0.0),
            horizontal: vec3!(4.0, 0.0, 0.0),
            vertical: vec3!(0.0, 2.0, 0.0),
        };

        let radius = 0.5;
        let origin = vec3!(0.0, 0.0, -1.0);
        scene.add_sphere(radius, origin);
        scene
    }
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            spheres: vec![],
            ..Default::default()
        }
    }

    pub fn add_sphere(&mut self, in_radius: f32, in_origin: Vec3<f32>) {
        self.spheres.push(Sphere::new(in_radius, in_origin));
    }

    pub fn spheres(&self) -> &Vec<Sphere> {
        &self.spheres
    }

    pub fn origin(&self) -> Vec3<f32> {
        self.origin
    }
}