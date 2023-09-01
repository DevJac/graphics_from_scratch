use std::ops::{Add, Div, Mul, Sub};

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
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let x = self.x + other.x;
        let y = self.y + other.y;
        Self { x, y }
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

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        Self { x, y, z }
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
    let a = Vec2::new(1.0, 2.0);
    let b = Vec2::new(3.0, 4.0);
    let c = Vec3::new(1.0, 2.0, 3.0);
    let d = Vec3::new(4.0, 5.0, 6.0);
    let e = Vec3::new(0.0, 0.0, 0.0);

    assert!(a + b == Vec2::new(4.0, 6.0));
    assert!(a - b == Vec2::new(-2.0, -2.0));
    assert!(c + d == Vec3::new(5.0, 7.0, 9.0));
    assert!(c - d == Vec3::new(-3.0, -3.0, -3.0));
    assert!(d - e == d);
}
