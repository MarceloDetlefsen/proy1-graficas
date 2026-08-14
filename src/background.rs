use raylib::prelude::*;

use crate::colors::{NBA_NAVY, NBA_ORANGE};
use crate::textures::TextureManager;

pub fn draw_screen_background(
    draw: &mut RaylibDrawHandle<'_>,
    texture_manager: &TextureManager,
    screen_width: u32,
    screen_height: u32,
    _elapsed_time: f32,
) {
    draw.draw_rectangle_gradient_v(
        0,
        0,
        screen_width as i32,
        screen_height as i32,
        Color::new(10, 14, 35, 255),
        NBA_NAVY,
    );

    draw_logo_collage(draw, texture_manager, screen_width, screen_height);
    draw_court_lines(draw, screen_width, screen_height);
}

fn draw_logo_collage(
    draw: &mut RaylibDrawHandle<'_>,
    texture_manager: &TextureManager,
    screen_width: u32,
    screen_height: u32,
) {
    let wall_count = texture_manager.wall_count();
    if wall_count == 0 {
        return;
    }

    // Posiciones dispersas y fijas para evitar titileo entre frames.
    let layout = [
        (0.10, 0.12, -14.0),
        (0.31, 0.08, 9.0),
        (0.54, 0.15, 18.0),
        (0.79, 0.10, -10.0),
        (0.88, 0.28, 13.0),
        (0.16, 0.34, -16.0),
        (0.38, 0.29, 6.0),
        (0.62, 0.33, -8.0),
        (0.83, 0.47, 15.0),
        (0.25, 0.50, -12.0),
    ];

    for (index, (nx, ny, rotation)) in layout.iter().copied().enumerate() {
        let texture_index = (index * 3) % wall_count;
        let Some(texture) = texture_manager.wall_texture_gpu(texture_index) else {
            continue;
        };

        let target_size = 80.0 + (index % 3) as f32 * 4.0;
        let texture_size = texture.width().max(texture.height()).max(1) as f32;
        let scale = target_size / texture_size;
        let x = screen_width as f32 * nx - target_size * 0.5;
        let y = screen_height as f32 * ny - target_size * 0.5;

        draw.draw_texture_ex(
            texture,
            Vector2::new(x, y),
            rotation,
            scale,
            Color::new(255, 255, 255, 40),
        );
    }
}

fn draw_court_lines(draw: &mut RaylibDrawHandle<'_>, screen_width: u32, screen_height: u32) {
    let center_x = screen_width as f32 * 0.5;
    let center_y = screen_height as f32 * 0.52;
    let line_color = Color::new(NBA_ORANGE.r, NBA_ORANGE.g, NBA_ORANGE.b, 72);

    draw.draw_circle_lines(center_x as i32, center_y as i32, 150.0, line_color);

    let arc_center_x = screen_width as f32 * 0.5;
    let arc_center_y = screen_height as f32 + 78.0;
    let arc_radius = screen_width.min(screen_height) as f32 * 0.46;
    draw_arc_line(draw, arc_center_x, arc_center_y, arc_radius, 205.0, 335.0, line_color);
}

fn draw_arc_line(
    draw: &mut RaylibDrawHandle<'_>,
    center_x: f32,
    center_y: f32,
    radius: f32,
    start_angle_deg: f32,
    end_angle_deg: f32,
    color: Color,
) {
    let segments = 40;
    let start = start_angle_deg.to_radians();
    let end = end_angle_deg.to_radians();
    let step = (end - start) / segments as f32;

    let mut previous = None;
    for segment in 0..=segments {
        let angle = start + step * segment as f32;
        let x = center_x + radius * angle.cos();
        let y = center_y + radius * angle.sin();
        let current = (x.round() as i32, y.round() as i32);

        if let Some((prev_x, prev_y)) = previous {
            draw.draw_line(prev_x, prev_y, current.0, current.1, color);
        }

        previous = Some(current);
    }
}
