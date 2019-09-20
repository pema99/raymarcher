use std::ops;
use super::Vec3;

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize)]
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

    //This spaghetti is rewritten from monogame source, 
    //I cba. to learn efficient matrix inversion 
    pub fn invert(&self) -> Self {
        let mut result = Mat4::new([0.0; 16]);

        let num1 = self.get_elem(0, 0);
	    let num2 = self.get_elem(1, 0);
	    let num3 = self.get_elem(2, 0);
	    let num4 = self.get_elem(3, 0);
	    let num5 = self.get_elem(0, 1);
	    let num6 = self.get_elem(1, 1);
	    let num7 = self.get_elem(2, 1);
	    let num8 = self.get_elem(3, 1);
	    let num9 = self.get_elem(0, 2);
	    let num10 = self.get_elem(1, 2);
	    let num11 = self.get_elem(2, 2);
	    let num12 = self.get_elem(3, 2);
	    let num13 = self.get_elem(0, 3);
	    let num14 = self.get_elem(1, 3);
	    let num15 = self.get_elem(2, 3);
	    let num16 = self.get_elem(3, 3);
	    let num17 = num11 * num16 - num12 * num15;
	    let num18 = num10 * num16 - num12 * num14;
	    let num19 = num10 * num15 - num11 * num14;
	    let num20 = num9 * num16 - num12 * num13;
	    let num21 = num9 * num15 - num11 * num13;
	    let num22 = num9 * num14 - num10 * num13;
	    let num23 = num6 *  num17 - num7 * num18 + num8 * num19;
	    let num24 = -(num5 * num17 - num7 * num20 + num8 * num21);
	    let num25 = num5 * num18 - num6 * num20 + num8 * num22;
	    let num26 = -(num5 * num19 - num6 * num21 + num7 * num22);
	    let num27 = 1.0 / ( num1 * num23 + num2 * num24 + num3 * num25 + num4 * num26);
    
	    result.set_elem(0, 0, num23 * num27);
	    result.set_elem(0, 1, num24 * num27);
	    result.set_elem(0, 2, num25 * num27);
	    result.set_elem(0, 3, num26 * num27);
	    result.set_elem(1, 0, -(num2 * num17 - num3 * num18 + num4 * num19) * num27);
	    result.set_elem(1, 1, (num1 * num17 - num3 * num20 + num4 * num21) * num27);
	    result.set_elem(1, 2, -(num1 * num18 - num2 * num20 + num4 * num22) * num27);
	    result.set_elem(1, 3, (num1 * num19 - num2 * num21 + num3 * num22) * num27);
	    let num28 = num7 * num16 - num8 * num15;
	    let num29 = num6 * num16 - num8 * num14;
	    let num30 = num6 * num15 - num7 * num14;
	    let num31 = num5 * num16 - num8 * num13;
	    let num32 = num5 * num15 - num7 * num13;
	    let num33 = num5 * num14 - num6 * num13;
	    result.set_elem(2, 0, (num2 * num28 - num3 * num29 + num4 * num30) * num27);
	    result.set_elem(2, 1, -(num1 * num28 - num3 * num31 + num4 * num32) * num27);
	    result.set_elem(2, 2, (num1 * num29 - num2 * num31 + num4 * num33) * num27);
	    result.set_elem(2, 3, -(num1 * num30 - num2 * num32 + num3 * num33) * num27);
	    let num34 = num7 * num12 - num8 * num11;
	    let num35 = num6 * num12 - num8 * num10;
	    let num36 = num6 * num11 - num7 * num10;
	    let num37 = num5 * num12 - num8 * num9;
	    let num38 = num5 * num11 - num7 * num9;
	    let num39 = num5 * num10 - num6 * num9;
	    result.set_elem(3, 0, -(num2 * num34 - num3 * num35 + num4 * num36) * num27);
	    result.set_elem(3, 1, (num1 * num34 - num3 * num37 + num4 * num38) * num27);
	    result.set_elem(3, 2, -(num1 * num35 - num2 * num37 + num4 * num39) * num27);
	    result.set_elem(3, 3, (num1 * num36 - num2 * num38 + num3 * num39) * num27);

        result
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


