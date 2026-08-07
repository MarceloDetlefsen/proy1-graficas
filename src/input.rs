/// Manejo de input: W/S para avanzar y retroceder, A/D para rotar.

use raylib::prelude::*;

use crate::map::MapGrid;
use crate::player::Player;

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
