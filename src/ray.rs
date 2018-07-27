use coord::prelude::*;
use rand::{thread_rng, Rng};
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
        for sphere in scene.spheres() {
            let mut t = self.collides_with(sphere);

            if t > 0.0 {
                let n = (self.at_time(t) - sphere.center()).norm();
                return vec3!(0.5*(n.x+1.0), 0.5*(n.y+1.0), 0.5*(n.z+1.0));
            }
        }
        let unit_direction = self.direction.norm();
        let t = 0.5 * (unit_direction.y + 1.0);
        vec3!(1.0-t, 1.0-t, 1.0-t) + vec3!(t*0.5, t*0.7, t*1.0)
    }

    pub fn collides_with(&self, sphere: &Sphere) -> f32 {
        let o_c = self.origin() - sphere.center();
        let a = dot(self.direction(), self.direction());
        let b = 2.0 * dot(o_c, self.direction());
        let c = dot(o_c, o_c) - sphere.radius()*sphere.radius();

        let discriminant = b*b - 4.0*a*c;
        if discriminant < 0.0 {
            return -1.0
        }
        ((-b - discriminant.sqrt()) / (2.0*a))
    }

    pub fn origin(&self) -> Vec3<f32> {
        self.origin
    }

    pub fn direction(&self) -> Vec3<f32> {
        self.direction
    }

    pub fn at_time(&self, t: f32) -> Vec3<f32> {
        self.origin + vec3!(t*self.direction.x, t*self.direction.y, t*self.direction.z)
    }
}

fn dot(a: Vec3<f32>, b: Vec3<f32>) -> f32 {
    a.x*b.x + a.y*b.y + a.z*b.z
}

fn magnitude(vec: Vec3<f32>) -> f32 {
    (vec.x.powf(2.0) + vec.y.powf(2.0) + vec.z.powf(2.0)).sqrt()
}

fn random_in_unit_sphere() -> Vec3<f32> {
    let mut cont = true;
    let mut point = Vec3::default();

    while cont {
        let point = vec3!(thread_rng().gen_range::<f32>(-1.0, 1.0)
            , thread_rng().gen_range::<f32>(-1.0, 1.0)
            , thread_rng().gen_range::<f32>(-1.0, 1.0));

        if magnitude(point) < 1.0 {
            cont = false;
        }
    }
    point
}