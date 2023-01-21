use bevy::prelude::*;

use crate::cursor::Cursor;

use super::{Grid, pathfinding::GridPathfinder};

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

                if x > 4 && x <= 7 && y > 10 && y <= 12 || x > 3 && x <= 5 && y > 1 && y <= 11 {
                    node.walkable = false;
                }

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
            grid
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

fn color_path(
    mut pathfinders: Query<&mut Grid>,
    mut grid_nodes: Query<(&Transform, &mut DebugGridNode)>,
    cursors: Query<&Transform, With<Cursor>>
) {
    let cursor = cursors.single();

    for (_, mut node) in grid_nodes.iter_mut() {
        node.color = Color::WHITE;
    }

    for mut pathfinder in pathfinders.iter_mut() {
        pathfinder.find_path(&cursor.translation, &Vec3::new(1., 1., 1.));

        for (node_transform, mut node) in grid_nodes.iter_mut() {
            for path_node in pathfinder.path.iter() {
                let current_node = pathfinder.get_node(&node_transform.translation);
                if current_node == path_node {
                    node.color = Color::RED;
                }

                if !current_node.walkable {
                    node.color = Color::BLUE;
                }
            }
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
                .add_system(color_path)
                .add_system(color_grid_nodes.after(color_path));
        }
    }
}