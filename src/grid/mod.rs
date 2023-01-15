use bevy::prelude::*;

pub mod plugin;

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

    pub fn get_grid_coords(&self, world_pos: &Vec3) -> (u32, u32) {
        (
            (((self.clamp_width(world_pos.x) - self.offset.x) / self.width as f32) as u32)
                .clamp(0, self.width - 1),
            ((-(self.clamp_height(world_pos.y) - self.offset.y) / self.height as f32) as u32)
                .clamp(0, self.height - 1),
        )
    }

    fn clamp_width(&self, x: f32) -> f32 {
        x.clamp(
            self.offset.x,
            self.offset.x + self.width as f32 * self.cell_size.x,
        )
    }

    fn clamp_height(&self, y: f32) -> f32 {
        y.clamp(
            self.offset.y - self.height as f32 * self.cell_size.y,
            self.offset.y,
        )
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


    fn cell_center_coords(&self, x: u32, y: u32) -> Vec3 {
        Vec3::new(
            x as f32 * self.cell_size.x + self.cell_size.x / 2. + self.offset.x,
            -(y as f32 * self.cell_size.y + self.cell_size.y / 2.) + self.offset.y,
            1.,
        )
    }
}
