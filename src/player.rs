/// Guarda posicion (x, y) en punto flotante, angulo de vision, y velocidades de movimiento/rotacion. try_move() aplica colision
/// contra muros revisando cada eje por separado, lo que de paso permite "deslizarse" a lo largo de una pared en vez de quedar pegado.

use crate::map::MapGrid;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub move_speed: f32, // celdas por segundo
    pub rot_speed: f32,  // radianes por segundo
}

impl Player {
    pub fn new(x: f32, y: f32, angle: f32) -> Self {
        Self {
            x,
            y,
            angle,
            move_speed: 3.0,
            rot_speed: 2.5,
        }
    }

    pub fn rotate(&mut self, delta_angle: f32) {
        self.angle += delta_angle;
    }

    /// Intenta mover al jugador por (dx, dy). Cada eje se comprueba por separado contra el mapa, con un pequeno margen para que la camara no quede pegada justo en el borde de la pared.
    pub fn try_move(&mut self, dx: f32, dy: f32, map: &MapGrid) {
        const WALL_MARGIN: f32 = 0.2;

        if dx != 0.0 {
            let target_x = self.x + dx;
            let probe_x = target_x + WALL_MARGIN * dx.signum();
            if !map.is_wall(probe_x, self.y) {
                self.x = target_x;
            }
        }

        if dy != 0.0 {
            let target_y = self.y + dy;
            let probe_y = target_y + WALL_MARGIN * dy.signum();
            if !map.is_wall(self.x, probe_y) {
                self.y = target_y;
            }
        }
    }
}
