use bevy::{ecs::system::EntityCommands, prelude::*};

const CELLS_GAP: f32 = 2.;

#[derive(Component)]
pub struct DebugGridNode;

#[derive(Clone)]
pub struct Node {
    x: u32,
    y: u32,
    walkable: bool,
}

#[derive(Component)]
pub struct Grid {
    width: u32,
    height: u32,
    cell_size: Vec2,
    offset: Vec2,
    nodes: Vec<Vec<Node>>,
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
            nodes: Grid::initialize_nodes(grid_width as usize, grid_height as usize),
        }
    }

    fn initialize_nodes(x: usize, y: usize) -> Vec<Vec<Node>> {
        let column = vec![
            Node {
                x: 0,
                y: 0,
                walkable: true
            };
            y as usize
        ];
        vec![column; x as usize]
    }

    pub fn setup(&mut self, commands: &mut Commands) -> Vec<Entity> {
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
                            DebugGridNode,
                        ))
                        .id(),
                );
            }
        }

        return entities;
    }

    fn cell_center_coords(&self, x: u32, y: u32) -> Vec3 {
        Vec3::new(
            x as f32 * self.cell_size.x + self.cell_size.x / 2. + self.offset.x,
            -(y as f32 * self.cell_size.y + self.cell_size.y / 2.) + self.offset.y,
            1.,
        )
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

fn color_grid_nodes(mut nodes: Query<&mut Sprite, With<DebugGridNode>>) {
    for mut sprite in nodes.iter_mut() {
        sprite.color = Color::WHITE;
    }
}

pub struct GridPlugin {
    pub debug: bool,
}

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        if self.debug {
            app.add_startup_system(spawn_debug_grid)
                .add_system(color_grid_nodes);
        }
    }
}
