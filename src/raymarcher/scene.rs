use super::shapes::DistanceField;
use super::Vec3;
use super::EPSILON;

pub struct Scene {
	pub shapes: Vec<Box<DistanceField + Sync>>
}

impl DistanceField for Scene {
    fn sdf(&self, from: &Vec3) -> f64 {
		let mut min = std::f64::MAX;
		for s in self.shapes.iter() {
			let dist = s.sdf(from);
			if dist < min {
    			min = dist;
    		}
		}
		min
	}
}

impl Scene {
    pub fn sdf_normal(&self, p: Vec3) -> Vec3 {
        Vec3::new(
            self.sdf(&Vec3::new(p.x + EPSILON, p.y, p.z)) - self.sdf(&Vec3::new(p.x - EPSILON, p.y, p.z)),
            self.sdf(&Vec3::new(p.x, p.y + EPSILON, p.z)) - self.sdf(&Vec3::new(p.x, p.y - EPSILON, p.z)),
            self.sdf(&Vec3::new(p.x, p.y, p.z + EPSILON)) - self.sdf(&Vec3::new(p.x, p.y, p.z - EPSILON)),
        ).normalize()
    }
}

