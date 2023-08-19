use pixel_renderer::PixelRenderer;
use sdl2::pixels::Color;

mod pixel_renderer {
    // Unsafe: TextureCreator and Texture are related, the sdl2 docs aren't 100% clear on how.
    // color_texture needs to be dropped before texture_creator.
    // The plan is to keep both of them together in this struct, and to keep them private so
    // that the unsafe behavior doesn't leak outside of this module.
    use sdl2::pixels::{Color, PixelFormat};
    use sdl2::render::{Canvas, Texture, TextureAccess, TextureCreator};
    use sdl2::video::{Window, WindowContext};

    pub struct PixelRenderer {
        pub width: u32,
        pub height: u32,
        pub context: sdl2::Sdl,
        pub canvas: Canvas<Window>,
        // Unsafe: color_texture must be dropped before texture_creator.
        // Struct fields are dropped in the order they are defined,
        // so color_texture must come before texture_creator in the struct definition.
        color_buffer: Box<[u8]>,
        color_texture: Box<Texture<'static>>,
        texture_creator: Box<TextureCreator<WindowContext>>,
    }

    impl PixelRenderer {
        pub fn new(width: u32, height: u32) -> Self {
            let context = sdl2::init().unwrap();
            let window = context
                .video()
                .unwrap()
                .window("Renderer", width, height)
                .borderless()
                .maximized()
                .build()
                .unwrap();
            let canvas = window
                .into_canvas()
                .accelerated()
                .present_vsync()
                .build()
                .unwrap();
            let pixel_count: usize = (width * height).try_into().unwrap();
            let color_buffer: Box<[u8]> =
                vec![0u8; pixel_count * std::mem::size_of::<Color>()].into_boxed_slice();
            let texture_creator: Box<TextureCreator<WindowContext>> =
                Box::new(canvas.texture_creator());
            // Unsafe: We cast through a raw pointer to forget the borrow.
            // We will ensure the borrow doesn't live too long ourselves by keeping
            // texture_creator and color_texture together in a struct.
            // They will live together and drop together.
            let texture_creator_forgotten_borrow: &TextureCreator<WindowContext> =
                unsafe { &*(&*texture_creator as *const _) };
            let color_texture = Box::new(
                texture_creator_forgotten_borrow
                    .create_texture(
                        texture_creator_forgotten_borrow.default_pixel_format(),
                        TextureAccess::Streaming,
                        width,
                        height,
                    )
                    .unwrap(),
            );
            Self {
                width,
                height,
                context,
                canvas,
                color_buffer,
                texture_creator,
                color_texture,
            }
        }

        pub fn set_pixel(&mut self, x: u32, y: u32, c: Color) {
            assert!(x < self.width);
            assert!(y < self.height);
            let size_of_color: usize = std::mem::size_of::<Color>();
            let width: usize = self.width.try_into().unwrap();
            let x: usize = x.try_into().unwrap();
            let y: usize = y.try_into().unwrap();
            let i = ((y * width) + x) * size_of_color;

            // Turn the color into bytes. The correct bytes for a color depend on
            // the pixel format and the system endianness.
            let color_bytes: &[u8; 4] = &c
                .to_u32(
                    &PixelFormat::try_from(self.texture_creator.default_pixel_format()).unwrap(),
                )
                .to_ne_bytes();

            self.color_buffer[i..i + size_of_color].copy_from_slice(color_bytes);
        }

        pub fn render(&mut self) {
            let size_of_color: usize = std::mem::size_of::<Color>();
            let width: usize = self.width.try_into().unwrap();
            let pitch: usize = width * size_of_color;

            self.color_texture
                .update(None, &self.color_buffer, pitch)
                .unwrap();

            self.canvas.copy(&self.color_texture, None, None).unwrap();

            self.canvas.present();
        }
    }
}

fn draw_grid(pixel_renderer: &mut PixelRenderer) {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    for x in 0..pixel_renderer.width {
        for y in 0..pixel_renderer.height {
            assert!(x < pixel_renderer.width);
            assert!(y < pixel_renderer.height);
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
