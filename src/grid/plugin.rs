use bevy::prelude::*;

use crate::cursor::Cursor;

use super::Grid;

const CELLS_GAP: f32 = 2.;

#[derive(Component)]
pub struct DebugGridNode {
    color: Color,
}

impl Grid {
    fn setup(&mut self, commands: &mut Commands) -> Vec<Entity> {
        let mut entities: Vec<Entity> = vec![];

        for x in 0..self.width {
            for y in 0..self.height {
                let node = &mut self.nodes[x as usize][y as usize];
                node.x = x;
                node.y = y;
                node.walkable = true;

                entities.push(
                    commands
                        .spawn((
                            Name::new("Node"),
                            SpriteBundle {
                                sprite: Sprite {
                                    color: Color::WHITE,
                                    ..default()
                                },
                                transform: Transform::from_translation(
                                    self.cell_center_coords(x, y),
                                )
                                .with_scale(Vec3::new(
                                    self.cell_size.x - CELLS_GAP,
                                    self.cell_size.y - CELLS_GAP,
                                    1.,
                                )),
                                ..default()
                            },
                            DebugGridNode { color: Color::WHITE },
                        ))
                        .id(),
                );
            }
        }

        return entities;
    }
}

fn spawn_debug_grid(mut commands: Commands) {
    let mut grid = Grid::new(20, 20, Vec2::new(400., 400.), Vec2::new(-200., 200.));
    let node_entities = grid.setup(&mut commands);

    let grid_entity = commands
        .spawn((
            Name::new("Grid"),
            VisibilityBundle::default(),
            TransformBundle::default(),
            grid,
        ))
        .id();
    commands
        .entity(grid_entity)
        .push_children(&node_entities[..]);
}

fn color_grid_nodes(mut nodes: Query<(&mut Sprite, &DebugGridNode)>) {
    for (mut sprite, node) in nodes.iter_mut() {
        sprite.color = node.color;
    }
}

fn color_cursor(
    cursors: Query<&Transform, With<Cursor>>,
    grids: Query<&Grid>,
    mut nodes: Query<(&mut DebugGridNode, &Transform)>,
) {
    let grid = grids.single();
    let cursor_transform = cursors.single();

    let grid_coord = grid.get_grid_coords(&cursor_transform.translation);
    for (mut node, transform) in nodes.iter_mut() {
        let sprite_coords = grid.get_grid_coords(&transform.translation);
        if sprite_coords.0 == grid_coord.0 && sprite_coords.1 == grid_coord.1 {
            node.color = Color::CYAN;
        } else {
            node.color = Color::WHITE;
        }
    }
}

pub struct GridPlugin {
    pub debug: bool,
}

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        if self.debug {
            app.add_startup_system(spawn_debug_grid)
                .add_system(color_cursor)
                .add_system(color_grid_nodes.after(color_cursor));
        }
    }
}