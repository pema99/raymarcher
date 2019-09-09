use super::Vec3;
use super::math::*;

trait DistanceField{
	fn SDF(&self, from: &Vec3) -> f64;
}

struct Scene {
	shapes: Vec<Box<DistanceField>>
}

impl DistanceField for Scene {
	fn SDF(&self, from: &Vec3) -> f64 {
		let mut min = std::f64::MAX;
		for s in self.shapes.iter() {
			let dist = s.SDF(from);
			if dist < min {
    			min = dist;
    		}
		}
		min
	}
}

struct Sphere {
	center: Vec3,
	radius: f64
}

impl Sphere {
	pub fn new(center: Vec3, radius: f64) -> Self {
		Self {
			center: center,
			radius: radius
		}
	}
}

impl DistanceField for Sphere {
	fn SDF(&self, from: &Vec3) -> f64 {
		(from - self.center).magnitude() - self.radius
	}
}

struct Cube {
	center: Vec3,
	size: Vec3
}

impl Cube {
	pub fn new(center: Vec3, size: Vec3) -> Self {
		Self {
    		center: center,
			size: size
		}
	}
}

impl DistanceField for Cube {
	fn SDF(&self, from: &Vec3) -> f64 {
		let diff = (from - self.center).abs() - self.size;
		diff.max(0.0).magnitude()
		// + diff.x.max(diff.y.max(diff.z)).min(0.0)
	}
}

enum CSGOperator {
	Union,
	Intersect,
	Difference,
	UnionSmooth(f64),
	IntersectSmooth(f64),
	DifferenceSmooth(f64)
}

struct CSG {
	a: Box<dyn DistanceField>,
	b: Box<dyn DistanceField>,
	op: CSGOperator
}

impl CSG {
	pub fn new(op: CSGOperator, a: Box<dyn DistanceField>, b: Box<dyn DistanceField>) -> Self {
		Self {
			a: a,
			b: b,
			op: op
		}
	}
}

impl DistanceField for CSG {
	fn SDF(&self, from: &Vec3) -> f64 {
		match self.op {
    		CSGOperator::Union => self.a.SDF(&from).min(self.b.SDF(&from)),
    		CSGOperator::Intersect => self.a.SDF(&from).max(self.b.SDF(&from)),
    		CSGOperator::Difference => self.a.SDF(&from).max(-self.b.SDF(&from)),
    		CSGOperator::UnionSmooth(k) => min_smooth(self.a.SDF(&from), self.b.SDF(&from), k),
			CSGOperator::IntersectSmooth(k) => max_smooth(self.a.SDF(&from), self.b.SDF(&from), k),
			CSGOperator::DifferenceSmooth(k) => difference_smooth(self.a.SDF(&from), self.b.SDF(&from), k),
    		_ => 0.0
		}
	}
}
