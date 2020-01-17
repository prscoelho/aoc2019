/*  Repair droid
    First part: find shortest path between starting point and oxygen system

    1. BFS a path to closest unknown tile
    2. Walk to it
    2. Repeat 1 until there are no more unknown tiles
    3. BFS a path from start to goal tile

    Second part: How long does it take to fill the entire area with oxygen starting from oxygen system tile (1 minute per edge)
    1. Explore
    2. BFS max depth starting at goal
*/
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn to_intcode(&self) -> i64 {
        match self {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Tile {
    Wall,
    Empty,
    Goal,
}

impl Tile {
    fn parse(value: i64) -> Tile {
        match value {
            0 => Tile::Wall,
            1 => Tile::Empty,
            2 => Tile::Goal,
            _ => panic!("Unknown tile value."),
        }
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn new(x: i32, y: i32) -> Self {
        Coordinate { x, y }
    }
    fn movement(&self, m: Direction) -> Coordinate {
        let mut result = self.clone();
        match m {
            Direction::North => result.y -= 1,
            Direction::South => result.y += 1,
            Direction::West => result.x -= 1,
            Direction::East => result.x += 1,
        };
        result
    }

    fn calculate_direction(&self, to: Coordinate) -> Direction {
        if self.x < to.x {
            Direction::East
        } else if self.x > to.x {
            Direction::West
        } else if self.y < to.y {
            Direction::South
        } else if self.y > to.y {
            Direction::North
        } else {
            panic!("Coordinate::calculate_direction() called between two same coordinates.");
        }
    }
}

struct RepairDroid {
    direction: Direction,                    // direction robot is trying to move
    position: Coordinate,                    // current position
    known_tiles: BTreeMap<Coordinate, Tile>, // tiles which robot has knowledge about
    planned_path: VecDeque<Coordinate>,
    finished_exploring: bool,
}

impl RepairDroid {
    fn new() -> RepairDroid {
        let mut tiles = BTreeMap::new();
        let starting_pos = Coordinate::new(0, 0);
        tiles.insert(starting_pos, Tile::Empty);

        // no point planning a path on first movement with no information, just try to move north
        let mut planned_path = VecDeque::new();
        let next_pos = Coordinate::new(0, -1);
        planned_path.push_front(next_pos);

        RepairDroid {
            direction: Direction::North,
            position: starting_pos,
            known_tiles: tiles,
            planned_path,
            finished_exploring: false,
        }
    }

    fn goal(&self) -> Option<Coordinate> {
        self.known_tiles.iter().find_map(|(&coord, tile)| {
            if tile == &Tile::Goal {
                Some(coord)
            } else {
                None
            }
        })
    }

    fn move_result(&mut self, tile: Tile) {
        let pos = self.position.movement(self.direction);
        if tile != Tile::Wall {
            self.position = pos;
        }
        self.known_tiles.insert(pos, tile);

        // plan new path
        if self.planned_path.is_empty() {
            self.plan();
        }
    }

    fn plan(&mut self) {
        // search for first unknown
        if let Some(path) = bfs(&self.known_tiles, self.position, |_, tile_option| {
            tile_option == None
        }) {
            self.planned_path = path;
        } else {
            self.finished_exploring = true;
        }
    }

    fn next_direction(&mut self) -> Direction {
        self.direction = self
            .position
            .calculate_direction(self.planned_path.pop_front().unwrap());
        self.direction
    }

    fn path_to_goal(&self) -> Option<VecDeque<Coordinate>> {
        let start = Coordinate::new(0, 0);
        // search for goal tile
        bfs(&self.known_tiles, start, |_, tile_option| {
            tile_option == Some(&Tile::Goal)
        })
    }
}

// breadth-first search
fn bfs<F>(
    known_tiles: &BTreeMap<Coordinate, Tile>,
    start: Coordinate,
    objective: F,
) -> Option<VecDeque<Coordinate>>
where
    F: Fn(Coordinate, Option<&Tile>) -> bool,
{
    let directions = vec![
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];
    let mut positions = VecDeque::new();
    positions.push_back(start);
    // keep track of visited tiles
    let mut visited = HashSet::new();
    visited.insert(start);

    let mut came_from = HashMap::new();

    while let Some(current) = positions.pop_front() {
        let current_tile = known_tiles.get(&current);
        // if current matches objective, path to it
        if objective(current, current_tile) {
            return Some(reconstruct_path(came_from, current));
        }

        // if position is not a wall, generate its children
        if current_tile != Some(&Tile::Wall) {
            for &dir in directions.iter() {
                let neighbor = current.movement(dir);
                if visited.contains(&neighbor) {
                    continue;
                }
                visited.insert(neighbor);
                positions.push_back(neighbor);
                came_from.insert(neighbor, current);
            }
        }
    }
    None
}

// doesn't include start position in path
fn reconstruct_path(
    came_from: HashMap<Coordinate, Coordinate>,
    mut position: Coordinate,
) -> VecDeque<Coordinate> {
    let mut total_path = VecDeque::new();
    while let Some(&current) = came_from.get(&position) {
        total_path.push_front(position);
        position = current;
    }
    total_path
}

fn explore(intcode: &mut Intcode, robot: &mut RepairDroid) {
    while !intcode.finished && !robot.finished_exploring {
        intcode.input.push_back(robot.next_direction().to_intcode());
        intcode.run_until_output();
        robot.move_result(Tile::parse(intcode.output.pop_front().unwrap()));
    }
}

pub fn solve_first(input: &str) -> usize {
    let memory = read_codes(input);
    let mut intcode = Intcode::new(memory);
    let mut robot = RepairDroid::new();
    explore(&mut intcode, &mut robot);
    robot.path_to_goal().unwrap().len()
}

// should not be called with undiscovered tiles as it can only walk empty tiles
// finds max depth of bfs search starting from a position
fn bfs_depth(tiles: &BTreeMap<Coordinate, Tile>, start: Coordinate) -> usize {
    let mut visited = HashSet::new();
    visited.insert(start);
    let mut current_depth = VecDeque::new();
    current_depth.push_back(start);
    let mut next_depth = VecDeque::new();

    let mut depth = 0;

    let directions = vec![
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    while !current_depth.is_empty() {
        while let Some(current) = current_depth.pop_front() {
            for &dir in directions.iter() {
                let neighbor = current.movement(dir);
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    if tiles.get(&neighbor) == Some(&Tile::Empty) {
                        next_depth.push_back(neighbor);
                    }
                }
            }
        }
        depth += 1;
        std::mem::swap(&mut current_depth, &mut next_depth);
    }

    depth - 1
}

pub fn solve_second(input: &str) -> usize {
    let memory = read_codes(input);
    let mut intcode = Intcode::new(memory);
    let mut robot = RepairDroid::new();
    explore(&mut intcode, &mut robot);
    bfs_depth(&robot.known_tiles, robot.goal().unwrap())
}

fn read_codes(input: &str) -> Vec<i64> {
    let mut result = Vec::new();

    for number_str in input.trim().split(',') {
        result.push(number_str.parse().unwrap());
    }
    result
}

// returns (A, B, C, DE)
fn decode_op(code: i64) -> (ParameterMode, ParameterMode, ParameterMode, i64) {
    let de = code % 100;
    let c = ParameterMode::decode(code / 100 % 10);
    let b = ParameterMode::decode(code / 1000 % 10);
    let a = ParameterMode::decode(code / 10000 % 10);

    (a, b, c, de)
}

struct Intcode {
    memory: Vec<i64>,
    ptr: usize,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
    relative: i64,
    finished: bool,
}

enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl ParameterMode {
    fn decode(n: i64) -> ParameterMode {
        match n {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => panic!("Unexpected parameter mode"),
        }
    }
}

impl Intcode {
    fn new(memory: Vec<i64>) -> Self {
        Intcode {
            memory,
            ptr: 0,
            relative: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
            finished: false,
        }
    }

    fn load_value(&mut self, index: usize, mode: ParameterMode) -> i64 {
        match mode {
            ParameterMode::Position => {
                let position = self.read_memory(index) as usize;
                self.read_memory(position)
            }
            ParameterMode::Immediate => self.read_memory(index),
            ParameterMode::Relative => {
                let position = self.read_memory(index) + self.relative;
                self.read_memory(position as usize)
            }
        }
    }

    fn save_value(&mut self, index: usize, mode: ParameterMode, value: i64) {
        match mode {
            ParameterMode::Position => {
                let position = self.read_memory(index) as usize;
                self.write_memory(position, value);
            }
            ParameterMode::Immediate => {
                self.write_memory(index, value);
            }
            ParameterMode::Relative => {
                let position = self.read_memory(index) + self.relative;
                self.write_memory(position as usize, value);
            }
        };
    }

    fn read_memory(&mut self, index: usize) -> i64 {
        if self.memory.len() <= index {
            self.memory.resize(index + 1, 0);
        }
        self.memory[index]
    }

    fn write_memory(&mut self, index: usize, value: i64) {
        if self.memory.len() <= index {
            self.memory.resize(index + 1, 0);
        }
        self.memory[index] = value;
    }

    // returns instruction ran
    fn run_instruction(&mut self) -> i64 {
        if self.finished {
            return 99;
        }

        let pointer = self.ptr;
        let code = self.memory[pointer];
        let (arg3_mode, arg2_mode, arg1_mode, op) = decode_op(code);
        let next_pointer = match op {
            1 | 2 => {
                let value1 = self.load_value(pointer + 1, arg1_mode);
                let value2 = self.load_value(pointer + 2, arg2_mode);
                let operation_result = match op {
                    1 => value1 + value2,
                    2 => value1 * value2,
                    _ => 0,
                };
                self.save_value(pointer + 3, arg3_mode, operation_result);
                pointer + 4
            }
            3 => {
                let input_value = self.input.pop_front().expect("Could not get input.");
                self.save_value(pointer + 1, arg1_mode, input_value);
                pointer + 2
            }
            4 => {
                let v = self.load_value(pointer + 1, arg1_mode);
                self.output.push_back(v);
                pointer + 2
            }
            5 => {
                let par1 = self.load_value(pointer + 1, arg1_mode);
                let par2 = self.load_value(pointer + 2, arg2_mode) as usize;
                if par1 != 0 {
                    par2
                } else {
                    pointer + 3
                }
            }
            6 => {
                let par1 = self.load_value(pointer + 1, arg1_mode);
                let par2 = self.load_value(pointer + 2, arg2_mode) as usize;
                if par1 == 0 {
                    par2
                } else {
                    pointer + 3
                }
            }
            7 => {
                let par1 = self.load_value(pointer + 1, arg1_mode);
                let par2 = self.load_value(pointer + 2, arg2_mode);

                let store_value = if par1 < par2 { 1 } else { 0 };
                self.save_value(pointer + 3, arg3_mode, store_value);
                pointer + 4
            }
            8 => {
                let par1 = self.load_value(pointer + 1, arg1_mode);
                let par2 = self.load_value(pointer + 2, arg2_mode);

                let store_value = if par1 == par2 { 1 } else { 0 };
                self.save_value(pointer + 3, arg3_mode, store_value);
                pointer + 4
            }
            9 => {
                let par1 = self.load_value(pointer + 1, arg1_mode);
                self.relative += par1;
                pointer + 2
            }
            99 => {
                self.finished = true;
                pointer + 1
            }
            _ => pointer + 1,
        };
        self.ptr = next_pointer;
        op
    }

    fn run_until_output(&mut self) {
        while self.ptr < self.memory.len() {
            let op = self.run_instruction();
            if op == 99 || op == 4 {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bfs() {
        let mut tiles = BTreeMap::new();
        tiles.insert(Coordinate::new(0, -1), Tile::Wall);
        tiles.insert(Coordinate::new(-1, 0), Tile::Wall);
        tiles.insert(Coordinate::new(1, 0), Tile::Wall);

        tiles.insert(Coordinate::new(1, 1), Tile::Wall);
        tiles.insert(Coordinate::new(1, 2), Tile::Wall);

        let expected_path = vec![
            Coordinate::new(0, 1),
            Coordinate::new(0, 2),
            Coordinate::new(0, 3),
            Coordinate::new(1, 3),
            Coordinate::new(2, 3),
            Coordinate::new(2, 2),
            Coordinate::new(2, 1),
            Coordinate::new(2, 0),
        ];

        let result: Vec<Coordinate> = bfs(&tiles, Coordinate::new(0, 0), |c, _| {
            c == Coordinate::new(2, 0)
        })
        .unwrap()
        .into();

        assert_eq!(result, expected_path);
    }

    #[test]
    fn test_bfs_unknown() {
        // test path finding in a closed area. should return none
        let mut tiles = BTreeMap::new();
        tiles.insert(Coordinate::new(0, -1), Tile::Wall);
        tiles.insert(Coordinate::new(-1, 0), Tile::Wall);
        tiles.insert(Coordinate::new(1, 0), Tile::Wall);
        tiles.insert(Coordinate::new(0, 1), Tile::Wall);
        tiles.insert(Coordinate::new(0, 0), Tile::Empty);

        assert_eq!(
            bfs(&tiles, Coordinate::new(0, 0), |_, tile_option| tile_option
                == None),
            None
        );
    }

    #[test]
    fn test_bfs_depth() {
        // example from part 2
        let mut tiles = BTreeMap::new();
        // walls
        tiles.insert(Coordinate::new(1, 0), Tile::Wall);
        tiles.insert(Coordinate::new(2, 0), Tile::Wall);
        tiles.insert(Coordinate::new(3, 1), Tile::Wall);
        tiles.insert(Coordinate::new(4, 1), Tile::Wall);
        tiles.insert(Coordinate::new(5, 2), Tile::Wall);
        tiles.insert(Coordinate::new(4, 3), Tile::Wall);
        tiles.insert(Coordinate::new(3, 4), Tile::Wall);
        tiles.insert(Coordinate::new(2, 4), Tile::Wall);
        tiles.insert(Coordinate::new(1, 4), Tile::Wall);
        tiles.insert(Coordinate::new(0, 1), Tile::Wall);
        tiles.insert(Coordinate::new(0, 2), Tile::Wall);
        tiles.insert(Coordinate::new(0, 3), Tile::Wall);
        tiles.insert(Coordinate::new(2, 2), Tile::Wall);
        // empty tiles
        tiles.insert(Coordinate::new(2, 3), Tile::Empty);
        tiles.insert(Coordinate::new(1, 3), Tile::Empty);
        tiles.insert(Coordinate::new(1, 2), Tile::Empty);
        tiles.insert(Coordinate::new(1, 1), Tile::Empty);
        tiles.insert(Coordinate::new(2, 1), Tile::Empty);
        tiles.insert(Coordinate::new(3, 3), Tile::Empty);
        tiles.insert(Coordinate::new(3, 2), Tile::Empty);
        tiles.insert(Coordinate::new(4, 2), Tile::Empty);

        assert_eq!(bfs_depth(&tiles, Coordinate::new(2, 3)), 4);
    }

    #[test]
    fn test_first() {
        let input = include_str!("input");
        assert_eq!(solve_first(input), 294);
    }

    #[test]
    fn test_second() {
        let input = include_str!("input");
        assert_eq!(solve_second(input), 388);
    }
}
