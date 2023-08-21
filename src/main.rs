use graphics_from_scratch::draw_grid;
use graphics_from_scratch::pixel_renderer::PixelRenderer;

fn main() {
    let width = 3440 / 2;
    let height = 1440 / 2;
    let mut pixel_renderer = PixelRenderer::new(width, height);
    'main_loop: loop {
        for event in pixel_renderer.context.event_pump().unwrap().poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => break 'main_loop,
                _ => {}
            }
        }

        draw_grid(&mut pixel_renderer);
        pixel_renderer.render();
    }
}
