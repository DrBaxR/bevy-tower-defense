use bevy::prelude::Vec3;

use super::{Grid, GridNode};

fn get_least_f_cost(nodes: &Vec<GridNode>) -> Option<(usize, GridNode)> {
    if let Some(node) = nodes.get(0) {
        let mut min = (0, node);

        nodes.iter().skip(1).enumerate().for_each(|(index, node)| {
            if node.f_cost < min.1.f_cost
                || node.f_cost == min.1.f_cost && node.h_cost < min.1.h_cost
            {
                min = (index, node);
            }
        });

        Some((min.0, min.1.clone()))
    } else {
        None
    }
}

pub trait GridPathfinder {
    fn find_path(&mut self, start: &Vec3, target: &Vec3);
}

impl GridPathfinder for Grid {
    fn find_path(&mut self, start: &Vec3, target: &Vec3) {
        let mut open_set: Vec<GridNode> = vec![self.get_node(start).clone()];
        let mut closed_set: Vec<GridNode> = Vec::new();

        loop {
            let min_cost = get_least_f_cost(&open_set);
            if let Some((min_index, current_node)) = min_cost {
                open_set.remove(min_index);
                closed_set.push(current_node.clone());

                let target_node = self.get_node(target).clone();
                if current_node == target_node {
                    self.path = self.retrace_path(&target_node);
                    self.reset_nodes();
                    return;
                }

                let neighbours = self.get_neighbours(&current_node);
                for neighbour in neighbours.iter() {
                    if !neighbour.walkable || closed_set.iter().any(|node| *node == *neighbour) {
                        continue;
                    }

                    let new_movement_cost =
                        current_node.g_cost + current_node.get_distance(&neighbour);
                    if new_movement_cost < neighbour.g_cost
                        || !open_set.iter().any(|node| *node == *neighbour)
                    {
                        let n = self.get_node_mut(neighbour);
                        n.g_cost = new_movement_cost;
                        n.h_cost = neighbour.get_distance(&target_node);
                        n.f_cost = neighbour.g_cost + neighbour.h_cost;
                        n.parent = Some((current_node.x, current_node.y));

                        if !open_set.iter().any(|node| *node == *neighbour) {
                            open_set.push((*neighbour).clone());
                        }
                    }
                }
            } else {
                // there are no nodes in the open set
                break;
            }
        }
    }
}
