pub mod pixel_renderer;
pub mod vec;

use pixel_renderer::PixelRenderer;
use sdl2::pixels::Color;
use vec::Vec3;

pub fn draw_grid(pixel_renderer: &mut PixelRenderer) {
    use rand::Rng;
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

pub fn draw_point_cube(pixel_renderer: &mut PixelRenderer) {
    let mut points: Vec<Vec3> = Vec::with_capacity(9 * 9 * 9);
    for x in 0..9 {
        for y in 0..9 {
            for z in 0..9 {
                let x: f32 = (x as f32) * 0.25 - 1.0;
                let y: f32 = (y as f32) * 0.25 - 1.0;
                let z: f32 = (z as f32) * 0.25 - 1.0;

                points.push(Vec3::new(x, y, z));
            }
        }
    }

    pixel_renderer.clear_pixels(Color::RGB(0, 0, 0));
    for p in points {
        let centered_x = (p.x * 128.0 + (pixel_renderer.width / 2) as f32).round() as u32;
        let centered_y = (p.y * 128.0 + (pixel_renderer.height / 2) as f32).round() as u32;
        if centered_x > 0 && centered_y > 0 {
            pixel_renderer.set_pixel(centered_x, centered_y, Color::RGB(255, 255, 0));
        }
    }
}
