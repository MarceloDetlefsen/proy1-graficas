use raylib::prelude::*;
use raylib::texture::Image;

/// El mismo framebuffer usado en los laboratorios de polygon-fill / Game of Life, extendido con helpers para rectangulos y pixeles individuales.
pub struct Framebuffer {
    color_buffer: Image,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32, background_color: Color) -> Self {
        let color_buffer = Image::gen_image_color(width as i32, height as i32, background_color);

        Self {
            color_buffer,
            current_color: background_color,
        }
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn set_pixel(&mut self, x: i32, y: i32) {
        if x < 0 || y < 0 || x >= self.color_buffer.width() || y >= self.color_buffer.height() {
            return;
        }

        self.color_buffer.draw_pixel(x, y, self.current_color);
    }

    /// Rellena un rectangulo directamente sobre el buffer de imagen.
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
