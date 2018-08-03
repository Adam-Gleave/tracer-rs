use ray::Ray;
use camera::Camera;
use scene::Scene;
use {SAMPLES, GAMMA};
use coord::prelude::*;
use rand::{thread_rng, Rng};
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
        let camera = Camera::default();

        match self.scene {
            Some(ref scene) => {
                for y in (0..image.y - 1).rev() {
                    for x in 0..image.x {
                        let mut colour = vec3!(0.0, 0.0, 0.0);

                        for s in 0..SAMPLES {
                            let u_r = thread_rng().gen_range::<f32>(0.0, 1.0);
                            let v_r = thread_rng().gen_range::<f32>(0.0, 1.0);
                            let u = (x as f32 + u_r )/ image.x as f32;
                            let v = (y as f32 + v_r )/ image.y as f32;

                            let ray = camera.ray_at(u, v);
                            colour = colour + ray.colour(&scene);
                        }

                        colour = vec3!(
                            (colour.x / SAMPLES as f32).powf(1.0 / GAMMA),
                            (colour.y / SAMPLES as f32).powf(1.0 / GAMMA),
                            (colour.z / SAMPLES as f32).powf(1.0 / GAMMA)
                        );

                        let out_r = (colour.x * 255.99) as u32;
                        let out_g = (colour.y * 255.99) as u32;
                        let out_b = (colour.z * 255.99) as u32;

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