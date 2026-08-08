use raylib::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

pub struct TextureManager {
    wall_textures: Vec<Image>,
    floor_texture: Image,
}

impl TextureManager {
    pub fn new() -> Self {
        let wall_textures = load_wall_textures("assets/walls");
        let floor_texture = load_floor_texture("assets/floor/wood.jpg");

        Self {
            wall_textures,
            floor_texture,
        }
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

    pub fn floor_dimensions(&self) -> (i32, i32) {
        (self.floor_texture.width().max(1), self.floor_texture.height().max(1))
    }

    pub fn floor_pixel_color(&self, tx: u32, ty: u32) -> Color {
        let width = self.floor_texture.width().max(1) as u32;
        let height = self.floor_texture.height().max(1) as u32;
        let x = tx.min(width - 1) as i32;
        let y = ty.min(height - 1) as i32;
        self.floor_texture.get_color(x, y)
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

fn load_floor_texture(path: &str) -> Image {
    match Image::load_image(path) {
        Ok(image) => image,
        Err(_) => generate_floor_fallback(),
    }
}

fn generate_floor_fallback() -> Image {
    let mut image = Image::gen_image_color(64, 64, Color::new(150, 105, 65, 255));

    let plank_colors = [
        Color::new(150, 105, 65, 255),
        Color::new(138, 96, 58, 255),
        Color::new(126, 88, 52, 255),
    ];

    for y in (0..64).step_by(8) {
        let color = plank_colors[(y / 8) % plank_colors.len()];
        image.draw_rectangle(0, y as i32, 64, 8, color);
    }

    for x in (0..64).step_by(8) {
        image.draw_line(x as i32, 0, x as i32, 63, Color::new(90, 60, 34, 120));
    }

    for y in (0..64).step_by(8) {
        image.draw_line(0, y as i32, 63, y as i32, Color::new(110, 76, 46, 80));
    }

    image
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
