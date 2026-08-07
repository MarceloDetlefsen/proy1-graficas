/// Render pseudo-3D con techo, piso y paredes texturizadas.

use raylib::prelude::*;

use crate::framebuffer::Framebuffer;
use crate::map::MapGrid;
use crate::player::Player;
use crate::raycaster::{cast_ray, RayHit};
use crate::textures::TextureManager;

const FOV: f32 = std::f32::consts::PI / 3.0; // 60 grados, tipico de Wolfenstein 3D

const CEILING_COLOR: Color = Color::new(40, 40, 70, 255);
const FLOOR_COLOR: Color = Color::new(55, 55, 55, 255);
const MINIMAP_BG_COLOR: Color = Color::new(18, 18, 22, 255);
const MINIMAP_WALL_COLOR: Color = Color::new(210, 210, 210, 255);
const MINIMAP_PLAYER_COLOR: Color = Color::new(255, 70, 70, 255);
const MINIMAP_FACING_COLOR: Color = Color::new(255, 220, 90, 255);
const MINIMAP_CELL_SIZE: i32 = 8;
const MINIMAP_PADDING: i32 = 8;
const MINIMAP_BORDER: i32 = 2;

pub fn render(
    framebuffer: &mut Framebuffer,
    player: &Player,
    map: &MapGrid,
    texture_manager: &TextureManager,
    screen_width: u32,
    screen_height: u32,
    show_minimap: bool,
) {
    let half_height = (screen_height / 2) as i32;

    framebuffer.draw_rect(0, 0, screen_width as i32, half_height, CEILING_COLOR);
    framebuffer.draw_rect(
        0,
        half_height,
        screen_width as i32,
        screen_height as i32 - half_height,
        FLOOR_COLOR,
    );

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
        draw_minimap(framebuffer, map, player);
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

    let texture_index = map.wall_texture_index(hit.map_x, hit.map_y) - 1;
    let texture_index = texture_index % texture_count;

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

fn draw_minimap(framebuffer: &mut Framebuffer, map: &MapGrid, player: &Player) {
    let map_width_px = map.width as i32 * MINIMAP_CELL_SIZE;
    let map_height_px = map.height as i32 * MINIMAP_CELL_SIZE;
    let origin_x = MINIMAP_PADDING;
    let origin_y = MINIMAP_PADDING;

    framebuffer.draw_rect(
        origin_x - MINIMAP_BORDER,
        origin_y - MINIMAP_BORDER,
        map_width_px + MINIMAP_BORDER * 2,
        map_height_px + MINIMAP_BORDER * 2,
        Color::BLACK,
    );
    framebuffer.draw_rect(origin_x, origin_y, map_width_px, map_height_px, MINIMAP_BG_COLOR);

    for y in 0..map.height {
        for x in 0..map.width {
            if map.get(x, y) == 1 {
                framebuffer.draw_rect(
                    origin_x + x as i32 * MINIMAP_CELL_SIZE,
                    origin_y + y as i32 * MINIMAP_CELL_SIZE,
                    MINIMAP_CELL_SIZE,
                    MINIMAP_CELL_SIZE,
                    MINIMAP_WALL_COLOR,
                );
            }
        }
    }

    let player_x = origin_x + (player.x * MINIMAP_CELL_SIZE as f32) as i32;
    let player_y = origin_y + (player.y * MINIMAP_CELL_SIZE as f32) as i32;

    framebuffer.draw_rect(player_x - 2, player_y - 2, 4, 4, MINIMAP_PLAYER_COLOR);

    let dir_length = 10.0;
    let dir_x = player_x + (player.angle.cos() * dir_length) as i32;
    let dir_y = player_y + (player.angle.sin() * dir_length) as i32;
    framebuffer.draw_line(player_x, player_y, dir_x, dir_y, MINIMAP_FACING_COLOR);
}
