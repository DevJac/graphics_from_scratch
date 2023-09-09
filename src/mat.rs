use crate::vec::Vec3;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Mat4 {
    data: [f32; 16],
}

impl Mat4 {
    #[rustfmt::skip]
    pub const fn new(
        f_00: f32, f_01: f32, f_02: f32, f_03: f32,
        f_10: f32, f_11: f32, f_12: f32, f_13: f32,
        f_20: f32, f_21: f32, f_22: f32, f_23: f32,
        f_30: f32, f_31: f32, f_32: f32, f_33: f32,
    ) -> Self {
        Self {
            #[rustfmt::skip]
            data: [
                f_00, f_10, f_20, f_30,
		f_01, f_11, f_21, f_31,
		f_02, f_12, f_22, f_32,
		f_03, f_13, f_23, f_33,
            ],
        }
    }

    #[rustfmt::skip]
    pub const fn zero() -> Self {
	Self::new(
	    0.0, 0.0, 0.0, 0.0,
	    0.0, 0.0, 0.0, 0.0,
	    0.0, 0.0, 0.0, 0.0,
	    0.0, 0.0, 0.0, 0.0,
	)
    }

    #[rustfmt::skip]
    pub const fn identity() -> Self {
	Self::new(
	    1.0, 0.0, 0.0, 0.0,
	    0.0, 1.0, 0.0, 0.0,
	    0.0, 0.0, 1.0, 0.0,
	    0.0, 0.0, 0.0, 1.0,
	)
    }

    pub fn get(&self, row: usize, column: usize) -> f32 {
        if row >= 4 || column >= 4 {
            panic!("Element {} {} is out of range for Mat4", row, column);
        }

        let i = (column * 4) + row;
        unsafe { *self.data.get_unchecked(i) }
    }

    pub fn set(&mut self, row: usize, column: usize, value: f32) {
        if row >= 4 || column >= 4 {
            panic!("Element {} {} is out of range for Mat4", row, column);
        }

        let i = (column * 4) + row;
        unsafe { *self.data.get_unchecked_mut(i) = value };
    }
}

impl Add for Mat4 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut data = [0.0; 16];

        for i in 0..16 {
            data[i] = self.data[i] + other.data[i];
        }

        Self { data }
    }
}

impl AddAssign for Mat4 {
    fn add_assign(&mut self, other: Self) {
        for i in 0..16 {
            self.data[i] += other.data[i];
        }
    }
}

impl Sub for Mat4 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut data = [0.0; 16];

        for i in 0..16 {
            data[i] = self.data[i] - other.data[i];
        }

        Self { data }
    }
}

impl SubAssign for Mat4 {
    fn sub_assign(&mut self, other: Self) {
        for i in 0..16 {
            self.data[i] -= other.data[i];
        }
    }
}

impl Mul for Mat4 {
    type Output = Self;

    #[rustfmt::skip]
    fn mul(self, other: Self) -> Self {
        let mut data = std::mem::MaybeUninit::<[f32; 16]>::uninit();
        let data_p = data.as_mut_ptr() as *mut f32;

        for column in 0..4 {
            for row in 0..4 {
                let i = (column * 4) + row;
                unsafe {
                    *data_p.add(i) =
                          self.get(row, 0) * other.get(0, column)
                        + self.get(row, 1) * other.get(1, column)
                        + self.get(row, 2) * other.get(2, column)
                        + self.get(row, 3) * other.get(3, column);
                }
            }
        }

        unsafe {
            Self {
                data: data.assume_init(),
            }
        }
    }
}

impl Mul<f32> for Mat4 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        let mut data = [0.0; 16];

        for i in 0..16 {
            data[i] = self.data[i] * other;
        }

        Self { data }
    }
}

impl MulAssign<f32> for Mat4 {
    fn mul_assign(&mut self, other: f32) {
        for i in 0..16 {
            self.data[i] *= other;
        }
    }
}

impl Mul<Vec3> for Mat4 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(
            (self.get(0, 0) * other.x) + (self.get(0, 1) * other.y) + (self.get(0, 2) * other.z),
            (self.get(1, 0) * other.x) + (self.get(1, 1) * other.y) + (self.get(1, 2) * other.z),
            (self.get(2, 0) * other.x) + (self.get(2, 1) * other.y) + (self.get(2, 2) * other.z),
        )
    }
}

#[test]
fn test_mat4_get_set() {
    let mut mat = Mat4::zero();
    mat.set(2, 3, 4.5);
    assert!(mat.get(2, 3) == 4.5);
    assert!(mat.get(2, 2) == 0.0);

    #[rustfmt::skip]
    let b = Mat4::new(
        2.0, 1.0, 2.0, 0.0,
	2.0, 1.0, 1.0, 1.0,
	1.0, 3.0, 2.0, 1.0,
	3.0, 1.0, 2.0, 3.0,
    );

    dbg!(b.get(2, 1));
    assert!(b.get(2, 1) == 3.0);
    assert!(b.get(0, 3) == 0.0);
}

#[test]
fn test_mat4_simple_ops_add_sub_f32_mul() {
    let a = Mat4::identity();

    assert!(a * 2.0 == a + a);

    #[rustfmt::skip]
    let mut b = Mat4::new(
        2.0, 1.0, 2.0, 0.0,
	2.0, 1.0, 1.0, 1.0,
	1.0, 3.0, 2.0, 1.0,
	3.0, 1.0, 2.0, 3.0,
    );

    assert!(b.get(2, 1) == 3.0);
    assert!(b.get(0, 3) == 0.0);
    assert!(b == b - b + b);
    assert!(Mat4::zero() == b - b);

    let mut c = b.clone();

    c -= b;

    assert!(c == Mat4::zero());

    let mut c = b.clone();

    c += b;
    b *= 2.0;

    assert!(c == b);
}

#[test]
fn test_mat4_mat4_mul() {
    #[rustfmt::skip]
    let a = Mat4::new(
	3.0, 1.0, 2.0, 2.0,
	3.0, 1.0, 2.0, 3.0,
	2.0, 0.0, 1.0, 3.0,
	1.0, 1.0, 0.0, 1.0,
    );
    #[rustfmt::skip]
    let b = Mat4::new(
	1.0, 2.0, 1.0, 0.0,
	0.0, 2.0, 2.0, 3.0,
	3.0, 3.0, 0.0, 1.0,
	0.0, 0.0, 2.0, 2.0,
    );
    #[rustfmt::skip]
    let c = Mat4::new(
	9.0, 14.0, 9.0, 9.0,
	9.0, 14.0, 11.0, 11.0,
	5.0, 7.0, 8.0, 7.0,
	1.0, 4.0, 5.0, 5.0,
    );

    assert!(a * b == c);
}

#[test]
fn test_mat4_vec3_mul() {
    #[rustfmt::skip]
    let a = Mat4::new(
	3.0, 1.0, 2.0, 2.0,
	3.0, 1.0, 2.0, 3.0,
	2.0, 0.0, 1.0, 3.0,
	1.0, 1.0, 0.0, 1.0,
    );
    let b = Vec3::new(1.0, 0.0, 3.0);

    assert!(a * b == Vec3::new(9.0, 9.0, 5.0));
}
