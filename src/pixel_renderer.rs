// Unsafe: TextureCreator and Texture are related.
// color_texture needs to be dropped before texture_creator.
// The plan is to keep both of them together in this struct,
// and eventually drop them together.
use sdl2::pixels::{Color, PixelFormat, PixelFormatEnum};
use sdl2::render::{Canvas, Texture, TextureAccess, TextureCreator};
use sdl2::video::{Window, WindowContext};

const SIZE_OF_COLOR: usize = std::mem::size_of::<Color>();

pub struct PixelRenderer {
    pub width: u32,
    pub height: u32,
    pub context: sdl2::Sdl,
    pub canvas: Canvas<Window>,
    pub color_buffer: Box<[u8]>,
    pub z_buffer: Box<[f32]>,
    pub pixel_format: PixelFormat,
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
        PixelRenderer::new_buffers_and_textures(width, height, context, canvas)
    }

    pub fn new_for_benchmark(width: u32, height: u32) -> Self {
        let context = sdl2::init().unwrap();
        let window = context
            .video()
            .unwrap()
            .window("Renderer", width, height)
            .hidden()
            .build()
            .unwrap();
        let canvas = window.into_canvas().accelerated().build().unwrap();
        PixelRenderer::new_buffers_and_textures(width, height, context, canvas)
    }

    fn new_buffers_and_textures(
        width: u32,
        height: u32,
        context: sdl2::Sdl,
        canvas: Canvas<Window>,
    ) -> Self {
        let pixel_count: usize = (width * height) as usize;
        let color_buffer: Box<[u8]> = vec![0u8; pixel_count * SIZE_OF_COLOR].into_boxed_slice();
        let z_buffer: Box<[f32]> = vec![f32::INFINITY; pixel_count].into_boxed_slice();
        // Unsafe: We will manage the life of texture_creator and color_texture ourselves.
        // We will keep them together in this struct and eventually drop them together.
        let texture_creator: *mut TextureCreator<WindowContext> =
            Box::into_raw(Box::new(canvas.texture_creator()));
        let pixel_format_enum: PixelFormatEnum =
            unsafe { &*texture_creator }.default_pixel_format();
        let pixel_format: PixelFormat = pixel_format_enum.try_into().unwrap();
        let color_texture: *mut Texture<'static> = Box::into_raw(Box::new(
            unsafe { &*texture_creator }
                .create_texture(pixel_format_enum, TextureAccess::Streaming, width, height)
                .unwrap(),
        ));
        let mouse = context.mouse();
        mouse.set_relative_mouse_mode(true);
        Self {
            width,
            height,
            context,
            canvas,
            color_buffer,
            z_buffer,
            pixel_format,
            texture_creator,
            color_texture,
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        if x >= self.width || y >= self.height {
            return;
        }

        let i = ((y * self.width) + x) as usize * SIZE_OF_COLOR;

        // Turn the color into bytes. The correct bytes for a color depend on
        // the pixel format and the system endianness.
        let color_bytes: &[u8; SIZE_OF_COLOR] = &color.to_u32(&self.pixel_format).to_ne_bytes();

        self.color_buffer[i..i + SIZE_OF_COLOR].copy_from_slice(color_bytes);
    }

    pub fn set_pixel_z(&mut self, x: u32, y: u32, z: f32, color: Color) {
        if x >= self.width || y >= self.height {
            return;
        }

        let i = ((y * self.width) + x) as usize;

        if z >= self.z_buffer[i] {
            return;
        } else {
            self.z_buffer[i] = z;
        }

        let i_color = i * SIZE_OF_COLOR;

        // Turn the color into bytes. The correct bytes for a color depend on
        // the pixel format and the system endianness.
        let color_bytes: &[u8; SIZE_OF_COLOR] = &color.to_u32(&self.pixel_format).to_ne_bytes();

        self.color_buffer[i_color..i_color + SIZE_OF_COLOR].copy_from_slice(color_bytes);
    }

    pub fn clear_pixels(&mut self, color: Color) {
        let color_bytes: &[u8; SIZE_OF_COLOR] = &color.to_u32(&self.pixel_format).to_ne_bytes();

        let width: usize = self.width as usize;
        let height: usize = self.height as usize;

        for y in 0_usize..height {
            for x in 0_usize..width {
                let i: usize = (y * width) + x;
                self.z_buffer[i] = f32::INFINITY;
                let i_color: usize = i * SIZE_OF_COLOR;
                self.color_buffer[i_color..i_color + SIZE_OF_COLOR].copy_from_slice(color_bytes);
            }
        }
    }

    pub fn render(&mut self) {
        let pitch: usize = self.width as usize * SIZE_OF_COLOR;

        unsafe { &mut *self.color_texture }
            .update(None, &self.color_buffer, pitch)
            .unwrap();

        self.canvas
            .copy(unsafe { &*self.color_texture }, None, None)
            .unwrap();

        self.canvas.present();
    }
}
