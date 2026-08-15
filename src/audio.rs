use crate::game_state::GameState;
use raylib::prelude::*;
use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MusicScene {
    Title,
    Level(u32),
    Ending,
}

pub struct MusicManager<'a> {
    title_track: Option<Music<'a>>,
    level_tracks: Vec<Option<Music<'a>>>,
    ending_track: Option<Music<'a>>,
    current_scene: Option<MusicScene>,
}

pub struct SoundManager<'a> {
    swish: Option<Sound<'a>>,
}

impl<'a> MusicManager<'a> {
    pub fn new(audio: &'a RaylibAudio) -> Self {
        let title_track = load_track(audio, "assets/music/tittle_screen.ogg");
        let level_tracks = [
            "assets/music/level1.ogg",
            "assets/music/level2.ogg",
            "assets/music/level3.ogg",
        ]
        .iter()
        .map(|path| load_track(audio, path))
        .collect();
        let ending_track = load_track(audio, "assets/music/ending_screen.ogg");

        Self {
            title_track,
            level_tracks,
            ending_track,
            current_scene: None,
        }
    }

    pub fn sync_state(&mut self, audio: &RaylibAudio, state: &GameState) {
        match state {
            GameState::Welcome { .. } => self.play_title(audio),
            GameState::Playing {
                current_level_index, ..
            } => self.play_level(audio, *current_level_index),
            GameState::LevelSuccess { .. } => self.stop_all(),
            GameState::Victory { .. } => self.play_ending(audio),
        }
    }

    pub fn play_level(&mut self, _audio: &RaylibAudio, level: u32) {
        let desired = MusicScene::Level(level);
        if self.current_scene == Some(desired) {
            return;
        }

        self.stop_current();

        if let Some(track) = self
            .level_tracks
            .get_mut(level.saturating_sub(1) as usize)
            .and_then(Option::as_mut)
        {
            track.set_looping(true);
            track.play_stream();
        }

        self.current_scene = Some(desired);
    }

    pub fn play_title(&mut self, _audio: &RaylibAudio) {
        if self.current_scene == Some(MusicScene::Title) {
            return;
        }

        self.stop_current();

        if let Some(track) = self.title_track.as_mut() {
            track.set_looping(true);
            track.play_stream();
        }

        self.current_scene = Some(MusicScene::Title);
    }

    pub fn play_ending(&mut self, _audio: &RaylibAudio) {
        if self.current_scene == Some(MusicScene::Ending) {
            return;
        }

        self.stop_current();

        if let Some(track) = self.ending_track.as_mut() {
            track.set_looping(true);
            track.play_stream();
        }

        self.current_scene = Some(MusicScene::Ending);
    }

    pub fn stop_all(&mut self) {
        self.stop_current();
        self.current_scene = None;
    }

    pub fn update(&mut self) {
        match self.current_scene {
            Some(MusicScene::Title) => {
                if let Some(track) = self.title_track.as_ref() {
                    track.update_stream();
                }
            }
            Some(MusicScene::Level(level)) => {
                if let Some(Some(track)) = self.level_tracks.get(level.saturating_sub(1) as usize) {
                    track.update_stream();
                }
            }
            Some(MusicScene::Ending) => {
                if let Some(track) = self.ending_track.as_ref() {
                    track.update_stream();
                }
            }
            None => {}
        }
    }

    fn stop_current(&mut self) {
        match self.current_scene {
            Some(MusicScene::Title) => {
                if let Some(track) = self.title_track.as_ref() {
                    track.stop_stream();
                }
            }
            Some(MusicScene::Level(level)) => {
                if let Some(Some(track)) = self.level_tracks.get(level.saturating_sub(1) as usize) {
                    track.stop_stream();
                }
            }
            Some(MusicScene::Ending) => {
                if let Some(track) = self.ending_track.as_ref() {
                    track.stop_stream();
                }
            }
            None => {}
        }
    }
}

impl<'a> SoundManager<'a> {
    pub fn new(audio: &'a RaylibAudio) -> Self {
        let swish = load_sound(audio, "assets/sfx/swish.ogg");

        Self { swish }
    }

    pub fn play_swish(&self, _audio: &RaylibAudio) {
        if let Some(swish) = self.swish.as_ref() {
            swish.play();
        }
    }
}

fn load_track<'a>(audio: &'a RaylibAudio, path: &str) -> Option<Music<'a>> {
    if !Path::new(path).exists() {
        eprintln!("No se encontró la canción: {path}");
        return None;
    }

    match audio.new_music(path) {
        Ok(mut music) => {
            music.set_looping(true);
            Some(music)
        }
        Err(error) => {
            eprintln!("No se pudo cargar la canción {path}: {error:?}");
            None
        }
    }
}

fn load_sound<'a>(audio: &'a RaylibAudio, path: &str) -> Option<Sound<'a>> {
    if !Path::new(path).exists() {
        eprintln!("No se encontró el sonido: {path}");
        return None;
    }

    match audio.new_sound(path) {
        Ok(sound) => Some(sound),
        Err(error) => {
            eprintln!("No se pudo cargar el sonido {path}: {error:?}");
            None
        }
    }
}
