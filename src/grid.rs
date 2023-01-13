use bevy::prelude::*;

const CELLS_GAP: f32 = 2.;

// TODO: this will probably be used as a resource
pub struct Grid {
    width: u32,
    height: u32,
    cell_size: Vec2,
    offset: Vec2,
}

impl Grid {
    pub fn new(grid_width: u32, grid_height: u32, pixels_size: Vec2, position: Vec2) -> Self {
        Self {
            width: grid_width,
            height: grid_height,
            cell_size: Vec2::new(
                pixels_size.x / grid_width as f32,
                pixels_size.y / grid_height as f32,
            ),
            offset: position,
        }
    }

    pub fn setup(&self, commands: &mut Commands) {
        for x in 0..self.width {
            for y in 0..self.height {
                commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::WHITE,
                        ..default()
                    },
                    transform: Transform::from_translation(self.cell_center_coords(x, y))
                        .with_scale(Vec3::new(
                            self.cell_size.x - CELLS_GAP,
                            self.cell_size.y - CELLS_GAP,
                            1.,
                        )),
                    ..default()
                });
            }
        }
    }

    fn cell_center_coords(&self, x: u32, y: u32) -> Vec3 {
        Vec3::new(
            x as f32 * self.cell_size.x + self.cell_size.x / 2. + self.offset.x,
            -(y as f32 * self.cell_size.y + self.cell_size.y / 2.) + self.offset.y,
            1.,
        )
    }
}
