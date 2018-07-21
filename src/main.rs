#![feature(extern_prelude)]

#[macro_use] extern crate coord;

mod ray;

use ray::Ray;
use std::fs::File;
use std::io::Write;
use coord::prelude::*;

const IMAGE_WIDTH: u16 = 200;
const IMAGE_HEIGHT: u16 = 100;

fn main() -> std::io::Result<()> {
    let mut buffer = File::create("out.ppm").expect("Unable to create file!");
    write!(buffer, "P3\n{} {}\n255\n", IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32)
        .expect("Unable to write to file!");

    let lower_left = vec3!(-2.0, -1.0, -1.0);
    let origin = vec3!(0.0, 0.0, 0.0);
    let horizontal = vec3!(4.0, 0.0, 0.0);
    let vertical = vec3!(0.0, 2.0, 0.0);

    for y in (0..IMAGE_HEIGHT - 1).rev() {
        for x in 0..IMAGE_WIDTH {
            let u = x as f32 / IMAGE_WIDTH as f32;
            let v = y as f32 / IMAGE_HEIGHT as f32;

            let ray_direction = lower_left + vec3!(u*horizontal.x, u*horizontal.y, u*horizontal.z)
                + vec3!(v*vertical.x, v*vertical.y, v*vertical.z);
            let ray = Ray::new(origin, ray_direction);
            let ray_colour = ray.colour();

            let out_r = (ray_colour.x*255.99) as u32;
            let out_g = (ray_colour.y*255.99) as u32;
            let out_b = (ray_colour.z*255.99) as u32;

            write!(buffer, "{} {} {}\n", out_r, out_g, out_b)
                .expect("Unable to write to file!");
        }
    }
    Ok(())
}
