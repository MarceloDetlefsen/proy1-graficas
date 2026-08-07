mod framebuffer;
mod colors;
mod game_state;
mod input;
mod level_data;
mod map;
mod player;
mod raycaster;
mod renderer;
mod textures;

use framebuffer::Framebuffer;
use colors::{NBA_CREAM, NBA_NAVY, NBA_ORANGE};
use game_state::GameState;
use level_data::{build_level, Level};
use input::handle_input;
use player::Player;
use raylib::prelude::*;
use textures::TextureManager;

fn main() {
    let (mut window, raylib_thread) = raylib::init()
        .size(800, 600)
        .title("Raycaster prototype")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    window.set_target_fps(60);

    let mut screen_width = window.get_screen_width().max(1) as u32;
    let mut screen_height = window.get_screen_height().max(1) as u32;
    let mut framebuffer = Framebuffer::new(screen_width, screen_height, Color::BLACK);
    let texture_manager = TextureManager::new();

    let mut game_state = GameState::Welcome { selected_level: 1 };
    let mut player = Player::new(0.0, 0.0, 0.0);
    let mut show_minimap = false;

    while !window.window_should_close() {
        screen_width = window.get_screen_width().max(1) as u32;
        screen_height = window.get_screen_height().max(1) as u32;
        if framebuffer.width() != screen_width as i32 || framebuffer.height() != screen_height as i32 {
            framebuffer = Framebuffer::new(screen_width, screen_height, Color::BLACK);
        }

        let delta_time = window.get_frame_time();

        if window.is_key_pressed(KeyboardKey::KEY_M) {
            show_minimap = !show_minimap;
        }

        let next_state = match &mut game_state {
            GameState::Welcome { selected_level } => {
                update_welcome(
                    &mut window,
                    &mut framebuffer,
                    &raylib_thread,
                    screen_width,
                    screen_height,
                    selected_level,
                    &mut player,
                )
            }
            GameState::Playing {
                level,
                current_level_index,
                hoops_scored,
            } => update_playing(
                    &mut window,
                    &mut framebuffer,
                    &raylib_thread,
                    &texture_manager,
                    screen_width,
                    screen_height,
                    delta_time,
                    show_minimap,
                    level,
                    *current_level_index,
                    hoops_scored,
                    &mut player,
                ),
            GameState::LevelSuccess { current_level_index } => update_level_success(
                    &mut window,
                    &mut framebuffer,
                    &raylib_thread,
                    screen_width,
                    screen_height,
                    *current_level_index,
                    &mut player,
                ),
            GameState::Victory => update_victory(
                    &mut window,
                    &mut framebuffer,
                    &raylib_thread,
                    screen_width,
                    screen_height,
                ),
        };

        if let Some(state) = next_state {
            game_state = state;
        }
    }
}

fn update_welcome(
    window: &mut RaylibHandle,
    framebuffer: &mut Framebuffer,
    raylib_thread: &RaylibThread,
    screen_width: u32,
    screen_height: u32,
    selected_level: &mut u32,
    player: &mut Player,
) -> Option<GameState> {
    if window.is_key_pressed(KeyboardKey::KEY_LEFT) || window.is_key_pressed(KeyboardKey::KEY_A) {
        *selected_level = (*selected_level).saturating_sub(1).max(1);
    }
    if window.is_key_pressed(KeyboardKey::KEY_RIGHT) || window.is_key_pressed(KeyboardKey::KEY_D) {
        *selected_level = (*selected_level + 1).min(3);
    }

    framebuffer.clear(NBA_NAVY);
    framebuffer.swap_buffers(window, raylib_thread, |draw| {
        let title = "NBA RAYCASTING";
        let title_font = 34;
        let title_width = draw.measure_text(title, title_font);
        let icon_size = 28;
        let icon_gap = 12;
        let combo_width = icon_size + icon_gap + title_width;
        let start_x = (screen_width as i32 - combo_width) / 2;
        let title_y = (screen_height as i32 * 14) / 100;
        draw_basketball_icon(draw, start_x, title_y + 8, icon_size);
        draw.draw_text(title, start_x + icon_size + icon_gap, title_y, title_font, NBA_ORANGE);

        let label_y = (screen_height as i32 * 34) / 100;
        draw.draw_text("Selecciona nivel:", (screen_width as i32 - 180) / 2, label_y, 24, NBA_CREAM);

        let labels = ["[1]", "[2]", "[3]"];
        let colors = [
            if *selected_level == 1 {
                NBA_ORANGE
            } else {
                NBA_CREAM
            },
            if *selected_level == 2 {
                NBA_ORANGE
            } else {
                NBA_CREAM
            },
            if *selected_level == 3 {
                NBA_ORANGE
            } else {
                NBA_CREAM
            },
        ];
        let font_size = 28;
        let spacing = 18;
        let total_width = labels
            .iter()
            .map(|label| draw.measure_text(label, font_size))
            .sum::<i32>()
            + spacing * 2;
        let mut x = (screen_width as i32 - total_width) / 2;
        let levels_y = label_y + 36;
        for (index, label) in labels.iter().enumerate() {
            draw.draw_text(label, x, levels_y, font_size, colors[index]);
            x += draw.measure_text(label, font_size) + spacing;
        }

        draw.draw_text(
            "Flechas o A/D para elegir, ENTER para empezar",
            (screen_width as i32 - draw.measure_text("Flechas o A/D para elegir, ENTER para empezar", 20)) / 2,
            levels_y + 58,
            20,
            NBA_CREAM,
        );
    });

    if window.is_key_pressed(KeyboardKey::KEY_ENTER) {
        let level = build_level(*selected_level);
        *player = spawn_player(&level);
        return Some(GameState::Playing {
            level,
            current_level_index: *selected_level,
            hoops_scored: 0,
        });
    }

    None
}

fn update_playing(
    window: &mut RaylibHandle,
    framebuffer: &mut Framebuffer,
    raylib_thread: &RaylibThread,
    texture_manager: &TextureManager,
    screen_width: u32,
    screen_height: u32,
    delta_time: f32,
    show_minimap: bool,
    level: &Level,
    current_level_index: u32,
    hoops_scored: &mut usize,
    player: &mut Player,
) -> Option<GameState> {
    handle_input(window, player, &level.grid, delta_time);

    if window.is_key_pressed(KeyboardKey::KEY_K) {
        *hoops_scored += 1;
        // TODO: reemplazar con detección real de encestada.
    }

    renderer::render(
        framebuffer,
        player,
        &level.grid,
        &level.hoop_positions,
        texture_manager,
        screen_width,
        screen_height,
        show_minimap,
    );

    framebuffer.swap_buffers(window, raylib_thread, |draw| {
        renderer::draw_hud(
            draw,
            screen_width,
            current_level_index,
            &level.level_name,
            *hoops_scored,
            level.hoops_required,
        );
    });

    if *hoops_scored >= level.hoops_required {
        Some(GameState::LevelSuccess {
            current_level_index,
        })
    } else {
        None
    }
}

fn update_level_success(
    window: &mut RaylibHandle,
    framebuffer: &mut Framebuffer,
    raylib_thread: &RaylibThread,
    screen_width: u32,
    screen_height: u32,
    current_level_index: u32,
    player: &mut Player,
) -> Option<GameState> {
    framebuffer.clear(NBA_NAVY);
    framebuffer.swap_buffers(window, raylib_thread, |draw| {
        let title = format!("¡NIVEL {} SUPERADO!", current_level_index);
        draw_centered_text(
            draw,
            &title,
            screen_width,
            34,
            (screen_height as i32 * 36) / 100,
            NBA_ORANGE,
        );
        draw_centered_text(
            draw,
            "Presiona ENTER para continuar",
            screen_width,
            24,
            (screen_height as i32 * 48) / 100,
            NBA_CREAM,
        );
    });

    if window.is_key_pressed(KeyboardKey::KEY_ENTER) {
        if current_level_index < 3 {
            let next_level = build_level(current_level_index + 1);
            *player = spawn_player(&next_level);
            return Some(GameState::Playing {
                level: next_level,
                current_level_index: current_level_index + 1,
                hoops_scored: 0,
            });
        }

        return Some(GameState::Victory);
    }

    None
}

fn update_victory(
    window: &mut RaylibHandle,
    framebuffer: &mut Framebuffer,
    raylib_thread: &RaylibThread,
    screen_width: u32,
    screen_height: u32,
) -> Option<GameState> {
    framebuffer.clear(NBA_NAVY);
    framebuffer.swap_buffers(window, raylib_thread, |draw| {
        draw_centered_text(
            draw,
            "¡CAMPEÓN! Completaste los 3 niveles",
            screen_width,
            32,
            (screen_height as i32 * 36) / 100,
            NBA_ORANGE,
        );
        draw_centered_text(
            draw,
            "Presiona ENTER para volver al inicio",
            screen_width,
            24,
            (screen_height as i32 * 48) / 100,
            NBA_CREAM,
        );
    });

    if window.is_key_pressed(KeyboardKey::KEY_ENTER) {
        return Some(GameState::Welcome { selected_level: 1 });
    }

    None
}

fn spawn_player(level: &Level) -> Player {
    Player::new(
        level.player_start_x,
        level.player_start_y,
        level.player_start_angle,
    )
}

fn draw_centered_text(
    draw: &mut RaylibDrawHandle<'_>,
    text: &str,
    screen_width: u32,
    font_size: i32,
    y: i32,
    color: Color,
) {
    let text_width = draw.measure_text(text, font_size);
    let x = (screen_width as i32 - text_width) / 2;
    draw.draw_text(text, x, y, font_size, color);
}

fn draw_basketball_icon(draw: &mut RaylibDrawHandle<'_>, x: i32, y: i32, radius: i32) {
    draw.draw_circle(x, y, radius as f32, NBA_ORANGE);
    draw.draw_circle_lines(x, y, radius as f32, Color::new(60, 30, 10, 255));

    let seam_color = Color::new(50, 25, 10, 255);
    draw.draw_line(x - radius + 3, y, x + radius - 3, y, seam_color);
    draw.draw_line(x, y - radius + 3, x, y + radius - 3, seam_color);
    draw.draw_line(
        x - radius + 5,
        y - radius / 2,
        x + radius - 5,
        y + radius / 2,
        seam_color,
    );
    draw.draw_line(
        x - radius + 5,
        y + radius / 2,
        x + radius - 5,
        y - radius / 2,
        seam_color,
    );
}
