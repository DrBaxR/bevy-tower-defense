use std::{rc::Rc, cell::RefCell};

use super::{Matrix, Node, GridCoord};


#[derive(Debug, PartialEq, Clone)]
pub enum MapNodeType {
    Walkable,
    Obstacle,
}

pub fn width<T>(mat: &Matrix<T>) -> usize {
    mat.len()
}

pub fn height<T>(mat: &Matrix<T>) -> usize {
    mat.get(0).map(|line| line.len()).unwrap_or(0)
}

pub fn get_map_size(map_str: &String) -> (usize, usize) {
    let split: Vec<&str> = map_str.split("\n").collect();

    (split[0].len(), split.len())
}

pub fn load_map_matrix(map_str: String) -> Matrix<MapNodeType> {
    let (width, height) = get_map_size(&map_str);
    let mut matrix: Matrix<MapNodeType> = vec![vec![MapNodeType::Walkable; height]; width];

    map_str.split("\n").enumerate().for_each(|(line_number, line)| {
        line.chars().enumerate().for_each(|(char_number, char)| {
            let node_type = match char {
                '1' => MapNodeType::Obstacle,
                _ => MapNodeType::Walkable
            };

            matrix[char_number][height - line_number - 1] = node_type;
        });
    });

    matrix
}

pub fn distance(start: GridCoord, end: GridCoord) -> i32 {
    let dif_x = (start.0 - end.0).abs();
    let dif_y = (start.1 - end.1).abs();

    if dif_x > dif_y {
        return 14 * dif_y + (dif_x - dif_y) * 10;
    }

    return 14 * dif_x + (dif_y - dif_x) * 10;
}

pub fn min_f(nodes: &Vec<Rc<RefCell<Node>>>) -> Option<(usize, Rc<RefCell<Node>>)> {
    let mut min_index = 0;
    if let Some(mut min) = nodes.get(0) {
        for node in nodes.iter().enumerate().skip(1) {
            if node.1.as_ref().borrow().f_score < min.as_ref().borrow().f_score {
                min = node.1;
                min_index = node.0;
            }
        }

        Some((min_index, Rc::clone(min)))
    } else {
        None
    }
}
