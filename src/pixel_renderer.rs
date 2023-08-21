// Unsafe: TextureCreator and Texture are related.
// color_texture needs to be dropped before texture_creator.
// The plan is to keep both of them together in this struct,
// and eventually drop them together.
use sdl2::pixels::{Color, PixelFormat};
use sdl2::render::{Canvas, Texture, TextureAccess, TextureCreator};
use sdl2::video::{Window, WindowContext};

pub struct PixelRenderer {
    pub width: u32,
    pub height: u32,
    pub context: sdl2::Sdl,
    pub canvas: Canvas<Window>,
    color_buffer: Box<[u8]>,
    // Unsafe: color_texture must be dropped before texture_creator.
    // We will handle this in the drop trait.
    color_texture: *mut Texture<'static>,
    texture_creator: *mut TextureCreator<WindowContext>,
}

impl Drop for PixelRenderer {
    fn drop(&mut self) {
        // Unsafe: color_texture must be dropped before texture_creator.
        unsafe {
            drop(Box::from_raw(&mut *self.color_texture));
            drop(Box::from_raw(&mut *self.texture_creator));
        }
    }
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
        // Unsafe: We will manage the life of texture_creator and color_texture ourselves.
        // We will keep them together in this struct and eventually drop them together.
        let texture_creator: *mut TextureCreator<WindowContext> =
            Box::into_raw(Box::new(canvas.texture_creator()));
        let color_texture: *mut Texture<'static> = Box::into_raw(Box::new(
            unsafe { &*texture_creator }
                .create_texture(
                    unsafe { &*texture_creator }.default_pixel_format(),
                    TextureAccess::Streaming,
                    width,
                    height,
                )
                .unwrap(),
        ));
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
                &PixelFormat::try_from(unsafe { &*self.texture_creator }.default_pixel_format())
                    .unwrap(),
            )
            .to_ne_bytes();

        self.color_buffer[i..i + size_of_color].copy_from_slice(color_bytes);
    }

    pub fn render(&mut self) {
        let size_of_color: usize = std::mem::size_of::<Color>();
        let width: usize = self.width.try_into().unwrap();
        let pitch: usize = width * size_of_color;

        unsafe { &mut *self.color_texture }
            .update(None, &self.color_buffer, pitch)
            .unwrap();

        self.canvas
            .copy(unsafe { &*self.color_texture }, None, None)
            .unwrap();

        self.canvas.present();
    }
}
