use graphics_from_scratch::pixel_renderer::PixelRenderer;
use graphics_from_scratch::{draw_mesh, get_cube_mesh, DrawOptions};

fn main() {
    let width = 430;
    let height = 180;
    assert!(width == 3440 / 8);
    assert!(height == 1440 / 8);
    let mut cube_mesh = get_cube_mesh();
    let mut pixel_renderer = PixelRenderer::new(width, height);
    let mut draw_options = DrawOptions {
        draw_wireframe: false,
        fill_triangles: true,
        backface_culling: true,
        slow_rendering: false,
    };
    'main_loop: loop {
        for event in pixel_renderer.context.event_pump().unwrap().poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => break 'main_loop,
                sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Num1),
                    ..
                } => {
                    draw_options.draw_wireframe = !draw_options.draw_wireframe;
                }
                sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Num2),
                    ..
                } => {
                    draw_options.fill_triangles = !draw_options.fill_triangles;
                }
                sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Num3),
                    ..
                } => {
                    draw_options.backface_culling = !draw_options.backface_culling;
                }
                sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Num4),
                    ..
                } => {
                    draw_options.slow_rendering = !draw_options.slow_rendering;
                }
                _ => {}
            }
        }

        draw_mesh(&mut pixel_renderer, &draw_options, &mut cube_mesh);
        pixel_renderer.render();
    }
}
