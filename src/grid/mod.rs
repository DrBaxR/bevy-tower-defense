pub mod a_star;

use std::fs;

use bevy::prelude::*;

use crate::grid::a_star::util::MapNodeType;

use a_star::Grid;

use self::a_star::GridCoord;

const MAP_FILE_PATH: &str = "assets/walls.map";

#[derive(Bundle)]
struct SquareBundle {
    #[bundle]
    sprite: SpriteBundle,
    name: Name,
}

impl SquareBundle {
    fn new(position: &Vec2, size: f32) -> Self {
        SquareBundle {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(position.x, position.y, 0.))
                    .with_scale(Vec3::new(size, size, 1.)),
                ..default()
            },
            name: Name::new("Square"),
        }
    }
}

#[derive(Component)]
pub struct DebugGrid {
    pub position: Vec2,
    pub cell_size: f32,
    pub size_x: usize,
    pub size_y: usize,
    cells_offset: Vec2,
}

#[derive(Component)]
pub struct DebugNode {
    pub color: Color,
    pub x: usize,
    pub y: usize,
}

impl DebugGrid {
    fn new(position: Vec2, cell_size: f32, size_x: usize, size_y: usize) -> Self {
        Self {
            position,
            cell_size,
            size_x,
            size_y,
            cells_offset: position
                - Vec2::new(
                    cell_size * size_x as f32 / 2.,
                    cell_size * size_y as f32 / 2.,
                ),
        }
    }

    pub fn to_screen_coords(&self, x: usize, y: usize) -> Vec2 {
        Vec2::new(
            x as f32 * self.cell_size - self.cell_size / 2.,
            y as f32 * self.cell_size - self.cell_size / 2.,
        ) + self.cells_offset
    }

    pub fn to_cell_coords(&self, pos: &Vec3) -> (usize, usize) {
        let grid_width = self.cell_size * self.size_x as f32;
        let grid_height = self.cell_size * self.size_y as f32;

        let clamped_pos = Vec2::new(
            pos.x.clamp(
                self.position.x - grid_width / 2. - self.cell_size,
                self.position.x + grid_width / 2. - self.cell_size,
            ),
            pos.y.clamp(
                self.position.y - grid_height / 2. - self.cell_size,
                self.position.y + grid_height / 2. - self.cell_size,
            ),
        );

        let left_bottom =
            self.to_screen_coords(0, 0) - Vec2::new(self.cell_size / 2., self.cell_size / 2.);
        let current_offset = clamped_pos - left_bottom;

        (
            ((current_offset.x / self.cell_size as f32) as usize).clamp(0, self.size_x - 1),
            ((current_offset.y / self.cell_size) as usize).clamp(0, self.size_y - 1),
        )
    }

    pub fn find_path(&self, start: GridCoord, end: GridCoord) -> Option<Vec<GridCoord>> {
        // TODO: optimization - don't create the grid every time, rethink
        let mut a_star_grid = Grid::from(MAP_FILE_PATH);

        a_star_grid.astar(start, end, true)
    }
}

fn spawn_grid(mut commands: Commands, debug: bool) {
    let map_str = fs::read_to_string(MAP_FILE_PATH).expect("Could not read .map file");
    let (width, height) = a_star::util::get_map_size(&map_str);
    let map_grid = a_star::util::load_map_matrix(map_str);
    let grid = DebugGrid::new(Vec2::new(0., 0.), 20., width, height);
    const CELL_GAP: f32 = 2.;

    if debug {
        for i in 0..grid.size_x {
            for j in 0..grid.size_y {
                commands.spawn((
                    SquareBundle::new(&grid.to_screen_coords(i, j), grid.cell_size - CELL_GAP),
                    DebugNode {
                        color: if map_grid[i][j] == MapNodeType::Walkable {
                            Color::WHITE
                        } else {
                            Color::BLUE
                        },
                        x: i,
                        y: j,
                    },
                ));
            }
        }
    }

    commands.spawn((grid, Name::new("Grid")));
}

fn color_nodes(mut nodes: Query<(&mut Sprite, &DebugNode)>) {
    for (mut sprite, node) in nodes.iter_mut() {
        sprite.color = node.color;
    }
}

pub struct GridPlugin {
    pub debug: bool,
}

impl Plugin for GridPlugin {
    // TODO: debug show path with lines
    fn build(&self, app: &mut App) {
        let debug = self.debug;
        app.add_startup_system_to_stage(StartupStage::PreStartup, move |commands: Commands| {
            spawn_grid(commands, debug)
        });

        if debug {
            app.add_system(color_nodes);
            // .add_system(find_and_color_path);
        }
    }
}
