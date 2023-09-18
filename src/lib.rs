pub mod mat;
pub mod mesh;
pub mod pixel_renderer;
pub mod vec;

use mat::Mat4;
use mesh::Mesh;
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TriangleFill {
    None,
    Color,
    Texture,
}

pub struct DrawOptions {
    pub draw_wireframe: bool,
    pub triangle_fill: TriangleFill,
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
        mesh.rotation.x = mesh.rotation.x * 0.9999 + rng.gen_range(-0.05..0.05);
        mesh.rotation.y = mesh.rotation.y * 0.9999 + rng.gen_range(-0.05..0.05);
        mesh.rotation.z = mesh.rotation.z * 0.9999 + rng.gen_range(-0.05..0.05);
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

        let face_normal = (vert_b - vert_a).cross(vert_c - vert_a);
        if draw_options.backface_culling {
            let vec_to_camera = CAMERA_LOCATION - vert_a;
            if face_normal.dot(vec_to_camera) <= 0.0 {
                continue;
            }
        }

        let uv_a = mesh.uvs[face.a_uv];
        let uv_b = mesh.uvs[face.b_uv];
        let uv_c = mesh.uvs[face.c_uv];
        // TODO: Do something with UVs

        let is_facing_light = face_normal.unit_norm().dot(LIGHT_DIRECTION.unit_norm());
        let (intensity_min, intensity_max) = (0.2, 1.2);
        let light_intensity = ((is_facing_light + 1.0) / (1.0 + 1.0))
            * (intensity_max - intensity_min)
            + intensity_min;

        let pa = project_point_to_screen_space(pixel_renderer.width, pixel_renderer.height, vert_a);
        let pb = project_point_to_screen_space(pixel_renderer.width, pixel_renderer.height, vert_b);
        let pc = project_point_to_screen_space(pixel_renderer.width, pixel_renderer.height, vert_c);

        if draw_options.triangle_fill == TriangleFill::Color {
            draw_triangle_color(
                pixel_renderer,
                color_mul(face.color, light_intensity),
                pa,
                pb,
                pc,
            );
        } else if draw_options.triangle_fill == TriangleFill::Texture {
            draw_triangle_texture(
                pixel_renderer,
                light_intensity,
                pa,
                pb,
                pc,
                uv_a,
                uv_b,
                uv_c,
            );
        }

        if draw_options.draw_wireframe {
            draw_line(pixel_renderer, Color::RGB(255, 255, 255), pa, pb);
            draw_line(pixel_renderer, Color::RGB(255, 255, 255), pb, pc);
            draw_line(pixel_renderer, Color::RGB(255, 255, 255), pc, pa);
        }

        if draw_options.triangle_fill == TriangleFill::None && !draw_options.draw_wireframe {
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

pub fn draw_triangle_color(
    pixel_renderer: &mut PixelRenderer,
    color: Color,
    a: Vec2,
    b: Vec2,
    c: Vec2,
) {
    let (x_min, x_max): (i32, i32) = min_max(a.x, b.x, c.x);
    let (y_min, y_max): (i32, i32) = min_max(a.y, b.y, c.y);

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

fn interpolate_uv(p: Vec2, a: Vec2, b: Vec2, c: Vec2, a_uv: Vec2, b_uv: Vec2, c_uv: Vec2) -> Vec2 {
    // TODO: We need to correct the perspective using z, but first we must pass z to this function somehow.
    let a_weight = ((p - b).cross_z(p - c)) / ((a - b).cross_z(a - c));
    let b_weight = ((p - a).cross_z(p - c)) / ((b - a).cross_z(b - c));
    let c_weight = ((p - a).cross_z(p - b)) / ((c - a).cross_z(c - b));

    debug_assert!((-0.001..=1.001).contains(&a_weight));
    debug_assert!((-0.001..=1.001).contains(&b_weight));
    debug_assert!((-0.001..=1.001).contains(&c_weight));
    debug_assert!(0.999 < a_weight + b_weight + c_weight && a_weight + b_weight + c_weight < 1.001);

    (a_uv * a_weight) + (b_uv * b_weight) + (c_uv * c_weight)
}

pub fn draw_triangle_texture(
    pixel_renderer: &mut PixelRenderer,
    light_intensity: f32,
    a: Vec2,
    b: Vec2,
    c: Vec2,
    a_uv: Vec2,
    b_uv: Vec2,
    c_uv: Vec2,
) {
    // TODO: Accept and use a texture
    let (x_min, x_max): (i32, i32) = min_max(a.x, b.x, c.x);
    let (y_min, y_max): (i32, i32) = min_max(a.y, b.y, c.y);

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
                let uv = interpolate_uv(p, a, b, c, a_uv, b_uv, c_uv);
                let r: u8 = (uv.x * 127.0).round() as u8;
                let b: u8 = (uv.y * 127.0).round() as u8;
                pixel_renderer.set_pixel(
                    x as u32,
                    y as u32,
                    color_mul(Color::RGB(255 - b, 255 - r - b, 255 - r), light_intensity),
                );
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
