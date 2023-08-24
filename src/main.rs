use graphics_from_scratch::draw_point_cube;
use graphics_from_scratch::pixel_renderer::PixelRenderer;

fn main() {
    let width = 860;
    let height = 360;
    assert!(width == 3440 / 4);
    assert!(height == 1440 / 4);
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

        draw_point_cube(&mut pixel_renderer);
        pixel_renderer.render();
    }
}
