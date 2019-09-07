use std::ops;

#[derive(Debug, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl_op_ex!(+ |a: &Vec3, b: &Vec3| -> Vec3 { 
    Vec3::new(a.x + b.x, a.y + b.y, a.z + b.z)
});

impl_op_ex!(- |a: &Vec3, b: &Vec3| -> Vec3 { 
    Vec3::new(a.x - b.x, a.y - b.y, a.z - b.z)
});

impl_op_ex!(* |a: &Vec3, b: &f64| -> Vec3 { 
    Vec3::new(a.x * b, a.y * b, a.z * b)
});

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
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag
        }
    }

    pub fn max(&self, val: f64) -> Self {
        Self {
            x: self.x.max(val),
            y: self.y.max(val),
            z: self.z.max(val)
        }
    }

    pub fn min(&self, val: f64) -> Self {
        Self {
            x: self.x.min(val),
            y: self.y.min(val),
            z: self.z.min(val)
        }
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
}