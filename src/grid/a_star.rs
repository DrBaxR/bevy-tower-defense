use std::{cell::RefCell, rc::Rc, fs};

const INFINITY: i32 = 99999999;

#[derive(Debug)]
struct Node {
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

#[derive(Debug, PartialEq, Clone)]
enum MapNodeType {
    Walkable,
    Obstacle,
}

type Matrix<T> = Vec<Vec<T>>;

fn width<T>(mat: &Matrix<T>) -> usize {
    mat.len()
}

fn height<T>(mat: &Matrix<T>) -> usize {
    mat.get(0).map(|line| line.len()).unwrap_or(0)
}

type GridCoord = (i32, i32);

fn distance(start: GridCoord, end: GridCoord) -> i32 {
    let dif_x = (start.0 - end.0).abs();
    let dif_y = (start.1 - end.1).abs();

    if dif_x > dif_y {
        return 14 * dif_y + (dif_x - dif_y) * 10;
    }

    return 14 * dif_x + (dif_y - dif_x) * 10;
}

fn min_f(nodes: &Vec<Rc<RefCell<Node>>>) -> Option<(usize, Rc<RefCell<Node>>)> {
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

impl From<&str> for Grid {
    fn from(path: &str) -> Self {
        let map_str = fs::read_to_string(path).expect("Error while reading the .map file");
        let node_type_mat = Grid::load_map_matrix(map_str);

        let mut nodes = vec![];
        for x in 0..width(&node_type_mat) {
            let mut y_nodes = vec![]; 

            for y in 0..height(&node_type_mat) {
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
            width: width(&node_type_mat) as i32,
            height: height(&node_type_mat) as i32,
            nodes
        }
    }
}

impl Grid {
    pub fn is_walkable(&self, coord: GridCoord) -> bool {
        self.nodes[coord.0 as usize][coord.1 as usize].as_ref().borrow().walkable
    }

    fn load_map_matrix(map_str: String) -> Matrix<MapNodeType> {
        let (width, height) = Grid::get_map_size(&map_str);
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

    pub fn get_map_size(map_str: &String) -> (usize, usize) {
        let split: Vec<&str> = map_str.split("\n").collect();

        (split[0].len(), split.len())
    }

    pub fn astar(&mut self, start: GridCoord, end: GridCoord) -> Option<Vec<GridCoord>> {
        let start_node = Rc::clone(&self.nodes[start.0 as usize][start.1 as usize]);
        let mut open: Vec<Rc<RefCell<Node>>> = vec![Rc::clone(&start_node)];

        start_node.borrow_mut().g_score = 0;
        start_node.borrow_mut().f_score = distance(start, end);

        loop {
            if let Some((current_index, current)) = min_f(&open) {
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
                        + distance(current_coords, neighbour_coords);
                    if tentative_g < neighbour.as_ref().borrow().g_score {
                        {
                            let mut neighbour = neighbour.as_ref().borrow_mut();
                            neighbour.parent = Some(current_coords);
                            neighbour.g_score = tentative_g;
                            neighbour.f_score =
                                tentative_g + distance(current_coords, neighbour_coords);
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
