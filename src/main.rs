#![feature(extern_prelude)]

#[macro_use]
extern crate coord;
extern crate rand;

mod ray;
mod scene;
mod render;
mod camera;

use coord::prelude::*;
use render::Renderer;
use std::fs::File;
use std::io::Write;

const IMAGE_WIDTH: u16 = 400;
const IMAGE_HEIGHT: u16 = 200;
pub const GAMMA: f32 = 1.4;
pub const SAMPLES: u16 = 100;

fn main() -> std::io::Result<()> {
    let mut buffer = File::create("out/out.ppm").expect("Unable to create file!");
    write!(buffer, "P3\n{} {}\n255\n", IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32)
        .expect("Unable to write to file!");
    let renderer = Renderer::default();
    renderer.render(buffer, vec2!(IMAGE_WIDTH, IMAGE_HEIGHT))
        .expect("Unable to render scene!");
    Ok(())
}