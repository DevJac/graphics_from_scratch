use crate::vec::Vec3;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Face {
    pub a: usize,
    pub b: usize,
    pub c: usize,
}

impl Face {
    pub const fn new(a: usize, b: usize, c: usize) -> Self {
        Self { a, b, c }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub faces: Vec<Face>,
    pub rotation: Vec3,
}
