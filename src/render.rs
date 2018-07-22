use ray::Ray;
use scene::Scene;
use coord::prelude::*;
use std::fs::File;
use std::io::Write;

pub struct Renderer {
    scene: Option<Scene>
}

impl Default for Renderer {
    fn default() -> Renderer {
        Renderer {
            scene: Some(Scene::default())
        }
    }
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            scene: None
        }
    }

    pub fn new_from_scene(in_scene: Scene) -> Renderer {
        Renderer {
            scene: Some(in_scene)
        }
    }

    pub fn switch_scene(&mut self, in_scene: Scene) {
        self.scene = Some(in_scene);
    }

    pub fn render(&self, mut buffer: File, image: Vec2<u16>) -> Result<(),()> {
        match self.scene {
            Some(ref scene) => {
                for y in (0..image.y - 1).rev() {
                    for x in 0..image.x {
                        let u = x as f32 / image.x as f32;
                        let v = y as f32 / image.y as f32;

                        let ray_direction = scene.lower_left
                            + vec3!(u*scene.horizontal.x, u*scene.horizontal.y, u*scene.horizontal.z)
                            + vec3!(v*scene.vertical.x, v*scene.vertical.y, v*scene.vertical.z);
                        let ray = Ray::new(scene.origin(), ray_direction);

                        let ray_colour = ray.colour(&scene);

                        let out_r = (ray_colour.x * 255.99) as u32;
                        let out_g = (ray_colour.y * 255.99) as u32;
                        let out_b = (ray_colour.z * 255.99) as u32;

                        write!(buffer, "{} {} {}\n", out_r, out_g, out_b)
                            .expect("Unable to write to file!");
                    }
                }
            },
            None => return Err(())
        }
        Ok(())
    }
}