use graphics_from_scratch::mesh::Mesh;
use graphics_from_scratch::pixel_renderer::PixelRenderer;
use graphics_from_scratch::{draw_mesh, DrawOptions, TriangleFill};

fn main() {
    let width = 860;
    let height = 360;
    assert!(width == 3440 / 4);
    assert!(height == 1440 / 4);
    let mut mesh = Mesh::load_mesh("./assets/cube.obj", "./assets/cube.png");
    let mut pixel_renderer = PixelRenderer::new(width, height);
    let mut draw_options = DrawOptions {
        draw_wireframe: false,
        triangle_fill: TriangleFill::Texture,
        backface_culling: true,
        pause_rendering: true,
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
                } => match draw_options.triangle_fill {
                    TriangleFill::None => draw_options.triangle_fill = TriangleFill::Color,
                    TriangleFill::Color => draw_options.triangle_fill = TriangleFill::Texture,
                    TriangleFill::Texture => draw_options.triangle_fill = TriangleFill::None,
                },
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
                    draw_options.pause_rendering = !draw_options.pause_rendering;
                }
                _ => {}
            }
        }

        draw_mesh(&mut pixel_renderer, &draw_options, &mut mesh);
        pixel_renderer.render();
    }
}
