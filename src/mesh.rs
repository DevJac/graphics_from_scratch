use crate::vec::Vec3;
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

impl Mesh {
    pub fn load_mesh(obj_file_path: &str) -> Self {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let mut vertices: Vec<Vec3> = Vec::new();
        let mut faces: Vec<Face> = Vec::new();

        let file = File::open(obj_file_path).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
            let words: Vec<&str> = line.split_whitespace().collect();
            if words.len() == 4 {
                if words[0] == "v" {
                    vertices.push(Vec3::new(
                        words[1].parse().unwrap(),
                        words[2].parse().unwrap(),
                        words[3].parse().unwrap(),
                    ));
                }
                if words[0] == "f" {
                    let a: usize = words[1].split('/').next().unwrap().parse().unwrap();
                    let b: usize = words[2].split('/').next().unwrap().parse().unwrap();
                    let c: usize = words[3].split('/').next().unwrap().parse().unwrap();
                    faces.push(Face::new(a - 1, b - 1, c - 1));
                }
            }
        }
        Mesh {
            vertices,
            faces,
            rotation: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}
