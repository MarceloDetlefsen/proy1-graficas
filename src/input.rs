/// Manejo de input: W/S para avanzar y retroceder, A/D para rotar.

use raylib::prelude::*;

use crate::hoop::{Hoop, HoopState};
use crate::map::MapGrid;
use crate::player::{normalize_angle, Player};

/// Ajustes de tiro; se pueden afinar jugando sin tocar el resto del sistema.
pub const SHOOT_ANGLE_THRESHOLD: f32 = 0.26;
pub const IDEAL_DISTANCE_MIN: f32 = 1.8;
pub const IDEAL_DISTANCE_MAX: f32 = 3.2;

pub fn handle_input(window: &RaylibHandle, player: &mut Player, map: &MapGrid, delta_time: f32) {
    if window.is_key_down(KeyboardKey::KEY_A) {
        player.rotate(-player.rot_speed * delta_time);
    }
    if window.is_key_down(KeyboardKey::KEY_D) {
        player.rotate(player.rot_speed * delta_time);
    }

    let mut move_dir: f32 = 0.0;
    if window.is_key_down(KeyboardKey::KEY_W) {
        move_dir += 1.0;
    }
    if window.is_key_down(KeyboardKey::KEY_S) {
        move_dir -= 1.0;
    }

    if move_dir != 0.0 {
        let step = player.move_speed * delta_time * move_dir;
        let dx = player.angle.cos() * step;
        let dy = player.angle.sin() * step;
        player.try_move(dx, dy, map);
    }
}

pub fn try_shoot(player: &Player, hoops: &mut Vec<Hoop>) -> Option<(bool, String)> {
    let mut best_index: Option<usize> = None;
    let mut best_distance = f32::MAX;

    for (index, hoop) in hoops.iter().enumerate() {
        if hoop.state != HoopState::Pending {
            continue;
        }

        let dx = hoop.x - player.x;
        let dy = hoop.y - player.y;
        let distance = (dx * dx + dy * dy).sqrt();
        let angle_to_sprite = dy.atan2(dx);
        let relative_angle = normalize_angle(angle_to_sprite - player.angle);

        if relative_angle.abs() > SHOOT_ANGLE_THRESHOLD {
            continue;
        }

        if distance < best_distance {
            best_distance = distance;
            best_index = Some(index);
        }
    }

    let Some(index) = best_index else {
        return None;
    };

    let hoop = &mut hoops[index];
    let dx = hoop.x - player.x;
    let dy = hoop.y - player.y;
    let distance = (dx * dx + dy * dy).sqrt();

    if distance < IDEAL_DISTANCE_MIN {
        return Some((false, "¡MUY FUERTE!".to_string()));
    }

    if distance > IDEAL_DISTANCE_MAX {
        return Some((false, "¡MUY DÉBIL!".to_string()));
    }

    hoop.state = HoopState::Scored;
    hoop.score_anim_timer = crate::hoop::SCORE_ANIM_DURATION;
    Some((true, "¡ENCESTASTE!".to_string()))
}
