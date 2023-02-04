use std::{cell::RefCell, rc::Rc, fs};

use self::util::MapNodeType;

pub mod util;

const INFINITY: i32 = 99999999;

#[derive(Debug)]
pub struct Node {
    x: usize,
    y: usize,
    f_score: i32,
    g_score: i32,
    parent: Option<GridCoord>,
    walkable: bool,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Node {}

pub struct Grid {
    pub width: i32,
    pub height: i32,
    nodes: Matrix<Rc<RefCell<Node>>>,
}

impl Grid {
    fn get_neighbours(&self, node_coords: GridCoord) -> Vec<Rc<RefCell<Node>>> {
        let mut neighbours = vec![];

        for x in -1..2 {
            for y in -1..2 {
                if x == 0 && y == 0 {
                    continue;
                }

                let new_x = node_coords.0 + x;
                let new_y = node_coords.1 + y;
                if new_x >= 0
                    && new_x < self.width
                    && new_y >= 0
                    && new_y < self.height
                    && self.nodes[new_x as usize][new_y as usize]
                        .as_ref()
                        .borrow()
                        .walkable
                {
                    neighbours.push(Rc::clone(&self.nodes[new_x as usize][new_y as usize]));
                }
            }
        }

        neighbours
    }
}

type Matrix<T> = Vec<Vec<T>>;

type GridCoord = (i32, i32);

impl From<&str> for Grid {
    fn from(path: &str) -> Self {
        let map_str = fs::read_to_string(path).expect("Error while reading the .map file");
        let node_type_mat = util::load_map_matrix(map_str);

        let mut nodes = vec![];
        for x in 0..util::width(&node_type_mat) {
            let mut y_nodes = vec![]; 

            for y in 0..util::height(&node_type_mat) {
                let x = x as usize;
                let y = y as usize;

                y_nodes.push(Rc::new(RefCell::new(Node {
                    x,
                    y,
                    f_score: INFINITY,
                    g_score: INFINITY,
                    parent: None,
                    walkable: node_type_mat[x][y] == MapNodeType::Walkable,
                })));
            }

            nodes.push(y_nodes);
        }

        Grid {
            width: util::width(&node_type_mat) as i32,
            height: util::height(&node_type_mat) as i32,
            nodes
        }
    }
}

impl Grid {
    pub fn is_walkable(&self, coord: GridCoord) -> bool {
        self.nodes[coord.0 as usize][coord.1 as usize].as_ref().borrow().walkable
    }

    pub fn astar(&mut self, start: GridCoord, end: GridCoord) -> Option<Vec<GridCoord>> {
        let start_node = Rc::clone(&self.nodes[start.0 as usize][start.1 as usize]);
        let end_node = Rc::clone(&self.nodes[end.0 as usize][end.1 as usize]);
        let mut open: Vec<Rc<RefCell<Node>>> = vec![Rc::clone(&start_node)];

        if !start_node.borrow().walkable || !end_node.borrow().walkable {
            return None;
        }

        start_node.borrow_mut().g_score = 0;
        start_node.borrow_mut().f_score = util::distance(start, end);

        loop {
            if let Some((current_index, current)) = util::min_f(&open) {
                if *current.as_ref().borrow()
                    == *self.nodes[end.0 as usize][end.1 as usize].as_ref().borrow()
                {
                    return Some(self.reconstruct_path((
                        current.as_ref().borrow().x as i32,
                        current.as_ref().borrow().y as i32,
                    )));
                }

                open.remove(current_index);

                let current_coords = (
                    current.as_ref().borrow().x as i32,
                    current.as_ref().borrow().y as i32,
                );
                for neighbour in self.get_neighbours(current_coords) {
                    let neighbour_coords = (
                        neighbour.as_ref().borrow().x as i32,
                        neighbour.as_ref().borrow().y as i32,
                    );

                    let tentative_g = current.as_ref().borrow().g_score
                        + util::distance(current_coords, neighbour_coords);
                    if tentative_g < neighbour.as_ref().borrow().g_score {
                        {
                            let mut neighbour = neighbour.as_ref().borrow_mut();
                            neighbour.parent = Some(current_coords);
                            neighbour.g_score = tentative_g;
                            neighbour.f_score =
                                tentative_g + util::distance(current_coords, neighbour_coords);
                        }

                        if !open.iter().any(|node| {
                            let node_coords = (
                                node.as_ref().borrow().x as i32,
                                node.as_ref().borrow().y as i32,
                            );

                            node_coords.0 == neighbour_coords.0
                                && node_coords.1 == neighbour_coords.1
                        }) {
                            open.push(Rc::clone(&neighbour));
                        }
                    }
                }
            } else {
                return None;
            }
        }
    }

    fn reconstruct_path(&self, coords: GridCoord) -> Vec<GridCoord> {
        let mut path = vec![coords];

        let mut current = &self.nodes[coords.0 as usize][coords.1 as usize];
        while let Some(parent) = current.as_ref().borrow().parent {
            path.push(parent);
            current = &self.nodes[parent.0 as usize][parent.1 as usize];
        }

        path.reverse();
        path
    }
}