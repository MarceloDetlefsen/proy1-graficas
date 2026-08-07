/// Render pseudo-3D con techo, piso y paredes texturizadas.

use raylib::prelude::*;

use crate::colors::{NBA_CREAM, NBA_ORANGE};
use crate::framebuffer::Framebuffer;
use crate::map::MapGrid;
use crate::player::Player;
use crate::raycaster::{cast_ray, RayHit};
use crate::textures::TextureManager;

const FOV: f32 = std::f32::consts::PI / 3.0; // 60 grados, tipico de Wolfenstein 3D

const CEILING_COLOR: Color = Color::new(40, 40, 70, 255);
const MINIMAP_BG_COLOR: Color = Color::new(20, 30, 70, 180);
const MINIMAP_WALL_COLOR: Color = Color::new(210, 210, 210, 255);
const MINIMAP_HOOP_COLOR: Color = Color::new(255, 140, 0, 255);
const MINIMAP_PLAYER_COLOR: Color = Color::new(255, 70, 70, 255);
const MINIMAP_FACING_COLOR: Color = Color::new(255, 220, 90, 255);
const MINIMAP_CELL_SIZE: i32 = 4;
const MINIMAP_MAX_SIZE: i32 = 180;
const MINIMAP_PADDING: i32 = 8;
const MINIMAP_BORDER: i32 = 2;

pub fn render(
    framebuffer: &mut Framebuffer,
    player: &Player,
    map: &MapGrid,
    hoop_positions: &[(f32, f32)],
    texture_manager: &TextureManager,
    screen_width: u32,
    screen_height: u32,
    show_minimap: bool,
) {
    let half_height = (screen_height / 2) as i32;

    framebuffer.draw_rect(0, 0, screen_width as i32, half_height, CEILING_COLOR);
    draw_floor(framebuffer, player, texture_manager, screen_width, screen_height);

    for x in 0..screen_width {
        let camera_x = 2.0 * x as f32 / screen_width as f32 - 1.0;
        let ray_angle = player.angle + camera_x * (FOV / 2.0);
        let hit = cast_ray(map, player, ray_angle);

        draw_wall_column(
            framebuffer,
            map,
            player,
            ray_angle,
            &hit,
            texture_manager,
            x as i32,
            screen_height as i32,
        );
    }

    if show_minimap {
        draw_minimap(framebuffer, map, player, hoop_positions);
    }
}

pub fn draw_hud(
    draw: &mut RaylibDrawHandle<'_>,
    screen_width: u32,
    level_number: u32,
    level_name: &str,
    hoops_scored: usize,
    hoops_required: usize,
) {
    let panel_margin = 10;
    let panel_padding = 12;
    let panel_height = 62;
    let level_text = format!("Nivel {} - {}", level_number, level_name);
    let score_text = format!("{hoops_scored}/{hoops_required}");
    let level_font_size = 18;
    let score_font_size = 26;
    let level_width = draw.measure_text(&level_text, level_font_size);
    let score_width = draw.measure_text(&score_text, score_font_size);
    let panel_width = level_width.max(score_width) + panel_padding * 2;
    let panel_x = screen_width as i32 - panel_margin - panel_width;
    let panel_y = panel_margin;

    draw.draw_rectangle(panel_x, panel_y, panel_width, panel_height, NBA_ORANGE);
    draw.draw_rectangle(
        panel_x + 2,
        panel_y + 2,
        panel_width - 4,
        panel_height - 4,
        Color::new(20, 30, 70, 200),
    );

    draw.draw_text(
        &level_text,
        panel_x + panel_padding,
        panel_y + 8,
        level_font_size,
        NBA_CREAM,
    );

    let score_x = panel_x + panel_padding;
    let score_y = panel_y + 28;
    draw.draw_text(
        &score_text,
        score_x + 1,
        score_y + 1,
        score_font_size,
        Color::new(10, 10, 10, 120),
    );
    draw.draw_text(
        &score_text,
        score_x,
        score_y,
        score_font_size,
        NBA_ORANGE,
    );
}

fn draw_floor(
    framebuffer: &mut Framebuffer,
    player: &Player,
    texture_manager: &TextureManager,
    screen_width: u32,
    screen_height: u32,
) {
    let half_height = (screen_height / 2) as i32;
    let ray_dir_x0 = (player.angle - FOV / 2.0).cos();
    let ray_dir_y0 = (player.angle - FOV / 2.0).sin();
    let ray_dir_x1 = (player.angle + FOV / 2.0).cos();
    let ray_dir_y1 = (player.angle + FOV / 2.0).sin();
    let (floor_tex_w, floor_tex_h) = texture_manager.floor_dimensions();

    if floor_tex_w <= 0 || floor_tex_h <= 0 {
        return;
    }

    let screen_width_f = screen_width as f32;

    for y in half_height..screen_height as i32 {
        let p = (y - half_height) as f32;
        if p <= 0.0 {
            continue;
        }

        let row_distance = (0.5 * screen_height as f32) / p;
        let floor_step_x = row_distance * (ray_dir_x1 - ray_dir_x0) / screen_width_f;
        let floor_step_y = row_distance * (ray_dir_y1 - ray_dir_y0) / screen_width_f;
        let mut floor_x = player.x + row_distance * ray_dir_x0;
        let mut floor_y = player.y + row_distance * ray_dir_y0;

        for x in 0..screen_width as i32 {
            let tex_x = ((floor_x.rem_euclid(1.0) * floor_tex_w as f32).floor() as i32)
                .clamp(0, floor_tex_w - 1) as u32;
            let tex_y = ((floor_y.rem_euclid(1.0) * floor_tex_h as f32).floor() as i32)
                .clamp(0, floor_tex_h - 1) as u32;
            let color = texture_manager.floor_pixel_color(tex_x, tex_y);

            framebuffer.set_current_color(color);
            framebuffer.set_pixel(x, y);

            floor_x += floor_step_x;
            floor_y += floor_step_y;
        }
    }
}

fn draw_wall_column(
    framebuffer: &mut Framebuffer,
    map: &MapGrid,
    player: &Player,
    ray_angle: f32,
    hit: &RayHit,
    texture_manager: &TextureManager,
    screen_x: i32,
    screen_height: i32,
) {
    let distance = hit.distance.max(0.0001);
    let wall_height = (screen_height as f32 / distance) as i32;
    let half_height = screen_height / 2;
    let draw_start = (-wall_height / 2 + half_height).max(0);
    let draw_end = (wall_height / 2 + half_height).min(screen_height);

    if draw_end <= draw_start {
        return;
    }

    let texture_count = texture_manager.wall_count();
    if texture_count == 0 {
        return;
    }

    let texture_index = (map.wall_texture_index(hit.map_x, hit.map_y) - 1) % texture_count;

    let Some((tex_w, tex_h)) = texture_manager.wall_dimensions(texture_index) else {
        return;
    };

    let ray_dir_x = ray_angle.cos();
    let ray_dir_y = ray_angle.sin();
    let wall_coord = if hit.vertical_wall {
        player.y + distance * ray_dir_y
    } else {
        player.x + distance * ray_dir_x
    };

    let mut tex_x = ((wall_coord.rem_euclid(1.0) * tex_w as f32).floor() as i32).clamp(0, tex_w - 1);

    // Ajuste de orientacion para que el lado visible de la pared no quede espejado.
    if hit.vertical_wall && ray_dir_x < 0.0 {
        tex_x = tex_w - tex_x - 1;
    }
    if !hit.vertical_wall && ray_dir_y > 0.0 {
        tex_x = tex_w - tex_x - 1;
    }

    let visible_height = draw_end - draw_start;
    if visible_height <= 0 {
        return;
    }

    for y in draw_start..draw_end {
        let tex_y = (((y - draw_start) * tex_h) / visible_height).clamp(0, tex_h - 1) as u32;
        let color = texture_manager.wall_pixel_color(texture_index, tex_x as u32, tex_y);

        if color.a == 0 {
            continue;
        }

        framebuffer.set_current_color(color);
        framebuffer.set_pixel(screen_x, y);
    }
}

fn draw_minimap(
    framebuffer: &mut Framebuffer,
    map: &MapGrid,
    player: &Player,
    hoop_positions: &[(f32, f32)],
) {
    let cell_size = minimap_cell_size(map);
    let map_width_px = map.width as i32 * cell_size;
    let map_height_px = map.height as i32 * cell_size;
    let origin_x = MINIMAP_PADDING;
    let origin_y = MINIMAP_PADDING;

    framebuffer.draw_rect(
        origin_x - MINIMAP_BORDER,
        origin_y - MINIMAP_BORDER,
        map_width_px + MINIMAP_BORDER * 2,
        map_height_px + MINIMAP_BORDER * 2,
        NBA_ORANGE,
    );
    framebuffer.draw_rect(origin_x, origin_y, map_width_px, map_height_px, MINIMAP_BG_COLOR);

    for y in 0..map.height {
        for x in 0..map.width {
            if map.get(x, y) == 1 {
                framebuffer.draw_rect(
                    origin_x + x as i32 * cell_size,
                    origin_y + y as i32 * cell_size,
                    cell_size,
                    cell_size,
                    MINIMAP_WALL_COLOR,
                );
            }
        }
    }

    for &(hoop_x, hoop_y) in hoop_positions {
        let px = origin_x + (hoop_x * cell_size as f32) as i32;
        let py = origin_y + (hoop_y * cell_size as f32) as i32;
        let hoop_size = (cell_size / 2).max(2);
        framebuffer.draw_rect(
            px - hoop_size / 2,
            py - hoop_size / 2,
            hoop_size,
            hoop_size,
            MINIMAP_HOOP_COLOR,
        );
    }

    let player_x = origin_x + (player.x * cell_size as f32) as i32;
    let player_y = origin_y + (player.y * cell_size as f32) as i32;

    framebuffer.draw_rect(player_x - 2, player_y - 2, 4, 4, MINIMAP_PLAYER_COLOR);

    let dir_length = 8.0;
    let dir_x = player_x + (player.angle.cos() * dir_length) as i32;
    let dir_y = player_y + (player.angle.sin() * dir_length) as i32;
    framebuffer.draw_line(player_x, player_y, dir_x, dir_y, MINIMAP_FACING_COLOR);
}

fn minimap_cell_size(map: &MapGrid) -> i32 {
    let max_dim = map.width.max(map.height).max(1) as i32;
    (MINIMAP_MAX_SIZE / max_dim).clamp(2, MINIMAP_CELL_SIZE)
}
