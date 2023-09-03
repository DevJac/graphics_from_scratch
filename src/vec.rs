use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn len(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let x = self.x + other.x;
        let y = self.y + other.y;
        Self { x, y }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let x = self.x - other.x;
        let y = self.y - other.y;
        Self { x, y }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        let x = self.x * other;
        let y = self.y * other;
        Self { x, y }
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        let x = self.x / other;
        let y = self.y / other;
        Self { x, y }
    }
}

impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, other: f32) {
        self.x /= other;
        self.y /= other;
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn len(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn cross(&self, other: Self) -> Self {
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;
        Self { x, y, z }
    }

    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let x = self.x + other.x;
        let y = self.y + other.y;
        let z = self.z + other.z;
        Self { x, y, z }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        Self { x, y, z }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        let x = self.x * other;
        let y = self.y * other;
        let z = self.z * other;
        Self { x, y, z }
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        let x = self.x / other;
        let y = self.y / other;
        let z = self.z / other;
        Self { x, y, z }
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

#[test]
fn test_vec_len() {
    let v2 = Vec2::new(3.0, 4.0);
    let v3_1 = Vec3::new(3.0, 4.0, 0.0);
    let v3_2 = Vec3::new(3.0, 0.0, 4.0);
    let v3_3 = Vec3::new(2.5, 6.4, 3.0);

    assert!(v2.len() == 5.0);
    assert!(v3_1.len() == 5.0);
    assert!(v3_2.len() == 5.0);
    assert!(v3_3.len() == 7.497333);
}

#[test]
fn test_vec_add_sub() {
    let mut a = Vec2::new(1.0, 2.0);
    let b = Vec2::new(3.0, 4.0);
    let mut c = Vec3::new(1.0, 2.0, 3.0);
    let d = Vec3::new(4.0, 5.0, 6.0);
    let e = Vec3::new(0.0, 0.0, 0.0);

    assert!(a + b == Vec2::new(4.0, 6.0));
    assert!(a - b == Vec2::new(-2.0, -2.0));
    assert!(c + d == Vec3::new(5.0, 7.0, 9.0));
    assert!(c - d == Vec3::new(-3.0, -3.0, -3.0));
    assert!(d - e == d);

    a += b;
    c += d;

    assert!(a == Vec2::new(4.0, 6.0));
    assert!(c == Vec3::new(5.0, 7.0, 9.0));

    a -= b;
    c -= d;

    assert!(a == Vec2::new(1.0, 2.0));
    assert!(c == Vec3::new(1.0, 2.0, 3.0));
}

#[test]
fn test_vec_mul_div() {
    let mut a = Vec2::new(1.0, 2.0);
    let mut b = Vec3::new(1.0, 2.0, 3.0);
    let mut c = Vec3::new(0.0, 0.0, 0.0);

    assert!(a * 1.5 == Vec2::new(1.5, 3.0));
    assert!(b * 1.5 == Vec3::new(1.5, 3.0, 4.5));
    assert!(c * 1.5 == Vec3::new(0.0, 0.0, 0.0));
    assert!((a * 1.5) / 1.5 == Vec2::new(1.0, 2.0));
    assert!((b * 1.5) / 1.5 == Vec3::new(1.0, 2.0, 3.0));
    assert!((c * 1.5) / 1.5 == Vec3::new(0.0, 0.0, 0.0));

    a *= 2.0;
    b *= 2.0;
    c *= 2.0;

    assert!(a == Vec2::new(2.0, 4.0));
    assert!(b == Vec3::new(2.0, 4.0, 6.0));
    assert!(c == Vec3::new(0.0, 0.0, 0.0));

    a /= 2.0;
    b /= 2.0;
    c /= 2.0;

    assert!(a == Vec2::new(1.0, 2.0));
    assert!(b == Vec3::new(1.0, 2.0, 3.0));
    assert!(c == Vec3::new(0.0, 0.0, 0.0));
}

#[test]
fn test_vec3_cross() {
    let a = Vec3::new(1.0, 0.0, 0.0);
    let b = Vec3::new(0.0, 1.0, 0.0);

    assert!(a.cross(b) == Vec3::new(0.0, 0.0, 1.0));

    let a = Vec3::new(1.5, 2.5, -1.0);
    let b = Vec3::new(2.0, -3.0, 4.0);

    assert!(a.cross(b) == Vec3::new(7.0, -8.0, -9.5));

    let a = Vec3::new(1.0, 0.0, 0.0);
    let b = Vec3::new(2.0, 2.0, 0.0);

    assert!(a.cross(b) == Vec3::new(0.0, 0.0, 2.0));
    assert!(b.cross(a) == Vec3::new(0.0, 0.0, -2.0));
}

#[test]
fn test_vec_dot() {
    assert!(Vec2::new(1.0, 0.0).dot(Vec2::new(0.0, 1.0)) == 0.0);
    assert!(Vec2::new(1.0, 1.0).dot(Vec2::new(0.0, 1.0)) == 1.0);
    assert!(Vec2::new(2.0, 1.0).dot(Vec2::new(0.0, 1.0)) == 1.0);

    assert!(Vec3::new(1.0, 0.0, 0.0).dot(Vec3::new(0.0, 1.0, 0.0)) == 0.0);
    assert!(Vec3::new(1.0, 1.0, 1.0).dot(Vec3::new(0.0, 1.0, 1.0)) == 2.0);
    assert!(Vec3::new(0.0, 0.0, 4.0).dot(Vec3::new(0.0, 0.0, 5.0)) == 20.0);
}
