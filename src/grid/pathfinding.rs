use std::ops::Deref;

use bevy::prelude::{Vec3, Component};

use super::{Grid, GridNode};

#[derive(Component)]
pub struct Pathfinder {
    pub grid: Grid,
    pub path: Vec<GridNode>,
}

impl Pathfinder {
    // find path with A* pathfinding
    pub fn find_path(&mut self, start: &Vec3, target: &Vec3) {
        let start_node = self.grid.get_node(start).clone();
        let target_node = self.grid.get_node(target).clone();

        let mut open_nodes: Vec<GridNode> = Vec::new();
        let mut closed_nodes: Vec<GridNode> = Vec::new();

        open_nodes.push(self.grid.get_node(start).clone());

        loop {
            if let Some((min_index, current_node)) = get_least_f_cost(&open_nodes) {
                let current_node = current_node.clone();

                open_nodes.remove(min_index);
                closed_nodes.push(current_node.clone());

                if current_node == target_node {
                    // done
                    self.path = self.retrace_path(&start_node, &target_node);
                    break;
                }

                // get children
                let children: Vec<GridNode> = self
                    .grid
                    .get_neighbours(&current_node)
                    .iter()
                    .map(|node| node.deref().clone())
                    .collect();
                // iterate children
                for mut child in children {
                    if closed_nodes.iter().any(|node| *node == child) || !child.walkable {
                        continue;
                    }

                    // calculate costs
                    // TODO: these should be set on the actial grid
                    let grid_node = self.grid.get_grid_node_mut(&child);
                    child.g_cost = compute_g_cost(&child, &current_node);
                    child.h_cost = compute_h_cost(&child, &target_node);

                    grid_node.g_cost = child.g_cost;
                    grid_node.h_cost = child.h_cost;

                    let new_f_cost = child.g_cost + child.h_cost;
                    if new_f_cost < child.f_cost || !open_nodes.iter().any(|node| *node == child) {
                        // optimization
                        child.f_cost = new_f_cost;
                        grid_node.f_cost = new_f_cost;

                        // self.grid.set_node_parent(&child, Some((current_node.x, current_node.y)));
                        grid_node.parent = Some((current_node.x, current_node.y));
                        if !open_nodes.iter().any(|node| *node == child) {
                            open_nodes.push(child);
                        }
                    }
                }
            } else {
                // means that list is empty
                break;
            }
        }
    }

    fn retrace_path(&self, start_node: &GridNode, target_node: &GridNode) -> Vec<GridNode> {
        let mut path: Vec<GridNode> = Vec::new();
        let mut current_node = target_node;

        while *current_node != *start_node {
            path.push(current_node.clone());
            if let Some((parent_x, parent_y)) = current_node.parent {
                current_node = &self.grid.nodes[parent_x as usize][parent_y as usize];
            } else {
                break;
            }
        }
        path.reverse();

        path
    }
}

fn get_least_f_cost<'a>(nodes: &'a Vec<GridNode>) -> Option<(usize, &'a GridNode)> {
    if let Some(node) = nodes.get(0) {
        let mut min = (0, node);

        nodes.iter().skip(1).enumerate().for_each(|(index, node)| {
            if node.f_cost < min.1.f_cost {
                min = (index, node);
            }
        });

        Some(min)
    } else {
        None
    }
}

fn compute_g_cost(child: &GridNode, parent: &GridNode) -> u32 {
    let distance_x: i32 = child.x as i32 - parent.x as i32;
    let distance_y: i32 = child.y as i32 - parent.y as i32;

    let extra = if distance_x != 0 && distance_y != 0 {
        14
    } else {
        0
    };

    return parent.g_cost + extra;
}

fn compute_h_cost(child: &super::GridNode, target: &super::GridNode) -> u32 {
    let distance_x: i32 = (child.x as i32 - target.x as i32).abs();
    let distance_y: i32 = (child.y as i32 - target.y as i32).abs();

    let (diagonal_movement, straight_movement) = if distance_x < distance_y {
        (distance_x * 14, (distance_y - distance_x) * 10)
    } else {
        (distance_y * 14, (distance_x - distance_y) * 10)
    };

    diagonal_movement as u32 + straight_movement as u32
}
