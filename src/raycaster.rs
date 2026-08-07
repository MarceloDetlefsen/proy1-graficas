/// Raycasting clasico usando el algoritmo DDA el mismo enfoque que usa Wolfenstein 3D / lodev's raycasting tutorial. 
/// Para cada rayo avanzamos celda por celda, siempre saltando a la siguiente linea de grid (vertical u horizontal) mas cercana, hasta encontrar un muro.

use crate::map::is_wall;
use crate::player::Player;

pub struct RayHit {
    // Distancia perpendicular desde el jugador hasta la pared golpeada, en unidades de mapa.
    pub distance: f32,
    // true si el rayo golpeo una pared "vertical" (moviendose en x),
    // false si golpeo una pared "horizontal" (moviendose en y). Solo se
    // usa para variar un poco el tono de color entre paredes N-S y E-O.
    pub vertical_wall: bool,
}

pub fn cast_ray(player: &Player, ray_angle: f32) -> RayHit {
    let ray_dir_x = ray_angle.cos();
    let ray_dir_y = ray_angle.sin();

    let mut map_x = player.x.floor() as i32;
    let mut map_y = player.y.floor() as i32;

    // Distancia (en unidades de rayo) para cruzar una celda completa en x o y.
    let delta_dist_x = if ray_dir_x.abs() < 1e-6 {
        1e30
    } else {
        (1.0 / ray_dir_x).abs()
    };
    let delta_dist_y = if ray_dir_y.abs() < 1e-6 {
        1e30
    } else {
        (1.0 / ray_dir_y).abs()
    };

    let step_x: i32;
    let step_y: i32;
    let mut side_dist_x: f32;
    let mut side_dist_y: f32;

    if ray_dir_x < 0.0 {
        step_x = -1;
        side_dist_x = (player.x - map_x as f32) * delta_dist_x;
    } else {
        step_x = 1;
        side_dist_x = (map_x as f32 + 1.0 - player.x) * delta_dist_x;
    }

    if ray_dir_y < 0.0 {
        step_y = -1;
        side_dist_y = (player.y - map_y as f32) * delta_dist_y;
    } else {
        step_y = 1;
        side_dist_y = (map_y as f32 + 1.0 - player.y) * delta_dist_y;
    }

    let mut hit = false;
    let mut vertical_wall = true;

    // Tope de iteraciones como red de seguridad para nunca hacer un loop infinito si algo del mapa esta mal formado.
    let max_steps = 200;
    for _ in 0..max_steps {
        if side_dist_x < side_dist_y {
            side_dist_x += delta_dist_x;
            map_x += step_x;
            vertical_wall = true;
        } else {
            side_dist_y += delta_dist_y;
            map_y += step_y;
            vertical_wall = false;
        }

        if is_wall(map_x as f32, map_y as f32) {
            hit = true;
            break;
        }
    }

    let perp_dist = if !hit {
        1e30
    } else if vertical_wall {
        (map_x as f32 - player.x + (1 - step_x) as f32 / 2.0) / ray_dir_x
    } else {
        (map_y as f32 - player.y + (1 - step_y) as f32 / 2.0) / ray_dir_y
    };

    RayHit {
        distance: perp_dist.abs(),
        vertical_wall,
    }
}
