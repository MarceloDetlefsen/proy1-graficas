mod background;
mod framebuffer;
mod colors;
mod game_state;
mod hoop;
mod input;
mod level_data;
mod map;
mod player;
mod raycaster;
mod renderer;
mod textures;

use framebuffer::Framebuffer;
use colors::{NBA_CREAM, NBA_NAVY, NBA_ORANGE};
use background::draw_screen_background;
use game_state::{ConfettiPiece, GameState};
use hoop::{Hoop, HoopState, SCORE_ANIM_DURATION};
use level_data::{build_level, Level};
use input::{handle_input, try_shoot};
use player::Player;
use raylib::prelude::*;
use rand::Rng;
use textures::TextureManager;

const RENDER_WIDTH: u32 = 800;
const RENDER_HEIGHT: u32 = 600;

fn main() {
    let (mut window, raylib_thread) = raylib::init()
        .size(800, 600)
        .title("Raycaster prototype")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    window.set_target_fps(60);

    let mut framebuffer = Framebuffer::new(
        &mut window,
        &raylib_thread,
        RENDER_WIDTH,
        RENDER_HEIGHT,
        Color::BLACK,
    );
    let texture_manager = TextureManager::new(&mut window, &raylib_thread);

    let mut game_state = GameState::Welcome {
        selected_level: 1,
        elapsed_time: 0.0,
    };
    let mut player = Player::new(0.0, 0.0, 0.0);
    let mut show_minimap = false;

    while !window.window_should_close() {
        let screen_width = window.get_screen_width().max(1) as u32;
        let screen_height = window.get_screen_height().max(1) as u32;

        let delta_time = window.get_frame_time();

        if window.is_key_pressed(KeyboardKey::KEY_M) {
            show_minimap = !show_minimap;
        }

        let next_state = match &mut game_state {
            GameState::Welcome {
                selected_level,
                elapsed_time,
            } => {
                update_welcome(
                    &mut window,
                    &mut framebuffer,
                    &raylib_thread,
                    &texture_manager,
                    screen_width,
                    screen_height,
                    delta_time,
                    selected_level,
                    elapsed_time,
                    &mut player,
                )
            }
            GameState::Playing {
                level,
                current_level_index,
                hoops_scored,
                hoops,
                feedback_message,
                feedback_timer,
                level_complete_timer,
            } => update_playing(
                    &mut window,
                    &mut framebuffer,
                    &raylib_thread,
                    &texture_manager,
                    RENDER_WIDTH,
                    RENDER_HEIGHT,
                    delta_time,
                    show_minimap,
                    level,
                    *current_level_index,
                    hoops_scored,
                    hoops,
                    feedback_message,
                    feedback_timer,
                    level_complete_timer,
                    &mut player,
                ),
            GameState::LevelSuccess {
                current_level_index,
                elapsed_time,
            } => update_level_success(
                    &mut window,
                    &mut framebuffer,
                    &raylib_thread,
                    &texture_manager,
                    screen_width,
                    screen_height,
                    delta_time,
                    *current_level_index,
                    elapsed_time,
                    &mut player,
                ),
            GameState::Victory {
                elapsed_time,
                confetti,
            } => update_victory(
                    &mut window,
                    &mut framebuffer,
                    &raylib_thread,
                    &texture_manager,
                    screen_width,
                    screen_height,
                    delta_time,
                    elapsed_time,
                    confetti,
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
    texture_manager: &TextureManager,
    screen_width: u32,
    screen_height: u32,
    delta_time: f32,
    selected_level: &mut u32,
    elapsed_time: &mut f32,
    player: &mut Player,
) -> Option<GameState> {
    *elapsed_time += delta_time;

    if window.is_key_pressed(KeyboardKey::KEY_LEFT) || window.is_key_pressed(KeyboardKey::KEY_A) {
        *selected_level = (*selected_level).saturating_sub(1).max(1);
    }
    if window.is_key_pressed(KeyboardKey::KEY_RIGHT) || window.is_key_pressed(KeyboardKey::KEY_D) {
        *selected_level = (*selected_level + 1).min(3);
    }

    framebuffer.clear(NBA_NAVY);
    framebuffer.swap_buffers(window, raylib_thread, |draw| {
        draw_screen_background(draw, texture_manager, screen_width, screen_height, *elapsed_time);

        let title = "NBA RAYCASTING";
        let title_font = 42;
        let icon_radius = 19;
        let icon_gap = 12;
        let title_width = draw.measure_text(title, title_font);
        let combo_width = icon_radius * 2 + icon_gap + title_width;
        let start_x = (screen_width as i32 - combo_width) / 2;
        let title_y = (screen_height as i32 * 13) / 100;
        draw_basketball_icon(
            draw,
            start_x + icon_radius,
            title_y + icon_radius + 2,
            icon_radius,
            *elapsed_time * 1.35,
        );
        draw_shadowed_text(
            draw,
            title,
            start_x + icon_radius * 2 + icon_gap,
            title_y,
            title_font,
            NBA_ORANGE,
            NBA_NAVY,
        );

        let label = "Selecciona nivel";
        let label_font = 24;
        let label_width = draw.measure_text(label, label_font);
        let label_y = (screen_height as i32 * 30) / 100;
        draw_shadowed_text(
            draw,
            label,
            (screen_width as i32 - label_width) / 2,
            label_y,
            label_font,
            NBA_CREAM,
            Color::new(12, 16, 34, 255),
        );

        draw_level_cards(draw, *selected_level, screen_width, screen_height, label_y + 44);

        let instruction = "Flechas o A/D para elegir, ENTER para empezar";
        let instruction_font = 20;
        let blinking = Color::new(NBA_CREAM.r, NBA_CREAM.g, NBA_CREAM.b, blinking_alpha(*elapsed_time));
        let instruction_width = draw.measure_text(instruction, instruction_font);
        draw.draw_text(
            instruction,
            (screen_width as i32 - instruction_width) / 2,
            (screen_height as i32 * 82) / 100,
            instruction_font,
            blinking,
        );
    });

    if window.is_key_pressed(KeyboardKey::KEY_ENTER) {
        let level = build_level(*selected_level);
        *player = spawn_player(&level);
        let hoops = build_hoops(&level);
        return Some(GameState::Playing {
            level,
            current_level_index: *selected_level,
            hoops_scored: 0,
            hoops,
            feedback_message: None,
            feedback_timer: 0.0,
            level_complete_timer: 0.0,
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
    hoops: &mut Vec<Hoop>,
    feedback_message: &mut Option<String>,
    feedback_timer: &mut f32,
    level_complete_timer: &mut f32,
    player: &mut Player,
) -> Option<GameState> {
    handle_input(window, player, &level.grid, delta_time);

    *feedback_timer = (*feedback_timer - delta_time).max(0.0);
    if *feedback_timer == 0.0 {
        *feedback_message = None;
    }

    for hoop in hoops.iter_mut() {
        hoop.score_anim_timer = (hoop.score_anim_timer - delta_time).max(0.0);
    }

    *level_complete_timer = (*level_complete_timer - delta_time).max(0.0);

    if window.is_key_pressed(KeyboardKey::KEY_SPACE) {
        if let Some((acerto, mensaje)) = try_shoot(player, hoops) {
            *feedback_message = Some(mensaje);
            *feedback_timer = 1.2;

            if acerto {
                *hoops_scored = hoops.iter().filter(|hoop| hoop.state == HoopState::Scored).count();
                if *hoops_scored >= level.hoops_required {
                    *level_complete_timer = SCORE_ANIM_DURATION;
                }
            }
        }
    }

    renderer::render(
        framebuffer,
        player,
        &level.grid,
        hoops,
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

        draw_feedback_message(draw, screen_width, screen_height, feedback_message.as_deref());
        draw_basketball_overlay(draw, screen_width, screen_height, texture_manager.basketball_texture());
    });

    if *hoops_scored >= level.hoops_required && *level_complete_timer <= 0.0 {
        Some(GameState::LevelSuccess {
            current_level_index,
            elapsed_time: 0.0,
        })
    } else {
        None
    }
}

fn update_level_success(
    window: &mut RaylibHandle,
    framebuffer: &mut Framebuffer,
    raylib_thread: &RaylibThread,
    texture_manager: &TextureManager,
    screen_width: u32,
    screen_height: u32,
    delta_time: f32,
    current_level_index: u32,
    elapsed_time: &mut f32,
    player: &mut Player,
) -> Option<GameState> {
    *elapsed_time += delta_time;

    framebuffer.clear(NBA_NAVY);
    framebuffer.swap_buffers(window, raylib_thread, |draw| {
        draw_screen_background(draw, texture_manager, screen_width, screen_height, *elapsed_time);

        let title = format!("¡NIVEL {} SUPERADO!", current_level_index);
        draw_shadowed_centered_text(
            draw,
            &title,
            screen_width,
            40,
            (screen_height as i32 * 24) / 100,
            NBA_ORANGE,
            NBA_NAVY,
        );

        let hoops_required = build_level(current_level_index).hoops_required;
        let ball_radius = 20;
        let ball_gap = 18;
        let total_width = hoops_required as i32 * ball_radius * 2
            + hoops_required.saturating_sub(1) as i32 * ball_gap;
        let mut ball_x = (screen_width as i32 - total_width) / 2 + ball_radius;
        let ball_y = (screen_height as i32 * 42) / 100;
        for _ in 0..hoops_required {
            draw_ball_marker(draw, ball_x, ball_y, ball_radius, NBA_ORANGE);
            ball_x += ball_radius * 2 + ball_gap;
        }

        draw.draw_text(
            "Presiona ENTER para continuar",
            (screen_width as i32
                - draw.measure_text("Presiona ENTER para continuar", 24))
                / 2,
            (screen_height as i32 * 72) / 100,
            24,
            Color::new(NBA_CREAM.r, NBA_CREAM.g, NBA_CREAM.b, blinking_alpha(*elapsed_time)),
        );
    });

    if window.is_key_pressed(KeyboardKey::KEY_ENTER) {
        if current_level_index < 3 {
            let next_level = build_level(current_level_index + 1);
            *player = spawn_player(&next_level);
            let hoops = build_hoops(&next_level);
            return Some(GameState::Playing {
                level: next_level,
                current_level_index: current_level_index + 1,
                hoops_scored: 0,
                hoops,
                feedback_message: None,
                feedback_timer: 0.0,
                level_complete_timer: 0.0,
            });
        }

        return Some(GameState::Victory {
            elapsed_time: 0.0,
            confetti: generate_victory_confetti(screen_width, screen_height),
        });
    }

    None
}

fn update_victory(
    window: &mut RaylibHandle,
    framebuffer: &mut Framebuffer,
    raylib_thread: &RaylibThread,
    texture_manager: &TextureManager,
    screen_width: u32,
    screen_height: u32,
    delta_time: f32,
    elapsed_time: &mut f32,
    confetti: &mut Vec<ConfettiPiece>,
) -> Option<GameState> {
    *elapsed_time += delta_time;
    update_confetti(confetti.as_mut_slice(), screen_width, screen_height, delta_time);

    framebuffer.clear(NBA_NAVY);
    framebuffer.swap_buffers(window, raylib_thread, |draw| {
        draw_screen_background(draw, texture_manager, screen_width, screen_height, *elapsed_time);

        for piece in confetti.iter() {
            draw_confetti_piece(draw, piece);
        }

        draw_trophy(draw, screen_width, screen_height);

        draw_shadowed_centered_text(
            draw,
            "¡CAMPEÓN!",
            screen_width,
            42,
            (screen_height as i32 * 57) / 100,
            NBA_ORANGE,
            NBA_NAVY,
        );

        draw_shadowed_centered_text(
            draw,
            "Completaste los 3 niveles",
            screen_width,
            26,
            (screen_height as i32 * 66) / 100,
            NBA_CREAM,
            Color::new(10, 14, 35, 255),
        );

        draw.draw_text(
            "Presiona ENTER para volver al inicio",
            (screen_width as i32
                - draw.measure_text("Presiona ENTER para volver al inicio", 24))
                / 2,
            (screen_height as i32 * 83) / 100,
            24,
            Color::new(NBA_CREAM.r, NBA_CREAM.g, NBA_CREAM.b, blinking_alpha(*elapsed_time)),
        );
    });

    if window.is_key_pressed(KeyboardKey::KEY_ENTER) {
        return Some(GameState::Welcome {
            selected_level: 1,
            elapsed_time: 0.0,
        });
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

fn build_hoops(level: &Level) -> Vec<Hoop> {
    level
        .hoop_positions
        .iter()
        .map(|(x, y)| Hoop::new(*x, *y))
        .collect()
}

fn draw_shadowed_centered_text(
    draw: &mut RaylibDrawHandle<'_>,
    text: &str,
    screen_width: u32,
    font_size: i32,
    y: i32,
    color: Color,
    shadow_color: Color,
) {
    let text_width = draw.measure_text(text, font_size);
    let x = (screen_width as i32 - text_width) / 2;
    draw.draw_text(text, x + 3, y + 3, font_size, shadow_color);
    draw.draw_text(text, x, y, font_size, color);
}

fn draw_shadowed_text(
    draw: &mut RaylibDrawHandle<'_>,
    text: &str,
    x: i32,
    y: i32,
    font_size: i32,
    color: Color,
    shadow_color: Color,
) {
    draw.draw_text(text, x + 3, y + 3, font_size, shadow_color);
    draw.draw_text(text, x, y, font_size, color);
}

fn draw_basketball_icon(
    draw: &mut RaylibDrawHandle<'_>,
    x: i32,
    y: i32,
    radius: i32,
    rotation: f32,
) {
    draw.draw_circle(x, y, radius as f32, NBA_ORANGE);
    draw.draw_circle_lines(x, y, radius as f32, Color::new(60, 30, 10, 255));

    let seam_color = Color::new(50, 25, 10, 255);
    let seam_points = [
        ((-radius + 3) as f32, 0.0_f32, (radius - 3) as f32, 0.0_f32),
        (0.0_f32, (-radius + 3) as f32, 0.0_f32, (radius - 3) as f32),
        (
            (-radius + 5) as f32,
            -(radius as f32) * 0.5,
            (radius - 5) as f32,
            (radius as f32) * 0.5,
        ),
        (
            (-radius + 5) as f32,
            (radius as f32) * 0.5,
            (radius - 5) as f32,
            -(radius as f32) * 0.5,
        ),
    ];

    for (x0, y0, x1, y1) in seam_points {
        let (rx0, ry0) = rotate_point(x0, y0, rotation);
        let (rx1, ry1) = rotate_point(x1, y1, rotation);
        draw.draw_line(
            x + rx0.round() as i32,
            y + ry0.round() as i32,
            x + rx1.round() as i32,
            y + ry1.round() as i32,
            seam_color,
        );
    }
}

fn draw_ball_marker(draw: &mut RaylibDrawHandle<'_>, x: i32, y: i32, radius: i32, color: Color) {
    draw.draw_circle(x, y, radius as f32, color);
    draw.draw_circle_lines(x, y, radius as f32, Color::new(60, 30, 10, 200));
}

fn draw_trophy(draw: &mut RaylibDrawHandle<'_>, screen_width: u32, screen_height: u32) {
    let center_x = screen_width as i32 / 2;
    let top_y = (screen_height as i32 * 14) / 100;
    let rim_radius = 24.0;
    let body_top_y = top_y + 24;
    let body_bottom_y = top_y + 96;

    draw.draw_circle(center_x, top_y + 16, rim_radius, NBA_ORANGE);
    draw.draw_triangle(
        Vector2::new((center_x - 56) as f32, body_top_y as f32),
        Vector2::new((center_x + 56) as f32, body_top_y as f32),
        Vector2::new(center_x as f32, body_bottom_y as f32),
        NBA_ORANGE,
    );
    draw.draw_rectangle(center_x - 34, body_bottom_y + 2, 68, 16, NBA_CREAM);
    draw.draw_rectangle(center_x - 10, body_bottom_y + 18, 20, 14, Color::new(230, 200, 130, 255));
}

fn draw_confetti_piece(draw: &mut RaylibDrawHandle<'_>, piece: &ConfettiPiece) {
    let rect = Rectangle::new(piece.x, piece.y, piece.size, piece.size * 0.6);
    let origin = Vector2::new(piece.size * 0.5, piece.size * 0.3);
    draw.draw_rectangle_pro(rect, origin, piece.rotation, piece.color);
}

fn update_confetti(
    confetti: &mut [ConfettiPiece],
    screen_width: u32,
    screen_height: u32,
    delta_time: f32,
) {
    let mut rng = rand::thread_rng();

    for piece in confetti.iter_mut() {
        piece.y += piece.vy * delta_time;
        piece.rotation += piece.rot_speed * delta_time;

        if piece.y > screen_height as f32 {
            *piece = random_confetti_piece(&mut rng, screen_width, screen_height);
            piece.y = -10.0;
        }
    }
}

fn generate_victory_confetti(screen_width: u32, screen_height: u32) -> Vec<ConfettiPiece> {
    let mut rng = rand::thread_rng();
    (0..40)
        .map(|_| random_confetti_piece(&mut rng, screen_width, screen_height))
        .collect()
}

fn random_confetti_piece(
    rng: &mut impl Rng,
    screen_width: u32,
    screen_height: u32,
) -> ConfettiPiece {
    let palette = [
        NBA_ORANGE,
        NBA_CREAM,
        Color::WHITE,
        Color::new(10, 14, 35, 255),
    ];

    ConfettiPiece {
        x: rng.gen_range(0.0..screen_width as f32),
        y: rng.gen_range(-(screen_height as f32)..-10.0),
        vy: rng.gen_range(40.0..90.0),
        rotation: rng.gen_range(0.0..360.0),
        rot_speed: rng.gen_range(-180.0..180.0),
        color: palette[rng.gen_range(0..palette.len())],
        size: rng.gen_range(4.0..8.0),
    }
}

fn rotate_point(x: f32, y: f32, rotation: f32) -> (f32, f32) {
    let sin = rotation.sin();
    let cos = rotation.cos();
    (x * cos - y * sin, x * sin + y * cos)
}

fn blinking_alpha(elapsed_time: f32) -> u8 {
    (128.0 + 127.0 * (elapsed_time * 2.5).sin()) as u8
}

fn draw_level_cards(
    draw: &mut RaylibDrawHandle<'_>,
    selected_level: u32,
    screen_width: u32,
    screen_height: u32,
    top_y: i32,
) {
    let card_width = 120;
    let card_height = 140;
    let gap = 24;
    let total_width = card_width * 3 + gap * 2;
    let start_x = (screen_width as i32 - total_width) / 2;
    let card_y = top_y + (screen_height as i32 * 2) / 100;

    for level in 1..=3 {
        let x = start_x + (level as i32 - 1) * (card_width + gap);
        let selected = selected_level == level;
        let hoops_required = build_level(level).hoops_required;
        draw_level_card(draw, level, hoops_required, x, card_y, card_width, card_height, selected);
    }
}

fn draw_level_card(
    draw: &mut RaylibDrawHandle<'_>,
    level: u32,
    hoops_required: usize,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    selected: bool,
) {
    let background = if selected {
        Color::new(26, 40, 88, 235)
    } else {
        Color::new(18, 28, 66, 205)
    };
    let border = if selected {
        NBA_ORANGE
    } else {
        Color::new(NBA_CREAM.r, NBA_CREAM.g, NBA_CREAM.b, 220)
    };

    draw.draw_rectangle(x, y, width, height, background);

    let border_thickness = if selected { 3 } else { 1 };
    for offset in 0..border_thickness {
        draw.draw_rectangle_lines(
            x + offset,
            y + offset,
            width - offset * 2,
            height - offset * 2,
            border,
        );
    }

    let number = level.to_string();
    let number_font = 40;
    let number_width = draw.measure_text(&number, number_font);
    draw.draw_text(
        &number,
        x + (width - number_width) / 2,
        y + 14,
        number_font,
        if selected { NBA_ORANGE } else { NBA_CREAM },
    );

    let icon_radius = 6;
    let icon_gap = 10;
    let total_icons_width = hoops_required as i32 * icon_radius * 2
        + hoops_required.saturating_sub(1) as i32 * icon_gap;
    let mut icon_x = x + (width - total_icons_width) / 2 + icon_radius;
    let icon_y = y + 82;

    for _ in 0..hoops_required {
        draw_ball_marker(draw, icon_x, icon_y, icon_radius, NBA_ORANGE);
        icon_x += icon_radius * 2 + icon_gap;
    }
}

fn draw_feedback_message(
    draw: &mut RaylibDrawHandle<'_>,
    screen_width: u32,
    screen_height: u32,
    message: Option<&str>,
) {
    let Some(message) = message else {
        return;
    };

    let font_size = 24;
    let color = if message.contains("ENCESTASTE") {
        NBA_ORANGE
    } else {
        NBA_CREAM
    };
    let text_width = draw.measure_text(message, font_size);
    let x = (screen_width as i32 - text_width) / 2;
    let y = screen_height as i32 - 78;
    draw.draw_text(message, x, y, font_size, color);
}

fn draw_basketball_overlay(
    draw: &mut RaylibDrawHandle<'_>,
    screen_width: u32,
    screen_height: u32,
    basketball: &Texture2D,
) {
    let dest_w = 48.0;
    let dest_h = 48.0;
    let x = screen_width as f32 / 2.0 - dest_w / 2.0;
    let y = screen_height as f32 - dest_h - 12.0;
    let tex_w = basketball.width().max(1) as f32;
    let tex_h = basketball.height().max(1) as f32;
    let source = Rectangle::new(0.0, 0.0, tex_w, tex_h);
    let dest = Rectangle::new(x, y, dest_w, dest_h);

    draw.draw_texture_pro(
        basketball,
        source,
        dest,
        Vector2::new(0.0, 0.0),
        0.0,
        Color::WHITE,
    );
}
