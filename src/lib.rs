pub mod pixel_renderer;
pub mod vec;

use pixel_renderer::PixelRenderer;
use rand::Rng;
use sdl2::pixels::Color;
use vec::Vec3;

/////////////////////////////////////////////////////////////////
// Junk pile - Half-baked abstractions and experiments go here //
/////////////////////////////////////////////////////////////////

pub fn draw_grid(pixel_renderer: &mut PixelRenderer) {
    let mut rng = rand::thread_rng();

    pixel_renderer.clear_pixels(Color::RGB(0, 0, 0));

    for x in 0..pixel_renderer.width {
        for y in 0..pixel_renderer.height {
            if x % 10 == 0 || y % 10 == 0 {
                pixel_renderer.set_pixel(x, y, Color::RGB(100, 100, 0));
            } else if x % 10 == 2 && y % 10 == 2 {
                draw_rect(
                    pixel_renderer,
                    Rect {
                        x,
                        y,
                        width: 7,
                        height: 7,
                    },
                    Color::RGB(
                        rng.gen_range(0..255),
                        rng.gen_range(0..255),
                        rng.gen_range(0..255),
                    ),
                );
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

pub fn draw_rect(pixel_renderer: &mut PixelRenderer, rect: Rect, color: Color) {
    for x in rect.x..(rect.x + rect.width) {
        for y in rect.y..(rect.y + rect.height) {
            pixel_renderer.set_pixel(x, y, color);
        }
    }
}

fn rotate(x: f32, y: f32, angle_degrees: f32) -> (f32, f32) {
    let (sin_a, cos_a) = angle_degrees.to_radians().sin_cos();
    (x * cos_a - y * sin_a, x * sin_a + y * cos_a)
}

use std::cell::RefCell;
thread_local! {
static POINTS: RefCell<[Vec3; 9 * 9 * 9]> = RefCell::new([Vec3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
}; 9 * 9 * 9]);
static POINTS_INITIALIZED: RefCell<bool> = RefCell::new(false);
}

pub fn draw_point_cube(pixel_renderer: &mut PixelRenderer) {
    POINTS_INITIALIZED.with(|points_initialized| {
        if !(*points_initialized.borrow()) {
            *points_initialized.borrow_mut() = true;
            let mut i: usize = 0;
            for x in 0..9 {
                for y in 0..9 {
                    for z in 0..9 {
                        let x: f32 = (x as f32) * 0.25 - 1.0;
                        let y: f32 = (y as f32) * 0.25 - 1.0;
                        let z: f32 = (z as f32) * 0.25 - 1.0;
                        POINTS.with(|points| {
                            points.borrow_mut()[i] = Vec3::new(x, y, z);
                        });
                        i += 1;
                    }
                }
            }
        }
    });

    let mut rng = rand::thread_rng();
    let rotation_per_frame: f32 = 0.12;
    let x_rpf = rng.gen_range(0.1..rotation_per_frame);
    let y_rpf = rng.gen_range(0.1..rotation_per_frame);
    let z_rpf = rng.gen_range(0.1..rotation_per_frame);

    POINTS.with(|points| {
        for p in points.borrow_mut().iter_mut() {
            let (rot_x, rot_y) = rotate(p.x, p.y, z_rpf);
            let (rot_x, rot_z) = rotate(rot_x, p.z, y_rpf);
            let (rot_y, rot_z) = rotate(rot_y, rot_z, x_rpf);
            p.x = rot_x;
            p.y = rot_y;
            p.z = rot_z;
        }

        pixel_renderer.clear_pixels(Color::RGB(0, 0, 0));

        for p in points.borrow().iter() {
            let half_width: f32 = pixel_renderer.width as f32 / 2.0;
            let half_height: f32 = pixel_renderer.height as f32 / 2.0;
            let camera_dist = 5.0;
            let scale = 500.0;
            let centered_x = (p.x / (p.z + camera_dist) * scale + half_width).round() as u32;
            let centered_y = (p.y / (p.z + camera_dist) * scale + half_height).round() as u32;
            if centered_x > 0 && centered_y > 0 {
                pixel_renderer.set_pixel(centered_x, centered_y, Color::RGB(255, 255, 0));
            }
        }
    });
}
