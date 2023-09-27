use graphics_from_scratch::mesh::Mesh;
use graphics_from_scratch::pixel_renderer::PixelRenderer;
use graphics_from_scratch::vec::Vec3;
use graphics_from_scratch::{
    draw_mesh, update_world, update_world_approach, update_world_rotate, DrawOptions, TriangleFill,
    World,
};
use sdl2::keyboard::{KeyboardState, Scancode};

fn main() {
    let width = 860;
    let height = 360;
    assert!(width == 3440 / 4);
    assert!(height == 1440 / 4);
    let mesh = Mesh::load_mesh("./assets/f22.obj", "./assets/f22.png");
    let mut pixel_renderer = PixelRenderer::new(width, height);
    let mut world = World {
        mesh,
        camera_location: Vec3::new(0.0, 0.0, -5.0),
        camera_look_at: Vec3::new(0.0, 0.0, 0.0),
        options: DrawOptions {
            draw_wireframe: false,
            triangle_fill: TriangleFill::Texture,
            backface_culling: true,
            pause_rendering: true,
        },
    };
    let mut prior_instant: std::time::Instant = std::time::Instant::now();
    'main_loop: loop {
        for event in pixel_renderer.context.event_pump().unwrap().poll_iter() {
            let draw_options = &mut world.options;
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
                sdl2::event::Event::MouseMotion { xrel, yrel, .. } => {
                    update_world_rotate(&mut world, (xrel, yrel));
                }
                _ => {}
            }
        }

        let mut motion_vec = (0.0, 0.0);
        let event_pump = pixel_renderer.context.event_pump().unwrap();
        let keyboard_state = KeyboardState::new(&event_pump);
        if keyboard_state.is_scancode_pressed(Scancode::W) {
            motion_vec.0 += 1.0;
        }
        if keyboard_state.is_scancode_pressed(Scancode::A) {
            motion_vec.1 += -1.0;
        }
        if keyboard_state.is_scancode_pressed(Scancode::S) {
            motion_vec.0 += -1.0;
        }
        if keyboard_state.is_scancode_pressed(Scancode::D) {
            motion_vec.1 += 1.0;
        }

        let delta_t = (std::time::Instant::now() - prior_instant).as_secs_f32();
        update_world_approach(&mut world, motion_vec.0, motion_vec.1, delta_t);
        update_world(&mut world, delta_t);
        prior_instant = std::time::Instant::now();
        draw_mesh(&mut pixel_renderer, &world);
        pixel_renderer.render();
    }
}
