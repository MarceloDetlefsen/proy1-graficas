use raylib::prelude::*;
use raylib::texture::Image;

/// Same Framebuffer used in the polygon-fill / Game of Life labs, extended
/// with a `draw_rect` helper so the raycaster can fill ceiling/floor/wall
/// columns without looping pixel-by-pixel in Rust (the Image drawing calls
/// are done in raylib's C side, which is much faster for big solid blocks).
pub struct Framebuffer {
    color_buffer: Image,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32, background_color: Color) -> Self {
        let color_buffer = Image::gen_image_color(width as i32, height as i32, background_color);

        Self { color_buffer }
    }

    /// Fills a rectangle directly on the image buffer. Used for ceiling,
    /// floor, and each vertical wall "column" the raycaster produces.
    pub fn draw_rect(&mut self, x: i32, y: i32, width: i32, height: i32, color: Color) {
        if width <= 0 || height <= 0 {
            return;
        }
        self.color_buffer.draw_rectangle(x, y, width, height, color);
    }

    pub fn swap_buffers(&self, window: &mut RaylibHandle, raylib_thread: &RaylibThread) {
        if let Ok(texture) = window.load_texture_from_image(raylib_thread, &self.color_buffer) {
            let mut renderer = window.begin_drawing(raylib_thread);
            renderer.draw_texture(&texture, 0, 0, Color::WHITE);
        }
    }
}
