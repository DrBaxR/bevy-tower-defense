use std::fmt::Debug;

use bevy::prelude::*;

mod pathfinding;
pub mod plugin;

const INFINITY: u32 = 9999999;

#[derive(Clone)]
pub struct GridNode {
    x: u32,
    y: u32,
    walkable: bool,
    f_cost: u32,
    g_cost: u32,
    h_cost: u32,
    parent: Option<(u32, u32)>, // grid coords of parent
}

impl Default for GridNode {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            walkable: true,
            f_cost: INFINITY,
            g_cost: INFINITY,
            h_cost: 0,
            parent: None,
        }
    }
}

impl Debug for GridNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GridNode")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

impl PartialEq for GridNode {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for GridNode {}

impl GridNode {
    fn get_distance(&self, other: &GridNode) -> u32 {
        let x_dist = (self.x as i32 - other.x as i32).abs();
        let y_dist = (self.y as i32 - other.y as i32).abs();

        if x_dist >= y_dist {
            return (14 * y_dist + 10 * (x_dist - y_dist)) as u32;
        }

        return (14 * x_dist + 10 * (y_dist - x_dist)) as u32;
    }
    //     fn get_distance(&self, other: &GridNode) -> u32 {
    //         let x_dist = (self.x as i32 - other.x as i32).abs();
    //         let y_dist = (self.y as i32 - other.y as i32).abs();

    //         return (x_dist * 10 + y_dist * 10) as u32;
    //     }
}

#[derive(Component)]
pub struct Grid {
    width: u32,
    height: u32,
    cell_size: Vec2,
    offset: Vec2,
    nodes: Vec<Vec<GridNode>>,
    path: Vec<GridNode>,
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
            path: Vec::new(),
        }
    }

    pub fn get_node<'a>(&'a self, world_pos: &Vec3) -> &'a GridNode {
        let coords = self.get_grid_coords(world_pos);

        &self.nodes[coords.0 as usize][coords.1 as usize]
    }

    pub fn get_node_mut<'a>(&'a mut self, node: &GridNode) -> &'a mut GridNode {
        &mut self.nodes[node.x as usize][node.y as usize]
    }

    pub fn get_grid_coords(&self, world_pos: &Vec3) -> (u32, u32) {
        (
            (((self.clamp_width(world_pos.x) - self.offset.x) / self.width as f32) as u32)
                .clamp(0, self.width - 1),
            ((-(self.clamp_height(world_pos.y) - self.offset.y) / self.height as f32) as u32)
                .clamp(0, self.height - 1),
        )
    }

    pub fn get_neighbours(&self, node: &GridNode) -> Vec<GridNode> {
        let mut neighbours = Vec::new();

        for x in -1..2 {
            for y in -1..2 {
                if x == 0 && y == 0 {
                    continue;
                }

                let new_x = node.x as i32 + x;
                let new_y = node.y as i32 + y;

                if self.in_bounds_width(new_x) && self.in_bounds_height(new_y) {
                    let neighbour = &self.nodes[new_x as usize][new_y as usize];

                    neighbours.push(neighbour.clone());
                }
            }
        }

        neighbours
    }

    pub fn reset_nodes(&mut self) {
        for i in 0..self.width {
            for j in 0..self.height {
                let node = &mut self.nodes[i as usize][j as usize];
                node.f_cost = INFINITY;
                node.h_cost = 0;
                node.g_cost = INFINITY;
                node.parent = None;
            }
        }
    }

    pub fn set_node_parent(&mut self, node: &GridNode, parent_value: Option<(u32, u32)>) {
        self.nodes[node.x as usize][node.y as usize].parent = parent_value;
    }

    fn in_bounds_width(&self, x: i32) -> bool {
        x >= 0 && x < self.width as i32
    }

    fn in_bounds_height(&self, y: i32) -> bool {
        y >= 0 && y < self.height as i32
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

    fn initialize_nodes(x: usize, y: usize) -> Vec<Vec<GridNode>> {
        let column = vec![GridNode::default(); y as usize];
        vec![column; x as usize]
    }

    fn cell_center_coords(&self, x: u32, y: u32) -> Vec3 {
        Vec3::new(
            x as f32 * self.cell_size.x + self.cell_size.x / 2. + self.offset.x,
            -(y as f32 * self.cell_size.y + self.cell_size.y / 2.) + self.offset.y,
            1.,
        )
    }

    fn retrace_path(&self, target: &GridNode) -> Vec<GridNode> {
        let mut current = target.clone();
        let mut path = vec![];

        while let Some((parent_x, parent_y)) = current.parent {
            path.push(current.clone());
            current = self.nodes[parent_x as usize][parent_y as usize].clone();
        }

        path.push(current);
        path
    }
}
