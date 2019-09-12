use super::math::*;
use super::Vec3;
use super::EPSILON;

pub trait DistanceField{
	fn sdf(&self, from: &Vec3) -> f64;
	fn sdf_normal(&self, p: Vec3) -> Vec3 {
		Vec3::new(
			self.sdf(&Vec3::new(p.x + EPSILON, p.y, p.z)) - self.sdf(&Vec3::new(p.x - EPSILON, p.y, p.z)),
			self.sdf(&Vec3::new(p.x, p.y + EPSILON, p.z)) - self.sdf(&Vec3::new(p.x, p.y - EPSILON, p.z)),
			self.sdf(&Vec3::new(p.x, p.y, p.z + EPSILON)) - self.sdf(&Vec3::new(p.x, p.y, p.z - EPSILON)),
		).normalize()
	}
}

pub struct Sphere {
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
	fn sdf(&self, from: &Vec3) -> f64 {
		(from - self.center).magnitude() - self.radius
	}
}

pub struct Cube {
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
	fn sdf(&self, from: &Vec3) -> f64 {
		let diff = (from - self.center).abs() - self.size;
		diff.max(0.0).magnitude()
		// + diff.x.max(diff.y.max(diff.z)).min(0.0)
	}
}

pub struct Plane {
	center: Vec3,
	normal: Vec3
}

impl Plane {
	pub fn new(center: Vec3, normal: Vec3) -> Self {
		Self {
			center: center,
			normal: normal
		}
	}
}

impl DistanceField for Plane {
	fn sdf(&self, from: &Vec3) -> f64 {
		(from - self.center).dot(&self.normal)
	}
}

pub enum CSGOperator {
	Union,
	Intersect,
	Difference,
	UnionSmooth(f64),
	IntersectSmooth(f64),
	DifferenceSmooth(f64)
}

pub struct CSG {
	a: Box<dyn DistanceField + Sync>,
	b: Box<dyn DistanceField + Sync>,
	op: CSGOperator
}

impl CSG {
	pub fn new(op: CSGOperator, a: Box<dyn DistanceField + Sync>, b: Box<dyn DistanceField + Sync>) -> Self {
		Self {
			a: a,
			b: b,
			op: op
		}
	}
}

impl DistanceField for CSG {
	fn sdf(&self, from: &Vec3) -> f64 {
		match self.op {
			CSGOperator::Union => self.a.sdf(&from).min(self.b.sdf(&from)),
			CSGOperator::Intersect => self.a.sdf(&from).max(self.b.sdf(&from)),
			CSGOperator::Difference => self.a.sdf(&from).max(-self.b.sdf(&from)),
			CSGOperator::UnionSmooth(k) => min_smooth(self.a.sdf(&from), self.b.sdf(&from), k),
			CSGOperator::IntersectSmooth(k) => max_smooth(self.a.sdf(&from), self.b.sdf(&from), k),
			CSGOperator::DifferenceSmooth(k) => difference_smooth(self.a.sdf(&from), self.b.sdf(&from), k),
			_ => 0.0
		}
	}
}

pub struct DomainRepetition {
	a: Box<dyn DistanceField + Sync>,
	offset: Vec3
}

impl DomainRepetition {
	pub fn new(a: Box<dyn DistanceField + Sync>, offset: Vec3) -> Self {
		Self {
			a: a,
			offset: offset
		}
	}
}

impl DistanceField for DomainRepetition {
	fn sdf(&self, from: &Vec3) -> f64 {
		let trans = Vec3::new(
    		from.x % self.offset.x,
    		from.y % self.offset.y,
    		from.z % self.offset.z)
    		- 0.5 * self.offset;
    	self.a.sdf(&trans)
	}
}
