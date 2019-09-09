use std::ops;
use super::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Mat4 {
	data: [f64; 16]
}

impl Mat4 {
	//Creation functions
	pub fn new(data: [f64; 16]) -> Self {
		Self {
			data: data
		}
	}
	
	pub fn empty() -> Self {
		Self {
			data: [0.0; 16]
		}
	}

	pub fn identity() -> Self {
		Self {
			data: [1.0, 0.0, 0.0, 0.0,
				   0.0, 1.0, 0.0, 0.0,
				   0.0, 0.0, 1.0, 0.0,
				   0.0, 0.0, 0.0, 1.0]
		}
	}

	pub fn translate(v: &Vec3) -> Self {
		Self {
			data: [1.0, 0.0, 0.0, 0.0,
				   0.0, 1.0, 0.0, 0.0,
				   0.0, 0.0, 1.0, 0.0,
				   v.x, v.y, v.z, 1.0]
		}
	}

	pub fn rotate(v: &Vec3) -> Self {
		Self::rotate_x(v.x) * 
		Self::rotate_y(v.y) * 
		Self::rotate_z(v.z)
	}

	pub fn rotate_x(v: f64) -> Self {
		Self {
			data: [1.0,      0.0,      0.0,      0.0,
				   0.0,      v.cos(),  -v.sin(), 0.0,
				   0.0,      v.sin(),  v.cos(),  0.0,
				   0.0,      0.0,      0.0,      1.0]
		}
	}

	pub fn rotate_y(v: f64) -> Self {
		Self {
			data: [v.cos(),  0.0,      v.sin(),  0.0,
				   0.0,      1.0,      0.0,      0.0,
				   -v.sin(), 0.0,      v.cos(),  0.0,
				   0.0,      0.0,      0.0,      1.0]
		}
	}

	pub fn rotate_z(v: f64) -> Self {
		Self {
			data: [v.cos(),  -v.sin(), 0.0,      0.0,
				   v.sin(),  v.cos(),  0.0,      0.0,
				   0.0,      0.0,      1.0,      0.0,
				   0.0,      0.0,      0.0,      1.0]
		}
	}

	pub fn scale(v: f64) -> Self {
		Self {
			data: [v,   0.0, 0.0, 0.0,
				   0.0, v,   0.0, 0.0,
				   0.0, 0.0, v,   0.0,
				   0.0, 0.0, 0.0, 1.0]
		}
	}

	//Indexing
	pub fn get_elem(&self, x: usize, y: usize) -> f64 {
		self.data[y * 4 + x]
	}

	pub fn set_elem(&mut self, x: usize, y: usize, val: f64) {
		self.data[y * 4 + x] = val;
	}

	pub fn get_row(&self, n: usize) -> [f64; 4] {
		let mut res = [0.0; 4];
		for (i, e) in self.data.iter().skip(n*4).take(4).enumerate() {
			res[i] = *e;
		}
		res
	}

	pub fn get_col(&self, n: usize) -> [f64; 4] {
		let mut res = [0.0; 4];
		for (i, e) in self.data.iter().skip(n).step_by(4).take(4).enumerate() {
			res[i] = *e;
		}
		res
	}
}

//Helpers
fn dot(a: &[f64], b: &[f64]) -> f64 {
	a.iter().zip(b.iter()).map(|(a, b)| {
		a * b
	}).sum()
}

fn dot_at(a: &Mat4, b: &Mat4, row: usize, col: usize) -> f64 {
	dot(&a.get_row(row), &b.get_col(col))
}

//Operators
impl_op_ex_commutative!(* |a: &Mat4, b: &f64| -> Mat4 { 
	let mut res = Mat4::empty();
	for i in 0..16 {
		res.data[i] = a.data[i] * b;
	}
	res
});

impl_op_ex!(* |a: &Mat4, b: &Mat4| -> Mat4 { 
	let mut res = Mat4::empty();
	for x in 0..4 {
		for y in 0..4 {
			res.set_elem(x, y, dot_at(a, b, y, x));
		}
	}
	res
});

impl_op_ex!(* |a: &Mat4, b: &Vec3| -> Vec3 { 
	let mut res = [0.0; 4];
	for i in 0..4 {
		res[i] = dot(&a.get_col(i), &[b.x, b.y, b.z, 1.0]);
	}
	Vec3::new(res[0], res[1], res[2])
});


