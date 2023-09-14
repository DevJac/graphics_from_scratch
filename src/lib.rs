pub mod mat;
pub mod mesh;
pub mod pixel_renderer;
pub mod vec;

use mat::Mat4;
use mesh::{Face, Mesh};
use pixel_renderer::PixelRenderer;
use rand::{seq::SliceRandom, Rng};
use sdl2::pixels::Color;
use vec::{Vec2, Vec3};

const CAMERA_LOCATION: Vec3 = Vec3::new(0.0, 0.0, -5.0);
const LIGHT_DIRECTION: Vec3 = Vec3::new(-100.0, -100.0, -50.0);
const ASPECT_RATIO: f32 = 9.0 / 21.0;
const F: f32 = 1.732_051; // (1 / (tan(FOV / 2))); FOV = 60 degrees
const Z_NEAR: f32 = 1.0;
const Z_FAR: f32 = 10.0;
const Z_RATIO: f32 = Z_FAR / (Z_FAR - Z_NEAR);

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

pub fn project_point_to_screen_space(screen_width: u32, screen_height: u32, p: Vec3) -> Vec2 {
    let projection_matrix = Mat4::new(
        // Row 1
        ASPECT_RATIO * F,
        0.0,
        0.0,
        0.0,
        // Row 2
        0.0,
        F,
        0.0,
        0.0,
        // Row 3
        0.0,
        0.0,
        Z_RATIO,
        -Z_RATIO * Z_NEAR,
        // Row 4
        0.0,
        0.0,
        1.0,
        0.0,
    );

    let p = projection_matrix * ((p - CAMERA_LOCATION).to_vec4());

    let half_width: f32 = screen_width as f32 / 2.0;
    let half_height: f32 = screen_height as f32 / 2.0;
    let centered_x = (p.x / p.w) * half_width + half_width;
    let centered_y = (p.y / p.w) * half_height + half_height;
    Vec2::new(centered_x, centered_y)
}

pub struct DrawOptions {
    pub draw_wireframe: bool,
    pub fill_triangles: bool,
    pub backface_culling: bool,
    pub pause_rendering: bool,
}

fn color_mul(color: Color, multiplier: f32) -> Color {
    let r = (color.r as f32 * multiplier).clamp(0.0, 255.0).round() as u8;
    let g = (color.g as f32 * multiplier).clamp(0.0, 255.0).round() as u8;
    let b = (color.b as f32 * multiplier).clamp(0.0, 255.0).round() as u8;
    Color::RGB(r, g, b)
}

pub fn draw_mesh(pixel_renderer: &mut PixelRenderer, draw_options: &DrawOptions, mesh: &mut Mesh) {
    let mut rng = rand::thread_rng();
    if !draw_options.pause_rendering && rng.gen::<f32>() < 0.03 {
        mesh.rotation.x = mesh.rotation.x * 0.9999 + rng.gen_range(-0.03..0.03);
        mesh.rotation.y = mesh.rotation.y * 0.9999 + rng.gen_range(-0.03..0.03);
        mesh.rotation.z = mesh.rotation.z * 0.9999 + rng.gen_range(-0.03..0.03);
    }

    if !draw_options.pause_rendering {
        for p in mesh.vertices.iter_mut() {
            let rx = Mat4::rotate_x(mesh.rotation.x);
            let ry = Mat4::rotate_y(mesh.rotation.y);
            let rz = Mat4::rotate_z(mesh.rotation.z);
            *p = rx * ry * rz * (*p);
        }
    }

    pixel_renderer.clear_pixels(Color::RGB(0, 0, 0));

    for face in mesh.faces.choose_multiple(&mut rng, mesh.faces.len()) {
        let vert_a = mesh.vertices[face.a];
        let vert_b = mesh.vertices[face.b];
        let vert_c = mesh.vertices[face.c];
        let vert_center = (vert_a + vert_b + vert_c) / 3.0;

        let face_normal = (vert_b - vert_a).cross(vert_c - vert_a);
        if draw_options.backface_culling {
            let vec_to_camera = CAMERA_LOCATION - vert_center;
            if face_normal.dot(vec_to_camera) <= 0.0 {
                continue;
            }
        }

        let is_facing_light = face_normal.unit_norm().dot(LIGHT_DIRECTION.unit_norm());
        let light_intensity = is_facing_light / 2.0 + 1.0;

        let pa = project_point_to_screen_space(pixel_renderer.width, pixel_renderer.height, vert_a);
        let pb = project_point_to_screen_space(pixel_renderer.width, pixel_renderer.height, vert_b);
        let pc = project_point_to_screen_space(pixel_renderer.width, pixel_renderer.height, vert_c);

        if draw_options.fill_triangles {
            draw_triangle(
                pixel_renderer,
                color_mul(face.color, light_intensity),
                pa,
                pb,
                pc,
            );
        }

        if draw_options.draw_wireframe {
            draw_line(pixel_renderer, Color::RGB(255, 255, 255), pa, pb);
            draw_line(pixel_renderer, Color::RGB(255, 255, 255), pb, pc);
            draw_line(pixel_renderer, Color::RGB(255, 255, 255), pc, pa);
        }

        if !draw_options.fill_triangles && !draw_options.draw_wireframe {
            pixel_renderer.set_pixel(
                pa.x.round() as u32,
                pa.y.round() as u32,
                Color::RGB(255, 255, 255),
            );
            pixel_renderer.set_pixel(
                pb.x.round() as u32,
                pb.y.round() as u32,
                Color::RGB(255, 255, 255),
            );
            pixel_renderer.set_pixel(
                pc.x.round() as u32,
                pc.y.round() as u32,
                Color::RGB(255, 255, 255),
            );
        }

        pixel_renderer.set_pixel(0, 0, Color::RGB(255, 255, 255));
        pixel_renderer.set_pixel(10, 0, Color::RGB(255, 0, 0));
        pixel_renderer.set_pixel(0, 10, Color::RGB(0, 255, 0));
        pixel_renderer.set_pixel(10, 10, Color::RGB(0, 0, 255));
    }
}

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

fn cross_edge(p: Vec2, vert: Vec2, edge_from_vert: Vec2) -> f32 {
    let vert_to_p = p - vert;
    let mut cross_z = edge_from_vert.cross_z(vert_to_p);
    if edge_from_vert.y > 0.0 || (edge_from_vert.y == 0.0 && edge_from_vert.x > 0.0) {
        cross_z += 0.000_001;
    } else {
        cross_z -= 0.000_001;
    }
    cross_z.signum()
}

pub fn draw_triangle(pixel_renderer: &mut PixelRenderer, color: Color, a: Vec2, b: Vec2, c: Vec2) {
    let (x_min, x_max) = min_max(a.x, b.x, c.x);
    let (y_min, y_max) = min_max(a.y, b.y, c.y);

    let x_min = x_min.max(0);
    let y_min = y_min.max(0);

    let edge_from_a = b - a;
    let edge_from_b = c - b;
    let edge_from_c = a - c;

    for x in x_min..=x_max {
        for y in y_min..=y_max {
            let p = Vec2::new(x as f32, y as f32);
            let in_a = cross_edge(p, a, edge_from_a);
            let in_b = cross_edge(p, b, edge_from_b);
            let in_c = cross_edge(p, c, edge_from_c);

            if in_a == in_b && in_a == in_c {
                pixel_renderer.set_pixel(x as u32, y as u32, color);
            }
        }
    }
}

pub fn draw_line(pixel_renderer: &mut PixelRenderer, color: Color, a: Vec2, b: Vec2) {
    let mut x0: i32 = a.x.round() as i32;
    let mut y0: i32 = a.y.round() as i32;
    let x1: i32 = b.x.round() as i32;
    let y1: i32 = b.y.round() as i32;

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
            if ((accumulated_error + accumulated_error) > delta_x_abs)
                || (delta_y > 0 && (accumulated_error + accumulated_error) == delta_x_abs)
            {
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
            if ((accumulated_error + accumulated_error) > delta_y_abs)
                || (delta_x > 0 && (accumulated_error + accumulated_error) == delta_y_abs)
            {
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
