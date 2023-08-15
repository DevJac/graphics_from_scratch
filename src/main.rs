use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{TextureAccess, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::{pixels::Color, render::Canvas, render::Texture};

struct State<'a> {
    width: u32,
    height: u32,
    context: sdl2::Sdl,
    canvas: Canvas<Window>,
    color_buffer: Box<[Color]>,
    texture_creator: TextureCreator<WindowContext>,
    texture: Texture<'a>,
}

impl State<'_> {
    fn new(width: u32, height: u32) -> Self {
        let context = sdl2::init().unwrap();
        let window = context
            .video()
            .unwrap()
            .window(
                "Renderer",
                width.try_into().unwrap(),
                height.try_into().unwrap(),
            )
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        let color_buffer =
            vec![Color::RGB(0, 0, 0); (width * height).try_into().unwrap()].into_boxed_slice();
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator
            .create_texture(
                PixelFormatEnum::ARGB8888,
                TextureAccess::Streaming,
                width,
                height,
            )
            .unwrap();
        Self {
            width,
            height,
            context,
            canvas,
            color_buffer,
            texture_creator,
            texture,
        }
    }

    fn present(&mut self) {
        self.canvas.set_draw_color(Color::RGB(50, 100, 150));
        self.canvas.clear();
        self.canvas.present();
    }
}

fn main() {
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
