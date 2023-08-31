pub mod mesh;
pub mod pixel_renderer;
pub mod vec;

use mesh::{Face, Mesh};
use pixel_renderer::PixelRenderer;
use rand::Rng;
use sdl2::pixels::Color;
use vec::{Vec2, Vec3};

fn rotate(x: f32, y: f32, angle_degrees: f32) -> (f32, f32) {
    let (sin_a, cos_a) = angle_degrees.to_radians().sin_cos();
    (x * cos_a - y * sin_a, x * sin_a + y * cos_a)
}

static POINTS: [Vec3; 8] = [
    Vec3::new(1.0, 1.0, -1.0),
    Vec3::new(1.0, -1.0, -1.0),
    Vec3::new(-1.0, -1.0, -1.0),
    Vec3::new(-1.0, 1.0, -1.0),
    Vec3::new(-1.0, 1.0, 1.0),
    Vec3::new(-1.0, -1.0, 1.0),
    Vec3::new(1.0, -1.0, 1.0),
    Vec3::new(1.0, 1.0, 1.0),
];

static FACES: [Face; 12] = [
    // Front
    Face::new(0, 1, 2),
    Face::new(2, 3, 0),
    // Back
    Face::new(4, 5, 6),
    Face::new(6, 7, 4),
    // Left
    Face::new(0, 3, 4),
    Face::new(4, 7, 0),
    // Right
    Face::new(2, 1, 6),
    Face::new(6, 5, 2),
    // Top
    Face::new(2, 5, 4),
    Face::new(4, 3, 2),
    // Bottom
    Face::new(0, 7, 6),
    Face::new(6, 1, 0),
];

pub fn get_cube_mesh() -> Mesh {
    Mesh {
        vertices: POINTS.to_vec(),
        faces: FACES.to_vec(),
        rotation: Vec3::new(0.0, 0.0, 0.0),
    }
}

fn project_point(pixel_renderer: &mut PixelRenderer, p: Vec3) -> Vec2 {
    let half_width: f32 = pixel_renderer.width as f32 / 2.0;
    let half_height: f32 = pixel_renderer.height as f32 / 2.0;
    let camera_dist = 5.0;
    let scale = 400.0;
    let centered_x = p.x / (p.z + camera_dist) * scale + half_width;
    let centered_y = p.y / (p.z + camera_dist) * scale + half_height;
    Vec2::new(centered_x, centered_y)
}

pub fn draw_mesh(pixel_renderer: &mut PixelRenderer, mesh: &mut Mesh) {
    let mut rng = rand::thread_rng();
    mesh.rotation.x = mesh.rotation.x * 0.9999 + rng.gen_range(-0.03..0.03);
    mesh.rotation.y = mesh.rotation.y * 0.9999 + rng.gen_range(-0.03..0.03);
    mesh.rotation.z = mesh.rotation.z * 0.9999 + rng.gen_range(-0.03..0.03);

    for p in mesh.vertices.iter_mut() {
        let (rot_x, rot_y) = rotate(p.x, p.y, mesh.rotation.z);
        let (rot_x, rot_z) = rotate(rot_x, p.z, mesh.rotation.y);
        let (rot_y, rot_z) = rotate(rot_y, rot_z, mesh.rotation.x);
        p.x = rot_x;
        p.y = rot_y;
        p.z = rot_z;
    }

    pixel_renderer.clear_pixels(Color::RGB(0, 0, 0));

    for face in mesh.faces.iter() {
        let pa = project_point(pixel_renderer, mesh.vertices[face.a]);
        let pb = project_point(pixel_renderer, mesh.vertices[face.b]);
        let pc = project_point(pixel_renderer, mesh.vertices[face.c]);

        draw_line(pixel_renderer, pa.x, pa.y, pb.x, pb.y);
        draw_line(pixel_renderer, pb.x, pb.y, pc.x, pc.y);
        draw_line(pixel_renderer, pc.x, pc.y, pa.x, pa.y);
    }
}

pub fn draw_line(pixel_renderer: &mut PixelRenderer, x0: f32, y0: f32, x1: f32, y1: f32) {
    let color = Color::RGB(150, 150, 255);

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
