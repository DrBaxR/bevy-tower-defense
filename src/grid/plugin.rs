use bevy::{ecs::bundle, prelude::*};

use crate::cursor::Cursor;

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

fn find_path(
    cursor: Query<&Transform, With<Cursor>>,
    grid: Query<&DebugGrid>,
    mut nodes: Query<&mut DebugNode>,
) {
    for transform in cursor.iter() {
        for grid in grid.iter() {
            let cursor_coords = grid.to_cell_coords(&transform.translation);

            for mut node in nodes.iter_mut() {
                if node.x == cursor_coords.0 && node.y == cursor_coords.1 {
                    node.color = Color::RED;
                } else {
                    node.color = Color::WHITE;
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
            .add_system(find_path);
    }
}
