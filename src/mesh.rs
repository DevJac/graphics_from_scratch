use crate::vec::{Vec2, Vec3};
use sdl2::pixels::Color;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Face {
    pub a: usize,
    pub b: usize,
    pub c: usize,
    pub a_uv: usize,
    pub b_uv: usize,
    pub c_uv: usize,
    pub color: Color,
}

impl Face {
    pub fn new(a: usize, b: usize, c: usize, a_uv: usize, b_uv: usize, c_uv: usize) -> Self {
        Self {
            a,
            b,
            c,
            a_uv,
            b_uv,
            c_uv,
            color: Color::RGB(200, 200, 200),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub uvs: Vec<Vec2>,
    pub faces: Vec<Face>,
    pub rotation: Vec3,
}

impl Mesh {
    pub fn load_mesh(obj_file_path: &str) -> Self {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let mut vertices: Vec<Vec3> = Vec::new();
        let mut uvs: Vec<Vec2> = Vec::new();
        let mut faces: Vec<Face> = Vec::new();

        let file = File::open(obj_file_path).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
            let words: Vec<&str> = line.split_whitespace().collect();
            if words.len() == 3 {
                if words[0] == "vt" {
                    uvs.push(Vec2::new(
                        words[1].parse().unwrap(),
                        words[2].parse().unwrap(),
                    ));
                }
            } else if words.len() == 4 {
                if words[0] == "v" {
                    vertices.push(Vec3::new(
                        words[1].parse().unwrap(),
                        words[2].parse().unwrap(),
                        words[3].parse().unwrap(),
                    ));
                }
                if words[0] == "f" {
                    let mut a_line = words[1].split('/');
                    let a: usize = a_line.next().unwrap().parse().unwrap();
                    let a_uv: usize = a_line.next().unwrap().parse().unwrap();
                    let mut b_line = words[2].split('/');
                    let b: usize = b_line.next().unwrap().parse().unwrap();
                    let b_uv: usize = b_line.next().unwrap().parse().unwrap();
                    let mut c_line = words[3].split('/');
                    let c: usize = c_line.next().unwrap().parse().unwrap();
                    let c_uv: usize = c_line.next().unwrap().parse().unwrap();
                    // The obj file is 1-indexed. Rust is 0-indexed. Subtract 1 to adjust.
                    faces.push(Face::new(a - 1, b - 1, c - 1, a_uv - 1, b_uv - 1, c_uv - 1));
                }
            }
        }
        Mesh {
            vertices,
            uvs,
            faces,
            rotation: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}
