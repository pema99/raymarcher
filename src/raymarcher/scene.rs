use super::shapes::{SceneObject, DistanceField};
use super::Vec3;

#[derive(Serialize, Deserialize)]
pub struct Scene {
	pub shapes: Vec<SceneObject>
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

