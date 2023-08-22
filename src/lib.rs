pub mod pixel_renderer;

use pixel_renderer::PixelRenderer;
use sdl2::pixels::Color;

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
