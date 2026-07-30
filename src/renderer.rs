// Fase 4: render pseudo-3D.
//
// Sin texturas, sin iluminacion, sin sprites. Solo tres colores solidos:
// techo, piso, y columnas verticales para las paredes, escaladas segun
// la distancia que devolvio el raycaster.

use raylib::prelude::*;

use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::raycaster::cast_ray;

const FOV: f32 = std::f32::consts::PI / 3.0; // 60 grados, tipico de Wolfenstein 3D

const CEILING_COLOR: Color = Color::new(40, 40, 70, 255);
const FLOOR_COLOR: Color = Color::new(55, 55, 55, 255);
const WALL_COLOR_VERTICAL: Color = Color::new(255, 140, 0, 255); // pared golpeada en eje X (naranja)
const WALL_COLOR_HORIZONTAL: Color = Color::new(0, 120, 255, 255); // pared golpeada en eje Y (azul)

pub fn render(framebuffer: &mut Framebuffer, player: &Player, screen_width: u32, screen_height: u32) {
    let half_height = (screen_height / 2) as i32;

    // Techo y piso: dos rectangulos solidos.
    framebuffer.draw_rect(0, 0, screen_width as i32, half_height, CEILING_COLOR);
    framebuffer.draw_rect(
        0,
        half_height,
        screen_width as i32,
        screen_height as i32 - half_height,
        FLOOR_COLOR,
    );

    // Un rayo por columna de pantalla.
    for x in 0..screen_width {
        // camera_x va de -1.0 (borde izquierdo) a 1.0 (borde derecho).
        let camera_x = 2.0 * x as f32 / screen_width as f32 - 1.0;
        let ray_angle = player.angle + camera_x * (FOV / 2.0);

        let hit = cast_ray(player, ray_angle);

        // La correccion de fish-eye ya viene aplicada en cast_ray porque
        // devuelve la distancia perpendicular al plano de la camara, no
        // la distancia en linea recta desde el jugador al muro.
        let distance = hit.distance.max(0.0001);

        let wall_height = (screen_height as f32 / distance) as i32;

        let draw_start = (-wall_height / 2 + half_height).max(0);
        let draw_end = (wall_height / 2 + half_height).min(screen_height as i32);

        let color = if hit.vertical_wall {
            WALL_COLOR_VERTICAL
        } else {
            WALL_COLOR_HORIZONTAL
        };

        framebuffer.draw_rect(x as i32, draw_start, 1, draw_end - draw_start, color);
    }
}
