use bevy::prelude::*;

use crate::cursor::Cursor;

use super::a_star::Grid;

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
struct DebugGrid {
    pub position: Vec2,
    pub cell_size: f32,
    pub size_x: usize,
    pub size_y: usize,
    cells_offset: Vec2,
}

#[derive(Component)]
struct DebugNode {
    color: Color,
    x: usize,
    y: usize,
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

    fn to_screen_coords(&self, x: usize, y: usize) -> Vec2 {
        Vec2::new(
            x as f32 * self.cell_size - self.cell_size / 2.,
            y as f32 * self.cell_size - self.cell_size / 2.,
        ) + self.cells_offset
    }

    fn to_cell_coords(&self, pos: &Vec3) -> (usize, usize) {
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
}

fn spawn_grid(mut commands: Commands) {
    let grid = DebugGrid::new(Vec2::new(0., 0.), 20., 30, 20);
    const CELL_GAP: f32 = 2.;

    for i in 0..grid.size_x {
        for j in 0..grid.size_y {
            commands.spawn((
                SquareBundle::new(&grid.to_screen_coords(i, j), grid.cell_size - CELL_GAP),
                DebugNode {
                    color: Color::WHITE,
                    x: i,
                    y: j,
                },
            ));
        }
    }

    commands.spawn((grid, Name::new("Grid")));
}

fn find_and_color_path(
    cursor: Query<&Transform, With<Cursor>>,
    grid: Query<&DebugGrid>,
    mut nodes: Query<&mut DebugNode>,
) {
    for transform in cursor.iter() {
        let target = (10, 10);

        for grid in grid.iter() {
            // TODO: create this grid and set the colors from a file
            // 0000
            // 0100
            // 0110
            let mut a_star_grid = Grid::new(grid.size_x as i32, grid.size_y as i32);
            let cursor_coords = grid.to_cell_coords(&transform.translation);

            let path = a_star_grid.astar(
                (cursor_coords.0 as i32, cursor_coords.1 as i32),
                (target.0 as i32, target.1 as i32),
            );

            for mut node in nodes.iter_mut() {
                node.color = if node.x == cursor_coords.0 && node.y == cursor_coords.1 {
                    Color::RED
                } else if node.x == target.0 && node.y == target.1 {
                    Color::GREEN
                } else {
                    Color::WHITE
                };

                if let Some(path) = &path {
                    if node.color == Color::WHITE && path.contains(&(node.x as i32, node.y as i32))
                    {
                        node.color = Color::CYAN;
                    }
                }

                if node.x >= 14 && node.x <= 18 && node.y >= 9 && node.y <= 11
                    || node.x == 25 && node.y == 10
                {
                    node.color = Color::BLUE;
                }
            }
        }
    }
}

fn color_nodes(mut nodes: Query<(&mut Sprite, &DebugNode)>) {
    for (mut sprite, node) in nodes.iter_mut() {
        sprite.color = node.color;
    }
}

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_grid)
            .add_system(color_nodes)
            .add_system(find_and_color_path);
    }
}
