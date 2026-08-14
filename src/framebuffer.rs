use raylib::prelude::*;
use raylib::texture::Image;
use raylib::core::texture::RaylibTexture2D;

/// Framebuffer CPU con una textura GPU persistente para evitar recrear uploads por frame.
pub struct Framebuffer {
    color_buffer: Image,
    texture: Texture2D,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(
        window: &mut RaylibHandle,
        raylib_thread: &RaylibThread,
        width: u32,
        height: u32,
        background_color: Color,
    ) -> Self {
        let color_buffer = Image::gen_image_color(width as i32, height as i32, background_color);
        let texture = window
            .load_texture_from_image(raylib_thread, &color_buffer)
            .expect("failed to create framebuffer texture");

        Self {
            color_buffer,
            texture,
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

    pub fn draw_line(&mut self, start_x: i32, start_y: i32, end_x: i32, end_y: i32, color: Color) {
        self.color_buffer
            .draw_line(start_x, start_y, end_x, end_y, color);
    }

    pub fn clear(&mut self, color: Color) {
        self.color_buffer
            .draw_rectangle(0, 0, self.color_buffer.width(), self.color_buffer.height(), color);
    }

    pub fn swap_buffers<F>(&mut self, window: &mut RaylibHandle, raylib_thread: &RaylibThread, overlay: F)
    where
        F: FnOnce(&mut RaylibDrawHandle<'_>),
    {
        let pixels = self.color_buffer.get_image_data_u8(false);
        let _ = self.texture.update_texture(&pixels);

        window.draw(raylib_thread, |mut renderer| {
            let screen_width = renderer.get_screen_width() as f32;
            let screen_height = renderer.get_screen_height() as f32;
            let source = Rectangle::new(
                0.0,
                0.0,
                self.color_buffer.width() as f32,
                self.color_buffer.height() as f32,
            );
            let destination = Rectangle::new(0.0, 0.0, screen_width, screen_height);
            renderer.draw_texture_pro(
                &self.texture,
                source,
                destination,
                Vector2::new(0.0, 0.0),
                0.0,
                Color::WHITE,
            );

            overlay(&mut renderer);
        });
    }
}
