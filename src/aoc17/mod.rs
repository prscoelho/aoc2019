use std::collections::{BTreeSet, VecDeque};

#[derive(PartialEq, Eq, Ord, PartialOrd, Clone, Copy, Debug)]
struct Coordinate {
    y: i32, // derive(PartialOrd) uses order here -- sort by y.then(x)
    x: i32,
}

impl Coordinate {
    fn new(x: i32, y: i32) -> Self {
        Coordinate { y, x }
    }

    fn next(&mut self) {
        self.x += 1;
    }

    fn next_newline(&mut self) {
        self.x = 0;
        self.y += 1;
    }

    fn neighbours(&self) -> Vec<Coordinate> {
        vec![
            Coordinate::new(self.x - 1, self.y),
            Coordinate::new(self.x, self.y - 1),
            Coordinate::new(self.x, self.y + 1),
            Coordinate::new(self.x + 1, self.y),
        ]
    }

    fn move_direction(&self, dir: &Direction) -> Coordinate {
        match dir {
            Direction::Left => Coordinate::new(self.x - 1, self.y),
            Direction::Right => Coordinate::new(self.x + 1, self.y),
            Direction::Up => Coordinate::new(self.x, self.y - 1),
            Direction::Down => Coordinate::new(self.x, self.y + 1),
        }
    }
}

// maps a board
struct AsciiBot {
    next: Coordinate,
    board: BTreeSet<Coordinate>,
    start: Coordinate,
    start_dir: Direction,
    s: String,
}
#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

struct Board {
    tiles: BTreeSet<Coordinate>,
    start_pos: Coordinate,
    start_dir: Direction,
}
impl Board {
    fn intersections(&self) -> Vec<Coordinate> {
        let mut result = Vec::new();

        for coord in self.tiles.iter() {
            let mut count = 0;
            for adjacent in coord.neighbours() {
                if self.tiles.contains(&adjacent) {
                    count += 1;
                }
            }
            if count == 4 {
                result.push(*coord);
            }
        }

        result
    }
}

impl AsciiBot {
    fn new() -> Self {
        AsciiBot {
            next: Coordinate::new(0, 0),
            board: BTreeSet::new(),
            start: Coordinate::new(0, 0),
            start_dir: Direction::Right,
            s: String::new(),
        }
    }

    fn next(&mut self, value: i64) {
        let value_char = std::char::from_u32(value as u32).unwrap();
        self.s.push(value_char);
        match value_char {
            '\n' => {
                self.next.next_newline();
            }
            '.' => {
                // empty
                self.next.next();
            }
            _ => {
                // '>' | '<' | '^' | 'v' | '#'
                self.board.insert(self.next);
                if value_char != '#' {
                    self.start = self.next;
                }
                self.next.next();
            }
        }

        match value_char {
            '>' => self.start_dir = Direction::Right,
            '<' => self.start_dir = Direction::Left,
            '^' => self.start_dir = Direction::Up,
            'v' => self.start_dir = Direction::Down,
            _ => {}
        }
    }

    fn get_board(self) -> Board {
        println!("{}", self.s);
        Board {
            tiles: self.board,
            start_pos: self.start,
            start_dir: self.start_dir,
        }
    }
}

fn read_ascii(input: &str) -> Board {
    let memory = read_codes(input);
    let mut intcode = Intcode::new(memory);
    let mut ascii = AsciiBot::new();
    intcode.run_until_output();
    while !intcode.finished {
        ascii.next(intcode.output.pop_front().unwrap());
        intcode.run_until_output();
    }
    ascii.get_board()
}

pub fn solve_first(input: &str) -> i32 {
    let board = read_ascii(input);
    let mut result = 0;
    for coord in board.intersections() {
        result += (coord.x * coord.y) as i32;
    }
    result
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Movement {
    Forward,
    Left,
    Right,
}

fn plan_path(board: Board) -> Vec<Movement> {
    let mut result = Vec::new();

    let mut current_pos = board.start_pos;
    let mut current_dir = board.start_dir;

    loop {
        let forward_pos = current_pos.move_direction(&current_dir);
        // move forward whenever possible
        if board.tiles.contains(&forward_pos) {
            result.push(Movement::Forward);
            current_pos = forward_pos;
        } else {
            let right_dir = current_dir.turn_right();
            let left_dir = current_dir.turn_left();

            let right_pos = current_pos.move_direction(&right_dir);
            let left_pos = current_pos.move_direction(&left_dir);

            // move right first if possible, move left only if moving right isn't possible
            if board.tiles.contains(&right_pos) {
                result.push(Movement::Right);
                result.push(Movement::Forward);

                current_pos = right_pos;
                current_dir = right_dir;
            } else if board.tiles.contains(&left_pos) {
                result.push(Movement::Left);
                result.push(Movement::Forward);

                current_pos = left_pos;
                current_dir = left_dir;
            } else {
                // end if we reach a dead end
                break;
            }
        }
    }
    result
}

fn compact_path(mut path: &str) -> String {
    let mut result = String::new();

    while !path.is_empty() {
        let (next_path, parsed) = parse_until_turn(path);
        path = next_path;
        result.push_str(&parsed);
    }
    result.pop();
    result
}

fn parse_until_turn(path: &str) -> (&str, String) {
    let mut count = 0;
    let mut length = 0;
    let mut turn = 'x';

    for c in path.chars() {
        length += 1;
        if c == 'F' {
            count += 1;
        } else {
            turn = c;
            break;
        }
    }

    let mut result = String::new();
    if count > 0 {
        result.push_str(&format!("{},", count));
    }
    if turn != 'x' {
        result.push_str(&format!("{},", turn));
    }
    (&path[length..], result)
}

fn path_to_string(path: &[Movement]) -> String {
    let mut result = String::new();
    for mov in path {
        match mov {
            Movement::Forward => result.push('F'),
            Movement::Left => result.push('L'),
            Movement::Right => result.push('R'),
        }
    }
    result
}

fn string_to_values(input: &str) -> VecDeque<i64> {
    let mut result = VecDeque::new();
    for c in input.chars() {
        result.push_back(c as i64);
    }
    result
}

pub fn solve_second(input: &str) -> i64 {
    let mut memory = read_codes(input);
    memory[0] = 2;
    let mut intcode = Intcode::new(memory);

    /*  Full path
        R,12,L,8,L,4,L,4,L,8,R,6,L,6,R,12,L,8,L,4,L,4,L,8,R,6,L,6,L,8,L,4,R,12,L,6,L,4,R,12,L,8,L,4,L,4,L,8,L,4,R,12,L,6,L,4,R,12,L,8,L,4,L,4,L,8,L,4,R,12,L,6,L,4,L,8,R,6,L,6,

        Solved by hand:
        main - "A,C,A,C,B,A,B,A,B,C"
        A    - "R,12,L,8,L,4,L,4"
        B    - "L,8,L,4,R,12,L,6,L,4"
        C    - "L,8,R,6,L,6"
    */
    let main = "A,C,A,C,B,A,B,A,B,C\n";
    let a = "R,12,L,8,L,4,L,4\n";
    let b = "L,8,L,4,R,12,L,6,L,4\n";
    let c = "L,8,R,6,L,6\n";
    let continuous = "n\n";

    intcode.input.append(&mut string_to_values(main));
    intcode.input.append(&mut string_to_values(a));
    intcode.input.append(&mut string_to_values(b));
    intcode.input.append(&mut string_to_values(c));
    intcode.input.append(&mut string_to_values(continuous));

    while !intcode.finished {
        intcode.run_until_output();
    }
    intcode.output.pop_back().unwrap()
}

fn board_from_string(input: &str) -> Board {
    let mut ascii = AsciiBot::new();
    for c in input.chars() {
        ascii.next(c as i64);
    }
    ascii.get_board()
}

// ------ INTCODE (same as day 15) -----------

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
    fn compact() {
        let input = "FFFFFR";
        let expected = "5,R";
        assert_eq!(compact_path(input), expected);
    }

    #[test]
    fn first() {
        let input = include_str!("input");
        let expected = 2788;
        assert_eq!(solve_first(input), expected);
    }

    #[test]
    fn second() {
        let input = include_str!("input");
        let expected = 761085;
        assert_eq!(solve_second(input), expected);
    }
}
