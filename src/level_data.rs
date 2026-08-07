use crate::map::MapGrid;

#[derive(Debug)]
pub struct Level {
    pub grid: MapGrid,
    pub player_start_x: f32,
    pub player_start_y: f32,
    pub player_start_angle: f32,
    pub hoop_positions: Vec<(f32, f32)>,
    pub hoops_required: usize,
    pub level_number: u32,
    pub level_name: String,
}

pub fn build_level(index: u32) -> Level {
    match index {
        1 => build_level_1(),
        2 => build_level_2(),
        3 => build_level_3(),
        _ => panic!("Nivel invalido: {index}. Solo se admiten los niveles 1, 2 y 3."),
    }
}

pub fn build_level_1() -> Level {
    let mut grid = MapGrid::new(20, 16);
    add_border(&mut grid);

    fill_horizontal_segment(&mut grid, 5, 3, 13, Some(8));
    fill_vertical_segment(&mut grid, 10, 5, 10, Some(8));
    fill_horizontal_segment(&mut grid, 10, 5, 15, Some(12));

    let level = build_level_from_parts(
        1,
        "First Quarter Warmup",
        grid,
        1.5,
        1.5,
        0.0,
        vec![(17.5, 12.5)],
    );

    verify_level(&level);
    level
}

pub fn build_level_2() -> Level {
    let mut grid = MapGrid::new(30, 22);
    add_border(&mut grid);

    fill_horizontal_segment(&mut grid, 6, 4, 24, Some(11));
    fill_vertical_segment(&mut grid, 12, 3, 17, Some(10));
    fill_horizontal_segment(&mut grid, 14, 8, 27, Some(19));
    fill_vertical_segment(&mut grid, 22, 7, 20, Some(15));

    let level = build_level_from_parts(
        2,
        "Second Quarter Run",
        grid,
        2.5,
        2.5,
        0.0,
        vec![(5.5, 18.5), (26.5, 4.5)],
    );

    verify_level(&level);
    level
}

pub fn build_level_3() -> Level {
    let mut grid = MapGrid::new(40, 28);
    add_border(&mut grid);

    // Pasillo decorativo con una secuencia larga de paredes para mostrar las 30 texturas.
    fill_horizontal_segment(&mut grid, 13, 2, 31, Some(9));
    fill_horizontal_segment(&mut grid, 13, 2, 31, Some(17));
    fill_horizontal_segment(&mut grid, 13, 2, 31, Some(25));

    fill_vertical_segment(&mut grid, 10, 2, 23, Some(8));
    fill_vertical_segment(&mut grid, 28, 4, 25, Some(18));
    fill_horizontal_segment(&mut grid, 7, 5, 24, Some(13));
    fill_horizontal_segment(&mut grid, 20, 14, 35, Some(26));

    let level = build_level_from_parts(
        3,
        "Championship Circuit",
        grid,
        2.5,
        2.5,
        0.0,
        vec![(35.5, 4.5), (18.5, 14.5), (6.5, 24.5)],
    );

    verify_level(&level);
    level
}

fn build_level_from_parts(
    level_number: u32,
    level_name: &str,
    grid: MapGrid,
    player_start_x: f32,
    player_start_y: f32,
    player_start_angle: f32,
    hoop_positions: Vec<(f32, f32)>,
) -> Level {
    Level {
        hoops_required: hoop_positions.len(),
        grid,
        player_start_x,
        player_start_y,
        player_start_angle,
        hoop_positions,
        level_number,
        level_name: level_name.to_string(),
    }
}

fn verify_level(level: &Level) {
    assert!(
        !level.grid.is_wall(level.player_start_x, level.player_start_y),
        "El punto de inicio del nivel {} cae dentro de un muro.",
        level.level_number
    );

    for (index, (x, y)) in level.hoop_positions.iter().copied().enumerate() {
        assert!(
            !level.grid.is_wall(x, y),
            "El aro {} del nivel {} cae dentro de un muro.",
            index + 1,
            level.level_number
        );
    }
}

fn add_border(grid: &mut MapGrid) {
    let width = grid.width;
    let height = grid.height;

    for x in 0..width {
        grid.set(x, 0, 1);
        grid.set(x, height - 1, 1);
    }

    for y in 0..height {
        grid.set(0, y, 1);
        grid.set(width - 1, y, 1);
    }
}

fn fill_horizontal_segment(
    grid: &mut MapGrid,
    y: usize,
    x_start: usize,
    x_end: usize,
    gap_x: Option<usize>,
) {
    for x in x_start..=x_end {
        if gap_x == Some(x) {
            continue;
        }

        grid.set(x, y, 1);
    }
}

fn fill_vertical_segment(
    grid: &mut MapGrid,
    x: usize,
    y_start: usize,
    y_end: usize,
    gap_y: Option<usize>,
) {
    for y in y_start..=y_end {
        if gap_y == Some(y) {
            continue;
        }

        grid.set(x, y, 1);
    }
}
