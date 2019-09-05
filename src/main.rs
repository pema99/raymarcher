extern crate image;
#[macro_use] extern crate impl_ops;
use image::{ImageBuffer, RgbImage, Rgb};
use std::f32::consts::PI;
use std::ops;

struct Vec3 {
    x: f32,
    y: f32,
    z: f32
}

impl_op_ex!(+ |a: &Vec3, b: &Vec3| -> Vec3 { 
    Vec3::new(a.x + b.x, a.y + b.y, a.z + b.z)
});

impl_op_ex!(- |a: &Vec3, b: &Vec3| -> Vec3 { 
    Vec3::new(a.x - b.x, a.y - b.y, a.z - b.z)
});

impl_op_ex!(* |a: &Vec3, b: &f32| -> Vec3 { 
    Vec3::new(a.x * b, a.y * b, a.z * b)
});

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x: x,
            y: y,
            z: z
        }
    }

    fn magnitude(&self) -> f32 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }

    fn normalize(&self) -> Self {
        let mag = self.magnitude();
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag
        }
    }

    fn to_rgb(&self) -> Rgb<u8> {
        image::Rgb([
            (self.x * 255.0) as u8, 
            (self.y * 255.0) as u8, 
            (self.z * 255.0) as u8])
    }
}

fn main() {
    let img_width = 600;
    let img_height = 400; 
    let fov = 75.0;
    let aspect_ratio = img_width as f32 / img_height as f32;
    let cam_orig = Vec3::new(0.0, 0.0, -10.0);

    let mut img: RgbImage = ImageBuffer::new(img_width, img_height);
    
    for x in 0..img_width {
        for y in 0..img_height {
            let far_x = (2.0 * ((x as f32 + 0.5) / img_width as f32) - 1.0) * (fov / 2.0 * PI / 180.0).tan() * aspect_ratio; 
            let far_y = (1.0 - 2.0 * ((y as f32 + 0.5) / img_height as f32)) * (fov / 2.0 * PI / 180.0).tan(); 
            
            let ray_dir = (Vec3::new(far_x, far_y, 1.0) - &cam_orig).normalize();

            img.put_pixel(x, y, trace(&cam_orig, &ray_dir));
        }
    }

    img.save("epic.png");
}

fn trace(orig: &Vec3, dir: &Vec3) -> Rgb<u8> {
    let max_steps = 100;
    let max_dist = 100.0;
    let epsilon = 0.000001;

    let mut depth = 0.0;
    let mut hit = false;
    
    for i in 0..max_steps {
        let dist = sphere_sdf(orig + dir * depth);
        
        if dist < epsilon {
            hit = true;
            break;
        }

        depth += dist;

        if depth >= max_dist {
            break;
        }
    }

    if hit {
        Vec3::new((10.0-depth)*2.0, 0.0, 0.0).to_rgb()
    } 
    else {
        Vec3::new(0.0, 0.0, 0.0).to_rgb()
    }  
}

fn sphere_sdf(from: Vec3) -> f32 {
    (from - Vec3::new(0.0, 0.0, 0.0)).magnitude() - 0.4
}