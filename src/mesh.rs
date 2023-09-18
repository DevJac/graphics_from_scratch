use crate::vec::Vec3;
use rand::Rng;
use sdl2::pixels::Color;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Face {
    pub a: usize,
    pub b: usize,
    pub c: usize,
    pub color: Color,
}

impl Face {
    pub fn new(a: usize, b: usize, c: usize) -> Self {
        Self {
            a,
            b,
            c,
            color: Color::RGB(200, 200, 200),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub faces: Vec<Face>,
    pub rotation: Vec3,
}
