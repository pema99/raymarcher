use super::shapes::DistanceField;
use super::Vec3;

pub struct Scene {
	pub shapes: Vec<Box<dyn DistanceField + Sync>>
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

