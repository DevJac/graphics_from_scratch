pub mod mesh;
pub mod pixel_renderer;
pub mod vec;

use mesh::{Face, Mesh};
use pixel_renderer::PixelRenderer;
use rand::{seq::SliceRandom, Rng};
use sdl2::pixels::Color;
use vec::{Vec2, Vec3};

const CAMERA_DIST: f32 = 5.0;
const SCALE: f32 = 400.0;

fn rotate(x: f32, y: f32, angle_degrees: f32) -> (f32, f32) {
    let (sin_a, cos_a) = angle_degrees.to_radians().sin_cos();
    (x * cos_a - y * sin_a, x * sin_a + y * cos_a)
}

pub fn get_cube_mesh() -> Mesh {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let mut vertices: Vec<Vec3> = Vec::new();
    let mut faces: Vec<Face> = Vec::new();

    let file = File::open("./assets/cube.obj").unwrap();
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

fn project_point_to_screen_space(pixel_renderer: &mut PixelRenderer, p: Vec3) -> Vec2 {
    let half_width: f32 = pixel_renderer.width as f32 / 2.0;
    let half_height: f32 = pixel_renderer.height as f32 / 2.0;
    let centered_x = p.x / (p.z + CAMERA_DIST) * SCALE + half_width;
    let centered_y = p.y / (p.z + CAMERA_DIST) * SCALE + half_height;
    Vec2::new(centered_x, centered_y)
}

pub fn draw_mesh(pixel_renderer: &mut PixelRenderer, mesh: &mut Mesh) {
    let mut rng = rand::thread_rng();
    mesh.rotation.x = mesh.rotation.x * 0.99 + rng.gen_range(-0.03..0.03);
    mesh.rotation.y = mesh.rotation.y * 0.99 + rng.gen_range(-0.03..0.03);
    mesh.rotation.z = mesh.rotation.z * 0.99 + rng.gen_range(-0.03..0.03);

    for p in mesh.vertices.iter_mut() {
        let (rot_x, rot_y) = rotate(p.x, p.y, mesh.rotation.z);
        let (rot_x, rot_z) = rotate(rot_x, p.z, mesh.rotation.y);
        let (rot_y, rot_z) = rotate(rot_y, rot_z, mesh.rotation.x);
        p.x = rot_x;
        p.y = rot_y;
        p.z = rot_z;
    }

    pixel_renderer.clear_pixels(Color::RGB(0, 0, 0));

    mesh.faces.shuffle(&mut rng);

    for face in mesh.faces.iter() {
        let vert_a = mesh.vertices[face.a];
        let vert_b = mesh.vertices[face.b];
        let vert_c = mesh.vertices[face.c];

        let face_normal = (vert_b - vert_a).cross(vert_c - vert_a);
        let vec_to_camera = Vec3::new(0.0, 0.0, -CAMERA_DIST) - vert_a;
        let vec_to_camera_b = Vec3::new(0.0, 0.0, -CAMERA_DIST) - vert_b;
        let vec_to_camera_c = Vec3::new(0.0, 0.0, -CAMERA_DIST) - vert_c;
        debug_assert!(
            face_normal.dot(vec_to_camera).is_sign_positive()
                == face_normal.dot(vec_to_camera_b).is_sign_positive()
        );
        debug_assert!(
            face_normal.dot(vec_to_camera).is_sign_positive()
                == face_normal.dot(vec_to_camera_c).is_sign_positive()
        );
        if face_normal.dot(vec_to_camera) < 0.0 {
            continue;
        }

        let pa = project_point_to_screen_space(pixel_renderer, vert_a);
        let pb = project_point_to_screen_space(pixel_renderer, vert_b);
        let pc = project_point_to_screen_space(pixel_renderer, vert_c);

        draw_triangle(pixel_renderer, face.color, pa, pb, pc);
    }
}

// TODO: Use min_max. Is it faster?
fn min_max(a: f32, b: f32, c: f32) -> (i32, i32) {
    let min;
    let max;

    if a <= b && a <= c {
        min = a;
    } else if b <= c {
        min = b;
    } else {
        min = c;
    }

    if a >= b && a >= c {
        max = a;
    } else if b >= c {
        max = b;
    } else {
        max = c;
    }

    (min.round() as i32, max.round() as i32)
}

pub fn draw_triangle(pixel_renderer: &mut PixelRenderer, color: Color, a: Vec2, b: Vec2, c: Vec2) {
    let (x_min, x_max) = min_max(a.x, b.x, c.x);
    let (y_min, y_max) = min_max(a.y, b.y, c.y);

    let x_min = x_min.max(0);
    let y_min = y_min.max(0);

    let a1 = a - c;
    let b1 = b - a;
    let c1 = c - b;

    for x in x_min..=x_max {
        for y in y_min..=y_max {
            let p = Vec2::new(x as f32, y as f32);
            let in_a = (p - a).cross_z(a1) > 0.0;
            let in_b = (p - b).cross_z(b1) > 0.0;
            let in_c = (p - c).cross_z(c1) > 0.0;

            if in_a && in_b && in_c {
                pixel_renderer.set_pixel(x as u32, y as u32, color);
            }
        }
    }
}

pub fn draw_line(
    pixel_renderer: &mut PixelRenderer,
    color: Color,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
) {
    let mut x0: i32 = x0.round() as i32;
    let mut y0: i32 = y0.round() as i32;
    let x1: i32 = x1.round() as i32;
    let y1: i32 = y1.round() as i32;

    let delta_x = x1 - x0;
    let delta_x_abs = delta_x.abs();
    let delta_y = y1 - y0;
    let delta_y_abs = delta_y.abs();

    let mut accumulated_error = 0;

    if delta_x_abs > delta_y_abs {
        loop {
            pixel_renderer.set_pixel(x0 as u32, y0 as u32, color);
            if x0 == x1 {
                break;
            }
            if delta_x > 0 {
                x0 += 1;
            } else {
                x0 -= 1;
            }
            accumulated_error += delta_y_abs;
            if (accumulated_error + accumulated_error) > delta_x_abs {
                accumulated_error -= delta_x_abs;
                if delta_y > 0 {
                    y0 += 1;
                } else {
                    y0 -= 1;
                }
            }
        }
    } else {
        loop {
            pixel_renderer.set_pixel(x0 as u32, y0 as u32, color);
            if y0 == y1 {
                break;
            }
            if delta_y > 0 {
                y0 += 1;
            } else {
                y0 -= 1;
            }
            accumulated_error += delta_x_abs;
            if (accumulated_error + accumulated_error) > delta_y_abs {
                accumulated_error -= delta_y_abs;
                if delta_x > 0 {
                    x0 += 1;
                } else {
                    x0 -= 1;
                }
            }
        }
    }
}
