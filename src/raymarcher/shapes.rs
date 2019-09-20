use super::math::*;
use super::Vec3;
use super::Mat4;
const EPSILON: f64 = 0.00001;

#[derive(Serialize, Deserialize)]
pub enum SceneObject {
    Sphere { radius: f64 },
    Cube { size: Vec3 },
    Plane { normal: Vec3 },
    CSG { a: Box<SceneObject>, b: Box<SceneObject>, op: CSGOperator },
    Transform { a: Box<SceneObject>, transform: Mat4 },
    Scale { a: Box<SceneObject>, factor: f64 },
    Repeat { a: Box<SceneObject>, period: Vec3 }
}

pub trait DistanceField {
    fn sdf(&self, from: &Vec3) -> f64;
    fn sdf_normal(&self, p: Vec3) -> Vec3 {
		Vec3::new(
			self.sdf(&Vec3::new(p.x + EPSILON, p.y, p.z)) - self.sdf(&Vec3::new(p.x - EPSILON, p.y, p.z)),
			self.sdf(&Vec3::new(p.x, p.y + EPSILON, p.z)) - self.sdf(&Vec3::new(p.x, p.y - EPSILON, p.z)),
			self.sdf(&Vec3::new(p.x, p.y, p.z + EPSILON)) - self.sdf(&Vec3::new(p.x, p.y, p.z - EPSILON)),
		).normalize()
	}
}

impl DistanceField for SceneObject {
    fn sdf(&self, from: &Vec3) -> f64 {
        match self {
            Self::Sphere { radius } => sphere_sdf(from, *radius),
            Self::Cube { size } => cube_sdf(from, size),
            Self::Plane { normal } => plane_sdf(from, normal),
            Self::CSG { a, b, op } => csg_sdf(from, a, b, op),
            Self::Transform { a, transform } => transform_sdf(from, a, transform),
            Self::Scale { a, factor } => scale_sdf(from, a, *factor),
            Self::Repeat { a, period } => repeat_sdf(from, a, period)
        }
    }
}

fn sphere_sdf(from: &Vec3, radius: f64) -> f64 {
    from.magnitude() - radius
}

fn cube_sdf(from: &Vec3, size: &Vec3) -> f64 {
    let diff = from.abs() - size;
    diff.max(0.0).magnitude()
    // + diff.x.max(diff.y.max(diff.z)).min(0.0)
}

fn plane_sdf(from: &Vec3, normal: &Vec3) -> f64 {
    from.dot(normal)
}

#[derive(Serialize, Deserialize)]
pub enum CSGOperator {
	Union,
	Intersect,
	Difference,
	UnionSmooth(f64),
	IntersectSmooth(f64),
	DifferenceSmooth(f64)
}

fn csg_sdf(from: &Vec3, a: &SceneObject, b: &SceneObject, op: &CSGOperator) -> f64 {
    match op {
        CSGOperator::Union => a.sdf(&from).min(b.sdf(&from)),
        CSGOperator::Intersect => a.sdf(&from).max(b.sdf(&from)),
        CSGOperator::Difference => a.sdf(&from).max(-b.sdf(&from)),
        CSGOperator::UnionSmooth(k) => min_smooth(a.sdf(&from), b.sdf(&from), *k),
        CSGOperator::IntersectSmooth(k) => max_smooth(a.sdf(&from), b.sdf(&from), *k),
        CSGOperator::DifferenceSmooth(k) => difference_smooth(a.sdf(&from), b.sdf(&from), *k),
        _ => 0.0
    }
}

fn transform_sdf(from: &Vec3, a: &SceneObject, transform: &Mat4) -> f64 {
    a.sdf(&(transform * from))
}

fn scale_sdf(from: &Vec3, a: &SceneObject, factor: f64) -> f64 {
    a.sdf(&(from / factor)) * factor
}

fn repeat_sdf(from: &Vec3, a: &SceneObject, period: &Vec3) -> f64 {
    let trans = Vec3::new(
        from.x.abs() % period.x,
        from.y.abs() % period.y,
        from.z.abs() % period.z)
        - 0.5 * period;
    a.sdf(&trans)
}
