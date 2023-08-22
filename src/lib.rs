pub mod pixel_renderer;

use pixel_renderer::PixelRenderer;
use sdl2::pixels::Color;

pub fn draw_grid(pixel_renderer: &mut PixelRenderer) {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    for x in 0..pixel_renderer.width {
        for y in 0..pixel_renderer.height {
            if x % 10 == 0 || y % 10 == 0 {
                pixel_renderer.set_pixel(x, y, Color::RGB(100, 100, 0));
            } else if x % 10 == 5 && y % 10 == 5 {
                pixel_renderer.set_pixel(
                    x,
                    y,
                    Color::RGB(
                        rng.gen_range(0..255),
                        rng.gen_range(0..255),
                        rng.gen_range(0..255),
                    ),
                );
            } else {
                pixel_renderer.set_pixel(x, y, Color::RGB(0, 0, 0));
            }
        }
    }
}
