use bevy::{ecs::bundle, prelude::*};

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

struct DebugGrid {
    pub position: Vec2,
    pub cell_size: f32,
    pub size_x: usize,
    pub size_y: usize,
    cells_offset: Vec2,
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
            (x * self.size_x) as f32 - self.cell_size / 2.,
            (y * self.size_y) as f32 - self.cell_size / 2.,
        ) + self.cells_offset
    }
}

fn spawn_grid(mut commands: Commands) {
    let grid = DebugGrid::new(Vec2::new(0., 0.), 20., 20, 20);
    const CELL_GAP: f32 = 2.;

    for i in 0..grid.size_x {
        for j in 0..grid.size_y {
            commands.spawn(SquareBundle::new(&grid.to_screen_coords(i, j), grid.cell_size - CELL_GAP));
        }
    }
}

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_grid);
    }
}
