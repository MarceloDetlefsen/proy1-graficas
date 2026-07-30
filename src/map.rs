// Fase 1: estructura de datos del mapa.
//
// Grid de 16x16. 1 = muro, 0 = espacio vacio. El borde exterior es
// siempre muro. El interior es un laberinto fijo pero navegable, con
// un punto de inicio valido para el jugador en (1.5, 1.5).

pub const MAP_WIDTH: usize = 16;
pub const MAP_HEIGHT: usize = 16;

pub const MAP: [[u8; MAP_WIDTH]; MAP_HEIGHT] = [
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 1],
    [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1],
    [1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1],
    [1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1],
    [1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1],
    [1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1],
    [1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1],
    [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1],
    [1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1],
    [1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1],
    [1, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1],
    [1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1],
    [1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

// Punto inicial valido para el jugador (celda abierta, con margen del muro).
pub const PLAYER_START_X: f32 = 1.5;
pub const PLAYER_START_Y: f32 = 1.5;
pub const PLAYER_START_ANGLE: f32 = 0.0;

/// Devuelve true si la celda que contiene (x, y) es muro (o esta fuera
/// del mapa, lo cual tratamos como muro para evitar salirse del grid).
pub fn is_wall(x: f32, y: f32) -> bool {
    if x < 0.0 || y < 0.0 {
        return true;
    }

    let ix = x as usize;
    let iy = y as usize;

    if ix >= MAP_WIDTH || iy >= MAP_HEIGHT {
        return true;
    }

    MAP[iy][ix] == 1
}
