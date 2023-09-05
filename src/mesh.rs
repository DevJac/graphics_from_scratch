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
        let mut rng = rand::thread_rng();
        let cr = rng.gen_range(0..255);
        let cg = rng.gen_range(0..255);
        let cb = rng.gen_range(0..255);
        Self {
            a,
            b,
            c,
            color: Color::RGB(cr, cg, cb),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub faces: Vec<Face>,
    pub rotation: Vec3,
}
