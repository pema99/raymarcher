#![allow(unused)]
mod raymarcher;
use raymarcher::{Vec3, Mat4};

use std::f64::consts::PI;
const EPSILON: f64 = 0.001;

#[macro_use]
extern crate impl_ops;
extern crate minifb;
use minifb::{Key, Window, WindowOptions, MouseMode};
extern crate rayon;
use rayon::prelude::*;

//Settings
const MAX_DIST: f64 = 30.0;
const IMG_WIDTH: usize = 600;
const IMG_HEIGHT: usize = 600;
const FOV: f64 = 90.0;

fn main() {
    //Mutable variables
    let mut cam_orig = Vec3::new(0.0, 0.0, -2.0);
    let mut cam_rot = Vec3::new(0.0, 0.0, 0.0);

    //Precalced variables
    let aspect_ratio = IMG_WIDTH as f64 / IMG_HEIGHT as f64;
    let inv_width = 1.0 / IMG_WIDTH as f64;
    let inv_height = 1.0 / IMG_HEIGHT as f64;
    let view_angle = (PI * 0.5 * FOV / 180.0).tan();
    let forward = Vec3::new(0.0, 0.0, 1.0);

    //Setup window
    let mut buffer: Vec<u32> = vec![0; IMG_WIDTH * IMG_HEIGHT];
    let mut window = Window::new(
        "Raymarcher thing",
        IMG_WIDTH,
        IMG_HEIGHT,
        WindowOptions::default(),
    ).unwrap();

    //Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        //Calculate rotation from mouse pos
        cam_rot.y = (1.0 - (window.get_mouse_pos(MouseMode::Clamp).unwrap().0 / IMG_WIDTH as f32) as f64) * 2.0 * PI - PI;
        cam_rot.x = (1.0 - (window.get_mouse_pos(MouseMode::Clamp).unwrap().1 / IMG_HEIGHT as f32) as f64) * PI - PI * 0.5;

        //Calculate rotation matrix
        let rot_matrix = Mat4::rotate(&cam_rot);

        //Handle movement
        window.get_keys().map(|keys| {
            for t in keys {
                match t {
                    Key::W => cam_orig = cam_orig + (rot_matrix * forward) * 0.1,
                    Key::S => cam_orig = cam_orig - (rot_matrix * forward) * 0.1,
                    Key::A => cam_orig = cam_orig + (Mat4::rotate_y(PI/2.0) * rot_matrix * forward) * 0.1,
                    Key::D => cam_orig = cam_orig + (Mat4::rotate_y(-PI/2.0) * rot_matrix * forward) * 0.1,
                    Key::Q => cam_orig.y += 0.1,
                    Key::E => cam_orig.y -= 0.1,

                    _ => (),
                }
            }
        });

        //Yield scanlines, build frame
        let frame = (0..IMG_HEIGHT).into_par_iter().map(|y| {
            //Yield pixels, build scanlines
            (0..IMG_WIDTH).into_par_iter().map(|x| {
                let mut ray_dir = Vec3::new(
                    (2.0 * (x as f64 * inv_width) - 1.0) * view_angle * aspect_ratio,
                    (1.0 - 2.0 * (y as f64 * inv_height)) * view_angle,
                    1.0,
                )
                .normalize();

                ray_dir = rot_matrix * ray_dir;

                trace(&cam_orig, &ray_dir)
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>();

        //Blit frame
        for (y, scanline) in frame.iter().enumerate() {
            for (x, pixel) in scanline.iter().enumerate() {
                buffer[y * IMG_WIDTH + x] = *pixel;
            }
        }

        window.update_with_buffer(&buffer).unwrap();
    }
}

//Trace a primary ray, return color
fn trace(orig: &Vec3, dir: &Vec3) -> u32 {
    let mut depth = 0.0;
    let mut hit = false;

    loop {
        let dist = scene_sdf(orig + dir * depth);

        if dist.abs() < EPSILON {
            return sdf_normal(orig + dir * depth)
                .apply(&|v: f64| v / 2.0 + 0.5)
                .to_color();
        }

        depth += dist;

        if depth >= MAX_DIST {
            break;
        }
    }

    Vec3::new(0.0, 0.0, 0.0).to_color()
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    (1.0 - t) * a + t * b
}

fn intersect(a: f64, b: f64) -> f64 {
    a.max(b)
}

fn union(a: f64, b: f64) -> f64 {
    a.min(b)
}

fn difference(a: f64, b: f64) -> f64 {
    a.max(-b)
}

fn union_smooth(a: f64, b: f64, k: f64) -> f64 {
    let h = (0.5 + 0.5 * (a - b) / k).max(0.0).min(1.0);
    lerp(a, b, h) - k * h * (1.0 - h)
}

fn intersect_smooth(a: f64, b: f64, k: f64) -> f64 {
    union_smooth(a, b, -k)
}

fn difference_smooth(a: f64, b: f64, k: f64) -> f64 {
    union_smooth(a, -b, -k)
}

fn sphere_sdf(from: &Vec3, center: Vec3, radius: f64) -> f64 {
    (from - center).magnitude() - radius
}

fn box_sdf(from: &Vec3, center: Vec3, size: Vec3) -> f64 {
    let d = (from - center).abs() - size;
    d.max(0.0).magnitude()
    // + d.x.max(d.y.max(d.z)).min(0.0)
}

fn scene_sdf(from: Vec3) -> f64 {
    union_smooth(
        box_sdf(&from, Vec3::new(0.0, -0.5, 0.0), Vec3::new(0.5, 0.5, 0.5)),
        sphere_sdf(&from, Vec3::new(0.0, 0.4, 0.0), 0.7),
        0.1,
    )
}

fn sdf_normal(p: Vec3) -> Vec3 {
    Vec3::new(
        scene_sdf(Vec3::new(p.x + EPSILON, p.y, p.z))
            - scene_sdf(Vec3::new(p.x - EPSILON, p.y, p.z)),
        scene_sdf(Vec3::new(p.x, p.y + EPSILON, p.z))
            - scene_sdf(Vec3::new(p.x, p.y - EPSILON, p.z)),
        scene_sdf(Vec3::new(p.x, p.y, p.z + EPSILON))
            - scene_sdf(Vec3::new(p.x, p.y, p.z - EPSILON)),
    )
    .normalize()
}
