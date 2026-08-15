use crate::hoop::Hoop;
use crate::level_data::Level;
use raylib::prelude::Color;

#[derive(Clone, Debug)]
pub struct ConfettiPiece {
    pub x: f32,
    pub y: f32,
    pub vy: f32,
    pub rotation: f32,
    pub rot_speed: f32,
    pub color: Color,
    pub size: f32,
}

#[derive(Debug)]
pub enum GameState {
    Welcome {
        selected_level: u32,
        elapsed_time: f32,
    },
    Playing {
        level: Level,
        current_level_index: u32,
        hoops_scored: usize,
        hoops: Vec<Hoop>,
        feedback_message: Option<String>,
        feedback_timer: f32,
        throw_anim_timer: f32,
        throw_anim_total: f32,
        level_complete_timer: f32,
    },
    LevelSuccess {
        current_level_index: u32,
        elapsed_time: f32,
    },
    Victory {
        elapsed_time: f32,
        confetti: Vec<ConfettiPiece>,
    },
}
