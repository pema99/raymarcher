#![allow(unused)]
mod raymarcher;
use raymarcher::Vec3;

#[macro_use] extern crate impl_ops;
use std::f64::consts::PI;
extern crate minifb;
use minifb::{Key, WindowOptions, Window};

impl Vec3 {
    fn to_color(&self) -> u32 {
        let r = (self.x * 255.0) as u32;
        let g = (self.y * 255.0) as u32;
        let b = (self.z * 255.0) as u32;

        b | (g << 8) | (r << 16) | (255 << 24)
    }
}

fn main() {
    let img_width = 600;
    let img_height = 600; 
    let fov = 90.0;
    let mut cam_orig = Vec3::new(0.0, 0.0, -2.0);
    let mut rot = Vec3::new(0.0, 0.0, 0.0);

    let aspect_ratio = img_width as f64 / img_height as f64;
    let inv_width = 1.0 / img_width as f64;
    let inv_height = 1.0 / img_height as f64;
    let view_angle = (PI * 0.5 * fov / 180.0).tan();
    let mut fwd = Vec3::new(0.0, 0.0, 0.0);

    let mut buffer: Vec<u32> = vec![0; img_width * img_height];
    let mut window = Window::new("Raymarcher thing", img_width, img_height, WindowOptions::default()).unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.get_keys().map(|keys| {
            for t in keys {
                match t {
                    Key::W => cam_orig = &cam_orig + &fwd * 0.1,
                    Key::S => cam_orig = &cam_orig - &fwd * 0.1,
                    Key::Q => cam_orig.y += 0.1,
                    Key::E => cam_orig.y -= 0.1,

                    Key::Left => rot.y -= 0.1,
                    Key::Right => rot.y += 0.1,
                    Key::Up => rot.x -= 0.1,
                    Key::Down => rot.x += 0.1,

                    _ => (),
                }
            }
        });

        for x in 0..img_width {
            for y in 0..img_height {
                let mut ray_dir = Vec3::new(
                    (2.0 * (x as f64 * inv_width) - 1.0) * view_angle * aspect_ratio, 
                    (1.0 - 2.0 * (y as f64 * inv_height)) * view_angle, 
                    1.0).normalize();

                //Rotate, temporary till i implement matrices
                ray_dir = Vec3::new(
                    ray_dir.x,
                    ray_dir.y * rot.x.cos() - ray_dir.z * rot.x.sin(),
                    ray_dir.y * rot.x.sin() + ray_dir.z * rot.x.cos()
                );
                ray_dir = Vec3::new(
                    ray_dir.x * rot.y.cos() + ray_dir.z * rot.y.sin(),
                    ray_dir.y,
                    -ray_dir.x * rot.y.sin() + ray_dir.z * rot.y.cos()
                );
                ray_dir = Vec3::new(
                    ray_dir.x * rot.z.cos() - ray_dir.y * rot.z.sin(),
                    ray_dir.x * rot.z.sin() + ray_dir.y * rot.z.cos(),
                    ray_dir.z
                );

                //UGLY HACK, REFACTOR
                if x == img_width / 2 && y == img_height / 2 {
                    fwd = ray_dir.clone();
                }

                buffer[y*img_width+x] = trace(&cam_orig, &ray_dir);
            }
        }

        window.update_with_buffer(&buffer).unwrap();
    }

    //img.save("epic.png");
}

fn trace(orig: &Vec3, dir: &Vec3) -> u32 {
    let max_steps = 100;
    let max_dist = 100.0;
    let epsilon = 0.001;

    let mut depth = 0.0;
    let mut hit = false;
    
    for i in 0..max_steps {
        let dist = scene_sdf(orig + dir * depth);
        
        if dist.abs() < epsilon {        
            hit = true;
            break;
        }

        depth += dist;

        if depth >= max_dist {
            break;
        }
    }

    if hit {
        //Vec3::new(1.0, 0.0, 0.0).to_color()
        sdf_normal(orig + dir * depth).apply(&|v: f64| { v / 2.0 + 0.5 }).to_color()
    } 
    else {
        Vec3::new(0.0, 0.0, 0.0).to_color()
    }  
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
    //let center = Vec3::new(0.0, 0.0, 0.0);
    (from - center).magnitude() - radius
}

fn box_sdf(p: &Vec3, b: &Vec3) -> f64 {
    let d = p.abs() - b;
    d.max(0.0).magnitude()
    //(d.max(0.0)).magnitude() + d.x.max(d.y.max(d.z)).min(0.0)
}

fn scene_sdf(from: Vec3) -> f64 {
    union_smooth(
        //sphere_sdf(&from, Vec3::new(0.0, 0.3, 0.0), 0.5),
        box_sdf(&(&from - Vec3::new(0.0, 0.0, 0.0)), &Vec3::new(0.5, 0.5, 0.5)),
        sphere_sdf(&from, Vec3::new(0.0, 0.4, 0.0), 0.7),
        0.1
    )
}

fn sdf_normal(p: Vec3) -> Vec3 {
    Vec3::new(
        scene_sdf(Vec3::new(p.x + 0.00001, p.y, p.z)) - scene_sdf(Vec3::new(p.x - 0.00001, p.y, p.z)),
        scene_sdf(Vec3::new(p.x, p.y + 0.00001, p.z)) - scene_sdf(Vec3::new(p.x, p.y - 0.00001, p.z)),
        scene_sdf(Vec3::new(p.x, p.y, p.z  + 0.00001)) - scene_sdf(Vec3::new(p.x, p.y, p.z - 0.00001))
    ).normalize()
}