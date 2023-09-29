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

    pub fn cross_z(&self, other: Self) -> f32 {
        self.x * other.y - self.y * other.x
    }

    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn unit_norm(&self) -> Self {
        let len = self.len();
        Self {
            x: self.x / len,
            y: self.y / len,
        }
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

    pub fn unit_norm(&self) -> Self {
        let len = self.len();
        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    pub fn to_vec4(&self) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, 1.0)
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn len(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn dot(&self, other: Self) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z) + (self.w * other.w)
    }

    pub fn unit_norm(&self) -> Self {
        let len = self.len();
        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
            w: self.w / len,
        }
    }

    pub fn to_vec2(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }

    pub fn to_vec3_w(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.w)
    }
}

impl Add for Vec4 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let x = self.x + other.x;
        let y = self.y + other.y;
        let z = self.z + other.z;
        let w = self.w + other.w;
        Self { x, y, z, w }
    }
}

impl AddAssign for Vec4 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self.w += other.w;
    }
}

impl Sub for Vec4 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        let w = self.w - other.w;
        Self { x, y, z, w }
    }
}

impl SubAssign for Vec4 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self.w -= other.w;
    }
}

impl Mul<f32> for Vec4 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        let x = self.x * other;
        let y = self.y * other;
        let z = self.z * other;
        let w = self.w * other;
        Self { x, y, z, w }
    }
}

impl MulAssign<f32> for Vec4 {
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
        self.w *= other;
    }
}

impl Div<f32> for Vec4 {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        let x = self.x / other;
        let y = self.y / other;
        let z = self.z / other;
        let w = self.w / other;
        Self { x, y, z, w }
    }
}

impl DivAssign<f32> for Vec4 {
    fn div_assign(&mut self, other: f32) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
        self.w /= other;
    }
}

#[test]
fn test_vec_len() {
    let v2 = Vec2::new(3.0, 4.0);
    let v3_1 = Vec3::new(3.0, 4.0, 0.0);
    let v3_2 = Vec3::new(3.0, 0.0, 4.0);
    let v3_3 = Vec3::new(2.5, 6.4, 3.0);

    assert_eq!(v2.len(), 5.0);
    assert_eq!(v3_1.len(), 5.0);
    assert_eq!(v3_2.len(), 5.0);
    assert_eq!(v3_3.len(), 7.497333);

    assert_eq!(Vec4::new(3.0, 4.0, 0.0, 0.0).len(), 5.0);
    assert_eq!(Vec4::new(0.0, 0.0, 4.0, 3.0).len(), 5.0);
}

#[test]
fn test_vec_add_sub() {
    let mut a = Vec2::new(1.0, 2.0);
    let b = Vec2::new(3.0, 4.0);
    let mut c = Vec3::new(1.0, 2.0, 3.0);
    let d = Vec3::new(4.0, 5.0, 6.0);
    let e = Vec3::new(0.0, 0.0, 0.0);

    assert_eq!(a + b, Vec2::new(4.0, 6.0));
    assert_eq!(a - b, Vec2::new(-2.0, -2.0));
    assert_eq!(c + d, Vec3::new(5.0, 7.0, 9.0));
    assert_eq!(c - d, Vec3::new(-3.0, -3.0, -3.0));
    assert_eq!(d - e, d);

    a += b;
    c += d;

    assert_eq!(a, Vec2::new(4.0, 6.0));
    assert_eq!(c, Vec3::new(5.0, 7.0, 9.0));

    a -= b;
    c -= d;

    assert_eq!(a, Vec2::new(1.0, 2.0));
    assert_eq!(c, Vec3::new(1.0, 2.0, 3.0));

    let mut f = Vec4::new(1.0, 2.0, 3.0, 1.0);
    let g = Vec4::new(4.0, 5.0, 6.0, 2.0);
    let h = Vec4::new(0.0, 0.0, 0.0, 0.0);

    assert_eq!(f + g, Vec4::new(5.0, 7.0, 9.0, 3.0));
    assert_eq!(f - g, Vec4::new(-3.0, -3.0, -3.0, -1.0));
    assert_eq!(g - h, g);

    f += g;

    assert_eq!(f, Vec4::new(5.0, 7.0, 9.0, 3.0));

    f -= g;

    assert_eq!(f, Vec4::new(1.0, 2.0, 3.0, 1.0));
}

#[test]
fn test_vec_mul_div() {
    let mut a = Vec2::new(1.0, 2.0);
    let mut b = Vec3::new(1.0, 2.0, 3.0);
    let mut c = Vec3::new(0.0, 0.0, 0.0);

    assert_eq!(a * 1.5, Vec2::new(1.5, 3.0));
    assert_eq!(b * 1.5, Vec3::new(1.5, 3.0, 4.5));
    assert_eq!(c * 1.5, Vec3::new(0.0, 0.0, 0.0));
    assert_eq!((a * 1.5) / 1.5, Vec2::new(1.0, 2.0));
    assert_eq!((b * 1.5) / 1.5, Vec3::new(1.0, 2.0, 3.0));
    assert_eq!((c * 1.5) / 1.5, Vec3::new(0.0, 0.0, 0.0));

    a *= 2.0;
    b *= 2.0;
    c *= 2.0;

    assert_eq!(a, Vec2::new(2.0, 4.0));
    assert_eq!(b, Vec3::new(2.0, 4.0, 6.0));
    assert_eq!(c, Vec3::new(0.0, 0.0, 0.0));

    a /= 2.0;
    b /= 2.0;
    c /= 2.0;

    assert_eq!(a, Vec2::new(1.0, 2.0));
    assert_eq!(b, Vec3::new(1.0, 2.0, 3.0));
    assert_eq!(c, Vec3::new(0.0, 0.0, 0.0));

    let mut d = Vec4::new(1.0, 2.0, 3.0, 4.0);
    let mut e = Vec4::new(0.0, 0.0, 0.0, 0.0);

    assert_eq!(d * 1.5, Vec4::new(1.5, 3.0, 4.5, 6.0));
    assert_eq!(e * 1.5, Vec4::new(0.0, 0.0, 0.0, 0.0));
    assert_eq!((d * 1.5) / 1.5, Vec4::new(1.0, 2.0, 3.0, 4.0));
    assert_eq!((e * 1.5) / 1.5, Vec4::new(0.0, 0.0, 0.0, 0.0));

    d *= 2.0;
    e *= 2.0;

    assert_eq!(d, Vec4::new(2.0, 4.0, 6.0, 8.0));
    assert_eq!(e, Vec4::new(0.0, 0.0, 0.0, 0.0));

    d /= 2.0;
    e /= 2.0;

    assert_eq!(d, Vec4::new(1.0, 2.0, 3.0, 4.0));
    assert_eq!(e, Vec4::new(0.0, 0.0, 0.0, 0.0));
}

#[test]
fn test_vec2_cross_z() {
    let a = Vec2::new(1.0, 0.0);
    let b = Vec2::new(0.0, 1.0);

    assert_eq!(a.cross_z(b), 1.0);
    assert_eq!(b.cross_z(a), -1.0);

    let a = Vec2::new(1.0, 0.0);
    let b = Vec2::new(2.0, 2.0);

    assert_eq!(a.cross_z(b), 2.0);
    assert_eq!(b.cross_z(a), -2.0);
}

#[test]
fn test_vec3_cross() {
    let a = Vec3::new(1.0, 0.0, 0.0);
    let b = Vec3::new(0.0, 1.0, 0.0);

    assert_eq!(a.cross(b), Vec3::new(0.0, 0.0, 1.0));

    let a = Vec3::new(1.5, 2.5, -1.0);
    let b = Vec3::new(2.0, -3.0, 4.0);

    assert_eq!(a.cross(b), Vec3::new(7.0, -8.0, -9.5));

    let a = Vec3::new(1.0, 0.0, 0.0);
    let b = Vec3::new(2.0, 2.0, 0.0);

    assert_eq!(a.cross(b), Vec3::new(0.0, 0.0, 2.0));
    assert_eq!(b.cross(a), Vec3::new(0.0, 0.0, -2.0));
}

#[test]
fn test_vec_dot() {
    assert_eq!(Vec2::new(1.0, 0.0).dot(Vec2::new(0.0, 1.0)), 0.0);
    assert_eq!(Vec2::new(1.0, 1.0).dot(Vec2::new(0.0, 1.0)), 1.0);
    assert_eq!(Vec2::new(2.0, 1.0).dot(Vec2::new(0.0, 1.0)), 1.0);

    assert_eq!(Vec3::new(1.0, 0.0, 0.0).dot(Vec3::new(0.0, 1.0, 0.0)), 0.0);
    assert_eq!(Vec3::new(1.0, 1.0, 1.0).dot(Vec3::new(0.0, 1.0, 1.0)), 2.0);
    assert_eq!(Vec3::new(0.0, 0.0, 4.0).dot(Vec3::new(0.0, 0.0, 5.0)), 20.0);

    assert_eq!(
        Vec4::new(1.0, 0.0, 0.0, 1.0).dot(Vec4::new(0.0, 1.0, 0.0, 0.0)),
        0.0
    );
    assert_eq!(
        Vec4::new(1.0, 1.0, 1.0, 1.0).dot(Vec4::new(0.0, 1.0, 1.0, 1.0)),
        3.0
    );
    assert_eq!(
        Vec4::new(0.0, 0.0, 4.0, 1.0).dot(Vec4::new(0.0, 0.0, 5.0, 2.0)),
        22.0
    );
}

#[test]
fn test_vec_unit_norm() {
    assert_eq!(Vec2::new(1.0, 0.0).unit_norm(), Vec2::new(1.0, 0.0));
    assert_eq!(Vec2::new(2.0, 0.0).unit_norm(), Vec2::new(1.0, 0.0));
    assert_eq!(
        Vec2::new(2.0, 2.0).unit_norm(),
        Vec2::new(0.70710677, 0.70710677)
    );

    assert_eq!(
        Vec3::new(1.0, 0.0, 0.0).unit_norm(),
        Vec3::new(1.0, 0.0, 0.0)
    );
    assert_eq!(
        Vec3::new(2.0, 0.0, 0.0).unit_norm(),
        Vec3::new(1.0, 0.0, 0.0)
    );
    assert_eq!(
        Vec3::new(2.0, 2.0, 0.0).unit_norm(),
        Vec3::new(0.70710677, 0.70710677, 0.0)
    );
    assert_eq!(
        Vec3::new(0.0, 0.0, 1.0).unit_norm(),
        Vec3::new(0.0, 0.0, 1.0)
    );
    assert_eq!(
        Vec3::new(2.0, 2.0, 1.0).unit_norm(),
        Vec3::new(0.66666666, 0.66666666, 0.33333333)
    );

    assert_eq!(
        Vec4::new(0.0, 0.0, 0.0, 1.0).unit_norm(),
        Vec4::new(0.0, 0.0, 0.0, 1.0)
    );
    assert_eq!(
        Vec4::new(0.0, 0.0, 0.0, 2.0).unit_norm(),
        Vec4::new(0.0, 0.0, 0.0, 1.0)
    );
    assert_eq!(
        Vec4::new(0.0, 0.0, 2.0, 2.0).unit_norm(),
        Vec4::new(0.0, 0.0, 0.70710677, 0.70710677)
    );
    assert_eq!(
        Vec4::new(0.0, 2.0, 2.0, 1.0).unit_norm(),
        Vec4::new(0.0, 0.66666666, 0.66666666, 0.33333333)
    );
}
