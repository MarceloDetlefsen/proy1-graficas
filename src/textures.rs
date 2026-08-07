use raylib::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

pub struct TextureManager {
    wall_textures: Vec<Image>,
}

impl TextureManager {
    pub fn new() -> Self {
        let wall_textures = load_wall_textures("assets/walls");

        Self { wall_textures }
    }

    pub fn wall_count(&self) -> usize {
        self.wall_textures.len()
    }

    pub fn wall_dimensions(&self, index: usize) -> Option<(i32, i32)> {
        self.wall_textures.get(index).map(|image| (image.width(), image.height()))
    }

    pub fn wall_pixel_color(&self, index: usize, tx: u32, ty: u32) -> Color {
        self.wall_textures.get(index).map_or(Color::WHITE, |image| {
            let width = image.width().max(1) as u32;
            let height = image.height().max(1) as u32;
            let x = tx.min(width - 1) as i32;
            let y = ty.min(height - 1) as i32;
            image.get_color(x, y)
        })
    }
}

fn load_wall_textures(dir: &str) -> Vec<Image> {
    let mut paths = list_image_paths(dir);
    paths.sort();

    let mut images = Vec::with_capacity(paths.len());
    for path in paths {
        match Image::load_image(path.to_string_lossy().as_ref()) {
            Ok(image) => images.push(image),
            Err(_) => {
                // Skip files that exist but cannot be decoded by raylib.
            }
        }
    }

    if images.is_empty() {
        images.push(Image::gen_image_color(64, 64, Color::MAGENTA));
    }

    images
}

fn list_image_paths(dir: &str) -> Vec<PathBuf> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Vec::new();
    };

    entries
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|path| is_supported_image(path))
        .collect()
}

fn is_supported_image(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|ext| ext.to_str()).map(|ext| ext.to_ascii_lowercase()),
        Some(ext) if ext == "png" || ext == "jpg" || ext == "jpeg"
    )
}
