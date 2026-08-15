use raylib::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

pub struct TextureManager {
    wall_textures: Vec<Image>,
    wall_textures_gpu: Vec<Texture2D>,
    floor_texture: Image,
    hoop_idle: Image,
    hoop_score_frames: Vec<Image>,
    basketball: Texture2D,
    trophy_image: Option<Image>,
}

impl TextureManager {
    pub fn new(window: &mut RaylibHandle, raylib_thread: &RaylibThread) -> Self {
        let wall_textures = load_wall_textures("assets/walls");
        let wall_textures_gpu = load_wall_textures_gpu(window, raylib_thread, &wall_textures);
        let floor_texture = load_floor_texture("assets/floor/wood.jpg");
        let hoop_idle = load_hoop_idle("assets/sprites/hoop_idle.png");
        let hoop_score_frames = load_hoop_score_frames("assets/sprites");
        let basketball = load_basketball_texture(window, raylib_thread, "assets/ball/basketball.png");
        let trophy_image = load_optional_image("assets/ui/trophy.png");

        Self {
            wall_textures,
            wall_textures_gpu,
            floor_texture,
            hoop_idle,
            hoop_score_frames,
            basketball,
            trophy_image,
        }
    }

    pub fn wall_count(&self) -> usize {
        self.wall_textures.len()
    }

    pub fn wall_texture_gpu(&self, index: usize) -> Option<&Texture2D> {
        self.wall_textures_gpu.get(index)
    }

    pub fn wall_pixel_color_bilinear(&self, index: usize, u: f32, v: f32) -> Color {
        self.wall_textures
            .get(index)
            .map_or(Color::WHITE, |image| sample_image_bilinear(image, u, v))
    }

    pub fn floor_pixel_color_bilinear(&self, u: f32, v: f32) -> Color {
        sample_image_bilinear(&self.floor_texture, u, v)
    }

    pub fn hoop_dimensions(&self) -> (i32, i32) {
        (self.hoop_idle.width().max(1), self.hoop_idle.height().max(1))
    }

    pub fn hoop_pixel_color(&self, frame: Option<usize>, tx: u32, ty: u32) -> Color {
        let image = match frame {
            None => &self.hoop_idle,
            Some(index) => self
                .hoop_score_frames
                .get(index)
                .unwrap_or(&self.hoop_idle),
        };

        let width = image.width().max(1) as u32;
        let height = image.height().max(1) as u32;
        let x = tx.min(width - 1) as i32;
        let y = ty.min(height - 1) as i32;
        image.get_color(x, y)
    }

    pub fn basketball_texture(&self) -> &Texture2D {
        &self.basketball
    }

    pub fn has_trophy_image(&self) -> bool {
        self.trophy_image.is_some()
    }

    pub fn trophy_dimensions(&self) -> Option<(i32, i32)> {
        self.trophy_image
            .as_ref()
            .map(|image| (image.width().max(1), image.height().max(1)))
    }

    pub fn trophy_image(&self) -> Option<&Image> {
        self.trophy_image.as_ref()
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

fn load_optional_image(path: &str) -> Option<Image> {
    if !Path::new(path).exists() {
        return None;
    }

    Image::load_image(path).ok()
}

fn load_basketball_texture(
    window: &mut RaylibHandle,
    raylib_thread: &RaylibThread,
    path: &str,
) -> Texture2D {
    let image = match Image::load_image(path) {
        Ok(image) => image,
        Err(_) => generate_basketball_fallback(),
    };

    window
        .load_texture_from_image(raylib_thread, &image)
        .unwrap_or_else(|_| {
            let fallback = Image::gen_image_color(64, 64, Color::MAGENTA);
            window
                .load_texture_from_image(raylib_thread, &fallback)
                .expect("failed to create fallback basketball texture")
        })
}

fn load_wall_textures_gpu(
    window: &mut RaylibHandle,
    raylib_thread: &RaylibThread,
    wall_textures: &[Image],
) -> Vec<Texture2D> {
    let mut textures = Vec::with_capacity(wall_textures.len());

    for image in wall_textures {
        let texture = window.load_texture_from_image(raylib_thread, image).unwrap_or_else(|_| {
            let fallback = Image::gen_image_color(64, 64, Color::MAGENTA);
            window
                .load_texture_from_image(raylib_thread, &fallback)
                .expect("failed to create fallback wall texture")
        });

        textures.push(texture);
    }

    textures
}

fn load_hoop_idle(path: &str) -> Image {
    match Image::load_image(path) {
        Ok(image) => image,
        Err(_) => generate_hoop_idle_fallback(),
    }
}

fn load_hoop_score_frames(dir: &str) -> Vec<Image> {
    let mut frames = Vec::with_capacity(3);

    for index in 0..3 {
        let path = format!("{dir}/hoop_score_{index}.png");
        match Image::load_image(&path) {
            Ok(image) => frames.push(image),
            Err(_) => frames.push(generate_hoop_score_fallback(index)),
        }
    }

    frames
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

fn generate_hoop_idle_fallback() -> Image {
    generate_hoop_frame(0, 255, false)
}

fn generate_hoop_score_fallback(frame_index: usize) -> Image {
    let alpha = if frame_index == 2 { 150 } else { 255 };
    generate_hoop_frame(frame_index, alpha, true)
}

fn generate_basketball_fallback() -> Image {
    let mut image = Image::gen_image_color(64, 64, Color::new(0, 0, 0, 0));
    let orange = Color::new(255, 140, 0, 255);
    let seam = Color::new(60, 30, 10, 255);

    image.draw_circle(32, 32, 28, orange);
    image.draw_circle_lines(32, 32, 28, seam);
    image.draw_line(4, 32, 60, 32, seam);
    image.draw_line(32, 4, 32, 60, seam);
    image.draw_line(10, 16, 54, 48, seam);
    image.draw_line(10, 48, 54, 16, seam);

    image
}

fn generate_hoop_frame(frame_index: usize, alpha: u8, scored: bool) -> Image {
    let mut image = Image::gen_image_color(64, 64, Color::new(0, 0, 0, 0));
    let orange = Color::new(255, 140, 0, alpha);
    let rim = Rectangle::new(18.0, 16.0, 28.0, 12.0);

    for thickness in 0..4 {
        image.draw_circle_lines(32, 22 + thickness, 14 - thickness, orange);
    }

    image.draw_rectangle_lines(rim, 2, orange);

    let net_color = Color::new(245, 245, 245, alpha.saturating_sub(30));
    let offset = match frame_index {
        0 => 0,
        1 => 2,
        _ => -2,
    };
    let wave = if scored { 2 } else { 0 };

    let lines = [
        (22, 28, 26 + offset, 44 + wave),
        (27, 28, 30 + offset, 47 + wave),
        (32, 28, 32 + offset, 48 + wave),
        (37, 28, 34 + offset, 47 + wave),
        (42, 28, 38 + offset, 44 + wave),
    ];

    for (x0, y0, x1, y1) in lines {
        image.draw_line(x0, y0, x1, y1, net_color);
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

fn sample_image_bilinear(image: &Image, u: f32, v: f32) -> Color {
    let width = image.width().max(1) as f32;
    let height = image.height().max(1) as f32;

    let x = u.clamp(0.0, 0.999_999) * (width - 1.0);
    let y = v.clamp(0.0, 0.999_999) * (height - 1.0);

    let x0 = x.floor() as i32;
    let y0 = y.floor() as i32;
    let x1 = (x0 + 1).min(image.width() - 1);
    let y1 = (y0 + 1).min(image.height() - 1);

    let tx = x - x0 as f32;
    let ty = y - y0 as f32;

    let c00 = image.get_color(x0, y0);
    let c10 = image.get_color(x1, y0);
    let c01 = image.get_color(x0, y1);
    let c11 = image.get_color(x1, y1);

    lerp_color(lerp_color(c00, c10, tx), lerp_color(c01, c11, tx), ty)
}

fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    let inv_t = 1.0 - t;

    Color::new(
        (a.r as f32 * inv_t + b.r as f32 * t).round() as u8,
        (a.g as f32 * inv_t + b.g as f32 * t).round() as u8,
        (a.b as f32 * inv_t + b.b as f32 * t).round() as u8,
        (a.a as f32 * inv_t + b.a as f32 * t).round() as u8,
    )
}
