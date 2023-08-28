pub mod pixel_renderer;
pub mod vec;

use pixel_renderer::PixelRenderer;
use rand::Rng;
use sdl2::pixels::Color;
use std::cell::RefCell;
use vec::Vec3;

fn rotate(x: f32, y: f32, angle_degrees: f32) -> (f32, f32) {
    let (sin_a, cos_a) = angle_degrees.to_radians().sin_cos();
    (x * cos_a - y * sin_a, x * sin_a + y * cos_a)
}

struct Face {
    a: usize,
    b: usize,
    c: usize,
}

impl Face {
    pub const fn new(a: usize, b: usize, c: usize) -> Self {
        Self { a, b, c }
    }
}

thread_local! {
    static POINTS: RefCell<Vec<Vec3>> = RefCell::new(vec![
    Vec3::new(1.0, 1.0, -1.0),
    Vec3::new(1.0, -1.0, -1.0),
    Vec3::new(-1.0, -1.0, -1.0),
    Vec3::new(-1.0, 1.0, -1.0),
    Vec3::new(1.0, 1.0, 1.0),
    Vec3::new(1.0, -1.0, 1.0),
    Vec3::new(-1.0, -1.0, 1.0),
    Vec3::new(-1.0, 1.0, 1.0),
    ]);

    static FACES: RefCell<Vec<Face>> = RefCell::new(vec![
    // Front
    Face::new(0, 1, 2),
    Face::new(2, 3, 0),
    // Back
    Face::new(6, 5, 4),
    Face::new(4, 7, 6),
    // Left
    Face::new(7, 4, 0),
    Face::new(0, 3, 7),
    // Right
    Face::new(1, 5, 6),
    Face::new(6, 2, 1),
    // Top
    Face::new(0, 4, 5),
    Face::new(5, 1, 0),
    // Bottom
    Face::new(6, 7, 3),
    Face::new(3, 2, 6),
    ]);

    static ROTATION: RefCell<Vec3> = RefCell::new(Vec3::new(0.0, 0.0, 0.0));
}

pub fn draw_point_cube(pixel_renderer: &mut PixelRenderer) {
    let mut rng = rand::thread_rng();
    ROTATION.with(|rotation| {
        POINTS.with(|points| {
            let mut r = rotation.borrow_mut();
            r.x = r.x * 0.99 + rng.gen_range(-0.1..0.1);
            r.y = r.y * 0.99 + rng.gen_range(-0.1..0.1);
            r.z = r.z * 0.99 + rng.gen_range(-0.1..0.1);
            for p in points.borrow_mut().iter_mut() {
                let (rot_x, rot_y) = rotate(p.x, p.y, r.z);
                let (rot_x, rot_z) = rotate(rot_x, p.z, r.y);
                let (rot_y, rot_z) = rotate(rot_y, rot_z, r.x);
                p.x = rot_x;
                p.y = rot_y;
                p.z = rot_z;
            }

            pixel_renderer.clear_pixels(Color::RGB(0, 0, 0));

            for p in points.borrow().iter() {
                let half_width: f32 = pixel_renderer.width as f32 / 2.0;
                let half_height: f32 = pixel_renderer.height as f32 / 2.0;
                let camera_dist = 5.0;
                let scale = 400.0;
                let centered_x = (p.x / (p.z + camera_dist) * scale + half_width).round() as u32;
                let centered_y = (p.y / (p.z + camera_dist) * scale + half_height).round() as u32;
                if centered_x > 0 && centered_y > 0 {
                    pixel_renderer.set_pixel(centered_x, centered_y, Color::RGB(255, 255, 0));
                }
            }
        })
    });
}
