/// Grid de mapa en runtime.
///
/// `1` representa muro y `0` representa espacio vacio. El acceso usa un
/// arreglo aplanado en 1D para mantener la estructura simple y rapida.

#[derive(Clone, Debug)]
pub struct MapGrid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<u8>,
}

impl MapGrid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![0; width.saturating_mul(height)],
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: u8) {
        if x >= self.width || y >= self.height {
            return;
        }

        let index = y * self.width + x;
        self.cells[index] = value;
    }

    pub fn get(&self, x: usize, y: usize) -> u8 {
        if x >= self.width || y >= self.height {
            return 0;
        }

        self.cells[y * self.width + x]
    }

    pub fn tile_at(&self, x: f32, y: f32) -> u8 {
        if !x.is_finite() || !y.is_finite() || x < 0.0 || y < 0.0 {
            return 1;
        }

        let ix = x.floor() as usize;
        let iy = y.floor() as usize;

        if ix >= self.width || iy >= self.height {
            return 1;
        }

        self.get(ix, iy)
    }

    pub fn is_wall(&self, x: f32, y: f32) -> bool {
        self.tile_at(x, y) == 1
    }

    /// Mantiene la distribucion historica de texturas: `(x + y) % 30 + 1`.
    pub fn wall_texture_index(&self, x: i32, y: i32) -> usize {
        const WALL_TEXTURE_COUNT: i32 = 30;

        (x + y).rem_euclid(WALL_TEXTURE_COUNT) as usize + 1
    }
}
