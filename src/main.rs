use graphics_from_scratch::pixel_renderer::PixelRenderer;
use graphics_from_scratch::{draw_mesh, get_cube_mesh};

fn main() {
    let width = 430;
    let height = 180;
    assert!(width == 3440 / 8);
    assert!(height == 1440 / 8);
    let mut cube_mesh = get_cube_mesh();
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

        draw_mesh(&mut pixel_renderer, &mut cube_mesh);
        pixel_renderer.render();
    }
}
