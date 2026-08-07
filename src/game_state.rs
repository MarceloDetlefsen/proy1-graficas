use crate::level_data::Level;

#[derive(Debug)]
pub enum GameState {
    Welcome { selected_level: u32 },
    Playing {
        level: Level,
        current_level_index: u32,
        hoops_scored: usize,
    },
    LevelSuccess { current_level_index: u32 },
    Victory,
}
