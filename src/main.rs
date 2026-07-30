mod framebuffer;
mod line;

use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use framebuffer::Framebuffer;
use line::line;
use raylib::prelude::*;

fn render(framebuffer: &mut Framebuffer, translate_x: f32, translate_y: f32) {
    framebuffer.set_current_color(Color::GREEN);
    line(
        framebuffer,
        Vector2::new(50.0 + translate_x, 50.0 + translate_y),
        Vector2::new(350.0 + translate_x, 350.0 + translate_y),
    );

    framebuffer.set_current_color(Color::RED);
    line(
        framebuffer,
        Vector2::new(350.0 + translate_x, 50.0 + translate_y),
        Vector2::new(50.0 + translate_x, 350.0 + translate_y),
    );
}

fn screenshot_name() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();

    format!("framebuffer_{timestamp}.png")
}

fn main() {
    let window_width = 800;
    let window_height = 600;

    let framebuffer_width = 800;
    let framebuffer_height = 600;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Window Example")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(
        framebuffer_width,
        framebuffer_height,
        Color::new(50, 50, 100, 255),
    );

    let mut translate_x: f32 = 0.0;
    let mut translate_y: f32 = 0.0;
    let mut velocity_x: f32 = 1.0;
    let mut velocity_y: f32 = 1.0;

    let min_x = 0.0;
    let min_y = 0.0;
    let max_x = framebuffer_width as f32 - 350.0;
    let max_y = framebuffer_height as f32 - 350.0;

    while !window.window_should_close() {
        translate_x += velocity_x;
        translate_y += velocity_y;

        if translate_x <= min_x {
            translate_x = min_x;
            velocity_x = velocity_x.abs();
        } else if translate_x >= max_x {
            translate_x = max_x;
            velocity_x = -velocity_x.abs();
        }

        if translate_y <= min_y {
            translate_y = min_y;
            velocity_y = velocity_y.abs();
        } else if translate_y >= max_y {
            translate_y = max_y;
            velocity_y = -velocity_y.abs();
        }

        framebuffer.clear();
        render(&mut framebuffer, translate_x, translate_y);

        if window.is_key_pressed(KeyboardKey::KEY_S) {
            let filename = screenshot_name();
            framebuffer.render_to_file(&filename);
        }

        framebuffer.swap_buffers(&mut window, &raylib_thread);

        thread::sleep(Duration::from_millis(16));
    }
}
