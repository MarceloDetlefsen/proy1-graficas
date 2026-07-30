// Prototipo de raycaster pseudo-3D estilo Wolfenstein 3D / DOOM SNES.
// Sin HUD, sin sprites, sin texturas, sin enemigos: solo el sistema de
// renderizado (mapa -> jugador -> rayos -> columnas verticales).

mod framebuffer;
mod input;
mod map;
mod player;
mod raycaster;
mod renderer;

use framebuffer::Framebuffer;
use input::handle_input;
use map::{PLAYER_START_ANGLE, PLAYER_START_X, PLAYER_START_Y};
use player::Player;
use raylib::prelude::*;

fn main() {
    let screen_width: u32 = 800;
    let screen_height: u32 = 600;

    let (mut window, raylib_thread) = raylib::init()
        .size(screen_width as i32, screen_height as i32)
        .title("Raycaster prototype")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    window.set_target_fps(60);

    let mut framebuffer = Framebuffer::new(screen_width, screen_height, Color::BLACK);

    let mut player = Player::new(PLAYER_START_X, PLAYER_START_Y, PLAYER_START_ANGLE);

    while !window.window_should_close() {
        let delta_time = window.get_frame_time();

        // Fase 2/5: input y movimiento del jugador dentro del laberinto.
        handle_input(&window, &mut player, delta_time);

        // Fase 3/4: un rayo por columna, dibujado como techo/piso/paredes.
        renderer::render(&mut framebuffer, &player, screen_width, screen_height);

        framebuffer.swap_buffers(&mut window, &raylib_thread);
    }
}
