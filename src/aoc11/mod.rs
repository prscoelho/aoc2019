/*  Hull painting robot
 *  Keeps track of its direction
 *  Exposes paint, detect and move method
 *  detect() - 0 if over a black panel, 1 if over a white panel
 *  paint(Black(0) | White(1)) - paints
 *  move(Left(0) | Right(1)) - rotates 90º and moves forward

 *  Intcode
 *  on input  - robot.detect()
 *  on output - First output    - |value| paint(value)
              - Second output   - |value| move(value)
*/
use std::collections::BTreeMap;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn new(x: i32, y: i32) -> Coordinate {
        Coordinate { x, y }
    }

    fn walk(&mut self, dir: Direction) {
        match dir {
            Direction::Left => {
                self.x -= 1;
            }
            Direction::Right => {
                self.x += 1;
            }
            Direction::Up => {
                self.y += 1;
            }
            Direction::Down => {
                self.y -= 1;
            }
        }
    }
}
#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn rotate_left(&self) -> Direction {
        match &self {
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
        }
    }
    fn rotate_right(&self) -> Direction {
        match &self {
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
        }
    }
}

struct PaintingRobot {
    board: BTreeMap<Coordinate, i64>,
    position: Coordinate,
    direction: Direction,
    paint_next: bool,
}

impl PaintingRobot {
    fn new() -> PaintingRobot {
        PaintingRobot {
            board: BTreeMap::new(),
            position: Coordinate::new(0, 0),
            direction: Direction::Up,
            paint_next: true,
        }
    }

    fn detect(&self) -> i64 {
        if let Some(v) = self.board.get(&self.position) {
            *v
        } else {
            0
        }
    }

    fn paint(&mut self, value: i64) {
        self.board.insert(self.position, value);
    }

    fn rotate_walk(&mut self, value: i64) {
        match value {
            0 => {
                self.direction = self.direction.rotate_left();
                self.position.walk(self.direction);
            }
            1 => {
                self.direction = self.direction.rotate_right();
                self.position.walk(self.direction);
            }
            _ => panic!("Unexpected walk value"),
        }
    }
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

trait Bus {
    fn input(&self) -> i64;
    fn output(&mut self, v: i64);
}

struct Intcode<T>
where
    T: Bus,
{
    memory: Vec<i64>,
    ptr: usize,
    bus: T,
    relative: i64,
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

impl<T> Intcode<T>
where
    T: Bus,
{
    fn new(memory: Vec<i64>, bus: T) -> Self {
        Intcode {
            memory,
            ptr: 0,
            bus,
            relative: 0,
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
                let input_value = self.bus.input();
                self.save_value(pointer + 1, arg1_mode, input_value);
                pointer + 2
            }
            4 => {
                let v = self.load_value(pointer + 1, arg1_mode);
                self.bus.output(v);
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
            _ => 1,
        };
        self.ptr = next_pointer;
        op
    }

    fn run(&mut self) {
        while self.ptr < self.memory.len() {
            let op = self.run_instruction();
            if op == 99 {
                break;
            }
        }
    }
}

impl Bus for PaintingRobot {
    fn input(&self) -> i64 {
        self.detect()
    }
    fn output(&mut self, v: i64) {
        if self.paint_next {
            self.paint(v);
        } else {
            self.rotate_walk(v);
        }
        self.paint_next = !self.paint_next;
    }
}

pub fn solve_first(input: &str) -> usize {
    let memory = read_codes(input);
    let robot = PaintingRobot::new();
    let mut intcode = Intcode::new(memory, robot);
    intcode.run();
    intcode.bus.board.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_walk() {
        let mut robot = PaintingRobot::new();
        // rotate left and walk
        robot.rotate_walk(0);
        assert_eq!(robot.position, Coordinate::new(-1, 0));
        // rotate right and walk
        robot.rotate_walk(1);
        assert_eq!(robot.position, Coordinate::new(-1, 1));
    }

    #[test]
    fn test_robot() {
        let mut robot = PaintingRobot::new();

        // paint 0,0
        robot.paint(0);
        assert_eq!(robot.board.len(), 1);
        robot.rotate_walk(0);

        // paint -1,0
        robot.paint(1);
        robot.rotate_walk(0);

        // paint -1, -1
        robot.paint(0);
        robot.rotate_walk(0);

        // paint 0, -1
        robot.paint(1);
        robot.rotate_walk(0);
        assert_eq!(robot.board.len(), 4);

        // repaint 0,0
        robot.paint(1);
        assert_eq!(robot.board.len(), 4);
    }

    #[test]
    fn test_first() {
        let input = include_str!("input");
        assert_eq!(solve_first(input), 2478);
    }
}
