mod state {
    // Unsafe: texture_creator and texture are related, the sdl2 docs aren't 100% clear on how.
    // texture needs to be dropped before texture_creator (I think).
    // The plan is to keep both of them together in this struct, and to keep them private so
    // that the unsafe behavior doesn't leak outside of this state module.
    use sdl2::pixels::{Color, PixelFormat};
    use sdl2::render::{Canvas, Texture, TextureAccess, TextureCreator};
    use sdl2::video::{Window, WindowContext};
    pub struct State {
        pub width: u32,
        pub height: u32,
        pub context: sdl2::Sdl,
        pub canvas: Canvas<Window>,
        // Unsafe: texture must be dropped before texture_creator.
        // Struct fields are dropped in the order they are defined,
        // so texture must come before texture_creator in the struct definition.
        color_buffer_texture: Box<Texture<'static>>,
        texture_creator: std::pin::Pin<Box<TextureCreator<WindowContext>>>,
    }

    impl State {
        pub fn new(width: u32, height: u32) -> Self {
            use std::ptr::addr_of_mut;

            let mut result = std::mem::MaybeUninit::<Self>::uninit();
            let rp = result.as_mut_ptr();
            unsafe {
                addr_of_mut!((*rp).width).write(width);
                addr_of_mut!((*rp).height).write(height);
                addr_of_mut!((*rp).context).write(sdl2::init().unwrap());
                let window = (*rp)
                    .context
                    .video()
                    .unwrap()
                    .window("Renderer", width, height)
                    .borderless()
                    .maximized()
                    .build()
                    .unwrap();
                addr_of_mut!((*rp).canvas).write(window.into_canvas().build().unwrap());
                addr_of_mut!((*rp).texture_creator).write(Box::pin((*rp).canvas.texture_creator()));
                addr_of_mut!((*rp).color_buffer_texture).write(Box::new(
                    (*rp)
                        .texture_creator
                        .create_texture(
                            (*rp).texture_creator.default_pixel_format(),
                            TextureAccess::Streaming,
                            width,
                            height,
                        )
                        .unwrap(),
                ));
                result.assume_init()
            }
        }

        pub fn set_pixel(&mut self, x: u32, y: u32, c: Color) {
            let soc: usize = std::mem::size_of::<Color>();
            let width_u: usize = self.width.try_into().unwrap();
            let pitch: usize = soc * width_u;

            let cb: &[u8] = &c
                .to_u32(
                    &PixelFormat::try_from(self.texture_creator.default_pixel_format()).unwrap(),
                )
                .to_ne_bytes();

            self.color_buffer_texture
                .update(
                    sdl2::rect::Rect::new(x.try_into().unwrap(), y.try_into().unwrap(), 1, 1),
                    cb,
                    pitch,
                )
                .unwrap();
        }

        pub fn render(&mut self) {
            use rand::Rng;
            self.canvas.set_draw_color(Color::RGB(255, 0, 255));
            self.canvas.clear();

            let mut rng = rand::thread_rng();
            let rx = rng.gen_range(0..self.width);
            let ry = rng.gen_range(0..self.height);
            let rr = rng.gen_range(0..255);
            let rg = rng.gen_range(0..255);
            let rb = rng.gen_range(0..255);
            self.set_pixel(rx, ry, Color::RGB(rr, rg, rb));

            self.canvas
                .copy(&self.color_buffer_texture, None, None)
                .unwrap();

            self.canvas.present();
        }
    }
}

fn main() {
    use state::State;

    let width = 3440 / 2;
    let height = 1440 / 2;
    let mut state = State::new(width, height);
    state.render();
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
        state.render();
    }
}
