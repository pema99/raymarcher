use super::math::*;
use super::Vec3;
use super::Mat4;
const EPSILON: f64 = 0.00001;

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
	radius: f64
}

impl Sphere {
	pub fn new(radius: f64) -> Box<Self> {
		Box::new(Self {
			radius: radius
		})
	}
}

impl DistanceField for Sphere {
	fn sdf(&self, from: &Vec3) -> f64 {
		from.magnitude() - self.radius
	}
}

pub struct Cube {
	size: Vec3
}

impl Cube {
	pub fn new(size: Vec3) -> Box<Self> {
		Box::new(Self {
			size: size
		})
	}
}

impl DistanceField for Cube {
	fn sdf(&self, from: &Vec3) -> f64 {
		let diff = from.abs() - self.size;
		diff.max(0.0).magnitude()
		// + diff.x.max(diff.y.max(diff.z)).min(0.0)
	}
}

pub struct Plane {
	normal: Vec3
}

impl Plane {
	pub fn new(normal: Vec3) -> Box<Self> {
		Box::new(Self {
			normal: normal
		})
	}
}

impl DistanceField for Plane {
	fn sdf(&self, from: &Vec3) -> f64 {
		from.dot(&self.normal)
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
	pub fn new(op: CSGOperator, a: Box<dyn DistanceField + Sync>, b: Box<dyn DistanceField + Sync>) -> Box<Self> {
		Box::new(Self {
			a: a,
			b: b,
			op: op
		})
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

pub struct Transform {
    a: Box<dyn DistanceField + Sync>,
	transform: Mat4
}

impl Transform {
	pub fn new(a: Box<dyn DistanceField + Sync>, transform: Mat4) -> Box<Self> {
		Box::new(Self {
			a: a,
			transform: transform.invert()
		})
	}
}

impl DistanceField for Transform {
	fn sdf(&self, from: &Vec3) -> f64 {
        self.a.sdf(&(self.transform * from))
	}
}

pub struct Scale {
    a: Box<dyn DistanceField + Sync>,
	factor: f64
}

impl Scale {
	pub fn new(a: Box<dyn DistanceField + Sync>, factor: f64) -> Box<Self> {
		Box::new(Self {
			a: a,
			factor: factor
		})
	}
}

impl DistanceField for Scale {
	fn sdf(&self, from: &Vec3) -> f64 {
        self.a.sdf(&(from / self.factor)) * self.factor
	}
}

pub struct DomainRepetition {
	a: Box<dyn DistanceField + Sync>,
	offset: Vec3
}

impl DomainRepetition {
	pub fn new(a: Box<dyn DistanceField + Sync>, offset: Vec3) -> Box<Self> {
		Box::new(Self {
			a: a,
			offset: offset
		})
	}
}

impl DistanceField for DomainRepetition {
	fn sdf(&self, from: &Vec3) -> f64 {
		let trans = Vec3::new(
    		from.x.abs() % self.offset.x,
    		from.y.abs() % self.offset.y,
    		from.z.abs() % self.offset.z)
    		- 0.5 * self.offset;
    	self.a.sdf(&trans)
	}
}
