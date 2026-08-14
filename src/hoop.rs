#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HoopState {
    Pending,
    Scored,
}

#[derive(Clone, Debug)]
pub struct Hoop {
    pub x: f32,
    pub y: f32,
    pub state: HoopState,
    pub score_anim_timer: f32,
}

pub const SCORE_ANIM_DURATION: f32 = 0.6;

impl Hoop {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            state: HoopState::Pending,
            score_anim_timer: 0.0,
        }
    }
}
