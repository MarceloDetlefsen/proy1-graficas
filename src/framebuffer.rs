use raylib::prelude::*;
use raylib::texture::Image;

/// El mismo framebuffer usado en los laboratorios de polygon-fill / Game of Life, extendido con un helper `draw_rect` para que el raycaster pueda llenar columnas de techo/suelo/pared sin recorrer píxel por píxel.
pub struct Framebuffer {
    color_buffer: Image,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32, background_color: Color) -> Self {
        let color_buffer = Image::gen_image_color(width as i32, height as i32, background_color);

        Self { color_buffer }
    }

    /// Rellena un rectángulo directamente sobre el buffer de imagen. Se usa para el techo, el suelo y cada "columna" vertical de pared que produce el raycaster.
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
