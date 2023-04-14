pub mod maze {
    use std::ops::ControlFlow;
    use std::collections::{HashSet, VecDeque};
    // use rand::prelude::SliceRandom;
    use rand::seq::SliceRandom;
    // 0.7.2
    use rand::Rng;

    pub type Coord = (usize, usize);

    #[derive(Debug, Clone)]
    pub struct HorizontalWall {
        coord: Coord,
    }

    #[derive(Debug, Clone)]
    pub struct VerticalWall {
        coord: Coord,
    }

    #[derive(Debug)]
    pub enum Wall {
        VerticalWall(VerticalWall),
        HorizontalWall(HorizontalWall),
    }

    #[derive(Clone, PartialEq, Debug)]
    pub enum Direction {
        North,
        South,
        East,
        West,
    }

    impl Default for Direction {
        fn default() -> Self {
            Direction::North
        }
    }

    #[derive(Debug)]
    pub struct MazeCell {
        cost: Option<usize>,
        coord: Coord,
        top: VerticalWall,
        down: VerticalWall,
        left: HorizontalWall,
        right: HorizontalWall,
    }

    pub struct Maze {
        pub horizontal_walls: Vec<Vec<bool>>,
        pub vertical_walls: Vec<Vec<bool>>,
        pub start: Coord,
        pub end: Coord,
        pub cost: Vec<Vec<Option<usize>>>,
    }

    impl Maze {
        pub fn get_maze_cell(&self, row_index: i32, col_index: i32) -> Option<MazeCell> {
            if row_index >= self.horizontal_walls.len() as i32 || col_index >= self.vertical_walls.len() as i32 || row_index < 0 || col_index < 0 {
                None
            } else {
                let row_index = row_index as usize;
                let col_index = col_index as usize;
                Some(MazeCell {
                    coord: (row_index, col_index),
                    cost: self.cost[row_index][col_index],
                    top: VerticalWall {
                        coord: (row_index, col_index),
                    },
                    down: VerticalWall {
                        coord: (row_index + 1, col_index),
                    },
                    left: HorizontalWall {
                        coord: (row_index, col_index),
                    },
                    right: HorizontalWall {
                        coord: (row_index, col_index + 1),
                    },
                })
            }
        }

        pub fn get_cell_by_direction(&self, maze_cell: &MazeCell, direction: &Direction) -> Option<MazeCell> {
            match direction {
                Direction::North => {
                    if self.is_traversable(&Wall::VerticalWall(maze_cell.top.clone())) {
                        self.get_maze_cell(maze_cell.coord.0 as i32 - 1, maze_cell.coord.1 as i32)
                    } else {
                        None
                    }
                }
                Direction::South => {
                    if self.is_traversable(&Wall::VerticalWall(maze_cell.down.clone())) {
                        self.get_maze_cell(maze_cell.coord.0 as i32 + 1, maze_cell.coord.1 as i32)
                    } else {
                        None
                    }
                }
                Direction::East => {
                    if self.is_traversable(&Wall::HorizontalWall(maze_cell.right.clone())) {
                        self.get_maze_cell(maze_cell.coord.0 as i32, maze_cell.coord.1 as i32 + 1)
                    } else {
                        None
                    }
                }
                Direction::West => {
                    if self.is_traversable(&Wall::HorizontalWall(maze_cell.left.clone())) {
                        self.get_maze_cell(maze_cell.coord.0 as i32, maze_cell.coord.1 as i32 - 1)
                    } else {
                        None
                    }
                }
            }
        }

        pub fn is_traversable<'a>(&self, wall: &Wall) -> bool {
            match wall {
                Wall::HorizontalWall(HorizontalWall { coord: (row, col) }) => {
                    if *row >= self.horizontal_walls.len() || *col >= self.horizontal_walls[0].len() {
                        false
                    } else {
                        !self.horizontal_walls[*row][*col]
                    }
                }
                Wall::VerticalWall(VerticalWall { coord: (row, col) }) => {
                    // println!("({},{})", row, col);
                    if *row >= self.vertical_walls.len() || *col >= self.vertical_walls[0].len() {
                        false
                    } else {
                        !self.vertical_walls[*col][*row]
                    }
                }
            }
        }

        pub fn set_wall_by_cell(
            &mut self,
            row_cell_index: usize,
            col_cell_index: usize,
            direction: Option<Direction>,
            value: bool,
        ) {
            let maze_cell_value = self.get_maze_cell(row_cell_index as i32, col_cell_index as i32);
            // println!("{:?}", cell_value);
            match maze_cell_value {
                Some(maze_cell) => match direction {
                    Some(Direction::North) => self.set_wall(&Wall::VerticalWall(maze_cell.top), value),
                    Some(Direction::East) => self.set_wall(&Wall::HorizontalWall(maze_cell.right), value),
                    Some(Direction::South) => self.set_wall(&Wall::VerticalWall(maze_cell.down), value),
                    Some(Direction::West) => self.set_wall(&Wall::HorizontalWall(maze_cell.left), value),
                    None => {}
                },
                None => {
                    // panic!("illegal cell indexes");
                }
            }
        }

        pub fn set_wall(&mut self, wall: &Wall, value: bool) {
            match wall {
                Wall::HorizontalWall(HorizontalWall { coord: (row, col) }) => {
                    self.horizontal_walls[*row][*col] = value;
                }
                Wall::VerticalWall(VerticalWall { coord: (row, col) }) => {
                    self.vertical_walls[*col][*row] = value;
                }
            }
        }

        pub fn new(width: usize, height: usize, start: Coord) -> Self {
            if width <= 0 || height <= 0 {
                panic!("illegal dimensions")
            } else {
                let mut result = Maze {
                    start: start.clone(),
                    end: (0, 0),
                    horizontal_walls: std::iter::repeat(
                        std::iter::repeat(true).take(width + 1).collect()
                    ).take(height).collect(),
                    vertical_walls: std::iter::repeat(
                        std::iter::repeat(true).take(height + 1).collect()
                    )
                        .take(width)
                        .collect(),
                    cost: std::iter::repeat(
                        std::iter::repeat(None).take(width).collect()
                    ).take(height).collect(),
                };
                // maze.generate_binary_maze();
                // maze.generate_random_walk_maze();
                // maze.generate_maze_via_dfs((0, 0), HashSet::new(), width, height);
                result.generate_maze_via_dfs_heap(start, width, height);
                let end = result.find_farthest_point(&start, width, height);
                result.end = end;
                result.fill_cost();
                result
            }
        }
        pub fn get_valid_adjascent_cells(&self, row: usize, col: usize) -> Vec<MazeCell> {
            let maze_cells: Vec<Option<MazeCell>> = [Direction::South, Direction::West, Direction::North, Direction::East].iter().map(|d| {
                match &self.get_maze_cell(row as i32, col as i32) {
                    None => None,
                    Some(maze_cell) => {
                        self.get_cell_by_direction(&maze_cell, d)
                    }
                }
            }).filter(|x| x.is_some()).collect();
            // println!("{:?}", maze_cells);
            maze_cells.into_iter()
                .map(|x| x.unwrap())
                .filter(|x| x.cost.is_none()).collect::<Vec<MazeCell>>()
        }

        pub fn fill_cost(&mut self) {
            let start = self.start.clone();
            let mut queue: VecDeque<(Coord, usize)> = VecDeque::new();
            queue.push_front((start.clone(), 0));
            while let Some(((row, col), cost)) = queue.pop_back() {
                self.get_valid_adjascent_cells(row, col).iter()
                    .for_each(|x| {
                        self.cost[x.coord.0][x.coord.1] = Some(cost + 1);
                        queue.push_front((x.coord.clone(), cost + 1));
                    });
            }
        }

        pub fn generate_binary_maze(&mut self) {
            fn random_direction(excludes: &[Direction]) -> Option<Direction> {
                let directions =
                    [Direction::North, /*Direction::South,*/ Direction::East, /*Direction::West*/];
                // .filter(|direction| !excludes.contains(direction)).collect();

                match directions.choose(&mut rand::thread_rng()) {
                    Some(x) if !excludes.contains(x) => Some(x.clone()),
                    Some(_) => None,
                    None => None
                }
            }

            fn indexes_to_exclude(row_index: usize, col_index: usize, width: usize, height: usize) -> Vec<Direction> {
                let mut result = Vec::with_capacity(2);
                if row_index == 0 {
                    result.push(Direction::North);
                }
                if col_index == 0 {
                    result.push(Direction::West);
                }

                if row_index == height - 1 {
                    result.push(Direction::South);
                }

                if col_index == width - 1 {
                    result.push(Direction::East);
                }
                // println!("{:?}, ({}, {})", result, row_index, col_index);
                result
            }

            let width = self.vertical_walls.len();
            let height = self.horizontal_walls.len();
            (0..width).for_each(|cell_row_index| {
                (0..height).for_each(|cell_col_index| {
                    self.set_wall_by_cell(cell_row_index, cell_col_index, random_direction(&indexes_to_exclude(cell_row_index, cell_col_index, width, height)), false);
                    // indexes_to_exclude(cell_row_index, cell_col_index, width, height).iter().for_each(|direction|{
                    //     self.set_wall_by_cell(width - 1, 1, direction.clone(), false);
                    // });
                })
            })
        }

        fn get_next<'a>(coord: (usize, usize), acc: &HashSet<(usize, usize)>, width: usize, height: usize) -> Vec<((usize, usize), Direction)> {
            let (row, col) = coord;
            let result: Vec<((usize, usize), Direction)> =
                vec![((row as i32 + 1, col as i32), Direction::South), ((row as i32 - 1, col as i32), Direction::North), ((row as i32, col as i32 + 1), Direction::East), ((row as i32, col as i32 - 1), Direction::West)]
                    .iter()
                    .filter(|((x, y), _)| {
                        (*x >= 0 && *y >= 0 && *y < height as i32 && *x < width as i32) && !acc.contains(&(*x as usize, *y as usize))
                    })
                    .map(|((x, y), d)| ((*x as usize, *y as usize), d.clone())).collect::<Vec<((usize, usize), Direction)>>();
            // if row == 0 || col == 0 || row == width - 1 || col == height - 1 {
            //     println!("{:?} -> {:?}", (row, col), result);
            // }
            result
        }

        pub fn generate_random_walk_maze(&mut self) {
            let width = self.vertical_walls.len();
            let height = self.horizontal_walls.len();
            let mut rng = rand::thread_rng();
            let mut start = (rng.gen_range(0..width), rng.gen_range(0..height));
            let area = width * height;
            let mut acc: HashSet<(usize, usize)> = HashSet::with_capacity(area);

            while acc.len().clone() < area {
                acc.insert(start.clone());
                match Self::get_next(start, &acc, width, height).choose(&mut rng) {
                    None => {
                        (0..width).map(|x| (0..height).map(move |y| (x, y))).flatten().try_for_each(|coord| {
                            match acc.contains(&coord) {
                                true => { ControlFlow::Continue(()) }
                                false => {
                                    start = coord;
                                    ControlFlow::Break(())
                                }
                            }
                        });
                    }
                    Some(((x, y), direction)) => {
                        // if *x == 0 {
                        // println!("{:?}", ((x, y), direction));
                        let new_direction = match direction {
                            Direction::North => Direction::South,
                            Direction::South => Direction::North,
                            Direction::East => Direction::West,
                            Direction::West => Direction::East
                        };
                        self.set_wall_by_cell(*x, *y, Some(new_direction), false);
                        // }
                        start = (*x, *y);
                    }
                }
            }
        }

        pub fn find_farthest_point(&self, start: &(usize, usize), width: usize, height: usize) -> (usize, usize) {
            let mut queue: VecDeque<((usize, usize), usize)> = VecDeque::with_capacity(width * height);
            let mut acc: HashSet<(usize, usize)> = HashSet::with_capacity(width * height);
            queue.push_front((start.clone(), 0));
            acc.insert(start.clone());
            let mut coord_with_max_distance = (start.clone(), 0);
            while let Some((coord, cost)) = queue.pop_back() {
                // println!("{}", queue.len());
                if coord_with_max_distance.1 < cost {
                    coord_with_max_distance = (coord, cost);
                }
                let other: Vec<Coord> = [Direction::North, Direction::East, Direction::West, Direction::South]
                    .iter()
                    .map(|direction| {
                        match self.get_maze_cell(coord.0 as i32, coord.1 as i32) {
                            None => None,
                            Some(maze_cell) => self.get_cell_by_direction(&maze_cell, &direction)
                        }
                    }).filter(|x| !x.is_none())
                    .map(|x| x.unwrap().coord)
                    .filter(|x| !(&acc).contains(x)).collect();
                // println!("{:?}", other);
                other.iter().for_each(|(next_row, next_col)| {
                    queue.push_front(((*next_row, *next_col), cost + 1));
                    let _ = &acc.insert((*next_row, *next_col));
                });
            }
            coord_with_max_distance.0
        }

        pub fn shuffle<T: Default>(mut collection: Vec<T>) -> Vec<T> {
            let mut rng = rand::thread_rng();
            for index in 0..collection.len() {
                let length = collection.len();
                let new_range = (index as usize)..length;
                if !new_range.is_empty() {
                    collection.swap(index, rng.gen_range(new_range));
                }
            }
            collection
        }
        pub fn generate_maze_via_dfs_heap(&mut self, mut start: (usize, usize), width: usize, height: usize) {
            let mut stack: VecDeque<(usize, usize)> = VecDeque::with_capacity(width * height);
            let mut acc: HashSet<(usize, usize)> = HashSet::with_capacity(width * height);
            stack.push_back(start.clone());
            while stack.len() > 0 {
                start = stack.pop_back().unwrap();
                acc.insert(start.clone());
                let adj = Self::get_next(start.clone(), &acc, width, height);
                if let Some(((next_row, next_col), direction)) = adj.choose(&mut rand::thread_rng()) {
                    self.set_wall_by_cell(start.0, start.1, Some(direction.clone()), false);
                    stack.push_back(start.clone());
                    stack.push_back((*next_row, *next_col));
                }
            }
        }

        pub fn generate_maze_via_dfs(&mut self, start: (usize, usize), mut acc: HashSet<(usize, usize)>, width: usize, height: usize) -> HashSet<(usize, usize)> {
            acc.insert(start.clone());
            let mut other = Self::get_next(start.clone(), &acc, width, height);
            other = Self::shuffle(other);

            for ((row, col), direction) in other.iter() {
                if !&acc.contains(&(*row, *col)) {
                    self.set_wall_by_cell(start.0, start.1, Some(direction.clone()), false);
                    acc = self.generate_maze_via_dfs((*row, *col), acc, width, height);
                }
            }
            acc
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::maze;

    #[test]
    fn test_shuffle() {
        let mut v = (0..10).collect::<Vec<usize>>();
        println!("{:?}", v);
        v = maze::Maze::shuffle(v);
        println!("{:?}", v);
    }
}
