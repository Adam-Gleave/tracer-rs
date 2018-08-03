use coord::prelude::*;
use rand::{thread_rng, Rng};
use scene::{Scene, Sphere, CollisionRecord};

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
            let mut rec = CollisionRecord::default();

            if self.collides_with(sphere, 0.001, std::f32::MAX, &mut rec) {
                let target = rec.position + rec.normal + random_in_unit_sphere();
                let diffuse_ray = Ray::new(rec.position, target - rec.position);
                let diffuse_colour = diffuse_ray.colour(scene);
                return vec3!(0.5*diffuse_colour.x, 0.5*diffuse_colour.y, 0.5*diffuse_colour.z);
            }
        }
        let t = 0.5*(self.direction.norm().y + 1.0);
        return vec3!(1.0 - t, 1.0 - t, 1.0 - t) + vec3!(t*0.5, t*0.7, t*1.0);
    }

    pub fn collides_with(&self, sphere: &Sphere, t_min: f32, t_max: f32, rec: &mut CollisionRecord) -> bool {
        let o_c = self.origin() - sphere.center();
        let a = dot(self.direction(), self.direction());
        let b = dot(o_c, self.direction());
        let c = dot(o_c, o_c) - sphere.radius().powf(2.0);

        let discriminant = b*b - a*c;
        if discriminant > 0.0 {
            let mut temp = ((-b - discriminant.sqrt()) /a );
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.position = self.at_time(rec.t);
                rec.normal = (rec.position - sphere.center()) / sphere.radius();
                return true;
            }

            temp = ((-b + discriminant.sqrt()) / a);
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.position = self.at_time(rec.t);
                rec.normal = (rec.position - sphere.center()) / sphere.radius();
                return true;
            }
        }
        false
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
        point = vec3!(thread_rng().gen_range::<f32>(-1.0, 1.0)
            , thread_rng().gen_range::<f32>(-1.0, 1.0)
            , thread_rng().gen_range::<f32>(-1.0, 1.0));

        if magnitude(point) < 1.0 {
            cont = false;
        }
    }
    point
}