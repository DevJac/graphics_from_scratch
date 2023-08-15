mod state {
    // Unsafe: texture_creator and texture are related, the sdl2 docs aren't 100% clear on how.
    // texture needs to be dropped before texture_creator (I think).
    // The plan is to keep both of them together in this struct, and to keep them private so
    // that the unsafe behavior doesn't leak outside of this state module.
    use sdl2::pixels::PixelFormatEnum;
    use sdl2::render::{TextureAccess, TextureCreator};
    use sdl2::video::{Window, WindowContext};
    use sdl2::{pixels::Color, render::Canvas, render::Texture};
    use std::pin::Pin;
    pub struct State {
        pub width: u32,
        pub height: u32,
        pub context: sdl2::Sdl,
        pub canvas: Canvas<Window>,
        pub color_buffer: Box<[Color]>,
        // Unsafe: texture must be dropped before texture_creator.
        // Struct fields are dropped in the order they are defined,
        // so texture must come before texture_creator in the struct definition.
        texture: Box<Texture<'static>>,
        texture_creator: Pin<Box<TextureCreator<WindowContext>>>,
    }

    impl State {
        pub fn new(width: u32, height: u32) -> Self {
            use std::ptr::addr_of_mut;

            let mut result = std::mem::MaybeUninit::<Self>::uninit();
            let result_ptr = result.as_mut_ptr();
            unsafe {
                addr_of_mut!((*result_ptr).context).write(sdl2::init().unwrap());
                let window = (*result_ptr)
                    .context
                    .video()
                    .unwrap()
                    .window(
                        "Renderer",
                        width.try_into().unwrap(),
                        height.try_into().unwrap(),
                    )
                    .build()
                    .unwrap();
                addr_of_mut!((*result_ptr).canvas).write(window.into_canvas().build().unwrap());
                addr_of_mut!((*result_ptr).color_buffer).write(
                    vec![Color::RGB(0, 0, 0); (width * height).try_into().unwrap()]
                        .into_boxed_slice(),
                );
                addr_of_mut!((*result_ptr).texture_creator)
                    .write(Box::pin((*result_ptr).canvas.texture_creator()));
                addr_of_mut!((*result_ptr).texture).write(Box::new(
                    (*result_ptr)
                        .texture_creator
                        .create_texture(
                            PixelFormatEnum::ARGB8888,
                            TextureAccess::Streaming,
                            width,
                            height,
                        )
                        .unwrap(),
                ));
                result.assume_init()
            }
        }

        pub fn present(&mut self) {
            self.canvas.set_draw_color(Color::RGB(50, 100, 150));
            self.canvas.clear();
            self.canvas.present();
        }
    }
}

fn main() {
    use state::State;

    let width = 800;
    let height = 600;
    let mut state = State::new(width, height);
    state.present();
    'main_loop: loop {
        for event in state.context.event_pump().unwrap().poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => break 'main_loop,
                _ => {}
            }
        }
        state.present();
    }
}
