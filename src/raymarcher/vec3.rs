use std::ops;

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x: x,
            y: y,
            z: z
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        self.apply(&|i| i / mag)
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x*other.x + self.y*other.y + self.z*other.z
    }

    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        let dot_prod = self.dot(normal);
        self - (2.0 * normal) * dot_prod
    }

    pub fn max(&self, val: f64) -> Self {
        self.apply(&|i| i.max(val))
    }

    pub fn min(&self, val: f64) -> Self {
        self.apply(&|i| i.min(val))
    }

    pub fn abs(&self) -> Vec3 {
        self.apply(&f64::abs)
    }

    pub fn apply(&self, fun: &Fn(f64) -> f64) -> Self {
        Self {
            x: fun(self.x),
            y: fun(self.y),
            z: fun(self.z)
        }
    }

    pub fn to_color(&self) -> u32 {
        let r = (self.x * 255.0) as u32;
        let g = (self.y * 255.0) as u32;
        let b = (self.z * 255.0) as u32;

        b | (g << 8) | (r << 16) | (255 << 24)
    }
}

impl_op_ex!(+ |a: &Vec3, b: &Vec3| -> Vec3 { 
    Vec3::new(a.x + b.x, a.y + b.y, a.z + b.z)
});

impl_op_ex!(- |a: &Vec3, b: &Vec3| -> Vec3 { 
    Vec3::new(a.x - b.x, a.y - b.y, a.z - b.z)
});

impl_op_ex_commutative!(* |a: &Vec3, b: &f64| -> Vec3 { 
    Vec3::new(a.x * b, a.y * b, a.z * b)
});

impl_op_ex!(/ |a: &Vec3, b: &f64| -> Vec3 { 
    Vec3::new(a.x / b, a.y / b, a.z / b)
});

impl_op!(- |a: &Vec3| -> Vec3 { 
    Vec3::new(-a.x, -a.y, -a.z)
});

impl_op!(- |a: Vec3| -> Vec3 { 
    Vec3::new(-a.x, -a.y, -a.z)
});
