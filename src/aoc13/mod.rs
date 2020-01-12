use std::collections::BTreeMap;

trait Bus {
    fn input(&self) -> i64;
    fn output(&mut self, v: i64);
}

#[derive(PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl Tile {
    fn id_to_tile(id: i64) -> Tile {
        match id {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => panic!("Invalid tile id."),
        }
    }
}

struct ArcadeCabinet {
    tiles: BTreeMap<(i32, i32), Tile>,
    output: Vec<i64>,
    score: i64,
}

impl ArcadeCabinet {
    fn new() -> Self {
        ArcadeCabinet {
            tiles: BTreeMap::new(),
            output: Vec::new(),
            score: 0,
        }
    }

    fn add_tile(&mut self, x: i32, y: i32, tile: Tile) {
        self.tiles.insert((x, y), tile);
    }
}

impl Bus for ArcadeCabinet {
    fn input(&self) -> i64 {
        // find paddle
        let paddle = self
            .tiles
            .iter()
            .find_map(|(k, t)| if *t == Tile::Paddle { Some(k.0) } else { None });

        let paddle_pos = paddle.expect("Could not find paddle in tile list.");

        // find ball
        let ball = self
            .tiles
            .iter()
            .find_map(|(k, t)| if *t == Tile::Ball { Some(k.0) } else { None });

        let ball_pos = ball.expect("Could not find ball in tile list.");

        // move paddle horizontally towards ball
        (ball_pos - paddle_pos).signum() as i64
    }

    fn output(&mut self, v: i64) {
        self.output.push(v);
        if self.output.len() == 3 {
            let tile_id = self.output.pop().unwrap();
            let y = self.output.pop().unwrap() as i32;
            let x = self.output.pop().unwrap() as i32;

            if x == -1 && y == 0 {
                self.score = tile_id;
            } else {
                let tile = Tile::id_to_tile(tile_id);
                self.add_tile(x, y, tile);
            }
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

pub fn solve_first(input: &str) -> usize {
    let memory = read_codes(input);
    let arcade = ArcadeCabinet::new();
    let mut intcode = Intcode::new(memory, arcade);
    intcode.run();
    intcode
        .bus
        .tiles
        .values()
        .filter(|t| **t == Tile::Block)
        .count()
}

pub fn solve_second(input: &str) -> i64 {
    let mut memory = read_codes(input);
    memory[0] = 2;
    let arcade = ArcadeCabinet::new();
    let mut intcode = Intcode::new(memory, arcade);
    intcode.run();
    intcode.bus.score
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_first() {
        let input = include_str!("input");
        assert_eq!(solve_first(input), 312);
    }

    #[test]
    fn test_second() {
        let input = include_str!("input");
        assert_eq!(solve_second(input), 15909);
    }
}
