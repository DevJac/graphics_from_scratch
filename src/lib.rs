pub mod mat;
pub mod mesh;
pub mod pixel_renderer;
pub mod vec;

use mat::Mat4;
use mesh::Mesh;
use pixel_renderer::PixelRenderer;
use rand::{seq::SliceRandom, Rng};
use sdl2::pixels::Color;
use vec::{Vec2, Vec3, Vec4};

const UP: Vec3 = Vec3::new(0.0, 1.0, 0.0);
const LIGHT_DIRECTION: Vec3 = Vec3::new(-100.0, 100.0, -50.0);
const ASPECT_RATIO: f32 = 9.0 / 21.0;
const FOV: f32 = 60.0; // 60 degrees
const Z_NEAR: f32 = 1.0;
const Z_FAR: f32 = 10.0;
const Z_RATIO: f32 = Z_FAR / (Z_FAR - Z_NEAR);

pub fn project_point_to_camera_space(p: Vec3, camera_location: Vec3, look_at: Vec3) -> Vec3 {
    camera_view_matrix(camera_location, look_at, UP) * p
}

pub fn project_point_to_screen_space(screen_width: u32, screen_height: u32, p: Vec3) -> Vec4 {
    //  f: f32 = 1.732_051;  (1 / (tan(FOV / 2)))
    let f: f32 = 1.0 / ((FOV.to_radians() / 2.0).tan());
    let projection_matrix = Mat4::new(
        // Row 1
        ASPECT_RATIO * f,
        0.0,
        0.0,
        0.0,
        // Row 2
        0.0,
        -f,
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

    let p = projection_matrix * p.to_vec4();

    let half_width: f32 = screen_width as f32 / 2.0;
    let half_height: f32 = screen_height as f32 / 2.0;
    let centered_x = (p.x / p.w) * half_width + half_width;
    let centered_y = (p.y / p.w) * half_height + half_height;
    Vec4::new(centered_x, centered_y, p.z, p.w)
}

pub fn camera_view_matrix(camera_location: Vec3, look_at: Vec3, up: Vec3) -> Mat4 {
    // Conceptually, we will create a set of orthonormal basis vectors and the Z basis vector will point
    // towards look_at. Once we have the matrix to transform the standard basis vectors we will find the
    // inverse (by simply transposing, since it's orthonormal) and apply the inverse to the world.
    // Applying the inverse to the world will transform the world and place look_at along the standard z axis.
    // (We will not store the intermediate matrix, we will calculate and return the inverse matrix directly.)

    // The axes that are aligned with look_at, we will call "aligned".
    let aligned_z_axis = (look_at - camera_location).unit_norm();
    let aligned_x_axis = up.cross(aligned_z_axis).unit_norm();
    // When cross arguments are already normal, the result will be normal.
    let aligned_y_axis = aligned_z_axis.cross(aligned_x_axis);

    Mat4::new(
        // Row 1
        aligned_x_axis.x,
        aligned_x_axis.y,
        aligned_x_axis.z,
        -aligned_x_axis.dot(camera_location),
        // Row 2
        aligned_y_axis.x,
        aligned_y_axis.y,
        aligned_y_axis.z,
        -aligned_y_axis.dot(camera_location),
        // Row 3
        aligned_z_axis.x,
        aligned_z_axis.y,
        aligned_z_axis.z,
        -aligned_z_axis.dot(camera_location),
        // Row 4
        0.0,
        0.0,
        0.0,
        1.0,
    )
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

pub struct World {
    pub mesh: Mesh,
    pub camera_location: Vec3,
    pub camera_look_at: Vec3,
    pub options: DrawOptions,
}

pub fn update_world_approach(world: &mut World, approach: f32, delta_t: f32) {
    let toward_look_at = (world.camera_look_at - world.camera_location).unit_norm();
    let move_vec = toward_look_at * approach * 10.0 * delta_t;
    world.camera_location += move_vec;
    world.camera_look_at += move_vec;
}

pub fn update_world_rotate(world: &mut World, motion: (i32, i32)) {
    let rot_x = Mat4::rotate_y(motion.0 as f32 * 0.03);
    let rot_y = Mat4::rotate_x(motion.1 as f32 * 0.03);
    let cam_to_look = world.camera_look_at - world.camera_location;
    let rotated = rot_x * rot_y * cam_to_look;
    world.camera_look_at = world.camera_location + rotated;
}

pub fn update_world(world: &mut World, delta_t: f32) {
    let mut rng = rand::thread_rng();
    if !world.options.pause_rendering && rng.gen::<f32>() < 0.03 {
        world.mesh.rotation.x = world.mesh.rotation.x * 0.999 + rng.gen_range(-10.0..10.0);
        world.mesh.rotation.y = world.mesh.rotation.y * 0.999 + rng.gen_range(-10.0..10.0);
        world.mesh.rotation.z = world.mesh.rotation.z * 0.999 + rng.gen_range(-10.0..10.0);
    }

    if !world.options.pause_rendering {
        for p in world.mesh.vertices.iter_mut() {
            let rx = Mat4::rotate_x(world.mesh.rotation.x * delta_t);
            let ry = Mat4::rotate_y(world.mesh.rotation.y * delta_t);
            let rz = Mat4::rotate_z(world.mesh.rotation.z * delta_t);
            *p = rx * ry * rz * (*p);
        }
    }
}

fn color_mul(color: Color, multiplier: f32) -> Color {
    let r = (color.r as f32 * multiplier).clamp(0.0, 255.0).round() as u8;
    let g = (color.g as f32 * multiplier).clamp(0.0, 255.0).round() as u8;
    let b = (color.b as f32 * multiplier).clamp(0.0, 255.0).round() as u8;
    Color::RGB(r, g, b)
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct ClipPlane {
    name: &'static str,
    point: Vec3,
    norm: Vec3,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct ClipVert {
    vert: Vec3,
    uv: Vec2,
}

impl ClipVert {
    fn new(vert: Vec3, uv: Vec2) -> Self {
        ClipVert { vert, uv }
    }
}

fn frustum_planes() -> Vec<ClipPlane> {
    vec![
        ClipPlane {
            name: "near",
            point: Vec3::new(0.0, 0.0, Z_NEAR),
            norm: Vec3::new(0.0, 0.0, 1.0),
        },
        ClipPlane {
            name: "far",
            point: Vec3::new(0.0, 0.0, Z_FAR),
            norm: Vec3::new(0.0, 0.0, -1.0),
        },
        ClipPlane {
            name: "left",
            point: Vec3::new(0.0, 0.0, 0.0),
            norm: Mat4::rotate_y(-FOV / 1.2) * Vec3::new(1.0, 0.0, 0.0),
        },
        ClipPlane {
            name: "right",
            point: Vec3::new(0.0, 0.0, 0.0),
            norm: Mat4::rotate_y(FOV / 1.2) * Vec3::new(-1.0, 0.0, 0.0),
        },
        ClipPlane {
            name: "top",
            point: Vec3::new(0.0, 0.0, 0.0),
            norm: Mat4::rotate_x(-FOV / 2.0) * Vec3::new(0.0, -1.0, 0.0),
        },
        ClipPlane {
            name: "bottom",
            point: Vec3::new(0.0, 0.0, 0.0),
            norm: Mat4::rotate_x(FOV / 2.0) * Vec3::new(0.0, 1.0, 0.0),
        },
    ]
}

fn in_plane(vert: Vec3, plane: &ClipPlane) -> f32 {
    ((vert - plane.point).dot(plane.norm) + 0.000_1).signum()
}

fn intersection(v0: ClipVert, v1: ClipVert, plane: &ClipPlane) -> ClipVert {
    // See: https://import.cdn.thinkific.com/167815/JoyKennethClipping-200905-175314.pdf
    let d0 = (v0.vert - plane.point).dot(plane.norm);
    let d1 = (v1.vert - plane.point).dot(plane.norm);
    let t = d0 / (d0 - d1);
    let intersect = v0.vert + ((v1.vert - v0.vert) * t);

    let line_segment_len = (v1.vert - v0.vert).len();
    let v0_dist_from_intersect = (v0.vert - intersect).len();
    let v1_dist_from_intersect = (v1.vert - intersect).len();
    let v0_weight = v1_dist_from_intersect / line_segment_len;
    let v1_weight = v0_dist_from_intersect / line_segment_len;
    assert!((0.99..1.01).contains(&(v0_weight + v1_weight)));
    let intersect_uv = v0.uv * v0_weight + v1.uv * v1_weight;

    ClipVert {
        vert: intersect,
        uv: intersect_uv,
    }
}

fn frustum_clip(poly: &mut Vec<ClipVert>, clip_planes: &[ClipPlane]) {
    for plane in clip_planes {
        let mut i: usize = 0;
        loop {
            if i >= poly.len() {
                break;
            }

            let v0 = poly[i];
            let v1 = if i + 1 < poly.len() {
                poly[i + 1]
            } else {
                poly[0]
            };

            if in_plane(v0.vert, plane) == in_plane(v1.vert, plane) {
                i += 1;
                continue;
            } else {
                let intersect = intersection(v0, v1, plane);
                poly.insert(i + 1, intersect);
                i += 1;
            }

            i += 1;
        }
        poly.retain(|v| in_plane(v.vert, plane) >= 0.0);
    }
}

pub fn draw_mesh(pixel_renderer: &mut PixelRenderer, world: &World) {
    let mut rng = rand::thread_rng();
    let mesh: &Mesh = &world.mesh;
    let draw_options = &world.options;

    pixel_renderer.clear_pixels(Color::RGB(0, 0, 0));

    let clip_planes = frustum_planes();

    'faces: for face in mesh.faces.choose_multiple(&mut rng, mesh.faces.len()) {
        let vert_a = mesh.vertices[face.a];
        let vert_b = mesh.vertices[face.b];
        let vert_c = mesh.vertices[face.c];

        let face_normal = (vert_b - vert_a).cross(vert_c - vert_a).unit_norm();
        if draw_options.backface_culling {
            let vec_to_camera = world.camera_location - vert_a;
            if face_normal.dot(vec_to_camera) <= 0.0 {
                continue 'faces;
            }
        }

        let uv_a = mesh.uvs[face.a_uv];
        let uv_b = mesh.uvs[face.b_uv];
        let uv_c = mesh.uvs[face.c_uv];

        let mut polygons = Vec::with_capacity(10);
        polygons.push(ClipVert::new(
            project_point_to_camera_space(vert_a, world.camera_location, world.camera_look_at),
            uv_a,
        ));
        polygons.push(ClipVert::new(
            project_point_to_camera_space(vert_b, world.camera_location, world.camera_look_at),
            uv_b,
        ));
        polygons.push(ClipVert::new(
            project_point_to_camera_space(vert_c, world.camera_location, world.camera_look_at),
            uv_c,
        ));

        frustum_clip(&mut polygons, &clip_planes);

        if polygons.is_empty() {
            continue 'faces;
        }

        assert!(polygons.len() >= 3);

        for i in 1..(polygons.len() - 1) {
            let vert_a = polygons[0].vert;
            let vert_b = polygons[i].vert;
            let vert_c = polygons[i + 1].vert;

            let uv_a = polygons[0].uv;
            let uv_b = polygons[i].uv;
            let uv_c = polygons[i + 1].uv;

            let is_facing_light = face_normal.dot(LIGHT_DIRECTION.unit_norm());
            let (intensity_min, intensity_max) = (0.4, 1.2);
            let light_intensity = ((is_facing_light + 1.0) / (1.0 + 1.0))
                * (intensity_max - intensity_min)
                + intensity_min;

            let pa =
                project_point_to_screen_space(pixel_renderer.width, pixel_renderer.height, vert_a);
            let pb =
                project_point_to_screen_space(pixel_renderer.width, pixel_renderer.height, vert_b);
            let pc =
                project_point_to_screen_space(pixel_renderer.width, pixel_renderer.height, vert_c);

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
                    mesh,
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
        cross_z += 1.0;
    }
    cross_z.signum()
}

pub fn draw_triangle_color(
    pixel_renderer: &mut PixelRenderer,
    color: Color,
    a: Vec4,
    b: Vec4,
    c: Vec4,
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
            let in_a = cross_edge(p, a.to_vec2(), edge_from_a.to_vec2());
            let in_b = cross_edge(p, b.to_vec2(), edge_from_b.to_vec2());
            let in_c = cross_edge(p, c.to_vec2(), edge_from_c.to_vec2());

            if in_a == in_b && in_a == in_c {
                let p0 = Vec2::new(0.0, 0.0);
                let (_uv, w) = interpolate_uv(Vec2::new(x as f32, y as f32), a, b, c, p0, p0, p0);
                pixel_renderer.set_pixel_z(x as u32, y as u32, w, color);
            }
        }
    }
}

fn interpolate_uv(
    p: Vec2,
    a4: Vec4,
    b4: Vec4,
    c4: Vec4,
    a_uv: Vec2,
    b_uv: Vec2,
    c_uv: Vec2,
) -> (Vec2, f32) {
    let a = a4.to_vec2();
    let b = b4.to_vec2();
    let c = c4.to_vec2();
    let mut a_weight = ((p - b).cross_z(p - c)) / ((a - b).cross_z(a - c));
    let mut b_weight = ((p - a).cross_z(p - c)) / ((b - a).cross_z(b - c));
    let mut c_weight = ((p - a).cross_z(p - b)) / ((c - a).cross_z(c - b));

    // debug_assert!((-0.1..1.1).contains(&a_weight));
    // debug_assert!((-0.1..1.1).contains(&b_weight));
    // debug_assert!((-0.1..1.1).contains(&c_weight));
    // debug_assert!((0.9..1.1).contains(&(a_weight + b_weight + c_weight)));

    // At this point in the code, the weights are correct for screen space.
    // These weights can be used in a linear combination of the screen space UV coordinates.
    // A linear combination in screen space is not what we want though, we want a world space combination.
    // See: https://www.comp.nus.edu.sg/~lowkl/publications/lowk_persp_interp_techrep.pdf

    // The world space coordinates were divided by their depth when projecting to screen space;
    // this division is not a linear transformation and must be specially accounted for.
    // We also need to divide the weights by the depth of their associated vertex. We do so below:

    a_weight /= a4.w;
    b_weight /= b4.w;
    c_weight /= c4.w;

    // At this point in the code, the world space coordinates have been divided by depth
    // as part of the projection to screen space, and their associated weights have also been divided by depth.
    // We are ready to use the weights to combine the UV coordinates.

    // We also need to divide by the sum of the weights in order to renormalize these weights
    // (get them back into a 0 to 1 range). The depth division probably shrunk the weights,
    // and thus shrunk the texture space UV coordinate. This will result in a small corner of the texture being mapped
    // over the entire object. To increase the UV coordinate back to their correct values we divide by the small weights
    // and thus increase the UV coordinates.

    let uv = ((a_uv * a_weight) + (b_uv * b_weight) + (c_uv * c_weight))
        / (a_weight + b_weight + c_weight);
    let w =
        ((a4 * a_weight) + (b4 * b_weight) + (c4 * c_weight)) / (a_weight + b_weight + c_weight);
    (uv, w.w)
}

#[allow(clippy::too_many_arguments)]
pub fn draw_triangle_texture(
    pixel_renderer: &mut PixelRenderer,
    mesh: &Mesh,
    light_intensity: f32,
    a: Vec4,
    b: Vec4,
    c: Vec4,
    a_uv: Vec2,
    b_uv: Vec2,
    c_uv: Vec2,
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
            let in_a = cross_edge(p, a.to_vec2(), edge_from_a.to_vec2());
            let in_b = cross_edge(p, b.to_vec2(), edge_from_b.to_vec2());
            let in_c = cross_edge(p, c.to_vec2(), edge_from_c.to_vec2());

            if in_a == in_b && in_a == in_c {
                let (uv, w) = interpolate_uv(p, a, b, c, a_uv, b_uv, c_uv);
                let u = (((mesh.texture.width() - 1) as f32 * uv.x).round() as u32)
                    .clamp(0, mesh.texture.width() - 1);
                let v = (((mesh.texture.height() - 1) as f32 * (1.0 - uv.y)).round() as u32)
                    .clamp(0, mesh.texture.height() - 1);
                let texture_color = mesh.texture.get_pixel(u, v);
                pixel_renderer.set_pixel_z(
                    x as u32,
                    y as u32,
                    w,
                    color_mul(
                        Color::RGB(texture_color[0], texture_color[1], texture_color[2]),
                        light_intensity,
                    ),
                );
            }
        }
    }
}

pub fn draw_line(pixel_renderer: &mut PixelRenderer, color: Color, a: Vec4, b: Vec4) {
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
