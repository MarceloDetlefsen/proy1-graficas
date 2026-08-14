use crate::hoop::Hoop;
use crate::level_data::Level;

#[derive(Debug)]
pub enum GameState {
    Welcome { selected_level: u32 },
    Playing {
        level: Level,
        current_level_index: u32,
        hoops_scored: usize,
        hoops: Vec<Hoop>,
        feedback_message: Option<String>,
        feedback_timer: f32,
    },
    LevelSuccess { current_level_index: u32 },
    Victory,
}
